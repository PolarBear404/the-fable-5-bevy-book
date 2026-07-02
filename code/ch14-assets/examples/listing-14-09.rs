//! Listing 14-9：白纸黑字——AssetSaver 与 save_using_saver：把改定的剧本存回磁盘

use bevy::asset::io::{Reader, Writer};
use bevy::asset::saver::{save_using_saver, AssetSaver, SaveAssetError, SavedAsset};
use bevy::asset::{AssetLoader, AssetPath, LoadContext};
use bevy::prelude::*;
use bevy::tasks::futures::check_ready;
use bevy::tasks::futures_lite::AsyncWriteExt;
use bevy::tasks::{IoTaskPool, Task};
use thiserror::Error;

// —— Script 资产与装载器，与 Listing 14-7 相同（只多派生了 Clone）——

#[derive(Asset, TypePath, Debug, Clone)]
struct Script {
    title: String,
    lines: Vec<ScriptLine>,
}

#[derive(Debug, Clone)]
struct ScriptLine {
    speaker: String,
    text: String,
}

#[derive(Debug, Error)]
enum ScriptLoaderError {
    #[error("读不了剧本文件：{0}")]
    Io(#[from] std::io::Error),
    #[error("剧本不是 UTF-8 文本：{0}")]
    NotText(#[from] std::string::FromUtf8Error),
    #[error("第 {line} 行不是“角色：台词”的格式：{content:?}")]
    BadLine { line: usize, content: String },
}

#[derive(Default, TypePath)]
struct ScriptLoader;

impl AssetLoader for ScriptLoader {
    type Asset = Script;
    type Settings = ();
    type Error = ScriptLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Script, ScriptLoaderError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text = String::from_utf8(bytes)?;
        let mut title = String::from("无题");
        let mut lines = Vec::new();
        for (number, raw) in text.lines().enumerate() {
            let raw = raw.trim();
            if raw.is_empty() || raw.starts_with('#') {
                continue;
            }
            let Some((head, tail)) = raw.split_once('：') else {
                return Err(ScriptLoaderError::BadLine {
                    line: number + 1,
                    content: raw.to_string(),
                });
            };
            if head == "幕名" {
                title = tail.to_string();
            } else {
                lines.push(ScriptLine {
                    speaker: head.to_string(),
                    text: tail.to_string(),
                });
            }
        }
        Ok(Script { title, lines })
    }

    fn extensions(&self) -> &[&str] {
        &["script"]
    }
}

// ANCHOR: saver
/// 存档器：装载器的镜像——那边把字节读成 Script，这边把 Script 写回字节
#[derive(TypePath)]
struct ScriptSaver;

impl AssetSaver for ScriptSaver {
    type Asset = Script;
    type Settings = ();
    /// 格式契约的另一半：声明“存出去的文件由谁读回来”
    type OutputLoader = ScriptLoader;
    type Error = std::io::Error;

    async fn save(
        &self,
        writer: &mut Writer,
        asset: SavedAsset<'_, '_, Script>,
        _settings: &(),
        _path: AssetPath<'_>,
    ) -> Result<(), Self::Error> {
        // 逆着 ScriptLoader 的解析规则拼字节：幕名一行，台词一行一句
        let mut text = format!("幕名：{}\n\n", asset.title);
        for line in &asset.lines {
            text.push_str(&format!("{}：{}\n", line.speaker, line.text));
        }
        writer.write_all(text.as_bytes()).await?;
        // 成功时返回 OutputLoader 的设置（这里是 ()），一并写进 .meta 档案
        Ok(())
    }
}
// ANCHOR_END: saver

// ANCHOR: plumbing
/// 一稿的提货单——定稿之后就用不上了
#[derive(Resource)]
struct FirstDraft(Handle<Script>);

/// 后台存盘差事的回执：Task 完成时给出成败
#[derive(Resource)]
struct SaveJob(Task<Result<(), SaveAssetError>>);

/// 从磁盘读回来的定稿
#[derive(Resource)]
struct Readback(Handle<Script>);
// ANCHOR_END: plumbing

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.08, 0.1)))
        .init_asset::<Script>()
        .init_asset_loader::<ScriptLoader>()
        .add_systems(Startup, setup)
        .add_systems(Update, (finalize_draft, await_save, proof_read))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(FirstDraft(asset_server.load("scripts/opening.script")));
    println!("老雷：把一稿拿来，今天当场定稿。");
}

// ANCHOR: finalize
/// 一稿到货的那一帧：老雷当场改词，改完立刻发起存盘
fn finalize_draft(
    mut commands: Commands,
    draft: Option<Res<FirstDraft>>,
    mut scripts: ResMut<Assets<Script>>,
    asset_server: Res<AssetServer>,
) {
    let Some(draft) = draft else { return }; // 定过稿了，不再进来
    let Some(mut script) = scripts.get_mut(&draft.0) else {
        return; // 一稿还在路上
    };

    // 改的是货架上的资产本体——所有持这张单的地方立刻见到新词
    script.title = String::from("渡口夜话·定稿");
    if let Some(last) = script.lines.last_mut() {
        last.text = String::from("他会来。掌灯，今夜把戏排完。");
    }
    println!("老雷：幕名与末句就这么改。定了，白纸黑字存下来！");

    // SavedAsset 只借不夺：誊一份词本带进后台任务，货架上的原件不动
    let fair_copy = script.clone();
    let server = asset_server.clone();
    let task = IoTaskPool::get().spawn(async move {
        save_using_saver(
            server,
            &ScriptSaver,
            &"scripts/opening-final.script".into(),
            SavedAsset::from_asset(&fair_copy),
            &(),
        )
        .await
    });
    commands.insert_resource(SaveJob(task));
    commands.remove_resource::<FirstDraft>(); // 差事交出去了，一稿的单子销掉
}
// ANCHOR_END: finalize

// ANCHOR: await_save
/// 每帧问一声后台：写完了吗？写完了就把定稿从盘上读回来验货
fn await_save(
    mut commands: Commands,
    job: Option<ResMut<SaveJob>>,
    asset_server: Res<AssetServer>,
) {
    let Some(mut job) = job else { return };
    let Some(result) = check_ready(&mut job.0) else {
        return; // 还在写盘，下一帧再问
    };

    match result {
        Ok(()) => {
            println!("场务：存好了——scripts/opening-final.script，旁边还多了一份 .meta。");
            // 圆环合拢：用我们自己的装载器，把我们自己存的文件读回来
            commands.insert_resource(Readback(
                asset_server.load("scripts/opening-final.script"),
            ));
        }
        Err(err) => eprintln!("场务：存盘出了岔子——{err}"),
    }
    commands.remove_resource::<SaveJob>();
}
// ANCHOR_END: await_save

// ANCHOR: proof
/// 定稿从盘上到货的那一帧：报幕、验末句、收工
fn proof_read(
    mut commands: Commands,
    readback: Option<Res<Readback>>,
    scripts: Res<Assets<Script>>,
) {
    let Some(readback) = readback else { return };
    let Some(script) = scripts.get(&readback.0) else {
        return;
    };
    println!("场务：读回来了——《{}》，{} 句词。", script.title, script.lines.len());
    if let Some(last) = script.lines.last() {
        println!("场务：末句是“{}：{}”，一字不差。", last.speaker, last.text);
    }
    println!("老雷：白纸黑字，收工。明天开机就用这份。");
    commands.remove_resource::<Readback>();
}
// ANCHOR_END: proof

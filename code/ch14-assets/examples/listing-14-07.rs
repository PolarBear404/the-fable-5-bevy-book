//! Listing 14-7：剧本也是资产——自定义 Asset 类型与自定义文本格式的 AssetLoader

use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::prelude::*;
use thiserror::Error;

// ANCHOR: asset
/// 一幕剧本：幕名 + 一串台词。derive(Asset) 让它有资格住进 Assets<Script> 货架
#[derive(Asset, TypePath, Debug)]
struct Script {
    title: String,
    lines: Vec<ScriptLine>,
}

#[derive(Debug)]
struct ScriptLine {
    speaker: String,
    text: String,
}
// ANCHOR_END: asset

// ANCHOR: error
/// 装载剧本可能出的错：读不动文件、不是文本、词写得不像词
#[derive(Debug, Error)]
enum ScriptLoaderError {
    #[error("读不了剧本文件：{0}")]
    Io(#[from] std::io::Error),
    #[error("剧本不是 UTF-8 文本：{0}")]
    NotText(#[from] std::string::FromUtf8Error),
    #[error("第 {line} 行不是“角色：台词”的格式：{content:?}")]
    BadLine { line: usize, content: String },
}
// ANCHOR_END: error

// ANCHOR: loader
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
        // 第一步：把字节全部读进来（这里可以 .await——装载在后台任务里跑）
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text = String::from_utf8(bytes)?;

        // 第二步：逐行解析我们自己定的格式
        let mut title = String::from("无题");
        let mut lines = Vec::new();
        for (number, raw) in text.lines().enumerate() {
            let raw = raw.trim();
            if raw.is_empty() || raw.starts_with('#') {
                continue; // 空行与批注，跳过
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

    /// 认领扩展名：今后凡是 .script 文件都归本装载器
    fn extensions(&self) -> &[&str] {
        &["script"]
    }
}
// ANCHOR_END: loader

// ANCHOR: register
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_asset::<Script>() // 给 Script 上户口：建货架、开广播
        .init_asset_loader::<ScriptLoader>() // 让库房认识 .script 文件
        .add_systems(Startup, fetch_script)
        .add_systems(Update, recite)
        .run();
}
// ANCHOR_END: register

// ANCHOR: fetch
#[derive(Resource)]
struct Rehearsal {
    script: Handle<Script>,
    cursor: usize,
    next_at: f32,
}

fn fetch_script(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scripts: ResMut<Assets<Script>>,
) {
    commands.spawn(Camera2d);

    // 走库房：从文件加载，异步到货
    commands.insert_resource(Rehearsal {
        script: asset_server.load("scripts/opening.script"),
        cursor: 0,
        next_at: 0.0,
    });

    // 不走库房：手写的垫场词直接上架（Assets::add），当场就能取到
    let warmup = scripts.add(Script {
        title: String::from("垫场"),
        lines: vec![ScriptLine {
            speaker: String::from("老顾"),
            text: String::from("各组就位，剧本马上到。"),
        }],
    });
    let ready = scripts.get(&warmup).is_some();
    println!("老顾：手抄的垫场词直接上架，当场可取——{ready}。");
}
// ANCHOR_END: fetch

// ANCHOR: recite
/// 剧本到货后，每两秒念一句
fn recite(mut rehearsal: ResMut<Rehearsal>, scripts: Res<Assets<Script>>, time: Res<Time>) {
    // 货还没到就接着等——对 Assets<Script> 的 get，和对 Assets<Image> 的一模一样
    let Some(script) = scripts.get(&rehearsal.script) else {
        return;
    };

    if rehearsal.cursor == 0 && rehearsal.next_at == 0.0 {
        println!("老雷：剧本到了——《{}》，{} 句词。对词！", script.title, script.lines.len());
        rehearsal.next_at = time.elapsed_secs() + 1.0;
    }
    if rehearsal.cursor < script.lines.len() && time.elapsed_secs() >= rehearsal.next_at {
        let line = &script.lines[rehearsal.cursor];
        println!("{}：{}", line.speaker, line.text);
        rehearsal.cursor += 1;
        rehearsal.next_at += 2.0;
        if rehearsal.cursor == script.lines.len() {
            println!("老雷：第一稿就这个样，咔。");
        }
    }
}
// ANCHOR_END: recite

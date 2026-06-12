//! Listing 14-8：现场改词——file_watcher 热重载：剧本改了立即重念，贴图改了画面自己换

use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::prelude::*;
use thiserror::Error;

// —— Script 资产与装载器，与 Listing 14-7 相同 ——

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

#[derive(Resource)]
struct Rehearsal {
    script: Handle<Script>,
    cursor: usize,
    next_at: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.08, 0.1)))
        .init_asset::<Script>()
        .init_asset_loader::<ScriptLoader>()
        .add_systems(Startup, setup)
        .add_systems(Update, (recite, watch_rewrites))
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 台上摆一把青霜剑：等会儿换它的贴图文件，画面会自己变
    commands.spawn(Sprite::from_image(asset_server.load("props/qingshuang-sword.png")));

    commands.insert_resource(Rehearsal {
        script: asset_server.load("scripts/opening.script"),
        cursor: 0,
        next_at: 0.0,
    });
    println!("老雷：今天连排。秋白就在棚里，词随时会改，机器不许停。");
}
// ANCHOR_END: setup

// ANCHOR: watch
/// 盯着剧本的 Modified 广播：编剧一存盘，从头重念
fn watch_rewrites(
    mut broadcasts: MessageReader<AssetEvent<Script>>,
    mut rehearsal: ResMut<Rehearsal>,
) {
    for event in broadcasts.read() {
        if let AssetEvent::Modified { id } = event {
            if *id == rehearsal.script.id() {
                println!("场务：秋白改稿送到！从头对词——");
                rehearsal.cursor = 0;
                rehearsal.next_at = 0.0; // 归零，recite 会重新报幕
            }
        }
    }
}
// ANCHOR_END: watch

fn recite(mut rehearsal: ResMut<Rehearsal>, scripts: Res<Assets<Script>>, time: Res<Time>) {
    let Some(script) = scripts.get(&rehearsal.script) else {
        return;
    };
    if rehearsal.cursor == 0 && rehearsal.next_at == 0.0 {
        println!("老雷：《{}》，{} 句词。对词！", script.title, script.lines.len());
        rehearsal.next_at = time.elapsed_secs() + 1.0;
    }
    if rehearsal.cursor < script.lines.len() && time.elapsed_secs() >= rehearsal.next_at {
        let line = &script.lines[rehearsal.cursor];
        println!("{}：{}", line.speaker, line.text);
        rehearsal.cursor += 1;
        rehearsal.next_at += 2.0;
    }
}

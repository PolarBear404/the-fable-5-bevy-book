//! 《长风渡》开机日——Asset 系统总装：
//! 嵌入的场记板撑起加载画面，整本家当装货看进度，全齐开机，
//! 剧本是自定义资产，编剧现场改词热重载。

use bevy::asset::{embedded_asset, io::Reader, load_embedded_asset};
use bevy::asset::{AssetLoader, LoadContext, UntypedHandle};
use bevy::prelude::*;
use thiserror::Error;

// ====================================================== 剧本资产与装载器

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

// ====================================================== 片场的资产打包成 Plugin

// ANCHOR: plugin
/// 青蝉影视城的资产底座：自定义资产上户口 + 场记板缝进二进制
struct StudioAssetsPlugin;

impl Plugin for StudioAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Script>()
            .init_asset_loader::<ScriptLoader>();
        // main.rs 在 src/ 下，宏用默认前缀即可；文件在 src/embedded/clapper.png
        embedded_asset!(app, "embedded/clapper.png");
    }
}
// ANCHOR_END: plugin

// ====================================================== 状态与资源

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum SetupPhase {
    #[default]
    Loading,
    Rolling,
}

/// 整本戏的家当清单（不记类型的提货单）
#[derive(Resource)]
struct PropManifest {
    handles: Vec<UntypedHandle>,
}

#[derive(Resource)]
struct Rehearsal {
    script: Handle<Script>,
    cursor: usize,
    next_at: f32,
}

#[derive(Component)]
struct BarFill;

#[derive(Component)]
struct Clapper;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StudioAssetsPlugin))
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .init_state::<SetupPhase>()
        .add_systems(Startup, open_the_warehouse)
        .add_systems(
            Update,
            (track_progress, wiggle_clapper).run_if(in_state(SetupPhase::Loading)),
        )
        .add_systems(OnEnter(SetupPhase::Rolling), build_the_set)
        .add_systems(
            Update,
            (recite, watch_rewrites).run_if(in_state(SetupPhase::Rolling)),
        )
        .run();
}

// ====================================================== Loading：装货与进度

// ANCHOR: open
/// 开机日清早：开出整本戏的提货单，摆好加载画面
fn open_the_warehouse(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 八件家当：四张幕布、三件道具、一本剧本——类型不同，单子混开
    let mut handles: Vec<UntypedHandle> = [
        "backdrops/night-crossing.png",
        "backdrops/bamboo-sea.png",
        "backdrops/old-road.png",
        "backdrops/ferry-dock.png",
        "props/qingshuang-sword.png",
        "props/lantern.png",
        "props/changfeng-banner.png",
    ]
    .iter()
    .map(|path| asset_server.load::<Image>(*path).untyped())
    .collect();

    let script: Handle<Script> = asset_server.load("scripts/opening.script");
    handles.push(script.clone().untyped());
    commands.insert_resource(Rehearsal { script, cursor: 0, next_at: 0.0 });

    println!("老顾：《长风渡》开机日，家当 {} 件，单子全开出去了。", handles.len());
    commands.insert_resource(PropManifest { handles });

    // 加载画面三件套：场记板（嵌入资产，不占清单）、底槽、金条
    commands.spawn((
        Clapper,
        Sprite::from_image(load_embedded_asset!(&*asset_server, "embedded/clapper.png")),
        Transform::from_xyz(0.0, 120.0, 0.0),
        DespawnOnExit(SetupPhase::Loading),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.16, 0.19), Vec2::new(560.0, 26.0)),
        DespawnOnExit(SetupPhase::Loading),
    ));
    commands.spawn((
        BarFill,
        Sprite::from_color(Color::srgb(0.93, 0.74, 0.29), Vec2::new(0.0, 16.0)),
        Transform::from_xyz(-280.0, 0.0, 1.0),
        DespawnOnExit(SetupPhase::Loading),
    ));
}
// ANCHOR_END: open

// ANCHOR: track
/// 清点到货，推进金条；八件全齐，状态切到 Rolling
fn track_progress(
    manifest: Res<PropManifest>,
    asset_server: Res<AssetServer>,
    fill: Single<(&mut Sprite, &mut Transform), With<BarFill>>,
    mut next: ResMut<NextState<SetupPhase>>,
    time: Res<Time>,
    mut last_done: Local<usize>,
) {
    let total = manifest.handles.len();
    let done = manifest
        .handles
        .iter()
        .filter(|handle| asset_server.is_loaded_with_dependencies(handle.id()))
        .count();

    let frac = done as f32 / total as f32;
    let (mut sprite, mut transform) = fill.into_inner();
    sprite.custom_size = Some(Vec2::new(560.0 * frac, 16.0));
    transform.translation.x = -280.0 + 280.0 * frac;

    if done != *last_done {
        println!("老顾：到货 {done}/{total}。");
        *last_done = done;
    }
    // 全齐也不抢拍：加载画面至少亮足 1.6 秒——快机器上几帧装完，
    // 不加这道闸，观众连场记板都没看清就开机了
    if done == total && time.elapsed_secs() > 1.6 {
        println!("老雷：全齐。装台——开机！");
        next.set(SetupPhase::Rolling);
    }
}
// ANCHOR_END: track

/// 等货的工夫，场记板小幅摇晃——画面没死机
fn wiggle_clapper(clapper: Single<&mut Transform, With<Clapper>>, time: Res<Time>) {
    let mut transform = clapper.into_inner();
    transform.rotation = Quat::from_rotation_z((time.elapsed_secs() * 6.0).sin() * 0.08);
}

// ====================================================== Rolling：装台与连排

// ANCHOR: build
/// 装台：夜渡幕布、三件道具、两位演员——全部资产此刻都已在架
fn build_the_set(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("backdrops/night-crossing.png"),
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
    // 灯笼挂高处，长风旗立右侧
    commands.spawn((
        Sprite::from_image(asset_server.load("props/lantern.png")),
        Transform::from_xyz(-420.0, 150.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("props/changfeng-banner.png")),
        Transform::from_xyz(430.0, -100.0, 0.0),
    ));
    // 阿燕（红衣）持剑站渡口，梢公（蓑衣）候在一旁
    commands.spawn((
        Sprite::from_color(Color::srgb(0.82, 0.21, 0.2), Vec2::new(34.0, 52.0)),
        Transform::from_xyz(-60.0, -190.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("props/qingshuang-sword.png")),
        Transform::from_xyz(-10.0, -180.0, 1.0),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.45, 0.4, 0.3), Vec2::new(34.0, 46.0)),
        Transform::from_xyz(160.0, -200.0, 0.0),
    ));
    println!("场务：台装好了，各就各位。");
}
// ANCHOR_END: build

/// 连排：每两秒过一句词；词念尽，等编剧
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
        if rehearsal.cursor == script.lines.len() {
            println!("老雷：先这样。秋白要改词直接存盘，机器不停。");
        }
    }
}

// ANCHOR: rewrites
/// 热重载待命：剧本文件一变，从头重念；贴图文件一变，画面自己换
fn watch_rewrites(
    mut broadcasts: MessageReader<AssetEvent<Script>>,
    mut rehearsal: ResMut<Rehearsal>,
) {
    for event in broadcasts.read() {
        if let AssetEvent::Modified { id } = event {
            if *id == rehearsal.script.id() {
                println!("场务：秋白改稿送到！从头对词——");
                rehearsal.cursor = 0;
                rehearsal.next_at = 0.0;
            }
        }
    }
}
// ANCHOR_END: rewrites

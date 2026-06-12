//! Listing 14-6：开机进度条——整本戏的家当一次开单，全齐才开机

use bevy::asset::UntypedHandle;
use bevy::prelude::*;

// ANCHOR: states
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum SetupPhase {
    /// 装货中：片场只有一根进度条
    #[default]
    Loading,
    /// 开机：装台开拍
    Rolling,
}

/// 整本戏的提货单。UntypedHandle 不记资产类型——以后混进剧本、音频也是这张单
#[derive(Resource)]
struct PropManifest {
    handles: Vec<UntypedHandle>,
}

#[derive(Component)]
struct BarFill;
// ANCHOR_END: states

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .init_state::<SetupPhase>()
        .add_systems(Startup, start_loading)
        .add_systems(Update, track_progress.run_if(in_state(SetupPhase::Loading)))
        .add_systems(OnEnter(SetupPhase::Rolling), build_the_set)
        .run();
}

// ANCHOR: start
const PROP_LIST: [&str; 7] = [
    "backdrops/night-crossing.png",
    "backdrops/bamboo-sea.png",
    "backdrops/old-road.png",
    "backdrops/ferry-dock.png",
    "props/qingshuang-sword.png",
    "props/lantern.png",
    "props/changfeng-banner.png",
];

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 整张单子一次开完，handle 全部攥在手里
    let handles: Vec<UntypedHandle> = PROP_LIST
        .iter()
        .map(|path| asset_server.load::<Image>(*path).untyped())
        .collect();
    println!("老顾：《长风渡》整本戏的家当，{} 件，单子全开出去了。", handles.len());
    commands.insert_resource(PropManifest { handles });

    // 进度条：深色底槽 + 金色填充，整套行头只活在 Loading 状态里
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
// ANCHOR_END: start

// ANCHOR: track
/// 数到货：每帧清点清单，进度条跟着走，全齐切状态
fn track_progress(
    manifest: Res<PropManifest>,
    asset_server: Res<AssetServer>,
    fill: Single<(&mut Sprite, &mut Transform), With<BarFill>>,
    mut next: ResMut<NextState<SetupPhase>>,
    mut last_done: Local<usize>,
    mut frames: Local<u32>,
) {
    *frames += 1;
    let total = manifest.handles.len();
    let done = manifest
        .handles
        .iter()
        .filter(|handle| asset_server.is_loaded_with_dependencies(handle.id()))
        .count();

    // 金条从左往右长：宽度与中心点都按到货比例算
    let frac = done as f32 / total as f32;
    let (mut sprite, mut transform) = fill.into_inner();
    sprite.custom_size = Some(Vec2::new(560.0 * frac, 16.0));
    transform.translation.x = -280.0 + 280.0 * frac;

    if done != *last_done {
        println!("老顾：（第 {} 帧）到货 {done}/{total}。", *frames);
        *last_done = done;
    }
    if done == total {
        println!("老雷：全齐了？——装台，开机！");
        next.set(SetupPhase::Rolling);
    }
}
// ANCHOR_END: track

// ANCHOR: build
/// 装台：此刻所有道具都已在架，再 load 同一路径只是取回同一张提货单
fn build_the_set(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("backdrops/night-crossing.png"),
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
    for (path, x) in [
        ("props/lantern.png", -420.0),
        ("props/qingshuang-sword.png", 0.0),
        ("props/changfeng-banner.png", 420.0),
    ] {
        commands.spawn((
            Sprite::from_image(asset_server.load(path)),
            Transform::from_xyz(x, -160.0, 0.0),
        ));
    }
    println!("场务：夜渡幕布挂好，三件道具各就各位。");
}
// ANCHOR_END: build

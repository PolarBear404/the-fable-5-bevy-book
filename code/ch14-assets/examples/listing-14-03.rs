//! Listing 14-3：跑腿的状态牌——轮询 LoadState，等一张 8192×8192 的大幕布

use bevy::asset::LoadState;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.08, 0.1)))
        .add_systems(Startup, order_backdrop)
        .add_systems(Update, read_status_board)
        .run();
}

// ANCHOR: order
/// 幕布的提货单存进 Resource——总得有人攥着单子，货才不会被回收
#[derive(Resource)]
struct Backdrop(Handle<Image>);

fn order_backdrop(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    println!("老顾：夜渡的全景幕布，八千里江山一整匹，阿迅你跑一趟。");
    commands.insert_resource(Backdrop(asset_server.load("backdrops/night-crossing.png")));
}
// ANCHOR_END: order

// ANCHOR: poll
/// 每帧看一眼状态牌，牌子翻面才出声；到货就挂、销单收工
fn read_status_board(
    mut commands: Commands,
    backdrop: Option<Res<Backdrop>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut frames: Local<u32>,
    mut last: Local<String>,
) {
    // 单据已销，没活了
    let Some(backdrop) = backdrop else { return };

    *frames += 1;
    let state = asset_server.load_state(&backdrop.0);
    let board = match &state {
        LoadState::NotLoaded => "还没开单",
        LoadState::Loading => "在路上",
        LoadState::Loaded => "到货",
        LoadState::Failed(_) => "出事了",
    };
    if board != *last {
        println!(
            "阿迅：（翻状态牌）第 {} 帧，{:.2} 秒——{board}。",
            *frames,
            time.elapsed_secs()
        );
        *last = board.to_string();
    }

    if matches!(state, LoadState::Loaded) {
        // 到货即挂；Sprite 接过提货单，幕布缩成 680 点见方好看全貌
        commands.spawn(Sprite {
            image: backdrop.0.clone(),
            custom_size: Some(Vec2::splat(680.0)),
            ..default()
        });
        commands.remove_resource::<Backdrop>();
        println!("老顾：挂上！八千里江山，这一匹值回票价。");
    }
}
// ANCHOR_END: poll

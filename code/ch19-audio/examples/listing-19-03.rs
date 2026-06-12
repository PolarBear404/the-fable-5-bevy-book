//! Listing 19-3：一声锣 spawn 一个实体——能响，但散场没人拆台

use bevy::prelude::*;

#[derive(Resource)]
struct GongSound(Handle<AudioSource>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (strike_gong, stage_count))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    // 锣的提货单开一次、存进 Resource，每声锣克隆一份（第 14 章的取单方式）
    commands.insert_resource(GongSound(asset_server.load("sfx/gong.wav")));
    println!("鼓师：按 G 敲锣。场记盯着后台数家伙。");
}

// ANCHOR: strike
/// 每按一次 G，spawn 一个新的发声实体——默认 PlaybackSettings::ONCE
fn strike_gong(
    keyboard: Res<ButtonInput<KeyCode>>,
    gong: Res<GongSound>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        commands.spawn(AudioPlayer::new(gong.0.clone()));
        println!("鼓师：哐——");
    }
}

/// 场记的后台点数：带 AudioPlayer 的实体有几个？数目变了才报
fn stage_count(players: Query<Entity, With<AudioPlayer>>, mut last: Local<usize>) {
    let count = players.iter().count();
    if count != *last {
        println!("场记：后台躺着 {count} 个音频实体。");
        *last = count;
    }
}
// ANCHOR_END: strike

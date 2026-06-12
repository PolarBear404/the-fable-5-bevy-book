//! Listing 19-1：第一声——曲子是资产，播放是实体

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, strike_up)
        .run();
}

fn strike_up(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    // 发出一个声音 = spawn 一个带 AudioPlayer 的实体
    commands.spawn(AudioPlayer::new(
        asset_server.load("music/changfeng-overture.wav"),
    ));
    println!("琴师：曲谱递上去了——《长风渡》序曲，奏一遍。");
}

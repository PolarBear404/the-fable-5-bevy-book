//! Listing 15-1：第一张画上台——默认采样把像素画洗成了一团糊

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.13)))
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 原画只有 32×40 像素，放大 8 倍摆上舞台
    commands.spawn(Sprite {
        image: asset_server.load("actors/ayan-still.png"),
        custom_size: Some(Vec2::new(32.0, 40.0) * 8.0),
        ..default()
    });

    println!("小棠：画送上台了。咦——怎么糊成这样？");
}
// ANCHOR_END: setup

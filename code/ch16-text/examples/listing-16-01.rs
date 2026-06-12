//! Listing 16-1：第一行字——Text2d 把一行字放进 2D 世界

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Text2d::new("THE NIGHT FERRY"));

    println!("老雷：新到的字幕机，先拿说明书上的洋文试试。");
}
// ANCHOR_END: setup

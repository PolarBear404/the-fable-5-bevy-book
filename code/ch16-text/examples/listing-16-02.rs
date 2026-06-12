//! Listing 16-2：把秋白的词换上去——满屏豆腐块之坑

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

    // 洋文对照行：它能上屏，证明机器是好的
    commands.spawn((
        Text2d::new("THE NIGHT FERRY"),
        Transform::from_xyz(0.0, 60.0, 0.0),
    ));

    // 秋白的台词——上屏之后呢？
    commands.spawn((
        Text2d::new("夜渡无人，秋水自横。"),
        Transform::from_xyz(0.0, -40.0, 0.0),
    ));

    println!("秋白：第二幕头一句，就这十个字。");
}
// ANCHOR_END: setup

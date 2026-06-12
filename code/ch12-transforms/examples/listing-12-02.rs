//! Listing 12-2：z 决定谁画在前面
//! 2D 里 translation.z 不产生位移，只裁决遮挡：z 大的盖住 z 小的

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

    // 暗红大方块坐镇中央，z = 0.0
    commands.spawn((
        Sprite::from_color(Color::srgb(0.8, 0.25, 0.2), Vec2::splat(160.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 金色方块往右上偏一点，z = 1.0：压在红色上面
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.8, 0.25), Vec2::splat(100.0)),
        Transform::from_xyz(60.0, 40.0, 1.0),
    ));

    // 天蓝方块往左下偏一点，z = -1.0：被红色压住
    commands.spawn((
        Sprite::from_color(Color::srgb(0.35, 0.7, 0.95), Vec2::splat(100.0)),
        Transform::from_xyz(-60.0, -40.0, -1.0),
    ));
}
// ANCHOR_END: setup

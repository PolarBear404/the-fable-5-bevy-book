//! Listing 13-1：开机——青蝉影视城搭景，相机还是那行老咒语

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    // 1 号机：从第 2 章用到现在的那台相机
    commands.spawn(Camera2d);

    // 戏台地毯：全片场的地面
    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.13, 0.11), Vec2::new(1000.0, 560.0)),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    // 两排灯笼柱：跟拍时的静止参照物
    for i in -3..=3 {
        for y in [-240.0, 240.0] {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.95, 0.75, 0.25), Vec2::splat(22.0)),
                Transform::from_xyz(i as f32 * 160.0, y, -5.0),
            ));
        }
    }

    // 主演：侠客阿燕（红衣）与白马踏雪
    commands.spawn((
        Sprite::from_color(Color::srgb(0.85, 0.2, 0.2), Vec2::splat(30.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.92, 0.92, 0.95), Vec2::new(52.0, 30.0)),
        Transform::from_xyz(260.0, -130.0, 0.0),
    ));

    println!("老雷：《侠客行》A 组，开机！");
    println!("老雷：等等——天幕怎么是一片灰？哪位调的色？");
}
// ANCHOR_END: setup

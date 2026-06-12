//! Listing 13-2：换天幕——ClearColor 资源定全场底色

use bevy::prelude::*;

// ANCHOR: clear_color
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 夜戏的天幕：清屏色换成深蓝夜空
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.15)))
        .add_systems(Startup, setup)
        .run();
}
// ANCHOR_END: clear_color

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.13, 0.11), Vec2::new(1000.0, 560.0)),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
    for i in -3..=3 {
        for y in [-240.0, 240.0] {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.95, 0.75, 0.25), Vec2::splat(22.0)),
                Transform::from_xyz(i as f32 * 160.0, y, -5.0),
            ));
        }
    }
    commands.spawn((
        Sprite::from_color(Color::srgb(0.85, 0.2, 0.2), Vec2::splat(30.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.92, 0.92, 0.95), Vec2::new(52.0, 30.0)),
        Transform::from_xyz(260.0, -130.0, 0.0),
    ));

    println!("老雷：好——夜幕挂上了，今晚拍夜戏。");
}

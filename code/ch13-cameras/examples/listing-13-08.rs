//! Listing 13-8：两台全屏机位——order 相同，引擎每帧抗议

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.15)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // ANCHOR: two_cameras
    // 1 号机：架在片场中央
    commands.spawn(Camera2d);
    // 2 号机：架在片场东侧——order 没改，仍是默认的 0
    commands.spawn((Camera2d, Transform::from_xyz(300.0, 0.0, 0.0)));
    // ANCHOR_END: two_cameras

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

    println!("老雷：把 2 号机也架上，两台都给我拍全场！");
}

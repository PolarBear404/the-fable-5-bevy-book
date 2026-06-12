//! Listing 12-11：两块失格的转盘——亲眼看 B0004
//! 转盘 A 缺 Transform，转盘 B 缺 Visibility：一个让孩子瘫在原点，一个让孩子隐形

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

    // 转盘 A：有 Visibility，却忘了 Transform
    commands.spawn((
        Name::new("转盘A"),
        Visibility::default(),
        children![(
            Name::new("行星甲"),
            Sprite::from_color(Color::srgb(0.9, 0.4, 0.3), Vec2::splat(30.0)),
            Transform::from_xyz(150.0, 0.0, 0.0),
        )],
    ));

    // 转盘 B：有 Transform，却忘了 Visibility
    commands.spawn((
        Name::new("转盘B"),
        Transform::IDENTITY,
        children![(
            Name::new("行星乙"),
            Sprite::from_color(Color::srgb(0.3, 0.6, 0.9), Vec2::splat(30.0)),
            Transform::from_xyz(-150.0, 0.0, 0.0),
        )],
    ));
}
// ANCHOR_END: setup

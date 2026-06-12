//! Listing 12-5：行星上轨——rotate_around 绕点公转
//! 每帧绕世界原点转一小步，步子大小由各自的角速度决定

use bevy::prelude::*;

/// 行星：绕太阳公转的角速度（弧度/秒）
#[derive(Component)]
struct Orbit {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, orbit)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 太阳：这回安分待在原点，不转不动
    commands.spawn(Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(80.0)));

    // 水星：贴着太阳，跑得快
    commands.spawn((
        Orbit { speed: 1.2 },
        Sprite::from_color(Color::srgb(0.7, 0.65, 0.6), Vec2::splat(20.0)),
        Transform::from_xyz(120.0, 0.0, 0.0),
    ));

    // 地球：远一圈，慢一半
    commands.spawn((
        Orbit { speed: 0.5 },
        Sprite::from_color(Color::srgb(0.3, 0.55, 0.9), Vec2::splat(30.0)),
        Transform::from_xyz(240.0, 0.0, 0.0),
    ));
}
// ANCHOR_END: setup

// ANCHOR: orbit
/// 公转：绕世界原点（太阳所在处）转
fn orbit(time: Res<Time>, mut planets: Query<(&Orbit, &mut Transform)>) {
    for (orbit, mut transform) in &mut planets {
        let step = Quat::from_rotation_z(orbit.speed * time.delta_secs());
        transform.rotate_around(Vec3::ZERO, step);
    }
}
// ANCHOR_END: orbit

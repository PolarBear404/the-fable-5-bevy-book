//! Listing 12-4：太阳上岗——构造器链、自转与脉动
//! Transform 的两种改法：rotate_z 在现状上累加，scale 直接盖写

use bevy::prelude::*;

/// 标记：天文馆的太阳
#[derive(Component)]
struct Sun;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (spin_sun, pulse_sun))
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sun,
        Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(80.0)),
        // 构造器链：从位置出发，逐项补上旋转和缩放
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(0.3))
            .with_scale(Vec3::splat(1.2)),
    ));
}
// ANCHOR_END: setup

// ANCHOR: spin
/// 自转：每秒 0.6 弧度，十秒半转完一圈——增量式修改
fn spin_sun(time: Res<Time>, mut suns: Query<&mut Transform, With<Sun>>) {
    for mut transform in &mut suns {
        transform.rotate_z(0.6 * time.delta_secs());
    }
}
// ANCHOR_END: spin

// ANCHOR: pulse
/// 脉动：缩放在 1.0 ± 0.15 之间随正弦呼吸——赋值式修改
fn pulse_sun(time: Res<Time>, mut suns: Query<&mut Transform, With<Sun>>) {
    for mut transform in &mut suns {
        let breath = 1.0 + 0.15 * (time.elapsed_secs() * 2.0).sin();
        transform.scale = Vec3::splat(breath);
    }
}
// ANCHOR_END: pulse

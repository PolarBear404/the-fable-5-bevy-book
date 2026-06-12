//! Listing 12-8：月亮的难题——绕一个自己也在跑的地球
//! 能写出来，但处处别扭：跨查询打听位置、防借用冲突、还得排顺序

use bevy::prelude::*;

/// 标记：地球
#[derive(Component)]
struct Earth;

/// 标记：月亮
#[derive(Component)]
struct Moon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // 顺序敏感：月亮必须等地球挪完才知道绕哪儿转
        .add_systems(Update, (orbit_earth, orbit_moon).chain())
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 太阳
    commands.spawn(Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(80.0)));

    // 地球：在太阳右侧 240 处
    commands.spawn((
        Earth,
        Sprite::from_color(Color::srgb(0.3, 0.55, 0.9), Vec2::splat(30.0)),
        Transform::from_xyz(240.0, 0.0, 0.0),
    ));

    // 月亮：在地球右侧 55 处
    commands.spawn((
        Moon,
        Sprite::from_color(Color::srgb(0.8, 0.8, 0.78), Vec2::splat(12.0)),
        Transform::from_xyz(295.0, 0.0, 0.0),
    ));
}
// ANCHOR_END: setup

// ANCHOR: systems
/// 地球照旧绕太阳公转
fn orbit_earth(time: Res<Time>, mut earth: Single<&mut Transform, With<Earth>>) {
    let step = Quat::from_rotation_z(0.5 * time.delta_secs());
    earth.rotate_around(Vec3::ZERO, step);
}

/// 月亮想绕地球转——得先打听地球此刻在哪，还得用 Without 错开借用
fn orbit_moon(
    time: Res<Time>,
    earth: Single<&Transform, (With<Earth>, Without<Moon>)>,
    mut moon: Single<&mut Transform, With<Moon>>,
) {
    let step = Quat::from_rotation_z(2.0 * time.delta_secs());
    moon.rotate_around(earth.translation, step);
}
// ANCHOR_END: systems

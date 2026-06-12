//! Listing 12-9：转盘上的太阳系——把轨道交给父子树
//! 每个轨道是一个看不见的转盘；天体往盘上一坐，公转就是盘的自转

use bevy::prelude::*;

/// 匀速旋转：转盘和太阳通用，speed 为弧度/秒
#[derive(Component)]
struct Spin {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, spin)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 太阳：挂上 Spin，让它自转
    commands.spawn((
        Spin { speed: 0.6 },
        Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(80.0)),
    ));

    // 水星的轨道盘：没有 Sprite 的空实体，肉眼看不见；
    // Transform 撑位置链，Visibility 撑可见性链——两条继承链都不能中途断档
    commands.spawn((
        Spin { speed: 1.2 },
        Transform::IDENTITY,
        Visibility::default(),
        children![(
            Sprite::from_color(Color::srgb(0.7, 0.65, 0.6), Vec2::splat(20.0)),
            Transform::from_xyz(120.0, 0.0, 0.0),
        )],
    ));

    // 地球的轨道盘 → 地球 → 月亮的轨道盘 → 月亮，一串挂到底
    commands.spawn((
        Spin { speed: 0.5 },
        Transform::IDENTITY,
        Visibility::default(),
        children![(
            Sprite::from_color(Color::srgb(0.3, 0.55, 0.9), Vec2::splat(30.0)),
            Transform::from_xyz(240.0, 0.0, 0.0),
            children![(
                Spin { speed: 2.0 },
                Transform::IDENTITY,
                Visibility::default(),
                children![(
                    Sprite::from_color(Color::srgb(0.8, 0.8, 0.78), Vec2::splat(12.0)),
                    Transform::from_xyz(55.0, 0.0, 0.0),
                )],
            )],
        )],
    ));
}
// ANCHOR_END: setup

// ANCHOR: spin
/// 一个系统统转天下：有 Spin 就转，不问它是盘还是太阳
fn spin(time: Res<Time>, mut gears: Query<(&Spin, &mut Transform)>) {
    for (gear, mut transform) in &mut gears {
        transform.rotate_z(gear.speed * time.delta_secs());
    }
}
// ANCHOR_END: spin

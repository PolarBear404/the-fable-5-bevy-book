//! Listing 12-13：青石天文馆开馆——本章零件全装配
//! 转盘层级管公转，Spin 管旋转，彗星管朝向，观测站对照两本账

use bevy::prelude::*;

/// 匀速旋转：转盘与太阳通用（弧度/秒）
#[derive(Component)]
struct Spin {
    speed: f32,
}

/// 脉动：缩放随正弦呼吸
#[derive(Component)]
struct Pulse;

/// 标记：彗星
#[derive(Component)]
struct Comet;

/// 标记：月亮
#[derive(Component)]
struct Moon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (spin, pulse, fly_comet, observe))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 太阳：自转 + 脉动
    commands.spawn((
        Spin { speed: 0.6 },
        Pulse,
        Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(80.0)),
    ));

    // 水星的轨道盘 → 水星
    commands.spawn((
        Spin { speed: 1.2 },
        Transform::IDENTITY,
        Visibility::default(),
        children![(
            Sprite::from_color(Color::srgb(0.7, 0.65, 0.6), Vec2::splat(20.0)),
            Transform::from_xyz(120.0, 0.0, 0.0),
        )],
    ));

    // 地球的轨道盘 → 地球 → 月亮的轨道盘 → 月亮
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
                    Moon,
                    Sprite::from_color(Color::srgb(0.8, 0.8, 0.78), Vec2::splat(12.0)),
                    Transform::from_xyz(55.0, 0.0, 0.0),
                )],
            )],
        )],
    ));

    // 彗星：不进层级，自由飞行
    commands.spawn((
        Comet,
        Sprite::from_color(Color::srgb(0.85, 0.95, 1.0), Vec2::new(14.0, 36.0)),
        Transform::from_xyz(320.0, 0.0, 0.0),
    ));

    println!("老盖：天文馆开馆，诸星就位。");
}

/// 有 Spin 就转：太阳的自转、各轨道盘的公转，一视同仁
fn spin(time: Res<Time>, mut gears: Query<(&Spin, &mut Transform)>) {
    for (gear, mut transform) in &mut gears {
        transform.rotate_z(gear.speed * time.delta_secs());
    }
}

/// 太阳呼吸：缩放在 1.0 ± 0.15 之间起伏
fn pulse(time: Res<Time>, mut pulsers: Query<&mut Transform, With<Pulse>>) {
    for mut transform in &mut pulsers {
        transform.scale = Vec3::splat(1.0 + 0.15 * (time.elapsed_secs() * 2.0).sin());
    }
}

/// 彗星沿椭圆飞，机头追着运动方向
fn fly_comet(time: Res<Time>, mut comets: Query<&mut Transform, With<Comet>>) {
    let t = time.elapsed_secs() * 0.8;
    for mut transform in &mut comets {
        let next = Vec3::new(320.0 * t.cos(), 180.0 * t.sin(), 0.0);
        let motion = (next - transform.translation).truncate();
        transform.translation = next;
        if let Ok(heading) = Dir2::new(motion) {
            transform.rotation = Quat::from_rotation_arc(Vec3::Y, heading.extend(0.0));
        }
    }
}

/// 观测站：每两秒对照月亮的两本账；彗星出入取景框时记一笔
fn observe(
    time: Res<Time>,
    moon: Single<(&Transform, &GlobalTransform), With<Moon>>,
    comet: Single<&Transform, With<Comet>>,
    mut report_clock: Local<f32>,
    mut comet_in_frame: Local<bool>,
) {
    // 月亮的台账：籍册（局部）对实测（世界）
    *report_clock += time.delta_secs();
    if *report_clock >= 2.0 {
        *report_clock -= 2.0;
        let (local, global) = *moon;
        println!(
            "观测站：月亮 籍册 [{:.0}, {:.0}]，实测 [{:.0}, {:.0}]",
            local.translation.x,
            local.translation.y,
            global.translation().x,
            global.translation().y
        );
    }

    // 彗星的出入镜记录：状态翻转才开口
    let frame = Rect::from_center_size(Vec2::ZERO, Vec2::new(600.0, 360.0));
    let inside = frame.contains(comet.translation.truncate());
    if inside != *comet_in_frame {
        println!(
            "观测站：彗星{}镜，位于 [{:.0}, {:.0}]。",
            if inside { "入" } else { "出" },
            comet.translation.x,
            comet.translation.y
        );
        *comet_in_frame = inside;
    }
}

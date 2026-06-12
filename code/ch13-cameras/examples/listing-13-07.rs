//! Listing 13-7：推拉镜头——scale 切分镜，FixedVertical 锁纵向视野

use bevy::camera::ScalingMode;
use bevy::prelude::*;

/// 标记：侠客阿燕
#[derive(Component)]
struct Hero;

/// 分镜表：名称与对应的投影 scale
const SHOTS: [(&str, f32); 3] = [("远景", 1.5), ("中景", 1.0), ("特写", 0.6)];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.15)))
        .add_systems(Startup, setup)
        .add_systems(Update, ((walk_hero, follow_hero).chain(), cut_shots))
        .run();
}

fn setup(mut commands: Commands) {
    // ANCHOR: camera
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            // 锁定纵向视野 600 世界单位，横向随窗口比例配平
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 600.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    // ANCHOR_END: camera

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
        Hero,
        Sprite::from_color(Color::srgb(0.85, 0.2, 0.2), Vec2::splat(30.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    println!("老雷：按分镜表走——远景开场，三秒一切。");
}

fn walk_hero(time: Res<Time>, mut hero: Single<&mut Transform, With<Hero>>) {
    let t = time.elapsed_secs() * 0.5;
    hero.translation.x = 420.0 * t.sin();
    hero.translation.y = 210.0 * (2.0 * t).sin();
}

fn follow_hero(
    time: Res<Time>,
    mut lens: Single<&mut Transform, (With<Camera2d>, Without<Hero>)>,
    hero: Single<&Transform, With<Hero>>,
) {
    let target = hero.translation.with_z(lens.translation.z);
    lens.translation.smooth_nudge(&target, 2.0, time.delta_secs());
}

// ANCHOR: cut
/// 三秒切一镜：改投影的 scale，镜头应声推拉
fn cut_shots(
    time: Res<Time>,
    mut projection: Single<&mut Projection, With<Camera2d>>,
    mut shot: Local<usize>,
    mut clock: Local<f32>,
) {
    *clock += time.delta_secs();
    if *clock < 3.0 {
        return;
    }
    *clock -= 3.0;
    *shot = (*shot + 1) % SHOTS.len();

    let (name, scale) = SHOTS[*shot];
    if let Projection::Orthographic(ortho) = &mut **projection {
        ortho.scale = scale;
        println!("老雷：切{name}！（scale = {scale}）");
    }
}
// ANCHOR_END: cut

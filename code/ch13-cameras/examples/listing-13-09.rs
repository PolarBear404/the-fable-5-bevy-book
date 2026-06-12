//! Listing 13-9：导播台分屏——order 排资论辈，viewport 划分地盘

use bevy::camera::{ScalingMode, Viewport};
use bevy::prelude::*;
use bevy::window::WindowResized;

/// 标记：侠客阿燕
#[derive(Component)]
struct Hero;

/// 标记：白马踏雪
#[derive(Component)]
struct Horse;

/// 标记：左路机位（跟拍阿燕）
#[derive(Component)]
struct LeftLens;

/// 标记：右路机位（跟拍踏雪）
#[derive(Component)]
struct RightLens;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.15)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (walk_hero, run_horse, follow_hero, follow_horse).chain(),
                split_viewports,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // ANCHOR: cameras
    // 左路：order 0，先画
    commands.spawn((
        Camera2d,
        LeftLens,
        Camera {
            order: 0,
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 600.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    // 右路：order 1，后画
    commands.spawn((
        Camera2d,
        RightLens,
        Camera {
            order: 1,
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 600.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    // ANCHOR_END: cameras

    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.13, 0.11), Vec2::new(1400.0, 900.0)),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
    for i in -3..=3 {
        for y in [-350.0, 350.0] {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.95, 0.75, 0.25), Vec2::splat(22.0)),
                Transform::from_xyz(i as f32 * 200.0, y, -5.0),
            ));
        }
    }
    commands.spawn((
        Hero,
        Sprite::from_color(Color::srgb(0.85, 0.2, 0.2), Vec2::splat(30.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Horse,
        Sprite::from_color(Color::srgb(0.92, 0.92, 0.95), Vec2::new(52.0, 30.0)),
        Transform::from_xyz(420.0, 0.0, 0.0),
    ));

    println!("老雷：导播台分屏——左路盯阿燕，右路盯踏雪！");
}

// ANCHOR: viewports
/// 窗口尺寸一变（包括开窗那一下），重新给两路机位划地盘
fn split_viewports(
    windows: Query<&Window>,
    mut resized: MessageReader<WindowResized>,
    mut left: Single<&mut Camera, (With<LeftLens>, Without<RightLens>)>,
    mut right: Single<&mut Camera, (With<RightLens>, Without<LeftLens>)>,
) {
    for message in resized.read() {
        let Ok(window) = windows.get(message.window) else {
            continue;
        };
        // 视口用物理像素计量：左右各占半个窗口
        let half = window.physical_size() / UVec2::new(2, 1);
        left.viewport = Some(Viewport {
            physical_position: UVec2::ZERO,
            physical_size: half,
            ..default()
        });
        right.viewport = Some(Viewport {
            physical_position: UVec2::new(half.x, 0),
            physical_size: half,
            ..default()
        });
        println!("场记：分屏就位，每路 {} × {} 物理像素。", half.x, half.y);
    }
}
// ANCHOR_END: viewports

fn walk_hero(time: Res<Time>, mut hero: Single<&mut Transform, With<Hero>>) {
    let t = time.elapsed_secs() * 0.5;
    hero.translation.x = 500.0 * t.sin();
    hero.translation.y = 250.0 * (2.0 * t).sin();
}

/// 踏雪的走位：绕场一周又一周
fn run_horse(time: Res<Time>, mut horse: Single<&mut Transform, With<Horse>>) {
    let t = time.elapsed_secs() * 0.4;
    horse.translation.x = 420.0 * t.cos();
    horse.translation.y = 300.0 * t.sin();
}

fn follow_hero(
    time: Res<Time>,
    mut lens: Single<&mut Transform, (With<LeftLens>, Without<Hero>)>,
    hero: Single<&Transform, With<Hero>>,
) {
    let target = hero.translation.with_z(lens.translation.z);
    lens.translation.smooth_nudge(&target, 2.0, time.delta_secs());
}

fn follow_horse(
    time: Res<Time>,
    mut lens: Single<&mut Transform, (With<RightLens>, Without<Horse>)>,
    horse: Single<&Transform, With<Horse>>,
) {
    let target = horse.translation.with_z(lens.translation.z);
    lens.translation.smooth_nudge(&target, 2.0, time.delta_secs());
}

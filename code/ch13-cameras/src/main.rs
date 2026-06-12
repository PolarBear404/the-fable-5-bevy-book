//! Listing 13-12：杀青——青蝉影视城全机位联动
//! 左路跟拍阿燕，右路跟拍踏雪，右上角沙盘鸟瞰全场；
//! 马克点与图例住工作层，正式机位看不见；场记定时报实拍范围

use bevy::camera::visibility::RenderLayers;
use bevy::camera::{ScalingMode, Viewport};
use bevy::prelude::*;
use bevy::window::WindowResized;

/// 工作层：马克点、沙盘图例——正式机位看不见
const CREW_LAYER: usize = 1;

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

/// 标记：沙盘机位（小地图）
#[derive(Component)]
struct MinimapLens;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.15)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (walk_hero, run_horse, follow_hero, follow_horse).chain(),
                layout_viewports,
                report_frame,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // —— 三台机位 ——
    // 左路：跟拍阿燕，先画
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
    // 右路：跟拍踏雪
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
    // 沙盘：最后画，叠在最上层；看第 0 层与工作层
    commands.spawn((
        Camera2d,
        MinimapLens,
        Camera {
            order: 2,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.09, 0.08, 0.07)),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 1600.0,
                height: 1200.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        RenderLayers::from_layers(&[0, CREW_LAYER]),
    ));

    // —— 布景 ——
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
    // 马克点：走位记号，只在工作层
    for (x, y) in [(0.0, 0.0), (500.0, 0.0), (-500.0, 0.0), (354.0, 250.0), (-354.0, -250.0)] {
        commands.spawn((
            Sprite::from_color(Color::srgb(1.0, 0.3, 0.7), Vec2::splat(16.0)),
            Transform::from_xyz(x, y, -4.0),
            RenderLayers::layer(CREW_LAYER),
        ));
    }

    // —— 主演与沙盘图例（图例在工作层，RenderLayers 不沿层级继承）——
    commands.spawn((
        Hero,
        Sprite::from_color(Color::srgb(0.85, 0.2, 0.2), Vec2::splat(30.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        children![(
            Sprite::from_color(Color::srgb(1.0, 0.4, 0.4), Vec2::splat(72.0)),
            Transform::from_xyz(0.0, 0.0, 5.0),
            RenderLayers::layer(CREW_LAYER),
        )],
    ));
    commands.spawn((
        Horse,
        Sprite::from_color(Color::srgb(0.92, 0.92, 0.95), Vec2::new(52.0, 30.0)),
        Transform::from_xyz(420.0, 0.0, 0.0),
        children![(
            Sprite::from_color(Color::srgb(0.95, 0.95, 1.0), Vec2::splat(72.0)),
            Transform::from_xyz(0.0, 0.0, 5.0),
            RenderLayers::layer(CREW_LAYER),
        )],
    ));

    println!("老雷：《侠客行》最后一场——全机位联动，开机！");
}

/// 走位：阿燕满场跑“8”字
fn walk_hero(time: Res<Time>, mut hero: Single<&mut Transform, With<Hero>>) {
    let t = time.elapsed_secs() * 0.5;
    hero.translation.x = 500.0 * t.sin();
    hero.translation.y = 250.0 * (2.0 * t).sin();
}

/// 走位：踏雪绕场奔驰
fn run_horse(time: Res<Time>, mut horse: Single<&mut Transform, With<Horse>>) {
    let t = time.elapsed_secs() * 0.4;
    horse.translation.x = 420.0 * t.cos();
    horse.translation.y = 300.0 * t.sin();
}

/// 左路跟拍：平滑追向阿燕
fn follow_hero(
    time: Res<Time>,
    mut lens: Single<&mut Transform, (With<LeftLens>, Without<Hero>)>,
    hero: Single<&Transform, With<Hero>>,
) {
    let target = hero.translation.with_z(lens.translation.z);
    lens.translation.smooth_nudge(&target, 2.0, time.delta_secs());
}

/// 右路跟拍：平滑追向踏雪
fn follow_horse(
    time: Res<Time>,
    mut lens: Single<&mut Transform, (With<RightLens>, Without<Horse>)>,
    horse: Single<&Transform, With<Horse>>,
) {
    let target = horse.translation.with_z(lens.translation.z);
    lens.translation.smooth_nudge(&target, 2.0, time.delta_secs());
}

/// 导播台排版：左右分屏 + 右上角沙盘，窗口一变全部重排
fn layout_viewports(
    windows: Query<&Window>,
    mut resized: MessageReader<WindowResized>,
    mut left: Single<&mut Camera, (With<LeftLens>, Without<RightLens>, Without<MinimapLens>)>,
    mut right: Single<&mut Camera, (With<RightLens>, Without<LeftLens>, Without<MinimapLens>)>,
    mut minimap: Single<&mut Camera, (With<MinimapLens>, Without<LeftLens>, Without<RightLens>)>,
) {
    for message in resized.read() {
        let Ok(window) = windows.get(message.window) else {
            continue;
        };
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
        let width = window.physical_width() / 4;
        let size = UVec2::new(width, width * 3 / 4);
        minimap.viewport = Some(Viewport {
            physical_position: UVec2::new(window.physical_width() - size.x - 16, 16),
            physical_size: size,
            ..default()
        });
        println!("场记：导播台重排——每路 {} × {}，沙盘 {} × {}。", half.x, half.y, size.x, size.y);
    }
}

/// 场记：每四秒报一次左路的实拍范围
fn report_frame(
    time: Res<Time>,
    lens: Single<(&GlobalTransform, &Projection), With<LeftLens>>,
    mut clock: Local<f32>,
) {
    *clock += time.delta_secs();
    if *clock < 4.0 {
        return;
    }
    *clock -= 4.0;
    let (lens_pos, projection) = *lens;
    let Projection::Orthographic(ortho) = projection else {
        return;
    };
    let center = lens_pos.translation().truncate();
    println!(
        "场记：左路实拍 x [{:.0}, {:.0}]，y [{:.0}, {:.0}]。",
        ortho.area.min.x + center.x,
        ortho.area.max.x + center.x,
        ortho.area.min.y + center.y,
        ortho.area.max.y + center.y
    );
}

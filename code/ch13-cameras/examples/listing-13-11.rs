//! Listing 13-11：特效棚验机——同一条灯笼柱走廊，左监正交、右监透视

use bevy::camera::{ScalingMode, Viewport};
use bevy::prelude::*;
use bevy::window::WindowResized;

/// 标记：左监（正交机位）
#[derive(Component)]
struct OrthoLens;

/// 标记：右监（透视机位）
#[derive(Component)]
struct PerspectiveLens;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.15)))
        .add_systems(Startup, setup)
        .add_systems(Update, split_viewports)
        .run();
}

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 走廊：地面一条，灯笼柱两列纵深排开
    // Mesh3d 与 StandardMaterial 是第 21 章的主角，今天先借来当模特
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(16.0, 64.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.22, 0.26, 0.22))),
    ));
    for i in 0..8 {
        for x in [-3.0, 3.0] {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 3.0, 1.0))),
                MeshMaterial3d(materials.add(Color::srgb(0.95, 0.75, 0.25))),
                Transform::from_xyz(x, 1.5, -4.0 * i as f32),
            ));
        }
    }
    // 棚顶大灯
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(8.0, 16.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 两台机位架在同一点、朝同一方向，只差投影
    let stand = Transform::from_xyz(0.0, 6.0, 14.0).looking_at(Vec3::new(0.0, 1.0, -8.0), Vec3::Y);

    // 左监：正交投影——平行线永远平行，远近一样大
    commands.spawn((
        Camera3d::default(),
        OrthoLens,
        stand,
        Camera {
            order: 0,
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 14.0,
            },
            ..OrthographicProjection::default_3d()
        }),
    ));
    // 右监：透视投影——Camera3d 的默认，纵向视角 45°
    commands.spawn((
        Camera3d::default(),
        PerspectiveLens,
        stand,
        Camera {
            order: 1,
            ..default()
        },
    ));

    println!("老雷：特效棚验机——左监平行机位，右监透视机位，自己看哪边有纵深。");
}
// ANCHOR_END: setup

/// 左右分屏：与 2D 导播台同一套手艺
fn split_viewports(
    windows: Query<&Window>,
    mut resized: MessageReader<WindowResized>,
    mut left: Single<&mut Camera, (With<OrthoLens>, Without<PerspectiveLens>)>,
    mut right: Single<&mut Camera, (With<PerspectiveLens>, Without<OrthoLens>)>,
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
    }
}

//! Listing 22-6：追光——一盏聚光灯，圆锥形光束，方向由旋转决定

use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.02, 0.03, 0.06)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 6.0, 12.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    // ANCHOR: spot
    // 追光：从高处斜罩台中央的绣球。圆锥有两道边——
    // inner_angle 以内全亮，到 outer_angle 渐隐为零；range 同点光，是射程
    commands.spawn((
        SpotLight {
            color: Color::srgb(1.0, 0.95, 0.85),
            intensity: 2_000_000.0,
            range: 20.0,
            inner_angle: PI / 12.0,
            outer_angle: PI / 7.0,
            shadows_enabled: true,
            ..default()
        },
        // 把光锥从 (0, 8, 4) 指向台中央绣球
        Transform::from_xyz(0.0, 8.0, 4.0).looking_at(Vec3::new(0.0, 1.0, 1.5), Vec3::Y),
    ));
    // ANCHOR_END: spot

    spawn_courtyard(&mut commands, &mut meshes, &mut materials);
}

fn spawn_courtyard(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.33, 0.32),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    let lacquer = materials.add(StandardMaterial {
        base_color: Color::srgb(0.52, 0.16, 0.12),
        perceptual_roughness: 0.7,
        ..default()
    });
    for x in [-3.5, 3.5] {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.6, 1.6, 1.6))),
            MeshMaterial3d(lacquer.clone()),
            Transform::from_xyz(x, 0.8, -1.0),
        ));
    }
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.4, 5.0))),
        MeshMaterial3d(lacquer.clone()),
        Transform::from_xyz(0.0, 2.5, -3.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.82, 0.74, 0.62),
            perceptual_roughness: 0.85,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 1.5),
    ));
}

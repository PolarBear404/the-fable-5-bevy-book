//! Listing 22-5：夜里的灯笼——一盏点光源，强度用「流明」，越远越暗

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 入夜：天黑下来，只靠灯笼照明
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

    // ANCHOR: lantern
    // 灯笼：点光源从一个点向四面八方发光。强度是「流明」（总光通量），
    // range 是它够得着的最远距离，超出之外不再费电
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.82, 0.55),
            intensity: 600_000.0,
            range: 14.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 1.5),
    ));
    // ANCHOR_END: lantern

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

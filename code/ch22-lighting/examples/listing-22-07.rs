//! Listing 22-7：环境光——给影子里补一层没有方向的天光

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.02, 0.03, 0.06)))
        // ANCHOR: ambient
        // GlobalAmbientLight 是个资源，引擎默认就插了一个（白色、亮度 80）。
        // 这里把它换成一层偏蓝的弱光，模拟入夜后从天幕漫下来的月色——
        // 它没有方向，每张面领到的一样多，专门用来托一托纯黑的暗部
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(0.6, 0.7, 1.0),
            brightness: 220.0,
            ..default()
        })
        // ANCHOR_END: ambient
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
    // 一盏偏暗的灯笼，好让环境光的补色看得分明
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.82, 0.55),
            intensity: 300_000.0,
            range: 14.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 1.5),
    ));

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

//! Listing 22-10：晨雾——给相机挂一层 DistanceFog，远处的东西按距离隐入雾里

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.74, 0.78, 0.80)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: fog
    // 雾是相机的属性：东西本身不变，是「看出去」这件事蒙了一层。
    // 挂在相机实体上，按到镜头的距离把颜色往雾色里调
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 14.0).looking_at(Vec3::new(0.0, 1.0, -6.0), Vec3::Y),
        DistanceFog {
            color: Color::srgb(0.78, 0.82, 0.85),
            falloff: FogFalloff::ExponentialSquared { density: 0.045 },
            ..default()
        },
    ));
    // ANCHOR_END: fog

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.3, -0.7, -0.6), Vec3::Y),
    ));

    // 一排朝远处退去的灯笼柱，好让雾显出纵深
    let stone = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.20, 0.16),
        perceptual_roughness: 0.8,
        ..default()
    });
    let pillar = meshes.add(Cylinder::new(0.5, 6.0));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(80.0, 80.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.32, 0.35, 0.34),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    for i in 0..8 {
        let z = -(i as f32) * 5.0;
        for x in [-4.0, 4.0] {
            commands.spawn((
                Mesh3d(pillar.clone()),
                MeshMaterial3d(stone.clone()),
                Transform::from_xyz(x, 3.0, z),
            ));
        }
    }
}

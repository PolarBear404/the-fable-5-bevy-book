//! Listing 22-3：影子的通病——把 bias 调成 0，自阴影噪点（shadow acne）爬满台面

use bevy::{light::DirectionalLightShadowMap, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.52, 0.66, 0.82)))
        // 把阴影贴图调小，让噪点更扎眼
        .insert_resource(DirectionalLightShadowMap { size: 1024 })
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

    // ANCHOR: acne
    // 把两个 bias 都按到 0：影子贴图的分辨率误差再没有缓冲，
    // 受光面会把自己误判成「在影子里」，于是爬满细密的暗纹
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::CLEAR_SUNRISE,
            color: Color::srgb(1.0, 0.95, 0.85),
            shadows_enabled: true,
            shadow_depth_bias: 0.0,
            shadow_normal_bias: 0.0,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.4, -0.8, -0.5), Vec3::Y),
    ));
    // ANCHOR_END: acne

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

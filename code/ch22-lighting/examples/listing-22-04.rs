//! Listing 22-4：调影子的三把旋钮——阴影贴图分辨率、级联、bias

use bevy::{
    light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.52, 0.66, 0.82)))
        // 旋钮一：阴影贴图分辨率。默认 2048，翻到 4096 边缘更利落（也更吃显存）
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
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

    // ANCHOR: tuned
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::CLEAR_SUNRISE,
            color: Color::srgb(1.0, 0.95, 0.85),
            shadows_enabled: true,
            // 旋钮三：bias。默认值（0.02 / 1.8）对大多数场景够用，这里写出来看清它在哪
            shadow_depth_bias: 0.02,
            shadow_normal_bias: 1.8,
            ..default()
        },
        // 旋钮二：级联。近处切得密、远处切得疏，让有限的贴图集中在镜头跟前
        CascadeShadowConfigBuilder {
            num_cascades: 4,
            maximum_distance: 40.0,
            ..default()
        }
        .build(),
        Transform::default().looking_to(Vec3::new(-0.4, -0.8, -0.5), Vec3::Y),
    ));
    // ANCHOR_END: tuned

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

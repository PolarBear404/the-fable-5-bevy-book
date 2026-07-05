//! Listing 23-2：亲眼看报错——忘了 #Scene0，把一整箱塞进装场景的口袋

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.5, 1.4, 2.6).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 4.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(6.0, 6.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.30, 0.26),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // ANCHOR: wrong
    // 少写了标签：整个 .gltf 是一只「箱子」，不是一件「场景」
    commands.spawn(WorldAssetRoot(asset_server.load("models/afu/afu.gltf")));
    // ANCHOR_END: wrong
}

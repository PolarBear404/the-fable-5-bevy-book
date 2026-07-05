//! Listing 23-4：只提一件——把 HeadMesh 当素坯，老鲁翻模上自家的釉

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
        Transform::from_xyz(0.0, 0.9, 2.2).looking_at(Vec3::new(0.0, 0.25, 0.0), Vec3::Y),
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

    // ANCHOR: primitive
    // 只提一件：Mesh0/Primitive0 是一张不折不扣的 Bevy Mesh——
    // 用枚举拼标签，拼错编译器当场拦
    let head: Handle<Mesh> = asset_server.load(
        GltfAssetLabel::Primitive { mesh: 0, primitive: 0 }.from_asset("models/afu/afu.gltf"),
    );
    // 同一句话的字符串写法：路径后面拿 # 接标签，拼错要到运行时才见分晓
    let head_again: Handle<Mesh> = asset_server.load("models/afu/afu.gltf#Mesh0/Primitive0");

    // 素坯归素坯，釉归釉：网格是阿福的头形，材质是老鲁自己的两罐釉
    commands.spawn((
        Mesh3d(head),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.62, 0.75, 0.68), // 青瓷
            perceptual_roughness: 0.25,
            ..default()
        })),
        Transform::from_xyz(-0.35, 0.16, 0.0),
    ));
    commands.spawn((
        Mesh3d(head_again),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.24, 0.20, 0.30), // 乌釉
            perceptual_roughness: 0.25,
            ..default()
        })),
        Transform::from_xyz(0.35, 0.16, 0.0),
    ));
    println!("老鲁：借你家头形翻两个坯，一青一乌——脸就不必给我了。");
    // ANCHOR_END: primitive
}

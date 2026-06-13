//! Listing 23-1：请角儿上台——用 SceneRoot 加载一份 glTF 场景

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.52, 0.55, 0.62)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: load
    // glTF 文件是个「集装箱」，一份文件里能装好几样东西。要摆上台的是其中的「场景」：
    // GltfAssetLabel::Scene(0) 点名第 0 号场景，SceneRoot 一挂，加载好就自动展开成实体。
    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("models/puppet.gltf"),
    )));
    // ANCHOR_END: load

    stage(&mut commands, &mut meshes, &mut materials);
}

// 台子：地面、主光、机位——都是第 21、22 章的老相识，放在锚点外不抢戏
fn stage(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    commands.insert_resource(GlobalAmbientLight {
        brightness: 200.0,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.30, 0.33),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.5, -0.9, -0.4), Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::new(0.0, 1.9, 0.0), Vec3::Y),
    ));
}

//! Listing 23-6：收箱新规——load_builder 定制开箱；同一箱开两回，规矩不同结果不同

use bevy::{gltf::GltfLoaderSettings, prelude::*};

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
        Transform::from_xyz(0.0, 1.7, 3.4).looking_at(Vec3::new(0.0, 0.15, 0.0), Vec3::Y),
    ));
    // 园子的灯故意压暗：作坊那盏摊位灯亮没亮，一眼便知
    commands.spawn((
        DirectionalLight {
            illuminance: 400.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 4.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.30, 0.26),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // ANCHOR: settings
    // 左手边开三件套：机位谢绝（重影看过一回就够了），作坊的摊位灯照请
    let bench_with_lamp = asset_server
        .load_builder()
        .with_settings(|s: &mut GltfLoaderSettings| {
            s.load_cameras = false;
        })
        .load(GltfAssetLabel::Scene(1).from_asset("models/afu/afu.gltf"));
    commands.spawn((
        WorldAssetRoot(bench_with_lamp),
        Transform::from_xyz(-1.2, 0.0, 0.0),
    ));

    // 右手边开单件箱：灯也谢绝。注意开的是 afu.glb——
    // 同一路径只认一套开箱规矩，要两种开法就得是两个路径（正文细说）
    let bench_plain = asset_server
        .load_builder()
        .with_settings(|s: &mut GltfLoaderSettings| {
            s.load_cameras = false;
            s.load_lights = false;
        })
        .load(GltfAssetLabel::Scene(1).from_asset("models/afu.glb"));
    commands.spawn((
        WorldAssetRoot(bench_plain),
        Transform::from_xyz(1.2, 0.0, 0.0),
    ));
    // ANCHOR_END: settings
    println!("老雷：左手照作坊的意思亮灯，右手全按园子的来——一样的货，两种开法。");
}

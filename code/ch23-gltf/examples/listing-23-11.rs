//! Listing 23-11：借漆——#Material1/std 直取现成的 StandardMaterial，给老鲁的木人穿

use bevy::{gltf::GltfMaterial, prelude::*};

/// 账房手里的第一本账：引擎无关的 GltfMaterial
#[derive(Resource)]
struct PaintBook(Handle<GltfMaterial>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, read_book)
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
        Transform::from_xyz(1.6, 1.5, 2.8).looking_at(Vec3::new(0.4, 0.55, 0.0), Vec3::Y),
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

    // 阿福照常上台，当个对照
    commands.spawn(WorldAssetRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/afu/afu.gltf")),
    ));

    // ANCHOR: borrow
    // 第二本账：#Material1/std——bevy_pbr 替你换算好的 StandardMaterial 成品漆
    let robe_paint: Handle<StandardMaterial> =
        asset_server.load("models/afu/afu.gltf#Material1/std");
    // 老鲁的胶囊木人，直接穿阿福的袍漆
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.22, 0.55))),
        MeshMaterial3d(robe_paint),
        Transform::from_xyz(1.0, 0.495, 0.0),
    ));

    // 第一本账：#Material1——引擎无关的 GltfMaterial，念两行给大家听
    commands.insert_resource(PaintBook(
        asset_server.load("models/afu/afu.gltf#Material1"),
    ));
    // ANCHOR_END: borrow
    println!("老鲁：借你家漆一罐——我这木人也穿件红袍，看看正不正。");
}

// ANCHOR: read_book
/// 账房：第一本账到手就念——上面记的是 glTF 原始口径的参数
fn read_book(book: Res<PaintBook>, ledgers: Res<Assets<GltfMaterial>>, mut done: Local<bool>) {
    if *done {
        return;
    }
    let Some(ledger) = ledgers.get(&book.0) else {
        return;
    };
    *done = true;
    println!(
        "账房：AfuRobe 的原账——base_color {:?}，roughness {}，metallic {}。",
        ledger.base_color, ledger.perceptual_roughness, ledger.metallic
    );
}
// ANCHOR_END: read_book

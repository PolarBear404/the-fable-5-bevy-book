//! Listing 23-5：换场——named_scenes 一换，台上从主角变成工作台

use bevy::{gltf::Gltf, prelude::*};

#[derive(Resource)]
struct Shipment(Handle<Gltf>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, swap_scene)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    // 开幕先上主角场；同时把整箱目录也留一份，换场要用
    commands.spawn(WorldAssetRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/afu/afu.gltf")),
    ));
    commands.insert_resource(Shipment(asset_server.load("models/afu/afu.gltf")));
    println!("老雷：开幕是 AfuShow。按空格，后台工作台搬上来给大家开开眼。");
}

// ANCHOR: swap
/// 老雷：空格换场。改的是同一个 WorldAssetRoot 组件——
/// 值一变，旧场自动拆、新场自动搭
fn swap_scene(
    keyboard: Res<ButtonInput<KeyCode>>,
    shipment: Res<Shipment>,
    gltfs: Res<Assets<Gltf>>,
    mut root: Single<&mut WorldAssetRoot>,
    mut on_bench: Local<bool>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    let Some(gltf) = gltfs.get(&shipment.0) else {
        return;
    };
    *on_bench = !*on_bench;
    let name = if *on_bench { "Workbench" } else { "AfuShow" };
    root.0 = gltf.named_scenes[name].clone();
    println!("老雷：换场——{name}！");
}
// ANCHOR_END: swap

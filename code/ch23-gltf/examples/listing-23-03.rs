//! Listing 23-3：装箱单——把 Gltf 总资产整箱抬进来，逐格清点

use bevy::{gltf::Gltf, prelude::*};

// ANCHOR: shipment
/// 到货登记：整箱 glTF 的提货单
#[derive(Resource)]
struct Shipment(Handle<Gltf>);
// ANCHOR_END: shipment

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, inspect)
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

    // ANCHOR: load_whole
    // 不带标签：请的不是哪一件，而是整箱的「目录」——Handle<Gltf>
    commands.insert_resource(Shipment(asset_server.load("models/afu/afu.gltf")));
    // ANCHOR_END: load_whole
}

// ANCHOR: inspect
/// 把 named_* 表的键取出来排个序——HashMap 自己不记顺序，报单要念得稳
fn roll_call<'a>(names: impl Iterator<Item = &'a Box<str>>) -> Vec<&'a str> {
    let mut names: Vec<&str> = names.map(|n| n.as_ref()).collect();
    names.sort();
    names
}

/// 验货员：箱子一到就开箱清点，点完把主角请上台
fn inspect(
    mut commands: Commands,
    shipment: Res<Shipment>,
    gltfs: Res<Assets<Gltf>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    // 还在路上就下帧再来（第 14 章的老规矩：Handle 只是提货单）
    let Some(gltf) = gltfs.get(&shipment.0) else {
        return;
    };
    *done = true;

    println!("验货员：巧手斋的箱子到了，开箱清点——");
    println!("验货员：场景 {} 出：{:?}", gltf.scenes.len(), roll_call(gltf.named_scenes.keys()));
    println!("验货员：节点 {} 处：{:?}", gltf.nodes.len(), roll_call(gltf.named_nodes.keys()));
    println!("验货员：网格 {} 件：{:?}", gltf.meshes.len(), roll_call(gltf.named_meshes.keys()));
    println!("验货员：材质 {} 罐：{:?}", gltf.materials.len(), roll_call(gltf.named_materials.keys()));
    println!("验货员：动画 {} 折：{:?}", gltf.animations.len(), roll_call(gltf.named_animations.keys()));

    // 出厂时钦定的默认场景（afu.gltf 里的 "scene": 0 写的就是它）
    let stage_scene = gltf.default_scene.clone().expect("箱单上没写默认场景");
    println!("验货员：货净单清，请主角上台。");
    commands.spawn(WorldAssetRoot(stage_scene));
}
// ANCHOR_END: inspect

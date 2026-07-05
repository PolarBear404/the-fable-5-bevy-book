//! Listing 23-7：借机位——相机谢绝入园，但作坊留的构图坐标照借不误

use bevy::{
    gltf::{GltfLoaderSettings, GltfNode},
    prelude::*,
};

const AFU: &str = "models/afu/afu.gltf";

/// 两个机位的座次表：作坊参考机位的提货单 + 眼下坐在哪
#[derive(Resource)]
struct SeatPlan {
    makers_seat: Handle<GltfNode>,
    on_makers_seat: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, swap_seat)
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
        house_seat(),
    ));
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

    // ANCHOR: borrow_seat
    // 工作台照 23-6 的规矩开箱：相机不请、摊位灯照点
    let bench = asset_server
        .load_builder()
        .with_settings(|s: &mut GltfLoaderSettings| {
            s.load_cameras = false;
        })
        .load(GltfAssetLabel::Scene(1).from_asset(AFU));
    commands.spawn(WorldAssetRoot(bench));

    // 相机没进园子，但节点还在箱里：MakerCam 是装箱单上的 7 号节点，
    // 提出来当 GltfNode 用——里面躺着作坊摆机位时的 Transform。
    // 注意 settings 与上面那次装载一字不差：同一条路径，规矩必须统一（本节末细说）
    let makers_seat = asset_server
        .load_builder()
        .with_settings(|s: &mut GltfLoaderSettings| {
            s.load_cameras = false;
        })
        .load(GltfAssetLabel::Node(7).from_asset(AFU));
    commands.insert_resource(SeatPlan {
        makers_seat,
        on_makers_seat: false,
    });
    // ANCHOR_END: borrow_seat
    println!("老雷：空格换机位——园子的看全景，作坊的看他们钦定的构图。");
}

/// 园子自己的机位：正面偏高，看全景
fn house_seat() -> Transform {
    Transform::from_xyz(0.0, 1.7, 3.4).looking_at(Vec3::new(0.0, 0.15, 0.0), Vec3::Y)
}

// ANCHOR: swap_seat
/// 空格在两个机位间换座：作坊那把椅子的坐标，从 GltfNode 资产里读
fn swap_seat(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut plan: ResMut<SeatPlan>,
    nodes: Res<Assets<GltfNode>>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    // 节点资产也是异步到货的（14 章老规矩）
    let Some(node) = nodes.get(&plan.makers_seat) else {
        return;
    };
    plan.on_makers_seat = !plan.on_makers_seat;
    if plan.on_makers_seat {
        // GltfNode::transform 是节点在 glTF 里的原始摆位
        **camera = node.transform;
        println!("老雷：坐上作坊的 {}——他们留的构图。", node.name);
    } else {
        **camera = house_seat();
        println!("老雷：回园子自己的机位。");
    }
}
// ANCHOR_END: swap_seat

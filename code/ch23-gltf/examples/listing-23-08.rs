//! Listing 23-8：到货回执——WorldInstanceReady 一响，把实体树点一遍名

use bevy::{prelude::*, world_serialization::WorldInstanceReady};

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

    // ANCHOR: observe
    // 挂场景，再在同一个实体上挂 observer：搭完台自然有人来送回执
    commands
        .spawn(WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/afu/afu.gltf")),
        ))
        .observe(print_receipt);
    // ANCHOR_END: observe
}

// ANCHOR: receipt
/// 跟包：收到回执就沿树点名——名字是 glTF 里带来的，播放器是装卸工添的
fn print_receipt(
    ready: On<WorldInstanceReady>,
    children: Query<&Children>,
    names: Query<&Name>,
    players: Query<Has<AnimationPlayer>>,
) {
    println!("跟包：回执到，实体 {} 名下的场子搭好了——", ready.entity);
    print_tree(ready.entity, 0, &children, &names, &players);
}

/// 从 entity 起递归打印：缩进两格一层
fn print_tree(
    entity: Entity,
    depth: usize,
    children: &Query<&Children>,
    names: &Query<&Name>,
    players: &Query<Has<AnimationPlayer>>,
) {
    let name = names
        .get(entity)
        .map(|n| n.as_str().to_owned())
        .unwrap_or_else(|_| format!("(无名 {entity})"));
    let badge = if players.get(entity) == Ok(true) {
        "　←带 AnimationPlayer"
    } else {
        ""
    };
    println!("{:indent$}{name}{badge}", "", indent = depth * 2);
    if let Ok(kids) = children.get(entity) {
        for &kid in kids {
            print_tree(kid, depth + 1, children, names, players);
        }
    }
}
// ANCHOR_END: receipt

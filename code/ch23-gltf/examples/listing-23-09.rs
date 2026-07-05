//! Listing 23-9：按名找人——去左袖挂一盏灯笼，货单 extras 里早写好了留灯位

use bevy::{gltf::GltfExtras, prelude::*, world_serialization::WorldInstanceReady};

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

    commands
        .spawn(WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/afu/afu.gltf")),
        ))
        .observe(hang_lantern);
}

// ANCHOR: hang
/// 跟包：拿着货单沿树找「LeftArm」，找到就把灯笼挂上去
fn hang_lantern(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    nodes: Query<(&Name, Option<&GltfExtras>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in children.iter_descendants(ready.entity) {
        let Ok((name, extras)) = nodes.get(entity) else {
            continue;
        };
        if name.as_str() != "LeftArm" {
            continue;
        }
        // 货单 extras 是作坊随手写的 JSON 原文，Bevy 一字不动递给你——
        // 连人家的缩进换行都在，念之前先把空白捋平
        if let Some(extras) = extras {
            let note = extras.value.split_whitespace().collect::<Vec<_>>().join(" ");
            println!("跟包：LeftArm 找到了，货单注着 {note}——挂灯。");
        }
        // 灯笼挂在袖口下方：红纱罩 + 一点暖光，跟着袖子走
        commands.entity(entity).with_child((
            Mesh3d(meshes.add(Sphere::new(0.055))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.9, 0.25, 0.15),
                emissive: LinearRgba::new(4.0, 0.9, 0.4, 1.0),
                ..default()
            })),
            PointLight {
                color: Color::srgb(1.0, 0.6, 0.35),
                intensity: 3_000.0,
                range: 3.0,
                ..default()
            },
            Transform::from_xyz(0.0, -0.50, 0.0),
        ));
    }
}
// ANCHOR_END: hang

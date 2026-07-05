//! Listing 23-10：换漆——GltfMaterialName 对上号，一罐漆管三件衣裳

use bevy::{gltf::GltfMaterialName, prelude::*, world_serialization::WorldInstanceReady};

// ANCHOR: paint_pot
/// 漆匠的家什：袍漆那罐的提货单，外带记住出厂色好改回去
#[derive(Resource)]
struct RobePaint {
    handle: Handle<StandardMaterial>,
    factory_color: Color,
    moon_white: bool,
}
// ANCHOR_END: paint_pot

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, repaint)
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
        .observe(find_robe_paint);
}

// ANCHOR: find
/// 漆匠：沿树数一遍，谁的漆罐上写着「AfuRobe」
fn find_robe_paint(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    painted: Query<(&MeshMaterial3d<StandardMaterial>, &GltfMaterialName)>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let mut pot: Option<Handle<StandardMaterial>> = None;
    let mut pieces = 0;
    for entity in children.iter_descendants(ready.entity) {
        let Ok((material, name)) = painted.get(entity) else {
            continue;
        };
        if name.0 == "AfuRobe" {
            pieces += 1;
            pot = Some(material.0.clone());
        }
    }
    let Some(handle) = pot else {
        return;
    };
    let factory_color = materials.get(&handle).unwrap().base_color;
    println!("漆匠：数完了——挂着 AfuRobe 号漆的一共 {pieces} 件，用的是同一罐。");
    println!("漆匠：空格上月白，再按回朱红。");
    commands.insert_resource(RobePaint {
        handle,
        factory_color,
        moon_white: false,
    });
}
// ANCHOR_END: find

// ANCHOR: repaint
/// 空格换色：改的是资产本身——所有用这罐漆的网格一起变
fn repaint(
    keyboard: Res<ButtonInput<KeyCode>>,
    paint: Option<ResMut<RobePaint>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(mut paint) = paint else {
        return;
    };
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    paint.moon_white = !paint.moon_white;
    let mut pot = materials.get_mut(&paint.handle).unwrap();
    if paint.moon_white {
        pot.base_color = Color::srgb(0.72, 0.80, 0.83);
        println!("漆匠：上月白——一罐漆动一下，袍和双袖三件一起变。");
    } else {
        pot.base_color = paint.factory_color;
        println!("漆匠：回朱红——出厂那罐色我留着底呢。");
    }
}
// ANCHOR_END: repaint

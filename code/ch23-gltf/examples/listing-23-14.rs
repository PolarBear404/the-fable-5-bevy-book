//! Listing 23-14：走两步——glTF 的「前」是 +Z，Bevy 的「前」是 −Z，转不转正当场见

use bevy::{
    gltf::{convert_coordinates::GltfConvertCoordinates, GltfLoaderSettings},
    prelude::*,
};

const AFU: &str = "models/afu/afu.gltf";

/// 走台的人：名字念给观众听
#[derive(Component)]
struct Walker(&'static str);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, walk)
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
        Transform::from_xyz(0.0, 1.7, 4.4).looking_at(Vec3::new(0.0, 0.7, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
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

    // ANCHOR: two_boxes
    // 甲：原样开箱——模型的脸还朝着 glTF 的「前」（+Z）
    commands.spawn((
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(AFU))),
        Transform::from_xyz(-0.9, 0.0, 0.0),
        Walker("甲（原样）"),
    ));
    // 乙：开箱时把场景根拧半圈，脸对齐 Bevy 的「前」（−Z）。
    // 开的是 .glb 那份拷贝——同一路径只认一套开箱规矩（23.5）
    let converted = asset_server
        .load_builder()
        .with_settings(|s: &mut GltfLoaderSettings| {
            s.convert_coordinates = Some(GltfConvertCoordinates {
                rotate_scene_entity: true,
                rotate_meshes: false,
            });
        })
        .load(GltfAssetLabel::Scene(0).from_asset("models/afu.glb"));
    commands.spawn((
        WorldAssetRoot(converted),
        Transform::from_xyz(0.9, 0.0, 0.0),
        Walker("乙（转正）"),
    ));
    // ANCHOR_END: two_boxes
    println!("老雷：甲乙各就位。空格喊一步——都朝自己实体的 forward() 走。");
}

// ANCHOR: walk
/// 空格喊步：游戏逻辑只认自己实体的 forward()——脸跟不跟得上，就看开箱转没转
fn walk(keyboard: Res<ButtonInput<KeyCode>>, mut walkers: Query<(&mut Transform, &Walker)>) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    for (mut transform, who) in &mut walkers {
        let step = transform.forward() * 0.5;
        transform.translation += step;
        println!(
            "老雷：{}走一步，站到 z = {:.1}。",
            who.0, transform.translation.z
        );
    }
}
// ANCHOR_END: walk

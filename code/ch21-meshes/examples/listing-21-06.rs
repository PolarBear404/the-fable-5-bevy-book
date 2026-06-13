//! Listing 21-6：贴上班旗——base_color_texture 初见

use bevy::prelude::*;

/// 标记：转台上的展品
#[derive(Component)]
struct Showpiece;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, turn)
        .run();
}

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 雷字班旗，一张普通的 PNG——第 14 章的提货单流程
    let banner: Handle<Image> = asset_server.load("textures/banner.png");

    // 原样一只：base_color 保持默认白色，贴图照原样上身
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(banner.clone()),
            ..default()
        })),
        Transform::from_xyz(-2.2, 1.1, 0.0),
        Showpiece,
    ));

    // 染色一只：base_color 当染料乘上来——第 15 章 Sprite 的同款规则
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.45, 0.65, 1.0),
            base_color_texture: Some(banner.clone()),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.1, 0.0),
        Showpiece,
    ));

    // 球也吃同一张图：内置图元都自带贴图坐标——它从哪来，下一节见分晓
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.85))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(banner),
            ..default()
        })),
        Transform::from_xyz(2.2, 1.1, 0.0),
        Showpiece,
    ));
    // ANCHOR_END: setup

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 8.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.33, 0.36, 0.34))),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.6, 5.5).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(2.5, 5.0, 4.0)));

    println!("小棠：一张雷字旗，箱笼贴两只，绣球裹一只——正面怎么是倒的？");
}

fn turn(mut pieces: Query<&mut Transform, With<Showpiece>>, time: Res<Time>) {
    for mut transform in &mut pieces {
        transform.rotate_y(time.delta_secs() * 0.4);
    }
}

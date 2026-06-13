//! Listing 21-8：旗子齐活——法线、贴图坐标补上，转着看正反面

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

/// 标记：旗面
#[derive(Component)]
struct Banner;

// ANCHOR: build
/// 一面 1.4 × 1.4 的方旗：位置、法线、贴图坐标、三角形，四样齐全
fn banner_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.7, 0.7, 0.0],  // 0 左上
            [0.7, 0.7, 0.0],   // 1 右上
            [0.7, -0.7, 0.0],  // 2 右下
            [-0.7, -0.7, 0.0], // 3 左下
        ],
    )
    // 法线：旗面朝 +Z，四个顶点说的是同一个方向
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    )
    // 贴图坐标：u 朝右、v 朝下，(0, 0) 是贴图的左上角
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
    )
    .with_inserted_indices(Indices::U32(vec![
        0, 3, 2, // 左上 → 左下 → 右下
        0, 2, 1, // 左上 → 右下 → 右上
    ]))
}
// ANCHOR_END: build

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, turn)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.4, 3.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(1.5, 3.0, 2.5)));

    // ANCHOR: spawn
    commands.spawn((
        Mesh3d(meshes.add(banner_mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/banner.png")),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Banner,
    ));
    // ANCHOR_END: spawn

    println!("老鲁：旗子齐活，上转台——盯住它转到背面的那一刻。");
}

/// 旗面匀速自转：每秒约四分之一圈
fn turn(mut banner: Single<&mut Transform, With<Banner>>, time: Res<Time>) {
    banner.rotate_y(time.delta_secs() * 1.5);
}

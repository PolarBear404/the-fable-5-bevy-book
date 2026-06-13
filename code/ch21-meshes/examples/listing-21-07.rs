//! Listing 21-7：老鲁手搓班旗——只交了顶点位置和三角形清单（法线还没谱）

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

// ANCHOR: build
/// 一面 1.4 × 1.4 的方旗：四个顶点，两个三角形
fn banner_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    // 顶点清单：每个顶点一个 [x, y, z]，旗面立在 XY 平面上
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.7, 0.7, 0.0],  // 0 左上
            [0.7, 0.7, 0.0],   // 1 右上
            [0.7, -0.7, 0.0],  // 2 右下
            [-0.7, -0.7, 0.0], // 3 左下
        ],
    )
    // 三角形清单：每三个索引钉一张面，
    // 从正面（+Z 那侧）看，三个角按逆时针报数
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
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.4, 3.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(1.5, 3.0, 2.5)));

    // 手搓的坯子照样入库——Assets<Mesh> 不问出身
    commands.spawn((
        Mesh3d(meshes.add(banner_mesh())),
        MeshMaterial3d(materials.add(Color::srgb(0.72, 0.16, 0.12))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    println!("老鲁：头一面手搓的旗，挂上——这颜色怎么不太对劲？");
}

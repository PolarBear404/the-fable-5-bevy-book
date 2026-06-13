//! Listing 21-9：亭盖的两种做法——共用顶点的“圆”与拆开顶点的“方”

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

/// 标记：转台上的展品
#[derive(Component)]
struct Showpiece;

// ANCHOR: build
/// 四棱锥的几何：五个顶点，六个三角形（四面锥身 + 两片底）
fn pavilion_roof_geometry() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.9, 0.0, -0.9], // 0 后左
            [0.9, 0.0, -0.9],  // 1 后右
            [0.9, 0.0, 0.9],   // 2 前右
            [-0.9, 0.0, 0.9],  // 3 前左
            [0.0, 1.1, 0.0],   // 4 锥尖
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        3, 2, 4, // 前坡
        2, 1, 4, // 右坡
        1, 0, 4, // 后坡
        0, 3, 4, // 左坡
        0, 1, 2, 0, 2, 3, // 底面两片，朝下
    ]))
}

/// 版本一：五个顶点四坡共用——法线只能折中，棱被抹掉
fn roof_smooth() -> Mesh {
    pavilion_roof_geometry().with_computed_normals()
}

/// 版本二：同一份几何，先把共用顶点拆开，再按面算法线
fn roof_flat() -> Mesh {
    pavilion_roof_geometry()
        .with_duplicated_vertices()
        .with_computed_normals()
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.8, 4.4).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(2.0, 4.0, 3.0)));

    let glaze = materials.add(Color::srgb(0.30, 0.46, 0.42));

    // 左：共用顶点版；右：拆开顶点版
    commands.spawn((
        Mesh3d(meshes.add(roof_smooth())),
        MeshMaterial3d(glaze.clone()),
        Transform::from_xyz(-1.4, 0.0, 0.0),
        Showpiece,
    ));
    commands.spawn((
        Mesh3d(meshes.add(roof_flat())),
        MeshMaterial3d(glaze),
        Transform::from_xyz(1.4, 0.0, 0.0),
        Showpiece,
    ));

    println!("老鲁：一样的料一样的尺寸，左边那顶怎么看着像个馒头？");
}

fn turn(mut pieces: Query<&mut Transform, With<Showpiece>>, time: Res<Time>) {
    for mut transform in &mut pieces {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

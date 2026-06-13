//! Listing 21-10：得月楼开张——立体布景合龙，空格给绣球换漆

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

/// 标记：转台上的展品
#[derive(Component)]
struct Showpiece;

/// 标记：吃漆的绣球
#[derive(Component)]
struct Ball;

/// 漆架：三桶漆的提货单，外加一句报幕词
#[derive(Resource)]
struct PaintRack {
    paints: [(&'static str, Handle<StandardMaterial>); 3],
    current: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.013, 0.022, 0.040)))
        .add_systems(Startup, setup)
        .add_systems(Update, (turn, repaint))
        .run();
}

// ANCHOR: roof
/// 亭盖：21.5 节的四棱锥，拆开顶点、按面取法线
fn pavilion_roof() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.9, 0.0, -0.9],
            [0.9, 0.0, -0.9],
            [0.9, 0.0, 0.9],
            [-0.9, 0.0, 0.9],
            [0.0, 1.1, 0.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        3, 2, 4, 2, 1, 4, 1, 0, 4, 0, 3, 4, 0, 1, 2, 0, 2, 3,
    ]))
    .with_duplicated_vertices()
    .with_computed_normals()
}
// ANCHOR_END: roof

/// 班旗：21.4 节的手搓方旗，位置、法线、贴图坐标、索引四样齐全
fn banner_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.7, 0.7, 0.0],
            [0.7, 0.7, 0.0],
            [0.7, -0.7, 0.0],
            [-0.7, -0.7, 0.0],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 0.0, 1.0]; 4],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
    )
    .with_inserted_indices(Indices::U32(vec![0, 3, 2, 0, 2, 1]))
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ---- 机位与堂灯 -------------------------------------------------------
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.2, 10.5).looking_at(Vec3::new(0.0, 1.5, -0.5), Vec3::Y),
    ));
    commands.spawn((
        PointLight {
            intensity: 5_000_000.0,
            range: 40.0,
            ..default()
        },
        Transform::from_xyz(2.0, 7.0, 6.0),
    ));

    // ---- 台面与立柱 -------------------------------------------------------
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.31, 0.30),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    let pillar = meshes.add(Cylinder::new(0.35, 5.0));
    let lacquer = materials.add(StandardMaterial {
        base_color: Color::srgb(0.48, 0.13, 0.10),
        perceptual_roughness: 0.7,
        ..default()
    });
    let roof = meshes.add(pavilion_roof());
    let glaze = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.46, 0.42),
        perceptual_roughness: 0.6,
        ..default()
    });
    for x in [-5.2, 5.2] {
        commands.spawn((
            Mesh3d(pillar.clone()),
            MeshMaterial3d(lacquer.clone()),
            Transform::from_xyz(x, 2.5, -3.2),
        ));
        // 柱头戴亭盖——手搓的坯子与内置图元同台
        commands.spawn((
            Mesh3d(roof.clone()),
            MeshMaterial3d(glaze.clone()),
            Transform::from_xyz(x, 5.0, -3.2),
        ));
    }

    // ---- 条案 -------------------------------------------------------------
    let wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.40, 0.24),
        perceptual_roughness: 0.85,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(3.2, 0.14, 1.2))),
        MeshMaterial3d(wood.clone()),
        Transform::from_xyz(-3.0, 1.0, 0.2),
    ));
    let leg = meshes.add(Cuboid::new(0.14, 0.96, 0.14));
    for (dx, dz) in [(-1.4, -0.4), (1.4, -0.4), (-1.4, 0.4), (1.4, 0.4)] {
        commands.spawn((
            Mesh3d(leg.clone()),
            MeshMaterial3d(wood.clone()),
            Transform::from_xyz(-3.0 + dx, 0.48, 0.2 + dz),
        ));
    }

    // ---- 武场家什：大鼓与铜锣 ---------------------------------------------
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.7, 0.9))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.52, 0.16, 0.12),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(3.4, 0.45, 0.0),
    ));
    // 锣斜倚着鼓：拉丝铜面——粗糙度抬一点，金属才接得住这盏孤灯
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.6, 0.08))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.86, 0.62, 0.32),
            metallic: 1.0,
            perceptual_roughness: 0.55,
            ..default()
        })),
        Transform::from_xyz(4.5, 0.62, 0.8)
            .with_rotation(Quat::from_rotation_x(1.30) * Quat::from_rotation_z(0.1)),
    ));

    // ---- 旗杆与班旗 -------------------------------------------------------
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.06, 4.4))),
        MeshMaterial3d(wood.clone()),
        Transform::from_xyz(-4.2, 2.2, -1.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(banner_mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/banner.png")),
            ..default()
        })),
        Transform::from_xyz(-3.42, 3.6, -1.5),
    ));

    // ---- 转台上的绣球 -----------------------------------------------------
    // ANCHOR: showcase
    // 六棱墩当转台，绣球坐镇中央
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.8, 0.5).mesh().resolution(6))),
        MeshMaterial3d(wood.clone()),
        Transform::from_xyz(0.0, 0.25, 1.6),
        Showpiece,
    ));
    let plain = materials.add(StandardMaterial {
        base_color: Color::srgb(0.82, 0.74, 0.62),
        perceptual_roughness: 0.9,
        ..default()
    });
    let gilt = materials.add(StandardMaterial {
        base_color: Color::srgb(0.86, 0.62, 0.32),
        metallic: 1.0,
        perceptual_roughness: 0.3,
        ..default()
    });
    let porcelain = materials.add(StandardMaterial {
        base_color: Color::srgb(0.80, 0.88, 0.86),
        perceptual_roughness: 0.1,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.62))),
        MeshMaterial3d(plain.clone()),
        Transform::from_xyz(0.0, 1.12, 1.6),
        Showpiece,
        Ball,
    ));
    commands.insert_resource(PaintRack {
        paints: [("素坯", plain), ("鎏金", gilt), ("亮瓷", porcelain)],
        current: 0,
    });
    // ANCHOR_END: showcase

    println!("老雷：得月楼头一晚，立体布景合龙——都转着看看。");
    println!("场记：空格换漆，绣球眼下是素坯。");
}

/// 转台缓缓走——立体的东西，得转着看
fn turn(mut pieces: Query<&mut Transform, With<Showpiece>>, time: Res<Time>) {
    for mut transform in &mut pieces {
        transform.rotate_y(time.delta_secs() * 0.6);
    }
}

// ANCHOR: repaint
/// 空格换漆：换的不是颜色，是整张材质提货单
fn repaint(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut rack: ResMut<PaintRack>,
    mut ball: Single<&mut MeshMaterial3d<StandardMaterial>, With<Ball>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        rack.current = (rack.current + 1) % rack.paints.len();
        let (name, paint) = &rack.paints[rack.current];
        ball.0 = paint.clone();
        println!("场记：绣球换{name}。");
    }
}
// ANCHOR_END: repaint

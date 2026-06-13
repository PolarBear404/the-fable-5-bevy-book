//! Listing 21-3：开料单——内置几何体全家福，外加一排细分实验

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: lineup
    // 素坯料：全场共用一份浅木色材质——克隆的只是提货单
    let blank = materials.add(Color::srgb(0.82, 0.74, 0.62));

    // 九件坯子，照料单从左到右排开
    let lineup = [
        meshes.add(Cuboid::new(1.2, 1.2, 1.2)),
        meshes.add(Sphere::new(0.7)),
        meshes.add(Cylinder::new(0.55, 1.3)),
        meshes.add(Cone::new(0.65, 1.3)),
        meshes.add(ConicalFrustum {
            radius_top: 0.35,
            radius_bottom: 0.65,
            height: 1.2,
        }),
        meshes.add(Capsule3d::new(0.45, 0.8)),
        meshes.add(Torus::new(0.35, 0.75)),
        meshes.add(Tetrahedron::default()),
        meshes.add(Extrusion::new(Annulus::new(0.4, 0.65), 1.0)),
    ];
    for (i, mesh) in lineup.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(blank.clone()),
            Transform::from_xyz(-6.4 + i as f32 * 1.6, 1.6, 0.0),
            Showpiece,
        ));
    }
    // ANCHOR_END: lineup

    // ANCHOR: resolution
    // 前排：同一种料，不同的细分——“圆”是三角形装出来的
    let trials = [
        meshes.add(Sphere::new(0.8).mesh().ico(0).unwrap()), // 不细分：20 面体
        meshes.add(Sphere::new(0.8).mesh().uv(8, 5)),        // 经纬球：8 × 5 格
        meshes.add(Sphere::new(0.8)),                        // 默认：ico 细分 5 档
        meshes.add(Cylinder::new(0.65, 0.8).mesh().resolution(6)), // 圆柱掰成六棱墩
    ];
    for (i, mesh) in trials.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(blank.clone()),
            Transform::from_xyz(-3.0 + i as f32 * 2.0, 0.8, 2.8),
            Showpiece,
        ));
    }
    // ANCHOR_END: resolution

    // ANCHOR: ground
    // 台面：一块朝上的平面，要多大裁多大
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 12.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.33, 0.36, 0.34))),
    ));
    // ANCHOR_END: ground

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.5, 11.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
    // 料单铺得宽，堂灯加大瓦数、挂高挂正——参数细调是第 22 章的事
    commands.spawn((
        PointLight {
            intensity: 2_500_000.0,
            range: 40.0,
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 6.0),
    ));

    println!("老鲁：九件坯子一排细分实验，全上转台——转着看才知道圆不圆。");
}

// ANCHOR: turn
/// 展品慢慢自转——立体的东西，得转着看
fn turn(mut pieces: Query<&mut Transform, With<Showpiece>>, time: Res<Time>) {
    for mut transform in &mut pieces {
        transform.rotate_y(time.delta_secs() * 0.4);
    }
}
// ANCHOR_END: turn

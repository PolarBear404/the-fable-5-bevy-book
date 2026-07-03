//! Listing 22-1：拆堂灯——PointLight 的旋钮，空格换档、R 收光程

use bevy::prelude::*;

// ANCHOR: grades
/// 灯谱：老烛的四档灯，一档一个流明数
const GRADES: [(&str, f32); 4] = [
    ("影院大灯", 1_000_000.0),
    ("堂会汽灯", 60_000.0),
    ("大红灯笼", 8_000.0),
    ("白炽灯泡", 800.0),
];

#[derive(Resource)]
struct LightDesk {
    grade: usize,
    short_range: bool,
}
// ANCHOR_END: grades

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 拉总闸：把默认那点环境光兜底关干净，全场只认这一盏灯
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.013, 0.022, 0.040)))
        .insert_resource(LightDesk {
            grade: 0,
            short_range: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, dial)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.2, 10.5).looking_at(Vec3::new(0.0, 1.5, -0.5), Vec3::Y),
    ));

    // 台面、绣球、大鼓、立柱——由近及远，各自离灯不一样远
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.31, 0.30),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    let wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.40, 0.24),
        perceptual_roughness: 0.85,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.8, 0.5).mesh().resolution(6))),
        MeshMaterial3d(wood.clone()),
        Transform::from_xyz(0.0, 0.25, 1.6),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.62))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.82, 0.74, 0.62),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.12, 1.6),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.7, 0.9))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.52, 0.16, 0.12),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(4.2, 0.45, -0.6),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.35, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.48, 0.13, 0.10),
            perceptual_roughness: 0.7,
            ..default()
        })),
        Transform::from_xyz(-5.2, 2.5, -3.2),
    ));

    // ANCHOR: lamp
    // 堂灯：光是 PointLight 组件发的；那颗“看得见的灯泡”是挂在同一实体
    // 下的子网格，自发光小球，纯粹给人看——灯本身没有形状
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.93, 0.82),
            intensity: GRADES[0].1, // 流明（lumens）：光源往四面八方泼出去的总量
            range: 20.0,            // 光程：出了这个半径，一点光都不给
            radius: 0.0,            // 发光体的几何大小，影响高光的胖瘦
            ..default()
        },
        Transform::from_xyz(0.0, 3.4, 2.4),
        children![(
            Mesh3d(meshes.add(Sphere::new(0.12))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.93, 0.82),
                emissive: LinearRgba::new(6.0, 5.2, 3.6, 1.0),
                ..default()
            })),
        )],
    ));
    // ANCHOR_END: lamp

    println!("老烛：总闸拉了，全场就这一盏灯——先看它的真本事。");
    println!("老烛：眼下是影院大灯，一百万流明。空格换档，R 收放光程。");
}

// ANCHOR: dial
/// 空格换灯的档位，R 把光程在 20 米和 5 米之间收放
fn dial(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut desk: ResMut<LightDesk>,
    mut light: Single<&mut PointLight>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        desk.grade = (desk.grade + 1) % GRADES.len();
        let (name, lumens) = GRADES[desk.grade];
        light.intensity = lumens;
        println!("老烛：换{name}——{lumens} 流明。");
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        desk.short_range = !desk.short_range;
        light.range = if desk.short_range { 5.0 } else { 20.0 };
        println!("老烛：光程收到 {} 米。", light.range);
    }
}
// ANCHOR_END: dial

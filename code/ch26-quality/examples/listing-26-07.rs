//! Listing 26-7：走马灯——空格拨快门角，S 换采样数，按住 ←→ 转机位看全屏糊

use bevy::camera::Exposure;
use bevy::post_process::bloom::Bloom;
use bevy::post_process::motion_blur::MotionBlur;
use bevy::prelude::*;

// ANCHOR: knobs
/// 快门角五档：0 = 快门根本不开（等于关掉模糊），2.0 = 拖影比实际走过的还长
const SHUTTER_ANGLES: [f32; 5] = [0.0, 0.25, 0.5, 1.0, 2.0];

#[derive(Resource)]
struct BlurDesk {
    angle: usize,
}

/// 走马灯的转轴标记
#[derive(Component)]
struct Carousel;
// ANCHOR_END: knobs

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.035)))
        .insert_resource(BlurDesk { angle: 2 }) // 开场 0.5——电影感的 180°
        .add_systems(Startup, setup)
        .add_systems(Update, (spin, blur_desk, orbit_camera))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    // MotionBlur 挂上即生效；它 require 的 MotionVectorPrepass 会自动跟来
    commands.spawn((
        Camera3d::default(),
        Exposure::INDOOR,      // 夜戏进光口径——22.2 的室内档
        MotionBlur::default(), // shutter_angle 0.5，samples 1
        Bloom::NATURAL,
        Transform::from_xyz(0.0, 2.6, 8.0).looking_at(Vec3::new(0.0, 1.8, 0.0), Vec3::Y),
    ));
    // ANCHOR_END: camera

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.31, 0.34),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // ANCHOR: carousel
    // 走马灯：中轴一根，六片彩屏绕轴排开，整台挂在一个会转的父实体上
    let panel_mesh = meshes.add(Cuboid::new(0.9, 1.3, 0.05));
    let colors = [
        Color::srgb(0.9, 0.2, 0.15),
        Color::srgb(0.95, 0.7, 0.2),
        Color::srgb(0.2, 0.8, 0.3),
        Color::srgb(0.2, 0.55, 0.95),
        Color::srgb(0.7, 0.3, 0.9),
        Color::srgb(0.95, 0.45, 0.7),
    ];
    commands
        .spawn((
            Carousel,
            Transform::from_xyz(0.0, 1.9, 0.0),
            Visibility::default(),
            children![(
                Mesh3d(meshes.add(Cylinder::new(0.09, 2.6))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.45, 0.32, 0.18),
                    ..default()
                })),
            )],
        ))
        .with_children(|carousel| {
            for (i, color) in colors.into_iter().enumerate() {
                let angle = i as f32 / colors.len() as f32 * std::f32::consts::TAU;
                carousel.spawn((
                    Mesh3d(panel_mesh.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: color,
                        // 屏板自发光：夜里自己亮，拖影才看得真切
                        emissive: color.to_linear() * 2.4,
                        ..default()
                    })),
                    Transform::from_xyz(angle.sin() * 1.7, 0.0, angle.cos() * 1.7)
                        .with_rotation(Quat::from_rotation_y(angle)),
                ));
            }
        });
    // ANCHOR_END: carousel

    // 两盏定场灯，照出不动的台面当对照组
    for (x, z) in [(-4.0, 3.0), (4.0, -1.0)] {
        commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.9, 0.75),
                intensity: 30_000.0,
                range: 22.0,
                ..default()
            },
            Transform::from_xyz(x, 4.0, z),
        ));
    }
    // 后排一列白立柱：它们纹丝不动——等会儿转机位时，看谁在糊
    let post = meshes.add(Cuboid::new(0.16, 2.2, 0.16));
    let whitewash = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.78, 0.72),
        perceptual_roughness: 0.7,
        ..default()
    });
    for i in 0..7 {
        commands.spawn((
            Mesh3d(post.clone()),
            MeshMaterial3d(whitewash.clone()),
            Transform::from_xyz(-6.0 + i as f32 * 2.0, 1.1, -4.5),
        ));
    }

    println!("盛师傅：走马灯转起来了，快门角 0.5——电影的 180 度。");
    println!("盛师傅：空格拨快门角，S 换采样数，按住左右键转机位。");
}

/// 走马灯匀速自转：每秒 1.8 弧度
fn spin(time: Res<Time>, mut carousel: Single<&mut Transform, With<Carousel>>) {
    carousel.rotate_y(1.8 * time.delta_secs());
}

// ANCHOR: desk
/// 空格拨快门角，S 在 1 与 4 之间换采样数
fn blur_desk(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut desk: ResMut<BlurDesk>,
    mut blur: Single<&mut MotionBlur>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        desk.angle = (desk.angle + 1) % SHUTTER_ANGLES.len();
        blur.shutter_angle = SHUTTER_ANGLES[desk.angle];
        println!("盛师傅：快门角拨到 {}。", blur.shutter_angle);
    }
    if keyboard.just_pressed(KeyCode::KeyS) {
        blur.samples = if blur.samples == 1 { 4 } else { 1 };
        println!("盛师傅：每向采样 {} 次。", blur.samples);
    }
}
// ANCHOR_END: desk

// ANCHOR: orbit
/// 按住 ←→ 让机位绕场转：镜头一动，不止走马灯，满台都在“动”
fn orbit_camera(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    let mut dir = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) {
        dir += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        dir -= 1.0;
    }
    if dir != 0.0 {
        let angle = dir * 2.2 * time.delta_secs();
        let rotated = Quat::from_rotation_y(angle) * camera.translation;
        camera.translation = rotated;
        camera.look_at(Vec3::new(0.0, 1.8, 0.0), Vec3::Y);
    }
}
// ANCHOR_END: orbit

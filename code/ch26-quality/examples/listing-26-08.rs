//! Listing 26-8：镜头三件套——V 暗角，←→ 拨畸变（正桶负枕），C 色差撞击档

use bevy::post_process::effect_stack::{ChromaticAberration, LensDistortion, Vignette};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.035)))
        .add_systems(Startup, setup)
        .add_systems(Update, lens_desk)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    // 三件套各是一个组件，想要哪件挂哪件；畸变从 0 起步（0 强度 = 整段后处理直接跳过）
    commands.spawn((
        Camera3d::default(),
        Vignette::default(),
        LensDistortion {
            intensity: 0.0,
            ..default()
        },
        ChromaticAberration::default(),
        Transform::from_xyz(0.0, 2.4, 8.5).looking_at(Vec3::new(0.0, 1.6, 0.0), Vec3::Y),
    ));
    // ANCHOR_END: camera

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 16.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.31, 0.34),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // 一排白栏杆加一根横梁：直线是畸变最好的试纸
    let post = meshes.add(Cuboid::new(0.09, 2.0, 0.09));
    let white = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.83, 0.78),
        perceptual_roughness: 0.5,
        ..default()
    });
    for i in 0..11 {
        commands.spawn((
            Mesh3d(post.clone()),
            MeshMaterial3d(white.clone()),
            Transform::from_xyz(-5.0 + i as f32, 1.0, -2.0),
        ));
    }
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10.4, 0.12, 0.12))),
        MeshMaterial3d(white.clone()),
        Transform::from_xyz(0.0, 2.06, -2.0),
    ));

    // 中央一面戏旗、两侧各一盏灯笼——高反差的边缘给色差当靶子
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.6, 2.2, 0.08))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.15, 0.12),
            perceptual_roughness: 0.55,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.1, -0.6),
    ));
    for x in [-4.6, 4.6] {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.32))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.1),
                emissive: LinearRgba::new(8.0, 5.6, 1.8, 1.0),
                ..default()
            })),
            Transform::from_xyz(x, 3.2, -0.8),
        ));
    }

    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.92, 0.8),
            intensity: 500_000.0,
            range: 40.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.5, 5.0),
    ));

    println!("盛师傅：栏杆横梁摆好，直线最怕镜头说谎。");
    println!("盛师傅：V 暗角开关，左右键拨畸变，C 在 0.02 和 0.4 之间跳色差。");
}

// ANCHOR: desk
/// 三件套调参台：Vignette 拆装组件，LensDistortion 拨强度，ChromaticAberration 两档跳
fn lens_desk(
    keyboard: Res<ButtonInput<KeyCode>>,
    camera: Single<(Entity, Has<Vignette>), With<Camera3d>>,
    mut distortion: Single<&mut LensDistortion>,
    mut aberration: Single<&mut ChromaticAberration>,
    mut commands: Commands,
) {
    // V：暗角整个组件拆上拆下
    if keyboard.just_pressed(KeyCode::KeyV) {
        let (entity, has_vignette) = *camera;
        if has_vignette {
            commands.entity(entity).remove::<Vignette>();
            println!("盛师傅：暗角摘了。");
        } else {
            commands.entity(entity).insert(Vignette::default());
            println!("盛师傅：暗角装回。");
        }
    }
    // ←→：畸变强度 ±0.2，正往外鼓（桶形）、负往里收（枕形）
    let mut delta = 0.0;
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        delta += 0.2;
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        delta -= 0.2;
    }
    if delta != 0.0 {
        distortion.intensity = (distortion.intensity + delta).clamp(-0.8, 0.8);
        println!("盛师傅：畸变 {:+.1}。", distortion.intensity);
    }
    // C：色差在“镜头本色”与“撞击特效”两档之间跳
    if keyboard.just_pressed(KeyCode::KeyC) {
        if aberration.intensity < 0.1 {
            aberration.intensity = 0.4;
            aberration.max_samples = 64; // 拉这么宽还用 8 步采样，彩带会断成一节节
        } else {
            aberration.intensity = 0.02;
            aberration.max_samples = 8;
        }
        println!(
            "盛师傅：色差 {:.2}，采样上限 {}。",
            aberration.intensity, aberration.max_samples
        );
    }
}
// ANCHOR_END: desk

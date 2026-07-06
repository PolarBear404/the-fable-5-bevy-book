//! Listing 26-10：磨边换挡台——1~5 选方案，Q/W/E/R/T 拨档位，0 开关锐化

use bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening;
use bevy::anti_alias::fxaa::{Fxaa, Sensitivity};
use bevy::anti_alias::smaa::{Smaa, SmaaPreset};
use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::camera::Hdr;
use bevy::core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass};
use bevy::prelude::*;
use bevy::render::camera::{MipBias, TemporalJitter};

// ANCHOR: taa_bundle
/// TAA 上身时 require 会自动带上的四个搭档。required components 只管上、不管下：
/// 摘 TAA 必须连它们一起点名，不然抖动和 prepass 还赖在相机上
type TaaBundle = (
    TemporalAntiAliasing,
    TemporalJitter,
    MipBias,
    DepthPrepass,
    MotionVectorPrepass,
);
// ANCHOR_END: taa_bundle

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (shift_gear, tune_gear, tune_sharpen))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    // 出厂即 MSAA 4x——Msaa 是 Camera 的 required component，没写也在。
    // 锐化先挂上但拨到关，等 0 键唤它
    commands.spawn((
        Camera3d::default(),
        Hdr,
        ContrastAdaptiveSharpening {
            enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 1.6, 6.2).looking_at(Vec3::new(0.0, 1.1, 0.0), Vec3::Y),
    ));
    // ANCHOR_END: camera

    // 白天彩排：太阳一盏，亮堂堂的场子里锯齿最扎眼
    commands.spawn((
        DirectionalLight {
            illuminance: 24_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, -0.6, -0.9)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(30.0, 30.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.32, 0.36, 0.33),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 细栏杆一路排向远处：越远越细，锯齿的重灾区
    let post = meshes.add(Cuboid::new(0.06, 1.4, 0.06));
    let lacquer = materials.add(StandardMaterial {
        base_color: Color::srgb(0.62, 0.14, 0.10),
        perceptual_roughness: 0.4,
        ..default()
    });
    for i in 0..14 {
        commands.spawn((
            Mesh3d(post.clone()),
            MeshMaterial3d(lacquer.clone()),
            Transform::from_xyz(-1.6 - i as f32 * 0.12, 0.7, -i as f32 * 1.1),
        ));
    }

    // 斜旗杆加三角旗：斜线的阶梯感比竖线更凶
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.035, 5.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.68, 0.55),
            perceptual_roughness: 0.5,
            ..default()
        })),
        Transform::from_xyz(1.6, 1.9, -1.5).with_rotation(Quat::from_rotation_z(-0.7)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 0.9, 0.03))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.75, 0.2),
            perceptual_roughness: 0.6,
            double_sided: true,
            cull_mode: None,
            ..default()
        })),
        Transform::from_xyz(3.35, 3.85, -1.5).with_rotation(Quat::from_rotation_z(-0.16)),
    ));

    // 上釉瓷柱：又亮又滑，弧面上一条烈日高光——高光锯齿（specular aliasing）
    // 的窝点，MSAA 治不了它
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.16, 2.6).mesh().resolution(64))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.88, 0.82, 0.72),
            perceptual_roughness: 0.08,
            ..default()
        })),
        Transform::from_xyz(0.2, 1.3, 0.8),
    ));

    println!("场记：磨边换挡台开张——1 素颜，2 MSAA，3 FXAA，4 SMAA，5 TAA。");
    println!("场记：出厂默认就是 MSAA 4x。");
}

// ANCHOR: shift
/// 1~5 换抗锯齿方案：MSAA 拨枚举档，其余三家上/下组件；同场只留一家
fn shift_gear(
    keyboard: Res<ButtonInput<KeyCode>>,
    camera: Single<(Entity, &mut Msaa), With<Camera3d>>,
    mut commands: Commands,
) {
    let (entity, mut msaa) = camera.into_inner();
    let strip = |commands: &mut Commands| {
        commands
            .entity(entity)
            .remove::<Fxaa>()
            .remove::<Smaa>()
            .remove::<TaaBundle>();
    };
    if keyboard.just_pressed(KeyCode::Digit1) {
        strip(&mut commands);
        *msaa = Msaa::Off;
        println!("场记：全下——素颜出场，阶梯看个够。");
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        strip(&mut commands);
        *msaa = Msaa::Sample4;
        println!("场记：MSAA 4x 上场（Q/W/E 拨 2/4/8）。");
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        strip(&mut commands);
        *msaa = Msaa::Off;
        commands.entity(entity).insert(Fxaa::default());
        println!("场记：FXAA 上场（Q/W/E/R/T 拨灵敏度）。");
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        strip(&mut commands);
        *msaa = Msaa::Off;
        commands.entity(entity).insert(Smaa::default());
        println!("场记：SMAA 上场（Q/W/E/R 拨质量）。");
    }
    if keyboard.just_pressed(KeyCode::Digit5) {
        strip(&mut commands);
        *msaa = Msaa::Off; // TAA 的硬规矩：MSAA 必须关
        commands
            .entity(entity)
            .insert(TemporalAntiAliasing::default());
        println!("场记：TAA 上场——瓷柱的高光交给时间。");
    }
}
// ANCHOR_END: shift

// ANCHOR: tune
/// Q/W/E/R/T 给当前方案拨档
fn tune_gear(
    keyboard: Res<ButtonInput<KeyCode>>,
    camera: Single<(&mut Msaa, Option<&mut Fxaa>, Option<&mut Smaa>), With<Camera3d>>,
) {
    let (mut msaa, fxaa, smaa) = camera.into_inner();
    let pressed = [
        KeyCode::KeyQ,
        KeyCode::KeyW,
        KeyCode::KeyE,
        KeyCode::KeyR,
        KeyCode::KeyT,
    ]
    .into_iter()
    .position(|key| keyboard.just_pressed(key));
    let Some(gear) = pressed else { return };

    if let Some(mut fxaa) = fxaa {
        let level = [
            Sensitivity::Low,
            Sensitivity::Medium,
            Sensitivity::High,
            Sensitivity::Ultra,
            Sensitivity::Extreme,
        ][gear];
        fxaa.edge_threshold = level;
        fxaa.edge_threshold_min = level;
        println!("场记：FXAA 灵敏度 {level:?}。");
    } else if let Some(mut smaa) = smaa {
        if gear < 4 {
            let (name, preset) = [
                ("Low", SmaaPreset::Low),
                ("Medium", SmaaPreset::Medium),
                ("High", SmaaPreset::High),
                ("Ultra", SmaaPreset::Ultra),
            ][gear];
            smaa.preset = preset;
            println!("场记：SMAA 质量 {name}。");
        }
    } else if *msaa != Msaa::Off && gear < 3 {
        *msaa = [Msaa::Sample2, Msaa::Sample4, Msaa::Sample8][gear];
        println!("场记：MSAA 每像素 {} 票。", msaa.samples());
    }
}
// ANCHOR_END: tune

// ANCHOR: sharpen
/// 0 开关锐化，-/= 拨强度：给 FXAA/TAA 抹掉的细节找补一点回来
fn tune_sharpen(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cas: Single<&mut ContrastAdaptiveSharpening>,
) {
    if keyboard.just_pressed(KeyCode::Digit0) {
        cas.enabled = !cas.enabled;
        println!(
            "场记：锐化{}（强度 {:.1}）。",
            if cas.enabled { "开" } else { "关" },
            cas.sharpening_strength
        );
    }
    if cas.enabled {
        let mut delta = 0.0;
        if keyboard.just_pressed(KeyCode::Equal) {
            delta += 0.2;
        }
        if keyboard.just_pressed(KeyCode::Minus) {
            delta -= 0.2;
        }
        if delta != 0.0 {
            cas.sharpening_strength = (cas.sharpening_strength + delta).clamp(0.0, 1.0);
            println!("场记：锐化强度 {:.1}。", cas.sharpening_strength);
        }
    }
}
// ANCHOR_END: sharpen

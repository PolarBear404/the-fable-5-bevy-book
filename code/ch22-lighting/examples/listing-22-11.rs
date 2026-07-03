//! Listing 22-11：真的天——Atmosphere 大气散射，左右键推日头看晨昏

use bevy::camera::Exposure;
use bevy::light::{atmosphere::ScatteringMedium, Atmosphere, AtmosphereEnvironmentMapLight, SunDisk};
use bevy::pbr::AtmosphereSettings;
use bevy::prelude::*;

/// 日头的高度角（弧度）
#[derive(Resource)]
struct SunArc {
    elevation: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 天自己会亮，兜底环境光全撤
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(SunArc { elevation: 0.5 })
        .add_systems(Startup, setup)
        .add_systems(Update, push_sun)
        .run();
}

fn setup(
    mut commands: Commands,
    mut media: ResMut<Assets<ScatteringMedium>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: atmosphere
    // 大气是一颗行星的实体：地球预设的散射介质 + 地球半径。
    // 不给 Transform 的话，它会自动把行星中心放到脚下 6,360 公里处
    let medium = media.add(ScatteringMedium::earth(256, 256));
    commands.spawn(Atmosphere::earth(medium));

    // 相机这头：挂上 AtmosphereSettings 才享受这片天；
    // 大气的亮度是真实白昼量级，曝光得按烈日调
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.6, 10.5).looking_at(Vec3::new(0.0, 2.4, -1.0), Vec3::Y),
        AtmosphereSettings::default(),
        Exposure { ev100: 13.0 },
        // 让大气反过来给场景当环境光：天越红，台上越红
        AtmosphereEnvironmentMapLight::default(),
    ));
    // ANCHOR_END: atmosphere

    // 台面往四外铺开去——地平线交给大气去画
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(600.0, 600.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.31, 0.30),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    let lacquer = materials.add(StandardMaterial {
        base_color: Color::srgb(0.48, 0.13, 0.10),
        perceptual_roughness: 0.7,
        ..default()
    });
    for x in [-5.2, 5.2] {
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.35, 5.0))),
            MeshMaterial3d(lacquer.clone()),
            Transform::from_xyz(x, 2.5, -3.2),
        ));
    }
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.8, 0.5).mesh().resolution(6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.40, 0.24),
            perceptual_roughness: 0.85,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.25, 1.6),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.62))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.93, 0.88),
            metallic: 1.0,
            perceptual_roughness: 0.05,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.12, 1.6),
    ));

    // ANCHOR: sun
    // 太阳：照度用 RAW_SUNLIGHT——大气层外的原始日光，散射交给大气去算；
    // SunDisk 让天上真的挂一轮日头
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::RAW_SUNLIGHT,
            shadow_maps_enabled: true,
            ..default()
        },
        SunDisk::EARTH,
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.35, -0.5)),
    ));
    // ANCHOR_END: sun

    println!("老烛：这回不挂布了——把天支起来。左右键推日头，1/2/3 直接跳档。");
}

/// 左右键细推日头，1/2/3 跳到拂晓/清晨/正午——太阳一低，天自己红
fn push_sun(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut arc: ResMut<SunArc>,
    mut sun: Single<&mut Transform, With<DirectionalLight>>,
) {
    let mut dir = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) {
        dir -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        dir += 1.0;
    }
    if dir != 0.0 {
        arc.elevation = (arc.elevation + dir * 0.25 * time.delta_secs()).clamp(-0.05, 1.4);
    }
    for (key, name, elevation) in [
        (KeyCode::Digit1, "拂晓", 0.02),
        (KeyCode::Digit2, "清晨", 0.35),
        (KeyCode::Digit3, "正午", 1.25),
    ] {
        if keyboard.just_pressed(key) {
            arc.elevation = elevation;
            println!("老烛：日头拨到{name}。");
        }
    }
    sun.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, 0.35, -arc.elevation);
}

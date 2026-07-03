//! Listing 22-13：晨雾——体积雾三件套：FogVolume 雾体、VolumetricLight 灯、VolumetricFog 相机

use bevy::light::{FogVolume, VolumetricFog, VolumetricLight};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight {
            brightness: 20.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.09, 0.11, 0.16)))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_fog)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    // 相机挂 VolumetricFog：这是“看得见雾”的开关，参数是雾里步进的步数
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.4, 11.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        VolumetricFog {
            ambient_intensity: 0.06,
            ..default()
        },
    ));
    // ANCHOR_END: camera

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.33, 0.31),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    // 五根立柱排一排，专门给光柱当篦子
    let lacquer = materials.add(StandardMaterial {
        base_color: Color::srgb(0.48, 0.13, 0.10),
        perceptual_roughness: 0.7,
        ..default()
    });
    for i in -2..=2 {
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.28, 5.0))),
            MeshMaterial3d(lacquer.clone()),
            Transform::from_xyz(i as f32 * 2.4, 2.5, -2.0),
        ));
    }
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.8, 0.5).mesh().resolution(6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.40, 0.24),
            perceptual_roughness: 0.85,
            ..default()
        })),
        Transform::from_xyz(0.6, 0.25, 2.2),
    ));

    // ANCHOR: fog
    // 晨光斜着从柱子后面打过来；挂 VolumetricLight 的灯才在雾里拉光柱，
    // 前提是它开着影子——光柱就是“影子在雾里的补集”
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.87, 0.66),
            illuminance: 8_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        VolumetricLight,
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 2.6, -0.35)),
    ));

    // 雾体：一只单位立方，Transform 撑成罩住全台的雾罩子
    commands.spawn((
        FogVolume {
            density_factor: 0.18,
            ..default()
        },
        Transform::from_xyz(0.0, 2.5, 0.0).with_scale(Vec3::new(26.0, 5.0, 14.0)),
    ));
    // ANCHOR_END: fog

    println!("老烛：起雾了。晨光从柱子缝里漏进来——F 键收放雾气。");
}

/// F 键拨雾的浓度：0.18 ↔ 0.02
fn toggle_fog(keyboard: Res<ButtonInput<KeyCode>>, mut fog: Single<&mut FogVolume>) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        fog.density_factor = if fog.density_factor > 0.1 { 0.02 } else { 0.18 };
        println!("老烛：雾气拨到 {:.2}。", fog.density_factor);
    }
}

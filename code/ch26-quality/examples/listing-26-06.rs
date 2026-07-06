//! Listing 26-6：一份参数两头喂——PhysicalCameraParameters 同时定曝光与景深

use bevy::camera::{Exposure, PhysicalCameraParameters};
use bevy::post_process::bloom::Bloom;
use bevy::post_process::dof::DepthOfField;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.035)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: physical
    // 报一台真相机的膛口数据：f/2、1/50 秒、ISO 800、Super 35 画幅
    let rig = PhysicalCameraParameters {
        aperture_f_stops: 2.0,
        shutter_speed_s: 1.0 / 50.0,
        sensitivity_iso: 800.0,
        sensor_height: 0.01866,
    };

    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 0.35, // 还是那支 53mm 定妆镜头
            ..default()
        }),
        // 同一份参数：曝光按它算 EV100，景深按它算光圈与画幅——两头对得上账
        Exposure::from_physical_camera(rig),
        DepthOfField {
            focal_distance: 3.2,
            ..DepthOfField::from_physical_camera(&rig)
        },
        Bloom::NATURAL,
        Transform::from_xyz(0.0, 1.9, 8.0).looking_at(Vec3::new(0.0, 1.1, 0.0), Vec3::Y),
    ));
    println!(
        "盛师傅：f/2、1/50 秒、ISO 800——这套膛口算出 EV100 = {:.2}。",
        rig.ev100()
    );
    // ANCHOR_END: physical

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(30.0, 24.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.31, 0.34),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.42))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.75, 0.72),
            perceptual_roughness: 0.15,
            ..default()
        })),
        Transform::from_xyz(-0.75, 0.9, 5.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.75, 0.9))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.17, 0.13),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(0.55, 0.45, 0.0),
    ));
    for (x, y, z) in [(-1.9, 2.6, 5.6), (1.9, 2.8, 0.8)] {
        commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.9, 0.75),
                intensity: 22_000.0,
                range: 14.0,
                ..default()
            },
            Transform::from_xyz(x, y, z),
        ));
    }
}

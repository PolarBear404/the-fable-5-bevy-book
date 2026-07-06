//! Listing 26-9：自动测光——P 键在亮暗两个机位间摇镜头，看曝光自己爬坡

use bevy::post_process::auto_exposure::{AutoExposure, AutoExposurePlugin};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

// ANCHOR: rig
/// 两个机位：一头对着灯笼堆，一头对着黑黢黢的后台
const BRIGHT_SEAT: (Vec3, Vec3) = (Vec3::new(2.5, 2.2, 5.5), Vec3::new(4.0, 2.0, -1.0));
const DARK_SEAT: (Vec3, Vec3) = (Vec3::new(-2.5, 2.2, 5.5), Vec3::new(-5.0, 1.2, -3.0));

#[derive(Resource)]
struct CameraSeat {
    at_bright: bool,
    fast: bool, // 适应速度：false = 默认档，true = 快门手
}
// ANCHOR_END: rig

fn main() {
    App::new()
        // ANCHOR: plugin
        // 自动测光不在 DefaultPlugins 里：要用，先把插件挂上
        .add_plugins((DefaultPlugins, AutoExposurePlugin))
        // ANCHOR_END: plugin
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.035)))
        .insert_resource(CameraSeat {
            at_bright: true,
            fast: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, swing)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    // AutoExposure 也 require Hdr——测光靠的是 HDR 底片上的真实亮度直方图
    commands.spawn((
        Camera3d::default(),
        AutoExposure::default(),
        Bloom::NATURAL,
        Transform::from_translation(BRIGHT_SEAT.0).looking_at(BRIGHT_SEAT.1, Vec3::Y),
    ));
    // ANCHOR_END: camera

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 16.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.23, 0.24, 0.27),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // 亮区：右手边灯笼堆，三盏挤在一处
    for (x, y, z) in [(4.0, 2.4, -1.0), (5.0, 1.6, -0.2), (3.2, 1.2, 0.4)] {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.35))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.1),
                emissive: LinearRgba::new(10.0, 6.0, 1.8, 1.0),
                ..default()
            })),
            Transform::from_xyz(x, y, z),
            children![(
                PointLight {
                    color: Color::srgb(1.0, 0.85, 0.5),
                    intensity: 10_000.0,
                    range: 12.0,
                    ..default()
                },
                Transform::IDENTITY,
            )],
        ));
    }

    // 暗区：左手边后台，一只木箱一面旗，只沾一点灯笼的余光
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 1.2, 1.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.3, 0.18),
            perceptual_roughness: 0.85,
            ..default()
        })),
        Transform::from_xyz(-5.0, 0.6, -3.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.3, 1.9, 0.07))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.3, 0.6),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(-6.4, 0.95, -2.2),
    ));

    println!("盛师傅：右边灯笼堆，左边黑后台，测光表自己转。");
    println!("盛师傅：P 摇机位，F 换适应速度（默认 入亮景 3 / 入暗景 1，快门手 8/8）。");
}

// ANCHOR: swing
/// P 摇镜头，F 拨适应速度。speed_* 的单位是每秒几档 F-stop，
/// 主语是场景：brighten = 场景变亮（画面压暗），darken = 场景变暗（画面爬亮）
fn swing(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut seat: ResMut<CameraSeat>,
    mut camera: Single<(&mut Transform, &mut AutoExposure), With<Camera3d>>,
) {
    let (transform, auto_exposure) = &mut *camera;
    if keyboard.just_pressed(KeyCode::KeyP) {
        seat.at_bright = !seat.at_bright;
        let (from, to) = if seat.at_bright {
            BRIGHT_SEAT
        } else {
            DARK_SEAT
        };
        transform.translation = from;
        transform.look_at(to, Vec3::Y);
        println!(
            "盛师傅：摇向{}——盯住画面，亮度要爬一小会儿。",
            if seat.at_bright {
                "灯笼堆"
            } else {
                "黑后台"
            }
        );
    }
    if keyboard.just_pressed(KeyCode::KeyF) {
        seat.fast = !seat.fast;
        if seat.fast {
            auto_exposure.speed_brighten = 8.0;
            auto_exposure.speed_darken = 8.0;
        } else {
            auto_exposure.speed_brighten = 3.0;
            auto_exposure.speed_darken = 1.0;
        }
        println!(
            "盛师傅：适应速度 入亮景 {}、入暗景 {}（F-stop/秒）。",
            auto_exposure.speed_brighten, auto_exposure.speed_darken
        );
    }
}
// ANCHOR_END: swing

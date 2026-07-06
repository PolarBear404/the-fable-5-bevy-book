//! Listing 26-4：抽底片——H 键移除 Hdr，Bloom 组件还挂着，晕却一声不吭地灭了

use bevy::camera::Exposure;
use bevy::camera::Hdr;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.035)))
        .add_systems(Startup, setup)
        .add_systems(Update, pull_film)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Exposure::INDOOR, // 夜戏进光口径——22.2 的室内档
        Bloom::NATURAL,   // require 帮忙挂上的 Hdr，下面照样可以手动拆走
        Transform::from_xyz(0.0, 3.0, 8.5).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(22.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.23, 0.24, 0.27),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // 独一盏金灯，晕在不在一眼便知
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.1),
            emissive: LinearRgba::new(9.0, 6.5, 2.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 2.8, 0.0),
        children![(
            PointLight {
                color: Color::srgb(1.0, 0.85, 0.45),
                intensity: 25_000.0,
                range: 20.0,
                ..default()
            },
            Transform::IDENTITY,
        )],
    ));

    println!("盛师傅：金灯一盏，晕圈在场。H 键抽底片试试。");
}

// ANCHOR: pull
/// H 键拔/插 Hdr：Bloom 组件全程挂在相机上没人动过
fn pull_film(
    keyboard: Res<ButtonInput<KeyCode>>,
    camera: Single<(Entity, Has<Hdr>, Has<Bloom>), With<Camera3d>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        let (entity, has_hdr, has_bloom) = *camera;
        if has_hdr {
            commands.entity(entity).remove::<Hdr>();
            println!(
                "盛师傅：底片抽了。Bloom 组件还在吗？——{}。晕呢？自己看。",
                if has_bloom { "在" } else { "不在" }
            );
        } else {
            commands.entity(entity).insert(Hdr);
            println!("盛师傅：底片插回去——晕圈原地复活。");
        }
    }
}
// ANCHOR_END: pull

//! Listing 22-2：测光表——同一盏 8000 流明的灯笼，E 键换四档曝光

use bevy::{camera::Exposure, prelude::*};

// ANCHOR: presets
/// 老烛的测光表：EV100 越小，相机吃进的光越多
const METER: [(&str, f32); 4] = [
    ("默认（对标 Blender）", Exposure::EV100_BLENDER), // 9.7
    ("室内", Exposure::EV100_INDOOR),                  // 7.0
    ("阴天", Exposure::EV100_OVERCAST),                // 12.0
    ("烈日", Exposure::EV100_SUNLIGHT),                // 15.0
];

#[derive(Resource)]
struct MeterDial(usize);
// ANCHOR_END: presets

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.013, 0.022, 0.040)))
        .insert_resource(MeterDial(0))
        .add_systems(Startup, setup)
        .add_systems(Update, meter)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    // Exposure 是相机组件：不挂就是默认的 EV100 = 9.7
    commands.spawn((
        Camera3d::default(),
        Exposure {
            ev100: METER[0].1,
        },
        Transform::from_xyz(0.0, 4.2, 10.5).looking_at(Vec3::new(0.0, 1.5, -0.5), Vec3::Y),
    ));
    // ANCHOR_END: camera

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.31, 0.30),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
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

    // 22.1 节的堂灯，拨在大红灯笼那一档，原封不动
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.93, 0.82),
            intensity: 8_000.0,
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

    println!("老烛：还是那盏 8000 流明的大红灯笼，一整晚都不碰它。");
    println!("老烛：按 E，我换的是测光表，不是灯。");
}

// ANCHOR: meter
/// E 键拨测光表：改的是相机的曝光，灯从头到尾没动过
fn meter(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut dial: ResMut<MeterDial>,
    mut exposure: Single<&mut Exposure>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        dial.0 = (dial.0 + 1) % METER.len();
        let (name, ev100) = METER[dial.0];
        exposure.ev100 = ev100;
        println!("老烛：曝光拨到{name}档，EV {ev100}。");
    }
}
// ANCHOR_END: meter

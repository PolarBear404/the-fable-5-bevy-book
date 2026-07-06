//! Listing 26-5：对焦——1/2/3 对准前中后三件货，Q/W/E 拨光圈，G 换虚化模式

use bevy::camera::Exposure;
use bevy::post_process::bloom::Bloom;
use bevy::post_process::dof::{DepthOfField, DepthOfFieldMode};
use bevy::prelude::*;

// ANCHOR: seats
/// 三件货各自离相机多远，就是三档 focal_distance（机位在 (0, 1.9, 8.0)）
const SEATS: [(&str, f32); 3] = [
    ("琉璃盏（近）", 3.2),
    ("堂鼓（中）", 8.2),
    ("锦旗（远）", 13.6),
];

/// 三档光圈：f 值越小，孔越大，焦外越糊
const APERTURES: [(&str, f32); 3] = [("f/1.4", 1.4), ("f/5.6", 5.6), ("f/16", 16.0)];
// ANCHOR_END: seats

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.035)))
        .add_systems(Startup, setup)
        .add_systems(Update, focus_desk)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    // 拍定妆照用长镜头：竖向视场角收到 0.35 弧度（约 20°，等效 53mm）。
    // 焦外糊不糊，镜头焦距和光圈说了才算——广角镜什么都清楚，景深根本看不出
    commands.spawn((
        Camera3d::default(),
        Exposure::INDOOR, // 夜戏进光口径——22.2 的室内档
        Projection::Perspective(PerspectiveProjection {
            fov: 0.35,
            ..default()
        }),
        DepthOfField {
            focal_distance: SEATS[0].1,       // 先对准最近的琉璃盏
            aperture_f_stops: APERTURES[0].1, // 大光圈 f/1.4，糊得最狠
            ..default()
        },
        Bloom::NATURAL,
        Transform::from_xyz(0.0, 1.9, 8.0).looking_at(Vec3::new(0.0, 1.1, 0.0), Vec3::Y),
    ));
    // ANCHOR_END: camera

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(30.0, 24.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.31, 0.34),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // 前中后三件货：琉璃盏 z=5、堂鼓 z=0、锦旗 z=-5.6
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
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.6, 2.4, 0.08))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.10, 0.35, 0.75),
            perceptual_roughness: 0.55,
            ..default()
        })),
        Transform::from_xyz(-0.7, 1.2, -5.6),
    ));

    // 货各配一盏小灯，近处照近货、远处照远货
    for (x, y, z) in [(-1.9, 2.6, 5.6), (1.9, 2.8, 0.8), (0.7, 3.0, -4.6)] {
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

    // 最深处一排珠灯：对焦在前排时，它们会糊成一排圆光斑——散景的试纸
    for i in 0..7 {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.07))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.05, 0.02),
                emissive: LinearRgba::new(18.0, 10.5, 3.2, 1.0),
                ..default()
            })),
            Transform::from_xyz(-3.6 + i as f32 * 1.2, 2.4, -8.5),
        ));
    }

    println!("盛师傅：三件货三个纵深，最深处挂一排珠灯。");
    println!("盛师傅：1/2/3 对焦，Q/W/E 拨光圈，G 换虚化模式。开档：对琉璃盏，f/1.4。");
}

// ANCHOR: desk
/// 对焦台：焦距、光圈、模式三个旋钮全是 DepthOfField 的字段
fn focus_desk(keyboard: Res<ButtonInput<KeyCode>>, mut dof: Single<&mut DepthOfField>) {
    for (key, (name, distance)) in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3]
        .into_iter()
        .zip(SEATS)
    {
        if keyboard.just_pressed(key) {
            dof.focal_distance = distance;
            println!("盛师傅：对焦{name}——{distance} 米。");
        }
    }
    for (key, (name, f_stops)) in [KeyCode::KeyQ, KeyCode::KeyW, KeyCode::KeyE]
        .into_iter()
        .zip(APERTURES)
    {
        if keyboard.just_pressed(key) {
            dof.aperture_f_stops = f_stops;
            println!("盛师傅：光圈拨到 {name}。");
        }
    }
    if keyboard.just_pressed(KeyCode::KeyG) {
        dof.mode = match dof.mode {
            DepthOfFieldMode::Gaussian => DepthOfFieldMode::Bokeh,
            DepthOfFieldMode::Bokeh => DepthOfFieldMode::Gaussian,
        };
        println!("盛师傅：虚化换 {:?} 模式。", dof.mode);
    }
}
// ANCHOR_END: desk

//! Listing 22-4：请太阳——DirectionalLight 只有方向；左右键推日头，M 键搬家（搬了也白搬）

use bevy::prelude::*;

/// 日头在天上的高度角（弧度，0 = 贴地平线，π/2 = 头顶正中）
#[derive(Resource)]
struct SunArc {
    elevation: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.30, 0.42, 0.55)))
        .insert_resource(SunArc { elevation: 0.9 })
        .add_systems(Startup, setup)
        .add_systems(Update, (push_sun, grade, relocate))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.2, 10.5).looking_at(Vec3::new(0.0, 1.5, -0.5), Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 14.0))),
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

    // ANCHOR: sun
    // 太阳：DirectionalLight 沿实体的 -Z 方向照，位置无关紧要——
    // 只旋转就能定日头，这里用高度角把它架在东边的天上
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY, // 勒克斯（lux）：落到地上的照度
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.6, -0.9)),
    ));
    // ANCHOR_END: sun

    println!("老烛：阴天棚光，一千勒克斯。左右键推日头，空格换天色。");
    println!("老烛：M 键把太阳这实体搬走二十米——你猜画面动不动。");
}

// ANCHOR: push
/// 左右键推日头：改的是旋转。高度角越低，光越贴着台面扫
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
        arc.elevation = (arc.elevation + dir * 0.5 * time.delta_secs()).clamp(0.05, 1.5);
        sun.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, 0.6, -arc.elevation);
    }
}
// ANCHOR_END: push

// ANCHOR: grade
/// 空格换天色：照度换档，数值全是与现实对表的常数
fn grade(keyboard: Res<ButtonInput<KeyCode>>, mut sun: Single<&mut DirectionalLight>) {
    use light_consts::lux;
    if keyboard.just_pressed(KeyCode::Space) {
        let (name, lx) = match sun.illuminance {
            x if x == lux::OVERCAST_DAY => ("日出斜照", lux::CLEAR_SUNRISE),
            x if x == lux::CLEAR_SUNRISE => ("全日光", lux::FULL_DAYLIGHT),
            x if x == lux::FULL_DAYLIGHT => ("烈日直晒", lux::DIRECT_SUNLIGHT),
            _ => ("阴天棚光", lux::OVERCAST_DAY),
        };
        sun.illuminance = lx;
        println!("老烛：{name}，{lx} 勒克斯。");
    }
}
// ANCHOR_END: grade

// ANCHOR: relocate
/// M 键搬太阳：平移对 DirectionalLight 毫无意义，画面纹丝不动
fn relocate(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut sun: Single<&mut Transform, With<DirectionalLight>>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        sun.translation.x += 20.0;
        println!(
            "场记：太阳搬到 x = {:.0} 了——画面一根汗毛都没动。",
            sun.translation.x
        );
    }
}
// ANCHOR_END: relocate

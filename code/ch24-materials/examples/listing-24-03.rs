//! Listing 24-3：自发光暗房——灯箱、亮度阶梯、发光不照亮，外加曝光的账

use bevy::{camera::Exposure, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.012, 0.016)))
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit_camera, dial_exposure, swap_real_lamp))
        .run();
}

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 暗房：整个场里一盏正经灯都没有——发的光全来自材质自己
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 4.2).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.42, 0.44),
            perceptual_roughness: 0.8,
            ..default()
        })),
    ));

    // 戏牌灯箱：底色纯黑，图案全靠 emissive_texture × emissive 点亮
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.4, 1.2, 0.12))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::BLACK,
            emissive: LinearRgba::rgb(80.0, 80.0, 80.0),
            emissive_texture: Some(asset_server.load("textures/lantern_sign.png")),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.6, -1.6),
    ));
    // ANCHOR_END: setup

    // ANCHOR: ladder
    // 亮度阶梯：同一款橙红，1 → 10 → 100 尼特
    let ball = meshes.add(Sphere::new(0.34));
    for (i, nits) in [1.0_f32, 10.0, 100.0].into_iter().enumerate() {
        commands.spawn((
            Mesh3d(ball.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::BLACK,
                emissive: LinearRgba::rgb(1.0, 0.35, 0.12) * nits,
                ..default()
            })),
            Transform::from_xyz(-1.7 + i as f32 * 1.2, 0.6, 0.2),
        ));
    }
    // 隔壁的素坯球：看看 100 尼特的邻居能不能借到光
    commands.spawn((
        Mesh3d(ball.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.72, 0.68),
            ..default()
        })),
        Transform::from_xyz(1.9, 0.6, 0.2),
    ));
    // ANCHOR_END: ladder

    // ANCHOR: weight
    // 曝光实验的两颗球：一样的 80 尼特青色，emissive_exposure_weight 一个 0 一个 1
    for (x, weight) in [(-1.5_f32, 0.0_f32), (1.5, 1.0)] {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.26))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::BLACK,
                emissive: LinearRgba::rgb(0.2, 0.75, 1.0) * 80.0,
                emissive_exposure_weight: weight,
                ..default()
            })),
            Transform::from_xyz(x, 0.5, 1.5),
        ));
    }
    // ANCHOR_END: weight

    println!("小棠：暗房验灯——灯箱一块、橙球三档：1、10、100 尼特。");
    println!("老烛：素坯挨着 100 尼特坐，一丝光都借不着才对。E 拨曝光，L 换真灯。");
}

// ANCHOR: dial
/// E：曝光在“默认 9.7”与“阴天 12”两档间切换（22.2 的测光表）
fn dial_exposure(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    camera: Single<(Entity, Option<&Exposure>), With<Camera3d>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }
    let (entity, exposure) = *camera;
    let now = exposure.map_or(Exposure::EV100_BLENDER, |e| e.ev100);
    let next = if now < 10.0 {
        Exposure::EV100_OVERCAST
    } else {
        Exposure::EV100_BLENDER
    };
    commands.entity(entity).insert(Exposure { ev100: next });
    println!("老烛：曝光拨到 EV100 = {next}——看好两颗青球谁掉了亮。");
}
// ANCHOR_END: dial

// ANCHOR: lamp
/// L：在 100 尼特橙球的位置挂/撤一盏真点光——自发光与灯的分别，当场见
#[derive(Component)]
struct RealLamp;

fn swap_real_lamp(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    lamp: Query<Entity, With<RealLamp>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyL) {
        return;
    }
    if let Ok(entity) = lamp.single() {
        commands.entity(entity).despawn();
        println!("老烛：真灯撤了——各回各的黑。");
    } else {
        commands.spawn((
            RealLamp,
            PointLight {
                color: Color::srgb(1.0, 0.55, 0.25),
                intensity: 60_000.0,
                range: 8.0,
                ..default()
            },
            Transform::from_xyz(0.7, 0.6, 0.2),
        ));
        println!("老烛：同一个位置换一盏真灯——地板、素坯、邻居全亮了。");
    }
}
// ANCHOR_END: lamp

/// 左键拖动转台（Listing 24-1 原样）
fn orbit_camera(
    window: Single<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    mut last_cursor: Local<Option<Vec2>>,
    mut yaw: Local<f32>,
) {
    if mouse.pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            if let Some(prev) = *last_cursor {
                *yaw -= (pos.x - prev.x) * 0.008;
            }
            *last_cursor = Some(pos);
        }
    } else {
        *last_cursor = None;
    }
    let center = Vec3::new(0.0, 0.8, 0.0);
    let seat = center + Vec3::new(4.2 * yaw.sin(), 0.7, 4.2 * yaw.cos());
    **camera = Transform::from_translation(seat).looking_at(center, Vec3::Y);
}

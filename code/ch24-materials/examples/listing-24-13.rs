//! Listing 24-13：晨雾进棚——unlit 不吃光，fog_enabled=false 不吃雾

use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.71, 0.73, 0.77)))
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit_camera, toggle_unlit, toggle_sign_fog))
        .run();
}

/// 提词板材质（U 键拨 unlit）与远处灯箱材质（G 键拨 fog_enabled）
#[derive(Resource)]
struct Board(Handle<StandardMaterial>);

#[derive(Resource)]
struct FarSign(Handle<StandardMaterial>);

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 晨雾：挂在相机上的 DistanceFog——5 米起雾，13 米开外白茫茫
    commands.spawn((
        Camera3d::default(),
        DistanceFog {
            color: Color::srgb(0.71, 0.73, 0.77),
            falloff: FogFalloff::Linear {
                start: 5.0,
                end: 13.0,
            },
            ..default()
        },
        Transform::from_xyz(0.0, 1.6, 4.6).looking_at(Vec3::new(0.0, 0.9, -2.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 4_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.0, 5.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(9.0, 30.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.42, 0.44),
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, -8.0),
    ));

    // 一列金球往雾深处退：3 米、6 米、9 米、12 米
    let plinth = meshes.add(Cylinder::new(0.4, 0.8).mesh().resolution(6));
    let plinth_paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.24, 0.25, 0.28),
        perceptual_roughness: 0.9,
        ..default()
    });
    let gold = materials.add(StandardMaterial {
        base_color: Color::srgb(0.93, 0.76, 0.44),
        metallic: 1.0,
        perceptual_roughness: 0.35,
        ..default()
    });
    let ball = meshes.add(Sphere::new(0.36));
    for i in 0..4 {
        let z = 1.5 - i as f32 * 3.0;
        commands.spawn((
            Mesh3d(plinth.clone()),
            MeshMaterial3d(plinth_paint.clone()),
            Transform::from_xyz(-1.1, 0.4, z),
        ));
        commands.spawn((
            Mesh3d(ball.clone()),
            MeshMaterial3d(gold.clone()),
            Transform::from_xyz(-1.1, 1.16, z),
        ));
    }
    // ANCHOR_END: setup

    // ANCHOR: signs
    // 雾最深处一对灯箱：同款自发光，右边那块 fog_enabled = false
    let sign_image: Handle<Image> = asset_server.load("textures/lantern_sign.png");
    let deep_sign = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: LinearRgba::rgb(60.0, 60.0, 60.0),
        emissive_texture: Some(sign_image.clone()),
        ..default() // fog_enabled 默认 true——老老实实吃雾
    });
    let piercing_sign = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: LinearRgba::rgb(60.0, 60.0, 60.0),
        emissive_texture: Some(sign_image),
        fog_enabled: false, // 雾里也扎眼——招牌、准星就要这待遇
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 0.1))),
        MeshMaterial3d(deep_sign),
        Transform::from_xyz(-1.3, 1.7, -10.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 0.1))),
        MeshMaterial3d(piercing_sign.clone()),
        Transform::from_xyz(1.3, 1.7, -10.5),
    ));
    commands.insert_resource(FarSign(piercing_sign));
    println!("小棠：雾最深处两块灯箱——左边老实吃雾，右边那块不吃。G 键让它也进雾。");
    // ANCHOR_END: signs

    // ANCHOR: board
    // 台口的提词板：U 键在“受光”与“不受光”之间切
    let board = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.87, 0.5),
        unlit: false,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.1, 0.75))),
        MeshMaterial3d(board.clone()),
        Transform::from_xyz(1.4, 1.0, 1.6)
            .with_rotation(Quat::from_rotation_x(1.35) * Quat::from_rotation_z(-0.12)),
    ));
    commands.insert_resource(Board(board));

    println!("老烛：晨雾放好了。U 拨提词板的 unlit——看它还认不认我这盏灯。");
}
// ANCHOR_END: board

// ANCHOR: toggles
/// U：unlit 开关——不吃光，但照样吃雾
fn toggle_unlit(
    keyboard: Res<ButtonInput<KeyCode>>,
    board: Res<Board>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyU) {
        return;
    }
    if let Some(mut material) = materials.get_mut(&board.0) {
        material.unlit = !material.unlit;
        println!("小棠：unlit = {}。", material.unlit);
    }
}

/// G：远灯箱的 fog_enabled 开关
fn toggle_sign_fog(
    keyboard: Res<ButtonInput<KeyCode>>,
    sign: Res<FarSign>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyG) {
        return;
    }
    if let Some(mut material) = materials.get_mut(&sign.0) {
        material.fog_enabled = !material.fog_enabled;
        println!("老烛：右灯箱 fog_enabled = {}。", material.fog_enabled);
    }
}
// ANCHOR_END: toggles

/// 左键拖动转台（Listing 24-1 原样，圆心往雾里挪了挪）
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
    let center = Vec3::new(0.0, 0.9, -2.0);
    let seat = center + Vec3::new(6.6 * yaw.sin(), 0.7, 6.6 * yaw.cos());
    **camera = Transform::from_translation(seat).looking_at(center, Vec3::Y);
}

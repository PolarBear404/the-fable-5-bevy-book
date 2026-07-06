//! Listing 24-7：视差贴图——法线只骗光，视差连位置一起骗

use bevy::image::ImageLoaderSettings;
use bevy::light::Skybox;
use bevy::pbr::ParallaxMappingMethod;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .add_systems(Startup, setup)
        .add_systems(Update, (hang_studio, orbit_camera, dial_parallax))
        .run();
}

/// 右盖的材质句柄，运行时拨参数用
#[derive(Resource)]
struct ParallaxLid(Handle<StandardMaterial>);

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 机位放斜：视差的戏在斜视角里才看得出（垂直俯视时没有偏移可言）
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 3.2).looking_at(Vec3::new(0.0, 0.95, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 4_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 2.2, 1.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.42, 0.44),
            perceptual_roughness: 0.8,
            ..default()
        })),
    ));
    commands.insert_resource(StudioWalls {
        image: asset_server.load("textures/studio_cubemap.png"),
        hung: false,
    });
    // 两张条案，盖子平躺着晾
    let table = meshes.add(Cuboid::new(1.7, 0.9, 1.7));
    let table_paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.24, 0.25, 0.28),
        perceptual_roughness: 0.9,
        ..default()
    });
    for x in [-1.0_f32, 1.0] {
        commands.spawn((
            Mesh3d(table.clone()),
            MeshMaterial3d(table_paint.clone()),
            Transform::from_xyz(x, 0.45, 0.0),
        ));
    }

    // ANCHOR: parallax
    let carve_normal: Handle<Image> = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load("textures/carve_normal.png");
    // 深度图：灰度、线性——白=深、黑=凸，跟直觉正相反
    let carve_depth: Handle<Image> = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load("textures/carve_height.png");

    // 左盖：只有法线贴图（24.5 的成品）
    let normal_only = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.08, 0.06),
        perceptual_roughness: 0.35,
        normal_map_texture: Some(carve_normal.clone()),
        ..default()
    });
    // 右盖：法线之上再加深度图——视差开动
    let parallax = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.08, 0.06),
        perceptual_roughness: 0.35,
        normal_map_texture: Some(carve_normal),
        depth_map: Some(carve_depth),
        parallax_depth_scale: 0.08,
        ..default()
    });
    // ANCHOR_END: parallax

    let lid = Mesh::from(Cuboid::new(1.5, 0.06, 1.5))
        .with_generated_tangents()
        .unwrap();
    let lid = meshes.add(lid);
    commands.spawn((
        Mesh3d(lid.clone()),
        MeshMaterial3d(normal_only),
        Transform::from_xyz(-1.0, 0.93, 0.0),
    ));
    commands.spawn((
        Mesh3d(lid),
        MeshMaterial3d(parallax.clone()),
        Transform::from_xyz(1.0, 0.93, 0.0),
    ));
    commands.insert_resource(ParallaxLid(parallax));

    println!("小棠：左盖只有法线，右盖加了深度图。压低了看——[ ] 拨深浅，M 换算法，N 换层数。");
}
// ANCHOR_END: setup

// ANCHOR: dial
/// [ ]：parallax_depth_scale；M：Occlusion/Relief；N：层数 4/16/64
fn dial_parallax(
    keyboard: Res<ButtonInput<KeyCode>>,
    lid: Res<ParallaxLid>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(mut material) = materials.get_mut(&lid.0) else {
        return;
    };
    if keyboard.just_pressed(KeyCode::BracketLeft) {
        material.parallax_depth_scale = (material.parallax_depth_scale - 0.04).max(0.0);
        println!("小棠：刻痕收浅——parallax_depth_scale = {:.2}", material.parallax_depth_scale);
    }
    if keyboard.just_pressed(KeyCode::BracketRight) {
        material.parallax_depth_scale = (material.parallax_depth_scale + 0.04).min(0.4);
        println!("小棠：刻痕加深——parallax_depth_scale = {:.2}", material.parallax_depth_scale);
    }
    if keyboard.just_pressed(KeyCode::KeyM) {
        material.parallax_mapping_method = match material.parallax_mapping_method {
            ParallaxMappingMethod::Occlusion => ParallaxMappingMethod::Relief { max_steps: 5 },
            ParallaxMappingMethod::Relief { .. } => ParallaxMappingMethod::Occlusion,
        };
        println!("小棠：算法换成 {:?}", material.parallax_mapping_method);
    }
    if keyboard.just_pressed(KeyCode::KeyN) {
        material.max_parallax_layer_count = match material.max_parallax_layer_count as u32 {
            4 => 16.0,
            16 => 64.0,
            _ => 4.0,
        };
        println!("小棠：切层数——max_parallax_layer_count = {}", material.max_parallax_layer_count);
    }
}
// ANCHOR_END: dial

/// 影棚墙（Listing 24-1 原样）
#[derive(Resource)]
struct StudioWalls {
    image: Handle<Image>,
    hung: bool,
}

fn hang_studio(
    mut walls: ResMut<StudioWalls>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    camera: Single<Entity, With<Camera3d>>,
) {
    if walls.hung || !asset_server.load_state(&walls.image).is_loaded() {
        return;
    }
    let mut image = images.get_mut(&walls.image).unwrap();
    if image.texture_descriptor.array_layer_count() == 1 {
        let layers = image.height() / image.width();
        image.reinterpret_stacked_2d_as_array(layers).unwrap();
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }
    commands.entity(*camera).insert((
        Skybox {
            image: Some(walls.image.clone()),
            brightness: 700.0,
            ..default()
        },
        GeneratedEnvironmentMapLight {
            environment_map: walls.image.clone(),
            intensity: 1_000.0,
            ..default()
        },
    ));
    walls.hung = true;
}

/// 左键拖动转台（低轨版：绕着两张条案掠视）
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
    let center = Vec3::new(0.0, 0.95, 0.0);
    let seat = center + Vec3::new(3.2 * yaw.sin(), 1.05, 3.2 * yaw.cos());
    **camera = Transform::from_translation(seat).looking_at(center, Vec3::Y);
}

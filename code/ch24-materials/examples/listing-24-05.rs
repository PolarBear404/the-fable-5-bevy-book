//! Listing 24-5：法线贴图——同一张云纹，左边的盖子是哑巴，右边的开了纹

use bevy::image::ImageLoaderSettings;
use bevy::light::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .add_systems(Startup, setup)
        .add_systems(Update, (hang_studio, orbit_camera))
        .run();
}

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 4.2).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
    ));
    // 主灯压低从左侧扫过来：浮雕要靠掠射光才立得起来
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

    // ANCHOR: lids
    // 法线图是数据不是颜色：一样要按线性读（24.4 的规矩）
    let carve_normal: Handle<Image> = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load("textures/carve_normal.png");
    // 同一罐朱漆、同一张云纹法线，抹给两块盖子
    let lacquer = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.08, 0.06),
        perceptual_roughness: 0.35,
        normal_map_texture: Some(carve_normal),
        ..default()
    });

    // 左盖：图元出厂的坯子直接用——位置、法线、UV 都有，独缺切线
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 0.1))),
        MeshMaterial3d(lacquer.clone()),
        Transform::from_xyz(-0.85, 1.1, 0.0).with_rotation(Quat::from_rotation_x(-0.35)),
    ));
    // 右盖：同一个坯子先“开纹”——生成切线，法线贴图才有了落脚的坐标系
    let carved = Mesh::from(Cuboid::new(1.5, 1.5, 0.1))
        .with_generated_tangents()
        .unwrap();
    commands.spawn((
        Mesh3d(meshes.add(carved)),
        MeshMaterial3d(lacquer),
        Transform::from_xyz(0.85, 1.1, 0.0).with_rotation(Quat::from_rotation_x(-0.35)),
    ));
    // ANCHOR_END: lids

    println!("小棠：两块盖子，同一罐朱漆、同一张云纹法线，一字排开。");
    println!("老鲁：左边这块平得像新刨的板——纹呢？我这就去查坯子。");
}
// ANCHOR_END: setup

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

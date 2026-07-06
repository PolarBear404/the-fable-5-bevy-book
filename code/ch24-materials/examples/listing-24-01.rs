//! Listing 24-1：样品间开张——影棚环境 + 拖动转台，三颗熟面孔的球先坐台

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
    // 主灯：一盏斜上方的平行光，负责高光与影子；氛围交给影棚墙
    commands.spawn((
        DirectionalLight {
            illuminance: 5_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.0, 5.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 台面与三座六棱展台
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.42, 0.44),
            perceptual_roughness: 0.8,
            ..default()
        })),
    ));
    let plinth = meshes.add(Cylinder::new(0.5, 0.9).mesh().resolution(6));
    let plinth_paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.24, 0.25, 0.28),
        perceptual_roughness: 0.9,
        ..default()
    });
    for x in [-1.6_f32, 0.0, 1.6] {
        commands.spawn((
            Mesh3d(plinth.clone()),
            MeshMaterial3d(plinth_paint.clone()),
            Transform::from_xyz(x, 0.45, 0.0),
        ));
    }

    // 三颗熟球：素坯、亮瓷、镜面金——21.3 材质墙的三个角，换个世界再看一眼
    let ball = meshes.add(Sphere::new(0.42));
    commands.spawn((
        Mesh3d(ball.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.72, 0.68),
            ..default()
        })),
        Transform::from_xyz(-1.6, 1.32, 0.0),
    ));
    commands.spawn((
        Mesh3d(ball.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.93, 0.93, 0.90),
            perceptual_roughness: 0.089,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.32, 0.0),
    ));
    commands.spawn((
        Mesh3d(ball),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.82, 0.55),
            metallic: 1.0,
            perceptual_roughness: 0.05,
            ..default()
        })),
        Transform::from_xyz(1.6, 1.32, 0.0),
    ));

    // 影棚墙是一张竖条 cubemap（6 面 256²），到货再挂
    commands.insert_resource(StudioWalls {
        image: asset_server.load("textures/studio_cubemap.png"),
        hung: false,
    });

    println!("小棠：样品间开张——先请三位老面孔坐台：素坯、亮瓷、镜面金。");
    println!("老雷：道具单在此——琉璃盏、鎏金锣、剔红漆盒、纱幕、灯箱，一样都不能含糊。");
}
// ANCHOR_END: setup

// ANCHOR: studio
/// 影棚墙：提货单在手，货到再挂（22.9 的星空天幕手法，原样照搬）
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
        // 画墙：柔光箱与灰墙成为画面背景
        Skybox {
            image: Some(walls.image.clone()),
            brightness: 700.0,
            ..default()
        },
        // 让墙发光：金属、清漆、玻璃都靠它有东西可照
        GeneratedEnvironmentMapLight {
            environment_map: walls.image.clone(),
            intensity: 1_000.0,
            ..default()
        },
    ));
    walls.hung = true;
    println!("场记：影棚墙挂好了——镜面金这回有的照了。");
}
// ANCHOR_END: studio

// ANCHOR: orbit
/// 左键拖动转台（23.11 的手法）：cursor_position 差分，机位吊在圆轨上
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
// ANCHOR_END: orbit

//! Listing 24-9：各向异性——把圆高光拉成拉丝纹，锣心的旋纹感

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
    commands.spawn((
        DirectionalLight {
            illuminance: 5_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.0, 5.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
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

    let plinth = meshes.add(Cylinder::new(0.42, 0.8).mesh().resolution(6));
    let plinth_paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.24, 0.25, 0.28),
        perceptual_roughness: 0.9,
        ..default()
    });

    // ANCHOR: aniso
    // 各向异性也在切线坐标系里干活——球坯照样要先开纹
    let ball = meshes.add(
        Mesh::from(Sphere::new(0.38))
            .with_generated_tangents()
            .unwrap(),
    );
    // 同一款抛光铜：拉丝力度 0 → 0.6 → 1.0，最右那颗把丝路拧转 90°
    let lineup: [(f32, f32, &str); 4] = [
        (0.0, 0.0, "不拉丝"),
        (0.6, 0.0, "半拉丝"),
        (1.0, 0.0, "全拉丝"),
        (1.0, std::f32::consts::FRAC_PI_2, "全拉丝拧转 90°"),
    ];
    for (i, (strength, rotation, name)) in lineup.into_iter().enumerate() {
        let x = -2.4 + i as f32 * 1.6;
        commands.spawn((
            Mesh3d(plinth.clone()),
            MeshMaterial3d(plinth_paint.clone()),
            Transform::from_xyz(x, 0.4, 0.0),
        ));
        commands.spawn((
            Mesh3d(ball.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.93, 0.76, 0.44),
                metallic: 1.0,
                perceptual_roughness: 0.45,
                anisotropy_strength: strength,
                anisotropy_rotation: rotation,
                ..default()
            })),
            Transform::from_xyz(x, 1.18, 0.0),
        ));
        println!(
            "小棠：{}——anisotropy_strength {}，rotation {:.2}。",
            name, strength, rotation
        );
    }
    // ANCHOR_END: aniso
}

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

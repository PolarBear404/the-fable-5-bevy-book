//! Listing 24-2：反光度一排五档 + 高光染色的金属/非金属对照

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

    let plinth = meshes.add(Cylinder::new(0.38, 0.7).mesh().resolution(6));
    let plinth_tall = meshes.add(Cylinder::new(0.44, 1.1).mesh().resolution(6));
    let plinth_paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.24, 0.25, 0.28),
        perceptual_roughness: 0.9,
        ..default()
    });
    let ball = meshes.add(Sphere::new(0.34));

    // ANCHOR: row
    // 前排：同一款白瓷，只拧 reflectance——0.0 到 1.0 五档
    for (i, reflectance) in [0.0_f32, 0.25, 0.5, 0.75, 1.0].into_iter().enumerate() {
        let x = -2.4 + i as f32 * 1.2;
        commands.spawn((
            Mesh3d(plinth.clone()),
            MeshMaterial3d(plinth_paint.clone()),
            Transform::from_xyz(x, 0.35, 0.0),
        ));
        commands.spawn((
            Mesh3d(ball.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.93, 0.91, 0.88),
                perceptual_roughness: 0.12,
                reflectance,
                ..default()
            })),
            Transform::from_xyz(x, 1.04, 0.0),
        ));
    }
    // ANCHOR_END: row

    // ANCHOR: tint
    // 后排 2×2 对照：黑釉与银金属各来一对，右手边的那颗抹石绿 specular_tint
    // ——黑釉压暗漫反射，镜面反光才看得真
    let green = Color::srgb(0.25, 0.85, 0.45);
    for (i, (metallic, tint)) in [
        (0.0_f32, None),        // 黑釉·素
        (0.0, Some(green)),     // 黑釉·石绿
        (1.0, None),            // 银·素
        (1.0, Some(green)),     // 银·石绿
    ]
    .into_iter()
    .enumerate()
    {
        let x = -2.25 + i as f32 * 1.5;
        commands.spawn((
            Mesh3d(plinth_tall.clone()),
            MeshMaterial3d(plinth_paint.clone()),
            Transform::from_xyz(x, 0.55, -1.8),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.42))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: if metallic > 0.5 {
                    Color::srgb(0.92, 0.92, 0.92)
                } else {
                    Color::BLACK
                },
                metallic,
                perceptual_roughness: 0.05,
                reflectance: 1.0,
                specular_tint: tint.unwrap_or(Color::WHITE),
                ..default()
            })),
            Transform::from_xyz(x, 1.66, -1.8),
        ));
    }
    // ANCHOR_END: tint

    println!("小棠：前排白瓷五连——反光度从 0 拧到 1，高光从无到扎眼。");
    println!("小棠：后排黑釉一对、银器一对，每对右手那颗抹了石绿——猜猜谁认这罐染料？");
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

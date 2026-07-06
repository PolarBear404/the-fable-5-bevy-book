//! Listing 24-8：清漆——糙底子罩一层亮壳，剔红漆器的质感一步到位

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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 4.6).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
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

    let plinth = meshes.add(Cylinder::new(0.5, 0.9).mesh().resolution(6));
    let plinth_paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.24, 0.25, 0.28),
        perceptual_roughness: 0.9,
        ..default()
    });
    let ball = meshes.add(Sphere::new(0.42));

    // ANCHOR: coat
    // 三颗同一款糙朱底：素身 → 罩亮清漆 → 罩哑清漆
    let coats: [(f32, f32, &str); 3] = [
        (0.0, 0.08, "素身"),
        (1.0, 0.08, "亮清漆"),
        (1.0, 0.45, "哑清漆"),
    ];
    for (i, (clearcoat, coat_roughness, name)) in coats.into_iter().enumerate() {
        let x = -2.4 + i as f32 * 1.6;
        commands.spawn((
            Mesh3d(plinth.clone()),
            MeshMaterial3d(plinth_paint.clone()),
            Transform::from_xyz(x, 0.45, 0.0),
        ));
        commands.spawn((
            Mesh3d(ball.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.52, 0.08, 0.05),
                perceptual_roughness: 0.85, // 底漆很糙——这是故意的
                clearcoat,
                clearcoat_perceptual_roughness: coat_roughness,
                ..default()
            })),
            Transform::from_xyz(x, 1.32, 0.0),
        ));
        println!("小棠：{}号台，{}——clearcoat {}，罩面粗糙度 {}。", i + 1, name, clearcoat, coat_roughness);
    }
    // ANCHOR_END: coat

    // ANCHOR: carved_coat
    // 四号台：雕花底 + 亮清漆——主法线贴图只拱底层，漆膜照旧平滑
    let carve_normal: Handle<Image> = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load("textures/carve_normal.png");
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 0.9).mesh().resolution(6))),
        MeshMaterial3d(plinth_paint.clone()),
        Transform::from_xyz(2.4, 0.45, 0.0),
    ));
    commands.spawn((
        Mesh3d(
            meshes.add(
                Mesh::from(Sphere::new(0.42))
                    .with_generated_tangents()
                    .unwrap(),
            ),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.52, 0.08, 0.05),
            perceptual_roughness: 0.85,
            normal_map_texture: Some(carve_normal),
            clearcoat: 1.0,
            clearcoat_perceptual_roughness: 0.08,
            ..default()
        })),
        Transform::from_xyz(2.4, 1.32, 0.0),
    ));
    println!("小棠：4号台，雕花罩亮漆——花纹在漆膜底下，膜面自己是平的。");
    // ANCHOR_END: carved_coat

    println!("老雷：二号台才是剔红的相——底子糙着，光却亮得起来。");
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

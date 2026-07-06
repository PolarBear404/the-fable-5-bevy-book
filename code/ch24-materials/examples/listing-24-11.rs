//! Listing 24-11：琉璃盏——specular_transmission 折射全家，与便宜的 diffuse_transmission 对照

use bevy::light::Skybox;
use bevy::pbr::ScreenSpaceTransmission;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .add_systems(Startup, setup)
        .add_systems(Update, (hang_studio, orbit_camera, dial_glass))
        .run();
}

/// 琉璃盏的材质句柄，运行时拨参数
#[derive(Resource)]
struct Glass(Handle<StandardMaterial>);

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

    let plinth_paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.24, 0.25, 0.28),
        perceptual_roughness: 0.9,
        ..default()
    });

    // 幕后布景：锈锣正对玻璃身后，竹影纱（Blend）斜在左后——待会儿都要隔着玻璃看
    let bamboo: Handle<Image> = asset_server.load("textures/bamboo_alpha.png");
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.6, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(bamboo),
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            double_sided: true,
            ..default()
        })),
        Transform::from_xyz(-1.0, 1.0, -1.5).with_rotation(Quat::from_rotation_x(1.45)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.44, 0.5).mesh().resolution(6))),
        MeshMaterial3d(plinth_paint.clone()),
        Transform::from_xyz(0.15, 0.25, -1.3),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.4))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.55, 0.2),
            metallic: 1.0,
            perceptual_roughness: 0.35,
            ..default()
        })),
        Transform::from_xyz(0.15, 0.9, -1.3),
    ));

    // ANCHOR: glass
    // 琉璃盏本盏：specular_transmission 一开，光走折射
    let glass = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        specular_transmission: 1.0,
        thickness: 0.36,            // 有厚度才有透镜感（0 = 无限薄膜）
        ior: 1.52,                  // 窗玻璃
        perceptual_roughness: 0.05, // 磨砂度：拨给 R 键
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 0.4).mesh().resolution(6))),
        MeshMaterial3d(plinth_paint.clone()),
        Transform::from_xyz(0.0, 0.2, 0.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.45))),
        MeshMaterial3d(glass.clone()),
        Transform::from_xyz(0.0, 0.86, 0.5),
    ));
    commands.insert_resource(Glass(glass));
    // 相机上的折射底片设置：steps=1 默认就在（Camera3d 的 required component）
    // ANCHOR_END: glass

    // ANCHOR: lantern
    // 对照组：纸灯笼一对——diffuse_transmission 让背面的光洇过来，便宜得多
    let backlight = |x: f32| {
        (
            PointLight {
                color: Color::srgb(1.0, 0.75, 0.4),
                intensity: 40_000.0,
                range: 4.0,
                ..default()
            },
            Transform::from_xyz(x, 0.92, -0.9),
        )
    };
    commands.spawn(backlight(-2.3));
    commands.spawn(backlight(2.3));
    let lantern_plinth = meshes.add(Cylinder::new(0.36, 0.6).mesh().resolution(6));
    for (x, diffuse_transmission, name) in [(-2.3_f32, 0.9, "纸灯笼"), (2.3, 0.0, "瓷灯笼")] {
        commands.spawn((
            Mesh3d(lantern_plinth.clone()),
            MeshMaterial3d(plinth_paint.clone()),
            Transform::from_xyz(x, 0.3, -0.4),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.32))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.98, 0.92, 0.82),
                perceptual_roughness: 0.9,
                diffuse_transmission,
                ..default()
            })),
            Transform::from_xyz(x, 0.92, -0.4),
        ));
        println!("小棠：{}——diffuse_transmission {}，灯在它身后。", name, diffuse_transmission);
    }
    // ANCHOR_END: lantern

    println!("小棠：琉璃盏当中坐，I 拨折射率，T 拨厚度，R 拨磨砂，S 拨底片步数。");
}
// ANCHOR_END: setup

// ANCHOR: dial
/// I/T/R：折射率、厚度、磨砂；S：ScreenSpaceTransmission 的 steps
fn dial_glass(
    keyboard: Res<ButtonInput<KeyCode>>,
    glass: Res<Glass>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transmission: Single<&mut ScreenSpaceTransmission, With<Camera3d>>,
) {
    let Some(mut material) = materials.get_mut(&glass.0) else {
        return;
    };
    if keyboard.just_pressed(KeyCode::KeyI) {
        material.ior = match material.ior {
            x if x < 1.2 => 1.33,
            x if x < 1.4 => 1.52,
            x if x < 2.0 => 2.42,
            _ => 1.0,
        };
        println!("小棠：ior = {}（1.0 空气 / 1.33 水 / 1.52 玻璃 / 2.42 钻石）", material.ior);
    }
    if keyboard.just_pressed(KeyCode::KeyT) {
        material.thickness = match material.thickness {
            x if x < 0.1 => 0.36,
            x if x < 0.5 => 0.9,
            _ => 0.0,
        };
        println!("小棠：thickness = {}（0 = 无限薄的膜，不弯光）", material.thickness);
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        material.perceptual_roughness = match material.perceptual_roughness {
            x if x < 0.1 => 0.25,
            x if x < 0.3 => 0.5,
            _ => 0.05,
        };
        println!("小棠：perceptual_roughness = {}（磨砂玻璃）", material.perceptual_roughness);
    }
    if keyboard.just_pressed(KeyCode::KeyS) {
        transmission.steps = match transmission.steps {
            0 => 1,
            1 => 3,
            _ => 0,
        };
        println!("老烛：底片 steps = {}（0 = 不抄画面，只折环境图）", transmission.steps);
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

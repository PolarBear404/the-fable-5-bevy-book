//! Listing 24-10：alpha_mode 一字排开——纱幕四态、加法幽灵、乘法茶镜，外加一个哑巴坑

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
        Transform::from_xyz(0.0, 1.7, 5.4).looking_at(Vec3::new(0.0, 0.85, 0.0), Vec3::Y),
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

    // ANCHOR: curtains
    // 竹影纱：同一张带 alpha 的竹枝图，裁四幅，只换 alpha_mode
    let bamboo: Handle<Image> = asset_server.load("textures/bamboo_alpha.png");
    let curtain = meshes.add(Plane3d::default().mesh().size(1.15, 1.5));
    let modes: [(AlphaMode, &str); 4] = [
        (AlphaMode::Opaque, "Opaque"),
        (AlphaMode::Mask(0.5), "Mask(0.5)"),
        (AlphaMode::Blend, "Blend"),
        (AlphaMode::AlphaToCoverage, "AlphaToCoverage"),
    ];
    for (i, (alpha_mode, name)) in modes.into_iter().enumerate() {
        let x = -2.55 + i as f32 * 1.7;
        commands.spawn((
            Mesh3d(curtain.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(bamboo.clone()),
                alpha_mode,
                // 纱幕两面都要能看：关掉背面剔除，顺手把背面打光也修对（24.11）
                cull_mode: None,
                double_sided: true,
                ..default()
            })),
            // Plane3d 的正面朝 +Y，立起来面向机位
            Transform::from_xyz(x, 1.15, 0.0).with_rotation(Quat::from_rotation_x(1.35)),
        ));
        println!("小棠：{} 号纱幕——alpha_mode::{name}。", i + 1);
    }
    // ANCHOR_END: curtains

    // ANCHOR: add_multiply
    // 加法幽灵：黑处不留痕，亮处叠光——放在暗台前看最真
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.34))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.65, 1.0),
            alpha_mode: AlphaMode::Add,
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(-1.2, 0.55, 1.7),
    ));
    // 乘法茶镜：白处不留痕，照谁谁变深——琥珀色的滤光片
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(0.95, 0.75))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.55, 0.28),
            alpha_mode: AlphaMode::Multiply,
            cull_mode: None,
            double_sided: true,
            ..default()
        })),
        Transform::from_xyz(1.2, 0.75, 1.7).with_rotation(Quat::from_rotation_x(1.35)),
    ));
    // ANCHOR_END: add_multiply

    // ANCHOR: trap
    // 同一款半透明白：from() 会替你把 alpha_mode 换成 Blend，struct 字面量不会
    let ghost_ok = materials.add(StandardMaterial::from(Color::srgba(0.9, 0.92, 1.0, 0.35)));
    let ghost_solid = materials.add(StandardMaterial {
        base_color: Color::srgba(0.9, 0.92, 1.0, 0.35),
        ..default()
    });
    for (x, material) in [(-0.35_f32, ghost_ok), (0.35, ghost_solid)] {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.26))),
            MeshMaterial3d(material),
            Transform::from_xyz(x, 0.5, 2.3),
        ));
    }
    // ANCHOR_END: trap

    println!("小棠：前排左幽灵右茶镜；居中一对“同款半透明”——右边那颗怎么是实心的？");
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
    let center = Vec3::new(0.0, 0.85, 0.0);
    let seat = center + Vec3::new(5.4 * yaw.sin(), 0.85, 5.4 * yaw.cos());
    **camera = Transform::from_translation(seat).looking_at(center, Vec3::Y);
}

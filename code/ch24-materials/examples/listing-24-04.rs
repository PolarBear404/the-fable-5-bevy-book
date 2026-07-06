//! Listing 24-4：金属度/粗糙度贴图接管——先踩“贴了没反应”的坑，再补 AO

use bevy::image::ImageLoaderSettings;
use bevy::light::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .add_systems(Startup, setup)
        .add_systems(Update, (hang_studio, orbit_camera, next_stage))
        .run();
}

/// 三个阶段的材质轮换：坑 → 接管 → 补 AO
#[derive(Resource)]
struct GongStages {
    stage: usize,
    handles: [Handle<StandardMaterial>; 3],
}

#[derive(Component)]
struct Gong;

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

    // ANCHOR: textures
    // 底色图走默认（sRGB 彩图）；ORM 图是数据不是颜色，必须按线性读
    let base: Handle<Image> = asset_server.load("textures/gong_base.png");
    let orm: Handle<Image> = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load("textures/gong_orm.png");

    // 阶段甲：两张图都贴上了，标量却留默认——金属度 0 × 贴图 = 白贴
    let naive = materials.add(StandardMaterial {
        base_color_texture: Some(base.clone()),
        metallic_roughness_texture: Some(orm.clone()),
        ..default()
    });
    // 阶段乙：标量拨到 1.0，把话语权全交给贴图
    let takeover = materials.add(StandardMaterial {
        base_color_texture: Some(base.clone()),
        metallic_roughness_texture: Some(orm.clone()),
        metallic: 1.0,
        perceptual_roughness: 1.0,
        ..default()
    });
    // 阶段丙：同一张图再兼任 AO——R 通道把锈坑里的环境光压暗
    let with_ao = materials.add(StandardMaterial {
        base_color_texture: Some(base),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
        metallic: 1.0,
        perceptual_roughness: 1.0,
        ..default()
    });
    // ANCHOR_END: textures

    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.62, 1.0).mesh().resolution(6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.24, 0.25, 0.28),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        Gong,
        Mesh3d(meshes.add(Sphere::new(0.58))),
        MeshMaterial3d(naive.clone()),
        Transform::from_xyz(0.0, 1.6, 0.0),
    ));
    commands.insert_resource(GongStages {
        stage: 0,
        handles: [naive, takeover, with_ao],
    });

    println!("小棠：旧铜锣坯来了——底色一张、ORM 一张，全贴上了。空格换阶段。");
    println!("老雷：说好的鎏金呢？这锣怎么一身塑料相？");
}
// ANCHOR_END: setup

// ANCHOR: stage
/// 空格：坑 → 接管 → 补 AO 三阶段轮换
fn next_stage(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut stages: ResMut<GongStages>,
    mut gong: Single<&mut MeshMaterial3d<StandardMaterial>, With<Gong>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    stages.stage = (stages.stage + 1) % 3;
    gong.0 = stages.handles[stages.stage].clone();
    match stages.stage {
        0 => println!("小棠：阶段甲——标量全默认。金属度 0 乘什么都是 0，贴图白贴。"),
        1 => println!("小棠：阶段乙——标量拨到 1，贴图接管：铜是铜，锈是锈。"),
        _ => println!("小棠：阶段丙——同一张图兼任 AO，锈坑里的天光被按了下去。"),
    }
}
// ANCHOR_END: stage

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

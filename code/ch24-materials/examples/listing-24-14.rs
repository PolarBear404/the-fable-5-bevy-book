//! Listing 24-14：UV 的手脚——uv_transform 修倒旗、铺瓦片

use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::light::Skybox;
use bevy::math::Affine2;
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

    // ANCHOR: flip
    // 21.3 的老疑案重演：Cuboid 正面的雷字是倒的
    let banner: Handle<Image> = asset_server.load("textures/banner.png");
    let box_mesh = meshes.add(Cuboid::new(1.1, 1.1, 1.1));
    commands.spawn((
        Mesh3d(box_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(banner.clone()),
            ..default()
        })),
        Transform::from_xyz(-1.5, 1.0, 0.0),
    ));
    // 这回不动坯子，动材质：采样前先给 UV 上下翻个身
    commands.spawn((
        Mesh3d(box_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(banner),
            uv_transform: StandardMaterial::FLIP_VERTICAL,
            ..default()
        })),
        Transform::from_xyz(1.5, 1.0, 0.0),
    ));
    println!("小棠：左箱原样——雷字倒着；右箱 FLIP_VERTICAL——不动坯子，字正了。");
    // ANCHOR_END: flip

    // ANCHOR: tile
    // 铺瓦片：uv_transform 放大 3×2，配上会“转圈”的采样器才真的重复。
    // 注意路径是 banner_tile.png——同一张图的另一份拷贝：上面的箱子已经按默认
    // settings 装载过 banner.png，一条路径只认一套 settings（23.5 的规矩）
    let banner_repeat: Handle<Image> = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..default()
            });
        })
        .load("textures/banner_tile.png");
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(3.3, 2.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(banner_repeat),
            uv_transform: Affine2::from_scale(Vec2::new(3.0, 2.0)),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.4, -2.2).with_rotation(Quat::from_rotation_x(1.5)),
    ));
    println!("小棠：后墙铺了一面 3×2 的旗瓦——UV 放大三倍，出界的部分靠 Repeat 采样兜回来。");
    // ANCHOR_END: tile
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

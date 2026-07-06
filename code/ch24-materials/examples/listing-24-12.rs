//! Listing 24-12：双面旗与贴脸戏报——cull_mode、double_sided 与 depth_bias

use bevy::light::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{Face, TextureViewDescriptor, TextureViewDimension};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .add_systems(Startup, setup)
        .add_systems(Update, (hang_studio, orbit_camera, flip_sides, nudge_poster))
        .run();
}

/// 两面旗共用的材质，F 键换三档
#[derive(Resource)]
struct Flag {
    handle: Handle<StandardMaterial>,
    stage: usize,
}

/// 戏报的材质，B 键拨 depth_bias
#[derive(Resource)]
struct Poster(Handle<StandardMaterial>);

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

    // ANCHOR: flags
    // 一杆双旗：右边这面正对机位，左边那面拧了 180°——你看到的是它的背面
    let banner: Handle<Image> = asset_server.load("textures/banner.png");
    let flag_material = materials.add(StandardMaterial {
        base_color_texture: Some(banner),
        // 出厂默认：cull_mode = Some(Face::Back)，double_sided = false
        ..default()
    });
    let flag = meshes.add(Plane3d::default().mesh().size(1.5, 1.0));
    for (x, yaw, name) in [(0.85_f32, 0.0_f32, "正面旗"), (-0.85, std::f32::consts::PI, "背面旗")] {
        commands.spawn((
            Mesh3d(flag.clone()),
            MeshMaterial3d(flag_material.clone()),
            Transform::from_xyz(x, 1.15, 0.0)
                .with_rotation(Quat::from_rotation_y(yaw) * Quat::from_rotation_x(1.5)),
        ));
        println!("小棠：{name}挂好。");
    }
    commands.insert_resource(Flag {
        handle: flag_material,
        stage: 0,
    });
    // ANCHOR_END: flags

    // ANCHOR: poster
    // 右手边一块告示板，戏报贴在板面上——纸没有厚度，贴上去就是共面
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.9, 1.35, 0.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.32, 0.24, 0.18),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(2.2, 1.15, -0.6),
    ));
    let poster_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.86, 0.79, 0.62),
        perceptual_roughness: 0.9,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.4, 0.95))),
        MeshMaterial3d(poster_material.clone()),
        // 板厚 0.1，前脸在 z = -0.55——戏报的平面严丝合缝贴上去
        Transform::from_xyz(2.2, 1.15, -0.55).with_rotation(Quat::from_rotation_x(1.570_796_3)),
    ));
    commands.insert_resource(Poster(poster_material));
    // ANCHOR_END: poster

    println!("老雷：左边的旗呢？！——按 F 拨旗的三档，按 B 给戏报垫纸。");
}
// ANCHOR_END: setup

// ANCHOR: flip
/// F：出厂默认 → 关剔除 → 关剔除 + 双面光照
fn flip_sides(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut flag: ResMut<Flag>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyF) {
        return;
    }
    flag.stage = (flag.stage + 1) % 3;
    let Some(mut material) = materials.get_mut(&flag.handle) else {
        return;
    };
    match flag.stage {
        0 => {
            material.cull_mode = Some(Face::Back);
            material.double_sided = false;
            println!("小棠：一档，出厂默认——背面剔除，旗从背后看直接没了。");
        }
        1 => {
            material.cull_mode = None;
            material.double_sided = false;
            println!("小棠：二档，cull_mode = None——背面画出来了，可它借的是正面的法线，黑着脸。");
        }
        _ => {
            material.double_sided = true;
            println!("小棠：三档，再加 double_sided——着色器替背面把法线翻个身，两面都受光。");
        }
    }
}
// ANCHOR_END: flip

// ANCHOR: bias
/// B：depth_bias 0 ↔ 2——共面打架与垫纸
fn nudge_poster(
    keyboard: Res<ButtonInput<KeyCode>>,
    poster: Res<Poster>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyB) {
        return;
    }
    let Some(mut material) = materials.get_mut(&poster.0) else {
        return;
    };
    material.depth_bias = if material.depth_bias == 0.0 { 2.0 } else { 0.0 };
    println!("小棠：depth_bias = {}——{}", material.depth_bias,
        if material.depth_bias > 0.0 { "垫了纸，戏报稳稳浮在台面上。" } else { "撤了纸，看它俩打架。" });
}
// ANCHOR_END: bias

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

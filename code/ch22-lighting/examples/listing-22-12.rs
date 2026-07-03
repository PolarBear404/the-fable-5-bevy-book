//! Listing 22-12：镜厅——两只反射探针各管半间屋，左右键推镜球，P 键关视差校正

use bevy::light::ParallaxCorrection;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};

/// 两张厅堂 cubemap：货到再立探针
#[derive(Resource)]
struct HallMaps {
    warm: Handle<Image>,
    cool: Handle<Image>,
    placed: bool,
}

/// 标记：台上被推来推去的镜球
#[derive(Component)]
struct MirrorBall;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight {
            brightness: 40.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.03, 0.03, 0.05)))
        .add_systems(Startup, setup)
        .add_systems(Update, (place_probes, push_ball, toggle_parallax))
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
        Transform::from_xyz(0.0, 3.4, 11.0).looking_at(Vec3::new(0.0, 1.4, 0.0), Vec3::Y),
    ));

    // 一间横着的长厅：左半边冰厅（青蓝），右半边暖阁（朱红）
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(16.0, 8.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.43, 0.40),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));
    let cool_wall = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.33, 0.43),
        perceptual_roughness: 0.85,
        ..default()
    });
    let warm_wall = materials.add(StandardMaterial {
        base_color: Color::srgb(0.59, 0.20, 0.16),
        perceptual_roughness: 0.85,
        ..default()
    });
    // 背墙两段 + 两侧山墙
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.0, 4.0, 0.2))),
        MeshMaterial3d(cool_wall.clone()),
        Transform::from_xyz(-4.0, 2.0, -4.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.0, 4.0, 0.2))),
        MeshMaterial3d(warm_wall.clone()),
        Transform::from_xyz(4.0, 2.0, -4.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.2, 4.0, 8.0))),
        MeshMaterial3d(cool_wall.clone()),
        Transform::from_xyz(-8.0, 2.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.2, 4.0, 8.0))),
        MeshMaterial3d(warm_wall.clone()),
        Transform::from_xyz(8.0, 2.0, 0.0),
    ));

    // 镜球：吃探针环境光的主角
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.9))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.93, 0.88),
            metallic: 1.0,
            perceptual_roughness: 0.05,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.4, 0.0),
        MirrorBall,
    ));

    commands.insert_resource(HallMaps {
        warm: asset_server.load("textures/hall_warm_cubemap.png"),
        cool: asset_server.load("textures/hall_cool_cubemap.png"),
        placed: false,
    });

    println!("老烛：镜厅两头各立一只探针。左右键推球，P 键关视差校正。");
}

/// 竖条 PNG 到货后裁成 cubemap（与 22.9 节同一套手艺）
fn as_cubemap(image: &mut Image) {
    if image.texture_descriptor.array_layer_count() == 1 {
        let layers = image.height() / image.width();
        image.reinterpret_stacked_2d_as_array(layers).unwrap();
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }
}

// ANCHOR: probes
/// 两张厅堂图到货后，各立一只反射探针：LightProbe 划地盘，
/// GeneratedEnvironmentMapLight 现场滤波，视差校正默认自动打开
fn place_probes(
    mut halls: ResMut<HallMaps>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    if halls.placed
        || !asset_server.load_state(&halls.warm).is_loaded()
        || !asset_server.load_state(&halls.cool).is_loaded()
    {
        return;
    }
    as_cubemap(&mut images.get_mut(&halls.cool).unwrap());
    as_cubemap(&mut images.get_mut(&halls.warm).unwrap());

    // 探针的地盘是单位立方，靠 Transform 撑大：一只罩左半厅，一只罩右半厅。
    // falloff 是两只探针交界处的过渡带宽度
    commands.spawn((
        LightProbe {
            falloff: Vec3::splat(0.3),
        },
        GeneratedEnvironmentMapLight {
            environment_map: halls.cool.clone(),
            intensity: 5000.0,
            ..default()
        },
        Transform::from_xyz(-4.0, 2.0, 0.0).with_scale(Vec3::new(8.0, 4.0, 8.0)),
    ));
    commands.spawn((
        LightProbe {
            falloff: Vec3::splat(0.3),
        },
        GeneratedEnvironmentMapLight {
            environment_map: halls.warm.clone(),
            intensity: 5000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 2.0, 0.0).with_scale(Vec3::new(8.0, 4.0, 8.0)),
    ));
    halls.placed = true;
    println!("场记：探针立好了——球在哪半间，就照见哪半间的墙。");
}
// ANCHOR_END: probes

/// 左右键推球，从冰厅推到暖阁
fn push_ball(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ball: Single<&mut Transform, With<MirrorBall>>,
) {
    let mut dir = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) {
        dir -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        dir += 1.0;
    }
    ball.translation.x = (ball.translation.x + dir * 3.0 * time.delta_secs()).clamp(-6.5, 6.5);
}

// ANCHOR: parallax
/// P 键：视差校正开（Auto，按探针地盘算反射）/ 关（None，当天边无穷远）
fn toggle_parallax(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut probes: Query<&mut ParallaxCorrection, With<LightProbe>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        let mut now_auto = false;
        for mut correction in &mut probes {
            *correction = match *correction {
                ParallaxCorrection::None => ParallaxCorrection::Auto,
                _ => ParallaxCorrection::None,
            };
            now_auto = matches!(*correction, ParallaxCorrection::Auto);
        }
        println!(
            "老烛：视差校正{}。",
            if now_auto {
                "开——反射贴着墙走，球挪一步，镜里的窗棂跟着换位置"
            } else {
                "关——墙被当成天边，球怎么挪，镜里都是同一幅画"
            }
        );
    }
}
// ANCHOR_END: parallax

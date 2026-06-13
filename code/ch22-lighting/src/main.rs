//! Listing 22-11：昼夜光照切换台——空格在黎明、正午、黄昏、入夜之间轮换

use bevy::{
    light::CascadeShadowConfigBuilder,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        view::Hdr,
    },
};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (switch_phase, apply_phase, reinterpret_cubemap))
        .run();
}

// ANCHOR: phase
/// 一档光照：天色、太阳的方向/颜色/强度、环境光、雾、夜灯开关，全在一处定好
#[derive(Clone, Copy)]
struct Phase {
    name: &'static str,
    sky: Color,
    sun_dir: Vec3,
    sun_color: Color,
    sun_lux: f32,
    ambient: Color,
    ambient_brightness: f32,
    fog: Color,
    fog_density: f32,
    lamps_on: bool,
}

/// 切换台：四档预设 + 当前档位
#[derive(Resource)]
struct Console {
    phases: Vec<Phase>,
    current: usize,
}
// ANCHOR_END: phase

#[derive(Component)]
struct Sun;

#[derive(Component)]
struct NightLamp;

#[derive(Resource)]
struct Surroundings {
    image: Handle<Image>,
    assembled: bool,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: presets
    let phases = vec![
        Phase {
            name: "黎明",
            sky: Color::srgb(0.42, 0.40, 0.46),
            sun_dir: Vec3::new(-0.85, -0.18, -0.5),
            sun_color: Color::srgb(1.0, 0.72, 0.55),
            sun_lux: light_consts::lux::CLEAR_SUNRISE,
            ambient: Color::srgb(0.55, 0.6, 0.85),
            ambient_brightness: 140.0,
            fog: Color::srgb(0.72, 0.66, 0.7),
            fog_density: 0.03,
            lamps_on: false,
        },
        Phase {
            name: "正午",
            sky: Color::srgb(0.52, 0.66, 0.82),
            sun_dir: Vec3::new(-0.3, -0.92, -0.25),
            sun_color: Color::srgb(1.0, 0.98, 0.94),
            sun_lux: light_consts::lux::DIRECT_SUNLIGHT,
            ambient: Color::srgb(0.8, 0.85, 1.0),
            ambient_brightness: 220.0,
            fog: Color::srgb(0.8, 0.85, 0.88),
            fog_density: 0.006,
            lamps_on: false,
        },
        Phase {
            name: "黄昏",
            sky: Color::srgb(0.5, 0.34, 0.32),
            sun_dir: Vec3::new(0.85, -0.2, -0.45),
            sun_color: Color::srgb(1.0, 0.55, 0.32),
            sun_lux: light_consts::lux::OVERCAST_DAY,
            ambient: Color::srgb(0.7, 0.5, 0.5),
            ambient_brightness: 120.0,
            fog: Color::srgb(0.6, 0.4, 0.38),
            fog_density: 0.035,
            lamps_on: true,
        },
        Phase {
            name: "入夜",
            sky: Color::srgb(0.02, 0.03, 0.06),
            sun_dir: Vec3::new(-0.3, -0.6, -0.5),
            sun_color: Color::srgb(0.5, 0.6, 0.9),
            sun_lux: light_consts::lux::FULL_MOON_NIGHT * 1500.0,
            ambient: Color::srgb(0.4, 0.5, 0.85),
            ambient_brightness: 90.0,
            fog: Color::srgb(0.05, 0.07, 0.12),
            fog_density: 0.02,
            lamps_on: true,
        },
    ];
    // ANCHOR_END: presets

    let surroundings = asset_server.load("textures/skybox.png");

    // 机位：开 Hdr，挂雾。环境光照组件等立方体贴图装配好再补（见 reinterpret_cubemap）
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Transform::from_xyz(0.0, 5.0, 12.0).looking_at(Vec3::new(0.0, 1.2, 0.0), Vec3::Y),
        DistanceFog::default(),
    ));

    // 太阳：一盏会投影子的平行光，调好级联
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 4,
            maximum_distance: 45.0,
            ..default()
        }
        .build(),
        Transform::default(),
        Sun,
    ));

    // 两盏灯笼（点光）与一盏台口追光（聚光），白天藏起、入夜点亮
    for x in [-4.0, 4.0] {
        commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.82, 0.55),
                intensity: 500_000.0,
                range: 16.0,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(x, 4.5, 1.0),
            NightLamp,
        ));
    }
    commands.spawn((
        SpotLight {
            color: Color::srgb(1.0, 0.95, 0.85),
            intensity: 1_800_000.0,
            range: 24.0,
            inner_angle: PI / 14.0,
            outer_angle: PI / 8.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 9.0, 5.0).looking_at(Vec3::new(0.0, 1.2, 0.0), Vec3::Y),
        NightLamp,
    ));

    spawn_courtyard(&mut commands, &mut meshes, &mut materials, &surroundings);

    commands.insert_resource(Surroundings {
        image: surroundings,
        assembled: false,
    });
    commands.insert_resource(Console { phases, current: 0 });
    println!("掌灯的：切换台就位——空格换一档天色。眼下是黎明。");
}

// ANCHOR: switch
/// 空格往下翻一档
fn switch_phase(keyboard: Res<ButtonInput<KeyCode>>, mut console: ResMut<Console>) {
    if keyboard.just_pressed(KeyCode::Space) {
        console.current = (console.current + 1) % console.phases.len();
        println!("掌灯的：换到「{}」。", console.phases[console.current].name);
    }
}

/// 档位一变，把这一档的参数泼到全场的灯上
fn apply_phase(
    console: Res<Console>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut sun: Single<(&mut DirectionalLight, &mut Transform), With<Sun>>,
    mut fog: Single<&mut DistanceFog>,
    mut lamps: Query<&mut Visibility, With<NightLamp>>,
) {
    if !console.is_changed() {
        return;
    }
    let phase = console.phases[console.current];

    clear.0 = phase.sky;

    ambient.color = phase.ambient;
    ambient.brightness = phase.ambient_brightness;

    let (sun_light, sun_transform) = &mut *sun;
    sun_light.color = phase.sun_color;
    sun_light.illuminance = phase.sun_lux;
    **sun_transform = Transform::default().looking_to(phase.sun_dir, Vec3::Y);

    fog.color = phase.fog;
    fog.falloff = FogFalloff::ExponentialSquared {
        density: phase.fog_density,
    };

    let lamp_vis = if phase.lamps_on {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    for mut visibility in &mut lamps {
        *visibility = lamp_vis;
    }
}
// ANCHOR_END: switch

fn reinterpret_cubemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut surroundings: ResMut<Surroundings>,
    camera: Single<Entity, With<Camera3d>>,
) {
    if surroundings.assembled || !asset_server.load_state(&surroundings.image).is_loaded() {
        return;
    }
    let image = images.get_mut(&surroundings.image).unwrap();
    let layers = image.height() / image.width();
    image
        .reinterpret_stacked_2d_as_array(layers)
        .expect("竖摞的六张面，高应当是宽的整数倍");
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
    // 装配完毕，再把环境光照交给相机
    commands.entity(*camera).insert(GeneratedEnvironmentMapLight {
        environment_map: surroundings.image.clone(),
        intensity: 900.0,
        ..default()
    });
    surroundings.assembled = true;
}

/// 得月楼的台面布景：青砖台、木箱、立柱、灯笼杆，台中央一只鎏金绣球
fn spawn_courtyard(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    _surroundings: &Handle<Image>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(40.0, 40.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.33, 0.32),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    let lacquer = materials.add(StandardMaterial {
        base_color: Color::srgb(0.52, 0.16, 0.12),
        perceptual_roughness: 0.7,
        ..default()
    });
    for x in [-4.0, 4.0] {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.6, 1.6, 1.6))),
            MeshMaterial3d(lacquer.clone()),
            Transform::from_xyz(x, 0.8, -2.0),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.12, 5.0))),
            MeshMaterial3d(lacquer.clone()),
            Transform::from_xyz(x, 2.5, 1.0),
        ));
    }
    // 台中央：鎏金绣球——金属，等的就是环境光照那个世界
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.86, 0.62, 0.32),
            metallic: 1.0,
            perceptual_roughness: 0.18,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.2, 0.0),
    ));
}

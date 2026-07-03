//! Listing 22-14：昼夜光照切换台——1 正午、2 黄昏、3 夜戏、4 晨雾

use bevy::asset::RenderAssetUsages;
use bevy::camera::Exposure;
use bevy::light::{
    atmosphere::ScatteringMedium, Atmosphere, AtmosphereEnvironmentMapLight, FogVolume, SunDisk,
    VolumetricFog, VolumetricLight,
};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::pbr::AtmosphereSettings;
use bevy::prelude::*;

/// 标记：日头（夜里客串月亮）
#[derive(Component)]
struct Sun;

/// 标记：一对灯笼
#[derive(Component)]
struct Lantern;

/// 标记：追光
#[derive(Component)]
struct FollowSpot;

/// 标记：得月楼的发光匾额（RectLight + 发光皮）
#[derive(Component)]
struct Plaque;

/// 标记：转台上的镜面绣球
#[derive(Component)]
struct MirrorBall;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.006, 0.010, 0.022)))
        .add_systems(Startup, setup)
        .add_systems(Update, (switch_cue, turntable))
        .run();
}

/// 亭盖：第 21 章手搓的四棱锥，拆开顶点、按面取法线
fn pavilion_roof() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.9, 0.0, -0.9],
            [0.9, 0.0, -0.9],
            [0.9, 0.0, 0.9],
            [-0.9, 0.0, 0.9],
            [0.0, 1.1, 0.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        3, 2, 4, 2, 1, 4, 1, 0, 4, 0, 3, 4, 0, 1, 2, 0, 2, 3,
    ]))
    .with_duplicated_vertices()
    .with_computed_normals()
}

fn setup(
    mut commands: Commands,
    mut media: ResMut<Assets<ScatteringMedium>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: rig
    // 天只有一套：大气行星全程在场。夜戏不换天——太阳落下去，天自己黑
    let medium = media.add(ScatteringMedium::earth(256, 256));
    commands.spawn(Atmosphere::earth(medium));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.6, 12.5).looking_at(Vec3::new(0.0, 2.2, -1.0), Vec3::Y),
        AtmosphereSettings::default(),
        AtmosphereEnvironmentMapLight::default(),
        Exposure { ev100: 13.5 },
    ));

    // 日头：白天是太阳，夜里调暗调蓝就客串月亮
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::RAW_SUNLIGHT,
            shadow_maps_enabled: true,
            ..default()
        },
        SunDisk::EARTH,
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.5, -1.25)),
        Sun,
    ));
    // ANCHOR_END: rig

    // ---- 台面与布景 --------------------------------------------------------
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(600.0, 600.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.31, 0.30),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    let pillar = meshes.add(Cylinder::new(0.35, 5.0));
    let lacquer = materials.add(StandardMaterial {
        base_color: Color::srgb(0.48, 0.13, 0.10),
        perceptual_roughness: 0.7,
        ..default()
    });
    let roof = meshes.add(pavilion_roof());
    let glaze = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.46, 0.42),
        perceptual_roughness: 0.6,
        ..default()
    });
    for x in [-5.2, 5.2] {
        commands.spawn((
            Mesh3d(pillar.clone()),
            MeshMaterial3d(lacquer.clone()),
            Transform::from_xyz(x, 2.5, -3.2),
        ));
        commands.spawn((
            Mesh3d(roof.clone()),
            MeshMaterial3d(glaze.clone()),
            Transform::from_xyz(x, 5.0, -3.2),
        ));
    }
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.7, 0.9))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.52, 0.16, 0.12),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(4.2, 0.45, -0.6),
    ));
    // 转台上的镜面绣球——全章的光都照给它看
    let wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.40, 0.24),
        perceptual_roughness: 0.85,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.8, 0.5).mesh().resolution(6))),
        MeshMaterial3d(wood.clone()),
        Transform::from_xyz(0.0, 0.25, 1.6),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.62))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.93, 0.88),
            metallic: 1.0,
            perceptual_roughness: 0.05,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.12, 1.6),
        MirrorBall,
    ));

    // ---- 夜戏的家什：灯笼、追光、匾额（白天全部归零/隐藏） -----------------
    // ANCHOR: night_rig
    // 灯笼罩子发暖光（自发光），匾额面板用 unlit——
    // 它贴着 RectLight 的发光平面，不能再吃自己发的光（22.5 节）
    let glow = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.55, 0.30),
        emissive: LinearRgba::new(7.0, 3.0, 1.0, 1.0),
        ..default()
    });
    let paper = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.55, 0.30),
        unlit: true,
        ..default()
    });
    for x in [-3.6, 3.6] {
        commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.62, 0.32),
                intensity: 0.0, // 白天歇着
                range: 25.0,
                radius: 0.16,
                shadow_maps_enabled: true,
                ..default()
            },
            Transform::from_xyz(x, 3.1, 0.8),
            Lantern,
            children![
                (
                    Mesh3d(meshes.add(Sphere::new(0.16))),
                    MeshMaterial3d(glow.clone()),
                    Visibility::Hidden,
                ),
                (
                    Mesh3d(meshes.add(Cylinder::new(0.035, 3.4))),
                    MeshMaterial3d(wood.clone()),
                    Transform::from_xyz(0.0, -1.55, 0.0),
                ),
            ],
        ));
    }
    commands.spawn((
        SpotLight {
            color: Color::srgb(1.0, 0.97, 0.88),
            intensity: 0.0,
            range: 40.0,
            outer_angle: 0.30,
            inner_angle: 0.22,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 7.5, 10.0).looking_at(Vec3::new(0.0, 1.1, 1.6), Vec3::Y),
        FollowSpot,
    ));
    // 得月楼的匾：挂在两柱之间的横眉上
    commands.spawn((
        RectLight {
            color: Color::srgb(1.0, 0.66, 0.38),
            intensity: 0.0,
            width: 3.2,
            height: 1.0,
            range: 25.0,
        },
        Transform::from_xyz(0.0, 4.4, -3.2)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        Plaque,
        children![(
            Mesh3d(meshes.add(Rectangle::new(3.2, 1.0))),
            MeshMaterial3d(paper.clone()),
            Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            Visibility::Hidden,
        )],
    ));
    // 晨雾的雾罩子，平时密度为零
    commands.spawn((
        FogVolume {
            density_factor: 0.0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.5, 0.0).with_scale(Vec3::new(30.0, 5.0, 16.0)),
    ));
    // ANCHOR_END: night_rig

    println!("老雷：昼夜切换台就位，全班听老烛的口令。");
    println!("场记：1 正午、2 黄昏、3 夜戏、4 晨雾。");
}

/// 转台缓缓走
fn turntable(mut ball: Single<&mut Transform, With<MirrorBall>>, time: Res<Time>) {
    ball.rotate_y(time.delta_secs() * 0.5);
}

// ANCHOR: cues
/// 切换台本体：一个数字键是一套 cue——日头、曝光、家什一把换。
/// 天从头到尾只有大气这一套：夜戏就是太阳交班给月亮，天自己黑透
#[allow(clippy::too_many_arguments)]
fn switch_cue(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    camera: Single<Entity, With<Camera3d>>,
    mut sun: Single<(Entity, &mut DirectionalLight, &mut Transform), With<Sun>>,
    mut lanterns: Query<(&mut PointLight, &Children), With<Lantern>>,
    mut spot: Single<&mut SpotLight, With<FollowSpot>>,
    mut plaque: Single<(&mut RectLight, &Children), With<Plaque>>,
    mut fog: Single<&mut FogVolume>,
    mut visibility: Query<&mut Visibility>,
    mut exposure: Single<&mut Exposure>,
) {
    let cue = if keyboard.just_pressed(KeyCode::Digit1) {
        1
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        2
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        3
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        4
    } else {
        return;
    };

    let (sun_entity, ref mut sun_light, ref mut sun_transform) = *sun;

    // 日头与曝光：一套 cue 一份参数。夜戏的“月亮”就是把太阳调暗调蓝挂回天上；
    // 晨雾把日头转到台后，光穿柱缝进来
    let (name, azimuth, elevation, illuminance, sun_color, ev100) = match cue {
        1 => ("正午", 0.5, 1.25, light_consts::lux::RAW_SUNLIGHT, Color::WHITE, 13.5),
        2 => ("黄昏", 0.5, 0.03, light_consts::lux::RAW_SUNLIGHT, Color::WHITE, 12.0),
        3 => ("夜戏", 0.5, 0.9, 0.3, Color::srgb(0.75, 0.82, 1.0), 6.5),
        _ => ("晨雾", 2.6, 0.16, light_consts::lux::RAW_SUNLIGHT, Color::WHITE, 12.5),
    };
    sun_transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, azimuth, -elevation);
    sun_light.illuminance = illuminance;
    sun_light.color = sun_color;
    exposure.ev100 = ev100;

    // 家什：灯笼（黄昏起点亮）、追光与匾额（只在夜戏）、晨雾的雾气
    let lantern_lm = if cue == 2 { 9_000.0 } else if cue == 3 { 14_000.0 } else { 0.0 };
    for (mut lamp, children) in &mut lanterns {
        lamp.intensity = lantern_lm;
        if let Some(&bulb) = children.first()
            && let Ok(mut vis) = visibility.get_mut(bulb)
        {
            *vis = if lantern_lm > 0.0 {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
    spot.intensity = if cue == 3 { 2_600_000.0 } else { 0.0 };
    let (ref mut plaque_light, plaque_children) = *plaque;
    plaque_light.intensity = if cue == 3 { 220_000.0 } else { 0.0 };
    if let Some(&panel) = plaque_children.first()
        && let Ok(mut vis) = visibility.get_mut(panel)
    {
        *vis = if cue == 3 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    fog.density_factor = if cue == 4 { 0.15 } else { 0.0 };
    if cue == 4 {
        commands.entity(*camera).insert(VolumetricFog {
            ambient_intensity: 0.05,
            ..default()
        });
        commands.entity(sun_entity).insert(VolumetricLight);
    } else {
        commands.entity(*camera).remove::<VolumetricFog>();
        commands.entity(sun_entity).remove::<VolumetricLight>();
    }

    println!("老烛：cue {cue}——{name}。");
}
// ANCHOR_END: cues

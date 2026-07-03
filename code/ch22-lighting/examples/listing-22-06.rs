//! Listing 22-6：影子开箱——Tab 换灯型看三种影子，[ ] 拨影子贴图分辨率，T 收紧级联

use bevy::light::{CascadeShadowConfig, CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;

/// 三盏灯轮流值班，谁在岗谁可见
#[derive(Component)]
struct Duty(usize);

#[derive(Resource)]
struct Desk {
    on_duty: usize,
    size_idx: usize,
    tight: bool,
}

const SIZES: [usize; 4] = [2048, 512, 4096, 3000];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight {
            brightness: 30.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.10, 0.14, 0.20)))
        .insert_resource(Desk {
            on_duty: 0,
            size_idx: 0,
            tight: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_duty, dial_size, tighten))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.6, 10.5).looking_at(Vec3::new(0.0, 1.2, -0.5), Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.45, 0.42),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    // 旗杆、绣球、大鼓——高矮胖瘦，各自的影子性格不同
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.09, 4.4))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.40, 0.24),
            perceptual_roughness: 0.85,
            ..default()
        })),
        Transform::from_xyz(-2.6, 2.2, 0.6),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.8, 0.5).mesh().resolution(6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.40, 0.24),
            perceptual_roughness: 0.85,
            ..default()
        })),
        Transform::from_xyz(0.6, 0.25, 1.6),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.62))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.82, 0.74, 0.62),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.6, 1.12, 1.6),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.7, 0.9))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.52, 0.16, 0.12),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(4.2, 0.45, -0.6),
    ));

    // ANCHOR: three
    // 三盏灯都开了影子，换班靠把下岗那盏的亮度归零。
    // （亮度为零的灯不出光也不出影，但实体、开关、配置全都原地待命）
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadow_maps_enabled: true, // 开影子：就这一个开关
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.8, -0.6)),
        Duty(0),
    ));
    commands.spawn((
        PointLight {
            intensity: 0.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 3.8, 2.4),
        Duty(1),
    ));
    commands.spawn((
        SpotLight {
            intensity: 0.0,
            range: 40.0,
            outer_angle: 0.5,
            inner_angle: 0.4,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 7.0, 9.0).looking_at(Vec3::new(0.0, 0.0, 0.6), Vec3::Y),
        Duty(2),
    ));
    // ANCHOR_END: three

    println!("老烛：三盏灯排班，都开了影子。Tab 换班，[ ] 拨影子贴图，T 收紧级联。");
    println!("场记：当班的是平行光。");
}

/// Tab 换班：太阳 → 堂灯 → 追光。当班的给足亮度，下岗的归零
fn rotate_duty(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut desk: ResMut<Desk>,
    mut lights: Query<(
        &Duty,
        Option<&mut DirectionalLight>,
        Option<&mut PointLight>,
        Option<&mut SpotLight>,
    )>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        desk.on_duty = (desk.on_duty + 1) % 3;
        for (duty, sun, lamp, spot) in &mut lights {
            let on = duty.0 == desk.on_duty;
            if let Some(mut sun) = sun {
                sun.illuminance = if on { light_consts::lux::OVERCAST_DAY } else { 0.0 };
            }
            if let Some(mut lamp) = lamp {
                lamp.intensity = if on { 400_000.0 } else { 0.0 };
            }
            if let Some(mut spot) = spot {
                spot.intensity = if on { 3_000_000.0 } else { 0.0 };
            }
        }
        let name = ["平行光", "点光", "聚光"][desk.on_duty];
        println!("场记：当班的是{name}。");
    }
}

// ANCHOR: size
/// [ ] 拨平行光影子贴图的边长——2 的幂才是正经尺寸，3000 会被引擎当场纠正
fn dial_size(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut desk: ResMut<Desk>,
    mut shadow_map: ResMut<DirectionalLightShadowMap>,
) {
    let step = if keyboard.just_pressed(KeyCode::BracketRight) {
        1
    } else if keyboard.just_pressed(KeyCode::BracketLeft) {
        SIZES.len() - 1
    } else {
        return;
    };
    desk.size_idx = (desk.size_idx + step) % SIZES.len();
    shadow_map.size = SIZES[desk.size_idx];
    println!("老烛：影子贴图拨到 {}。", shadow_map.size);
}
// ANCHOR_END: size

// ANCHOR: tighten
/// T 收放级联：把四层影子贴图全花在眼前 30 米，近处立刻清晰
fn tighten(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut desk: ResMut<Desk>,
    mut commands: Commands,
    sun: Single<Entity, With<DirectionalLight>>,
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        desk.tight = !desk.tight;
        let config: CascadeShadowConfig = if desk.tight {
            CascadeShadowConfigBuilder {
                first_cascade_far_bound: 6.0,
                maximum_distance: 30.0,
                ..default()
            }
            .into()
        } else {
            CascadeShadowConfig::default()
        };
        commands.entity(*sun).insert(config);
        println!(
            "老烛：级联{}。",
            if desk.tight {
                "收紧——四层全铺在眼前 30 米"
            } else {
                "放回默认——照顾到 150 米开外"
            }
        );
    }
}
// ANCHOR_END: tighten

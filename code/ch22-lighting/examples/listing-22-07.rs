//! Listing 22-7：影子的毛病——B 键拨偏置看粉刺与彼得潘，C 键接触阴影，N 键让绸幕不投影

use bevy::light::NotShadowCaster;
use bevy::light::DirectionalLightShadowMap;
use bevy::pbr::ContactShadows;
use bevy::prelude::*;

/// 偏置的三档：默认 → 全关（粉刺）→ 加猛（彼得潘）
const BIAS: [(&str, f32, f32); 3] = [
    ("默认（0.02 / 1.8）", 0.02, 1.8),
    ("全关（0 / 0）", 0.0, 0.0),
    ("加猛（2.0 / 1.8）", 2.0, 1.8),
];

#[derive(Resource)]
struct Fixes {
    bias_idx: usize,
    contact: bool,
    curtain_casts: bool,
}

/// 标记：台口那幅绸幕
#[derive(Component)]
struct Curtain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight {
            brightness: 30.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.10, 0.14, 0.20)))
        // 粉刺在低分辨率贴图上看得最清楚
        .insert_resource(DirectionalLightShadowMap { size: 1024 })
        .insert_resource(Fixes {
            bias_idx: 0,
            contact: false,
            curtain_casts: true,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (dial_bias, toggle_contact, toggle_curtain))
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

    // 台口右侧挂一幅绸幕——大块的遮光布，影子问题的放大镜
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.6, 3.2, 0.04))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.46, 0.20, 0.30),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(3.6, 2.4, 0.2),
        Curtain,
    ));

    // 太阳斜照，影子拉长——偏置的毛病全在影子里
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.8, -0.6)),
    ));

    println!("老烛：影子有影子的毛病。B 拨偏置，C 上接触阴影，N 让绸幕不投影。");
    println!("老烛：偏置眼下是默认（0.02 / 1.8）。");
}

// ANCHOR: bias
/// B 键拨偏置：默认没毛病 → 全关长粉刺 → 加猛影子飞走
fn dial_bias(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut fixes: ResMut<Fixes>,
    mut sun: Single<&mut DirectionalLight>,
) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        fixes.bias_idx = (fixes.bias_idx + 1) % BIAS.len();
        let (name, depth, normal) = BIAS[fixes.bias_idx];
        sun.shadow_depth_bias = depth;
        sun.shadow_normal_bias = normal;
        println!("老烛：偏置拨到{name}。");
    }
}
// ANCHOR_END: bias

// ANCHOR: contact
/// C 键接触阴影：灯与相机各开一个开关，屏幕空间里把影子根须补回去
fn toggle_contact(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut fixes: ResMut<Fixes>,
    mut commands: Commands,
    camera: Single<Entity, With<Camera3d>>,
    mut sun: Single<&mut DirectionalLight>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        fixes.contact = !fixes.contact;
        sun.contact_shadows_enabled = fixes.contact;
        if fixes.contact {
            commands.entity(*camera).insert(ContactShadows::default());
            println!("老烛：接触阴影上了——根须回来了。");
        } else {
            commands.entity(*camera).remove::<ContactShadows>();
            println!("老烛：接触阴影撤了。");
        }
    }
}
// ANCHOR_END: contact

// ANCHOR: curtain
/// N 键：给绸幕挂上/摘下 NotShadowCaster——布还在，影子说没就没
fn toggle_curtain(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut fixes: ResMut<Fixes>,
    mut commands: Commands,
    curtain: Single<Entity, With<Curtain>>,
) {
    if keyboard.just_pressed(KeyCode::KeyN) {
        fixes.curtain_casts = !fixes.curtain_casts;
        if fixes.curtain_casts {
            commands.entity(*curtain).remove::<NotShadowCaster>();
            println!("场记：绸幕照常投影。");
        } else {
            commands.entity(*curtain).insert(NotShadowCaster);
            println!("场记：绸幕挂上 NotShadowCaster——布在，影没了。");
        }
    }
}
// ANCHOR_END: curtain

//! Listing 25-4：不听台词看状态牌——Hovered、PickingInteraction 与 PointerInteraction

use bevy::picking::hover::{Hovered, PickingInteraction};
use bevy::picking::pointer::PointerInteraction;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (hovered_report, interaction_report, draw_hit))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.8, 6.4).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.0, 6.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.40, 0.42),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // ANCHOR: components
    // 琉璃盏：什么牌都不挂——三态牌等着看它自动出现
    commands.spawn((
        Name::new("琉璃盏"),
        Mesh3d(meshes.add(Sphere::new(0.55))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.35, 0.62, 0.60, 0.35),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.089,
            ..default()
        })),
        Transform::from_xyz(-2.0, 1.0, 0.0),
    ));
    // 鎏金锣领了看客牌（Hovered）：这块牌要自己挂，管线只更新、不发放
    commands.spawn((
        Name::new("鎏金锣"),
        Mesh3d(meshes.add(Torus::new(0.28, 0.72))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.82, 0.55),
            metallic: 1.0,
            perceptual_roughness: 0.25,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.05, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        Hovered::default(),
    ));
    // 剔红漆盒也不挂——和琉璃盏一起当对照组
    commands.spawn((
        Name::new("剔红漆盒"),
        Mesh3d(meshes.add(Cuboid::new(0.95, 0.95, 0.95))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.62, 0.11, 0.08),
            perceptual_roughness: 0.35,
            ..default()
        })),
        Transform::from_xyz(2.0, 0.98, 0.0),
    ));
    // ANCHOR_END: components
    println!("小棠：这回不挂观察者了——看货的状态牌，谁想知道谁自己来查。");
}

// ANCHOR: report
/// 两态牌翻面就报（Changed 过滤，ch04 的老手艺）
fn hovered_report(wares: Query<(&Name, &Hovered), Changed<Hovered>>) {
    for (name, hovered) in &wares {
        println!("场记：{name}的看客牌翻到——{}。", if hovered.get() { "有人看" } else { "没人看" });
    }
}

/// 三态牌翻面也报
fn interaction_report(wares: Query<(&Name, &PickingInteraction), Changed<PickingInteraction>>) {
    for (name, state) in &wares {
        let word = match state {
            PickingInteraction::Pressed => "按住了",
            PickingInteraction::Hovered => "有人看",
            PickingInteraction::None => "没人理",
        };
        println!("场记：{name}的三态牌翻到——{word}。");
    }
}
// ANCHOR_END: report

// ANCHOR: gizmo
/// 指针实体身上的 PointerInteraction 存着排好序的命中清单：
/// 取最近一条，在命中点画个小球、顺法线画支箭
fn draw_hit(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.06, Color::srgb(0.95, 0.25, 0.20));
        gizmos.arrow(
            point,
            point + normal.normalize() * 0.5,
            Color::srgb(1.0, 0.75, 0.75),
        );
    }
}
// ANCHOR_END: gizmo

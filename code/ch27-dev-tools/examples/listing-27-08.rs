//! Listing 27-8：现成的描边——后台仓库里，包围盒与灯形各归各的开关。
//! B 给主箱单独描框；A 全场描框；L 摘挂灯形；C 轮换灯形配色策略。

use bevy::light::gizmos::{LightGizmoColor, LightGizmoConfigGroup, ShowLightGizmo};
use bevy::prelude::*;

/// 主箱：B 键单独描框的那一只
#[derive(Component)]
struct StarCrate;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_overlays)
        .run();
}

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 仓库地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(14.0, 14.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.24, 0.22, 0.20))),
    ));

    // 一排道具箱：尺寸各异，包围盒也就各异
    let crate_material = materials.add(Color::srgb(0.52, 0.42, 0.30));
    let sizes = [
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.6, 0.8, 1.2),
        Vec3::new(0.7, 1.8, 0.7),
        Vec3::new(2.2, 0.5, 0.9),
    ];
    for (i, size) in sizes.into_iter().enumerate() {
        let x = -3.6 + i as f32 * 2.4;
        let mut crate_entity = commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_size(size))),
            MeshMaterial3d(crate_material.clone()),
            Transform::from_xyz(x, size.y / 2.0, 0.0),
        ));
        if i == 0 {
            crate_entity.insert(StarCrate);
        }
    }

    // 一只斜躺的堂鼓凑数：圆柱的包围盒还是方盒
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.6, 0.8))),
        MeshMaterial3d(materials.add(Color::srgb(0.62, 0.30, 0.26))),
        Transform::from_xyz(2.4, 0.9, -2.6).with_rotation(Quat::from_rotation_z(0.9)),
    ));

    // 四种灯各来一盏，先不描形——L 键统一挂牌
    commands.spawn((
        PointLight {
            color: Color::srgb(0.4, 0.8, 1.0),
            range: 6.0,
            ..default()
        },
        Transform::from_xyz(-2.0, 2.5, 1.5),
    ));
    commands.spawn((
        SpotLight {
            color: Color::srgb(1.0, 0.7, 0.3),
            range: 8.0,
            outer_angle: 0.6,
            inner_angle: 0.45,
            ..default()
        },
        Transform::from_xyz(3.0, 3.5, 2.0).looking_at(Vec3::new(2.4, 0.0, -2.6), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 3_000.0,
            ..default()
        },
        Transform::from_xyz(-4.0, 5.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        RectLight {
            color: Color::srgb(0.9, 0.4, 0.9),
            intensity: 60_000.0,
            width: 1.6,
            height: 0.9,
            ..default()
        },
        Transform::from_xyz(0.0, 2.2, -4.5).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-7.0, 6.5, 9.0).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
    ));

    println!("检场：仓库清点。B 给主箱描框，A 全场描框，L 摘挂灯形，C 换灯形配色。");
}
// ANCHOR_END: setup

// ANCHOR: toggles
fn toggle_overlays(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    star_crate: Single<(Entity, Option<&ShowAabbGizmo>), With<StarCrate>>,
    lights: Query<(Entity, Option<&ShowLightGizmo>), Or<(With<PointLight>, With<SpotLight>, With<DirectionalLight>, With<RectLight>)>>,
) {
    // B：只给主箱挂/摘 ShowAabbGizmo——逐实体描框
    if keyboard.just_pressed(KeyCode::KeyB) {
        let (entity, showing) = *star_crate;
        if showing.is_some() {
            commands.entity(entity).remove::<ShowAabbGizmo>();
            println!("检场：主箱的框擦了。");
        } else {
            commands.entity(entity).insert(ShowAabbGizmo {
                color: Some(Color::srgb(1.0, 0.85, 0.3)),
            });
            println!("检场：主箱描上金框。");
        }
    }

    // A：拨配置组的 draw_all——全场无差别描框，没挂组件的也算上
    if keyboard.just_pressed(KeyCode::KeyA) {
        let (_, aabb_group) = config_store.config_mut::<AabbGizmoConfigGroup>();
        aabb_group.draw_all = !aabb_group.draw_all;
        println!(
            "检场：全场描框{}。",
            if aabb_group.draw_all { "，一件不落" } else { "收工" }
        );
    }

    // L：给每盏灯挂/摘 ShowLightGizmo——灯形、朝向、照多远一目了然
    if keyboard.just_pressed(KeyCode::KeyL) {
        for (entity, showing) in &lights {
            if showing.is_some() {
                commands.entity(entity).remove::<ShowLightGizmo>();
            } else {
                commands.entity(entity).insert(ShowLightGizmo::default());
            }
        }
        println!("检场：灯形牌翻了个面。");
    }

    // C：轮换灯形配色策略
    if keyboard.just_pressed(KeyCode::KeyC) {
        let (_, light_group) = config_store.config_mut::<LightGizmoConfigGroup>();
        light_group.color = match light_group.color {
            LightGizmoColor::MatchLightColor => LightGizmoColor::ByLightType,
            LightGizmoColor::ByLightType => LightGizmoColor::Varied,
            LightGizmoColor::Varied => LightGizmoColor::Manual(Color::WHITE),
            LightGizmoColor::Manual(_) => LightGizmoColor::MatchLightColor,
        };
        println!("检场：灯形配色改 {:?}。", light_group.color);
    }
}
// ANCHOR_END: toggles

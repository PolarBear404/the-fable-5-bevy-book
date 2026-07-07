//! Listing 27-13：迷你检修间——无限网格打底，点选道具、拽把手挪位。
//! 点箱选中；1/2/3 换搬/转/缩把手；X 换世界/本地轴；G 开吸格。

use bevy::dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings};
use bevy::gizmos::transform_gizmo::{
    TransformGizmoCamera, TransformGizmoFocus, TransformGizmoMode, TransformGizmoPlugin,
    TransformGizmoSettings, TransformGizmoSpace,
};
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;

// ANCHOR: app
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,    // 点选靠它（第 25 章的老相识）
            TransformGizmoPlugin, // 把手的交互逻辑；画把手的渲染插件随 DefaultPlugins 自动就位
            InfiniteGridPlugin,   // 无限网格地面
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, gizmo_keys)
        .run();
}
// ANCHOR_END: app

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 无限网格：检修间的地面基准。设置摆在同一实体上
    commands.spawn((
        InfiniteGrid,
        InfiniteGridSettings {
            fadeout_distance: 60.0, // 网格淡出的距离（相对相机）
            ..default()
        },
    ));

    // 三只待检修的道具箱
    let crate_material = materials.add(Color::srgb(0.52, 0.42, 0.30));
    for (i, position) in [
        Vec3::new(-2.5, 0.5, 0.0),
        Vec3::new(0.0, 0.5, -1.5),
        Vec3::new(2.5, 0.4, 0.8),
    ]
    .into_iter()
    .enumerate()
    {
        let size = if i == 2 { 0.8 } else { 1.0 };
        let mut crate_entity = commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_length(size))),
            MeshMaterial3d(crate_material.clone()),
            Transform::from_translation(position),
        ));
        crate_entity.observe(select_on_click);
        // 开场先让 0 号箱挂上把手
        if i == 0 {
            crate_entity.insert(TransformGizmoFocus);
        }
    }

    commands.spawn((
        DirectionalLight {
            illuminance: 4_000.0,
            ..default()
        },
        Transform::from_xyz(-4.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-4.5, 4.5, 7.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        TransformGizmoCamera, // 多相机时把手认这顶帽子；单相机可省
    ));

    println!("检场：检修间开门。点箱选中，1/2/3 换把手，X 换轴系，G 吸格。");
}
// ANCHOR_END: setup

// ANCHOR: interact
/// 点谁修谁：把手焦点是一顶帽子，摘下旧的、扣上新的
fn select_on_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    focused: Query<Entity, With<TransformGizmoFocus>>,
) {
    if click.button != PointerButton::Primary {
        return;
    }
    for previous in &focused {
        commands.entity(previous).remove::<TransformGizmoFocus>();
    }
    commands.entity(click.entity).insert(TransformGizmoFocus);
}

/// 模式、轴系、吸格全在 TransformGizmoSettings 资源里
fn gizmo_keys(keyboard: Res<ButtonInput<KeyCode>>, mut settings: ResMut<TransformGizmoSettings>) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        settings.mode = TransformGizmoMode::Translate;
        println!("检场：换搬运把手。");
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        settings.mode = TransformGizmoMode::Rotate;
        println!("检场：换旋转把手。");
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        settings.mode = TransformGizmoMode::Scale;
        println!("检场：换缩放把手。");
    }
    if keyboard.just_pressed(KeyCode::KeyX) {
        settings.space = match settings.space {
            TransformGizmoSpace::World => TransformGizmoSpace::Local,
            TransformGizmoSpace::Local => TransformGizmoSpace::World,
        };
        println!("检场：轴系换 {:?}。", settings.space);
    }
    if keyboard.just_pressed(KeyCode::KeyG) {
        // 吸格：搬运每 0.5 米一格，旋转每 15 度一挡
        let snapping = settings.snap_translate.is_none();
        settings.snap_translate = snapping.then_some(0.5);
        settings.snap_rotate = snapping.then_some(15.0_f32.to_radians());
        println!("检场：吸格{}。", if snapping { "开" } else { "关" });
    }
}
// ANCHOR_END: interact

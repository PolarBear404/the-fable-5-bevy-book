//! Listing 25-5：`bevy_camera_controller` 的 FreeCamera。

use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    prelude::*,
};

fn main() {
    App::new()
        // ANCHOR: plugin
        .add_plugins((DefaultPlugins, FreeCameraPlugin))
        // ANCHOR_END: plugin
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.07)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(16.0, 16.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.14, 0.15, 0.17))),
    ));
    for x in -3..=3 {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.7, 0.7, 0.7))),
            MeshMaterial3d(materials.add(Color::srgb(0.22 + x as f32 * 0.04, 0.45, 0.76))),
            Transform::from_xyz(x as f32 * 1.2, 0.35, -1.0 - x as f32 * 0.25),
        ));
    }
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.4, -0.8, -0.4), Vec3::Y),
    ));

    // ANCHOR: camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.4, 6.0).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
        FreeCamera {
            // 本章最终 demo 把左键留给 picking，所以这里改用右键抓光标转视角。
            mouse_key_cursor_grab: MouseButton::Right,
            walk_speed: 4.0,
            run_speed: 12.0,
            friction: 35.0,
            ..default()
        },
    ));
    // ANCHOR_END: camera

    let font = asset_server.load("fonts/book-sans-sc-regular.otf");
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(16),
            top: px(16),
            ..default()
        },
        Pickable::IGNORE,
        children![(
            Text::new("右键拖动看向；WASD 平移；Q/E 升降；Shift 加速；滚轮调速度"),
            TextFont {
                font,
                font_size: 21.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Pickable::IGNORE,
        )],
    ));
}

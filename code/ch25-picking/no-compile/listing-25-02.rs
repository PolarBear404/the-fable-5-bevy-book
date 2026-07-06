//! Listing 25-2：行不通——把 Click 直接当事件挂，忘了 Pointer 这层包装

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: bare_click
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.8, 6.4).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
    ));
    // 陆掌柜催得急，小棠手一快：Click 就 Click 吧，包装纸拆了再挂
    commands
        .spawn((
            Name::new("剔红漆盒"),
            Mesh3d(meshes.add(Cuboid::new(0.95, 0.95, 0.95))),
            MeshMaterial3d(materials.add(Color::srgb(0.62, 0.11, 0.08))),
            Transform::from_xyz(0.0, 0.98, 0.0),
        ))
        .observe(|click: On<Click>| {
            println!("场记：漆盒收到一点（{:?} 键）。", click.button);
        });
}
// ANCHOR_END: bare_click

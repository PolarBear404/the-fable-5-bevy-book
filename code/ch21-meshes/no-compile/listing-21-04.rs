//! Listing 21-4：行不通——把形状直接塞给 Mesh3d，想省掉一道入库手续

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: skip_add
fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 老鲁图省事：形状现成的，何必先送库房再领提货单？
    commands.spawn((
        Mesh3d(Cuboid::new(2.0, 1.0, 1.0)),
        MeshMaterial3d(materials.add(Color::srgb(0.72, 0.16, 0.12))),
    ));
}
// ANCHOR_END: skip_add

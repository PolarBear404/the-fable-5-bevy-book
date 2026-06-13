//! Listing 21-2：点灯——一盏堂灯，箱笼亮出三副面孔

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.72, 0.16, 0.12))),
    ));

    // ANCHOR: light
    // 一盏堂灯，挂在台口右上方。为什么需要它，这一节说清；
    // 怎么调教它，第 22 章的正题
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(2.0, 4.0, 3.0),
    ));
    // ANCHOR_END: light

    println!("老鲁：灯一点，坯子就有棱有角了——三面三个亮法。");
}

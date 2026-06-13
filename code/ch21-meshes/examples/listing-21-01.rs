//! Listing 21-1：头件活——把第 15 章的铸造手艺原样升维（还没点灯）

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 机位：架在斜上方，盯住原点——第 13 章的 looking_at
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 头件活：一只朱漆箱笼——铸形状、调材质，跟 2D 一字不差的两步
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.72, 0.16, 0.12))),
    ));

    println!("老鲁：坯子上台了——咦，怎么黑灯瞎火的？");
}
// ANCHOR_END: setup

//! Listing 15-11：不用画的道具——月亮、光晕，与一桩忘了上料的事故

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.10)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // ANCHOR: moon
    // 圆月：形状铸成 Mesh、颜色调成 ColorMaterial，两张提货单各管各的
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(64.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.93, 0.91, 0.82))),
        Transform::from_xyz(320.0, 170.0, 1.0),
    ));

    // 光晕：两圈半透明的同心圆环
    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(64.0, 92.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.93, 0.91, 0.82, 0.06))),
        Transform::from_xyz(320.0, 170.0, 0.9),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(92.0, 126.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.93, 0.91, 0.82, 0.03))),
        Transform::from_xyz(320.0, 170.0, 0.8),
    ));
    // ANCHOR_END: moon

    // ANCHOR: shapes
    // 远山两座、江面一条——全是现铸的几何形状
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(-280.0, -95.0),
            Vec2::new(280.0, -95.0),
            Vec2::new(-30.0, 95.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.07, 0.10, 0.16))),
        Transform::from_xyz(-300.0, -50.0, 0.5),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(-220.0, -70.0),
            Vec2::new(220.0, -70.0),
            Vec2::new(40.0, 70.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.05, 0.08, 0.13))),
        Transform::from_xyz(260.0, -75.0, 0.45),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1280.0, 220.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.09, 0.16, 0.24))),
        Transform::from_xyz(0.0, -250.0, 0.6),
    ));

    // 星斗：一份网格、一份材质，挂十四份提货单——上一章的省钱诀窍
    let star_mesh = meshes.add(RegularPolygon::new(4.0, 4));
    let star_material = materials.add(Color::srgba(0.92, 0.94, 1.00, 0.85));
    for i in 0..14 {
        let x = ((i * 173) % 1200) as f32 - 600.0;
        let y = ((i * 97) % 230) as f32 + 90.0;
        commands.spawn((
            Mesh2d(star_mesh.clone()),
            MeshMaterial2d(star_material.clone()),
            Transform::from_xyz(x, y, 0.3),
        ));
    }
    // ANCHOR_END: shapes

    // ANCHOR: accident
    // 西天的伴月盘——只铸了形状，忘了配材质：画面上不会有任何东西
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(36.0))),
        Transform::from_xyz(-320.0, 190.0, 1.0),
    ));

    // 旁边这块挂的是"默认材质"：洋红是渲染器的缺料警告色
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(36.0))),
        MeshMaterial2d::<ColorMaterial>::default(),
        Transform::from_xyz(-470.0, 190.0, 1.0),
    ));
    // ANCHOR_END: accident

    println!("老雷：伴月盘呢？西天怎么空了一块？");
    println!("小棠：形状铸了，料忘了配……旁边那块洋红的挂了默认料，等于贴了张催料单。");
}

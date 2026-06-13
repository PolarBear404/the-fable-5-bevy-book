//! Listing 24-1：自发光——暗场里，素球发暗、自发光球自己亮、unlit 球不理会光

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.03)))
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: emissive
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball = meshes.add(Sphere::new(0.7));

    // 左：素球——只有 base_color，全靠那盏弱光照亮，暗场里几乎隐没
    commands.spawn((
        Mesh3d(ball.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.30, 0.25),
            ..default()
        })),
        Transform::from_xyz(-2.2, 0.7, 0.0),
    ));

    // 中：自发光球——emissive 把颜色「加」在表面上，没有光也自己亮。
    // emissive 的类型是 LinearRgba（不是 Color）；通道值可以超过 1.0，越大越亮
    commands.spawn((
        Mesh3d(ball.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::BLACK,
            emissive: LinearRgba::rgb(0.1, 2.4, 2.1),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.7, 0.0),
    ));

    // 右：unlit 球——完全不理会光照，直接把 base_color 平铺出来
    commands.spawn((
        Mesh3d(ball.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.30, 0.25),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(2.2, 0.7, 0.0),
    ));
    // ANCHOR_END: emissive

    // 地面 + 一盏压得很弱的灯：暗场才看得出谁靠光、谁自己亮
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.20, 0.22),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));
    commands.spawn((
        PointLight {
            intensity: 200_000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 3.5),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.6, 6.5).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
    ));

    println!("小棠：左边这球得借光才看得见；中间这缸漆自己会亮，灯灭了照旧；右边的，干脆不认光。");
}

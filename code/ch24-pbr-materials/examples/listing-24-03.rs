//! Listing 24-3：法线贴图——同一张法线图，左片没切线渲成平板，右片生成切线才凸起

use bevy::{image::ImageLoaderSettings, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.07, 0.09)))
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: normal
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 法线贴图存的是「方向」不是颜色，必须按线性图加载（is_srgb = false），
    // 否则光照算出来不对
    let normal = asset_server.load_with_settings(
        "textures/studs-normal.png",
        |s: &mut ImageLoaderSettings| s.is_srgb = false,
    );
    let plate = |normal_map: Handle<Image>| StandardMaterial {
        base_color: Color::srgb(0.55, 0.57, 0.62),
        perceptual_roughness: 0.5,
        metallic: 0.1,
        normal_map_texture: Some(normal_map),
        ..default()
    };

    let quad = Rectangle::new(2.6, 2.6);

    // 左：直接用内置网格。它有 UV 和法线，却没有「切线」——法线贴图无从施力，渲成平板
    commands.spawn((
        Mesh3d(meshes.add(quad)),
        MeshMaterial3d(materials.add(plate(normal.clone()))),
        Transform::from_xyz(-1.6, 1.3, 0.0),
    ));

    // 右：同一张图，但给网格补上切线（with_generated_tangents），凹凸这才浮现
    let with_tangents = Mesh::from(quad)
        .with_generated_tangents()
        .expect("矩形有 UV 和法线，能生成切线");
    commands.spawn((
        Mesh3d(meshes.add(with_tangents)),
        MeshMaterial3d(materials.add(plate(normal.clone()))),
        Transform::from_xyz(1.6, 1.3, 0.0),
    ));
    // ANCHOR_END: normal

    // 斜着打光，凹凸的明暗才拉得开
    commands.spawn((
        DirectionalLight {
            illuminance: 6000.0,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.7, -0.4, -0.6), Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.3, 6.0).looking_at(Vec3::new(0.0, 1.3, 0.0), Vec3::Y),
    ));

    println!("小棠：同一张点子图，左边贴上去还是块平板，右边补了切线，铆钉这就鼓起来了。");
}

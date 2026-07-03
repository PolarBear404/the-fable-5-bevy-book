//! Listing 22-9：星空天幕——竖条 PNG 变 cubemap，Skybox 画天、GeneratedEnvironmentMapLight 让天发光

use bevy::light::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};

/// 天幕：提货单在手，货到再挂
#[derive(Resource)]
struct Backdrop {
    image: Handle<Image>,
    hung: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.006, 0.010, 0.022)))
        .add_systems(Startup, setup)
        .add_systems(Update, hang_backdrop)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 机位抬头三分，画面里得有天
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.6, 10.5).looking_at(Vec3::new(0.0, 2.6, -1.0), Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.31, 0.30),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    let plinth = meshes.add(Cylinder::new(0.8, 0.5).mesh().resolution(6));
    let wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.40, 0.24),
        perceptual_roughness: 0.85,
        ..default()
    });
    commands.spawn((
        Mesh3d(plinth.clone()),
        MeshMaterial3d(wood.clone()),
        Transform::from_xyz(0.0, 0.25, 1.6),
    ));
    // 镜面金球坐台中央，等着照星星
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.62))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.93, 0.88),
            metallic: 1.0,
            perceptual_roughness: 0.05,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.12, 1.6),
    ));

    // ANCHOR: load
    // 天幕布是一张竖条 PNG：六张 512×512 的面从上到下摞成 512×3072
    commands.insert_resource(Backdrop {
        image: asset_server.load("textures/night_cubemap.png"),
        hung: false,
    });
    // ANCHOR_END: load

    println!("场记：夜戏开演前，得把星空天幕挂上——布还在路上。");
}

// ANCHOR: hang
/// 货一到就裁布挂幕：竖条重新解释成 6 层数组，再声明按 cubemap 采样
fn hang_backdrop(
    mut backdrop: ResMut<Backdrop>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    camera: Single<Entity, With<Camera3d>>,
) {
    if backdrop.hung || !asset_server.load_state(&backdrop.image).is_loaded() {
        return;
    }
    let mut image = images.get_mut(&backdrop.image).unwrap();
    // PNG 自己不知道自己是 cubemap：到手是一张 1 层的长条贴图，
    // 得把它按高度切成 6 层，再把视图声明成 Cube
    if image.texture_descriptor.array_layer_count() == 1 {
        let layers = image.height() / image.width();
        image.reinterpret_stacked_2d_as_array(layers).unwrap();
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }
    commands.entity(*camera).insert((
        // 天幕本体：只管画天，一点光都不出
        Skybox {
            image: Some(backdrop.image.clone()),
            brightness: 400.0, // 天幕在画面里的亮度，cd/m²
            ..default()
        },
        // 让这片天真的发光：运行时滤波出漫反射图与镜面反射图
        GeneratedEnvironmentMapLight {
            environment_map: backdrop.image.clone(),
            intensity: 1200.0,
            ..default()
        },
    ));
    backdrop.hung = true;
    println!("场记：天幕挂好了——满天星斗，台上洒了层月色，镜球里也是一片星空。");
}
// ANCHOR_END: hang

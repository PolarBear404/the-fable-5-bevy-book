//! Listing 22-9：给金属一个世界——环境光照（IBL），让镜面金属照出周遭

use bevy::{
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        view::Hdr,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.06, 0.09)))
        .add_systems(Startup, setup)
        .add_systems(Update, assemble_surroundings)
        .run();
}

/// 攥着那张待装配成立方体贴图的提货单
#[derive(Resource)]
struct Surroundings {
    image: Handle<Image>,
    assembled: bool,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 六张面竖着摞成一张 PNG，先当普通图片加载，装配交给下面的系统
    let surroundings = asset_server.load("textures/skybox.png");

    // 相机开 Hdr——环境光照吃高动态范围。环境光照组件先不挂，等贴图装配好再说
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Transform::from_xyz(0.0, 2.5, 7.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    commands.insert_resource(Surroundings {
        image: surroundings,
        assembled: false,
    });

    // 一盏压暗的平行光：这一节的主角是反射，不是直接光
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.4, -0.7, -0.5), Vec3::Y),
    ));

    // ANCHOR: metal
    // 那颗一直黑着的镜面金属球——金属度拉满、磨得锃亮
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.86, 0.62, 0.32),
            metallic: 1.0,
            perceptual_roughness: 0.12,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.2, 0.0),
    ));
    // ANCHOR_END: metal

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.22, 0.24),
            perceptual_roughness: 0.6,
            ..default()
        })),
    ));
}

// ANCHOR: assemble
/// PNG 不带「我是立方体贴图」的标记，加载完默认是一张普通 2D 图。
/// 必须先把六张面切成数组、贴上 Cube 视图，装配成立方体贴图，
/// 「之后」才能挂 GeneratedEnvironmentMapLight——它在装配前会嫌图不是正方形而当场报错
fn assemble_surroundings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut surroundings: ResMut<Surroundings>,
    camera: Single<Entity, With<Camera3d>>,
) {
    if surroundings.assembled || !asset_server.load_state(&surroundings.image).is_loaded() {
        return;
    }
    let image = images.get_mut(&surroundings.image).unwrap();
    let layers = image.height() / image.width();
    image
        .reinterpret_stacked_2d_as_array(layers)
        .expect("竖摞的六张面，高应当是宽的整数倍");
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });

    // 装配完毕，把「周遭世界」交给相机：GPU 实时滤成环境光，镜面金属从此有的可照
    commands.entity(*camera).insert(GeneratedEnvironmentMapLight {
        environment_map: surroundings.image.clone(),
        intensity: 1500.0,
        ..default()
    });
    surroundings.assembled = true;
}
// ANCHOR_END: assemble

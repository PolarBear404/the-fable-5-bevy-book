//! ch24 全本《琉璃记·验货》（Listing 24-15）：
//! 材质球画廊——数字键巡展，左键拖动转台，展品缓缓自转

use bevy::image::ImageLoaderSettings;
use bevy::light::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};

fn main() {
    App::new()
        // canvas/fit_canvas_to_parent 只在网页构建里生效（20.7 交代过），桌面照常
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-ch24".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GlobalAmbientLight::NONE)
        .add_systems(Startup, setup)
        .add_systems(Update, (hang_studio, orbit_camera, visit_exhibit, spin_exhibits))
        .run();
}

/// 一件展品：名字、圆心、看它用的轨道半径
struct Exhibit {
    name: &'static str,
    recipe: &'static str,
    center: Vec3,
    radius: f32,
}

/// 展品名录 + 当前机位
#[derive(Resource)]
struct Tour {
    exhibits: Vec<Exhibit>,
    current: usize, // 0 = 全景
}

/// 挂在展品上的转台标记
#[derive(Component)]
struct Spin;

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.4, 7.6).looking_at(Vec3::new(0.0, 0.9, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 5_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.0, 5.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.42, 0.44),
            perceptual_roughness: 0.8,
            ..default()
        })),
    ));
    commands.insert_resource(StudioWalls {
        image: asset_server.load("textures/studio_cubemap.png"),
        hung: false,
    });
    // 竹影纱压后景：一来有个背景，二来给琉璃盏一点可折的东西
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(7.0, 2.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/bamboo_alpha.png")),
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            double_sided: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.3, -3.4).with_rotation(Quat::from_rotation_x(1.5)),
    ));
    // ANCHOR_END: setup

    // ANCHOR: exhibits
    // 八件道具沿浅弧排开——每件都是本章某一节的成品配方
    let orm: Handle<Image> = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load("textures/gong_orm.png");
    let carve_normal: Handle<Image> = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load("textures/carve_normal.png");
    let carve_depth: Handle<Image> = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load("textures/carve_height.png");

    let recipes: [(&str, &str, StandardMaterial); 8] = [
        (
            "白瓷",
            "roughness 0.089，reflectance 1.0",
            StandardMaterial {
                base_color: Color::srgb(0.93, 0.93, 0.90),
                perceptual_roughness: 0.089,
                reflectance: 1.0,
                ..default()
            },
        ),
        (
            "黑釉石绿",
            "黑底 + specular_tint 石绿，金属免疫这罐染料",
            StandardMaterial {
                base_color: Color::BLACK,
                perceptual_roughness: 0.05,
                reflectance: 1.0,
                specular_tint: Color::srgb(0.25, 0.85, 0.45),
                ..default()
            },
        ),
        (
            "灯箱球",
            "emissive 40 尼特橙——发光但不照亮邻居",
            StandardMaterial {
                base_color: Color::BLACK,
                emissive: LinearRgba::rgb(1.0, 0.35, 0.12) * 40.0,
                ..default()
            },
        ),
        (
            "锈锣",
            "ORM 三通道打包，标量拨 1 让贴图说话",
            StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/gong_base.png")),
                metallic_roughness_texture: Some(orm.clone()),
                occlusion_texture: Some(orm),
                metallic: 1.0,
                perceptual_roughness: 1.0,
                ..default()
            },
        ),
        (
            "雕花漆盖",
            "法线 + 深度图视差，坯子开过纹",
            StandardMaterial {
                base_color: Color::srgb(0.45, 0.08, 0.06),
                perceptual_roughness: 0.35,
                normal_map_texture: Some(carve_normal),
                depth_map: Some(carve_depth),
                parallax_depth_scale: 0.08,
                ..default()
            },
        ),
        (
            "剔红",
            "糙朱底 + clearcoat 1.0，亮壳浮在哑底上",
            StandardMaterial {
                base_color: Color::srgb(0.52, 0.08, 0.05),
                perceptual_roughness: 0.85,
                clearcoat: 1.0,
                clearcoat_perceptual_roughness: 0.08,
                ..default()
            },
        ),
        (
            "拉丝金",
            "anisotropy_strength 1.0——高光拉成纬线",
            StandardMaterial {
                base_color: Color::srgb(0.93, 0.76, 0.44),
                metallic: 1.0,
                perceptual_roughness: 0.45,
                anisotropy_strength: 1.0,
                ..default()
            },
        ),
        (
            "琉璃盏",
            "specular_transmission 1.0，ior 1.52，thickness 0.36",
            StandardMaterial {
                base_color: Color::WHITE,
                specular_transmission: 1.0,
                thickness: 0.36,
                ior: 1.52,
                perceptual_roughness: 0.05,
                ..default()
            },
        ),
    ];

    let plinth = meshes.add(Cylinder::new(0.46, 0.9).mesh().resolution(6));
    let plinth_paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.24, 0.25, 0.28),
        perceptual_roughness: 0.9,
        ..default()
    });
    // 球坯统一开纹：法线、视差、拉丝都要切线，其余展品带着也不碍事
    let ball = meshes.add(
        Mesh::from(Sphere::new(0.4))
            .with_generated_tangents()
            .unwrap(),
    );
    let lid = meshes.add(
        Mesh::from(Cuboid::new(0.82, 0.82, 0.82))
            .with_generated_tangents()
            .unwrap(),
    );

    let mut exhibits = Vec::new();
    for (i, (name, recipe, material)) in recipes.into_iter().enumerate() {
        let x = -5.25 + i as f32 * 1.5;
        let z = -0.045 * x * x; // 浅弧：两端往里收一点
        commands.spawn((
            Mesh3d(plinth.clone()),
            MeshMaterial3d(plinth_paint.clone()),
            Transform::from_xyz(x, 0.45, z),
        ));
        // 雕花漆盖用方盒，其余上球
        let mesh = if name == "雕花漆盖" { lid.clone() } else { ball.clone() };
        commands.spawn((
            Spin,
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(material)),
            Transform::from_xyz(x, 1.31, z),
        ));
        exhibits.push(Exhibit {
            name,
            recipe,
            center: Vec3::new(x, 1.31, z),
            radius: 1.7,
        });
    }
    commands.insert_resource(Tour {
        exhibits,
        current: 0,
    });

    println!("老雷：《琉璃记》验货——数字键 1~8 逐件看，0 退回全景，左键拖着转。");
}
// ANCHOR_END: exhibits

// ANCHOR: tour
/// 数字键巡展：换的只是转台的圆心与半径
fn visit_exhibit(keyboard: Res<ButtonInput<KeyCode>>, mut tour: ResMut<Tour>) {
    const DIGITS: [KeyCode; 8] = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
    ];
    for (i, key) in DIGITS.into_iter().enumerate() {
        if keyboard.just_pressed(key) {
            tour.current = i + 1;
            let exhibit = &tour.exhibits[i];
            println!("小棠：{} 号台《{}》——{}。", i + 1, exhibit.name, exhibit.recipe);
        }
    }
    if keyboard.just_pressed(KeyCode::Digit0) {
        tour.current = 0;
        println!("老雷：退回全景——八件齐活，这单货我认。");
    }
}

/// 展品缓缓自转：高光、拉丝、视差都得转着看才活
fn spin_exhibits(time: Res<Time>, mut exhibits: Query<&mut Transform, With<Spin>>) {
    for mut transform in &mut exhibits {
        transform.rotate_y(0.35 * time.delta_secs());
    }
}
// ANCHOR_END: tour

/// 影棚墙（Listing 24-1 原样）
#[derive(Resource)]
struct StudioWalls {
    image: Handle<Image>,
    hung: bool,
}

fn hang_studio(
    mut walls: ResMut<StudioWalls>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    camera: Single<Entity, With<Camera3d>>,
) {
    if walls.hung || !asset_server.load_state(&walls.image).is_loaded() {
        return;
    }
    let mut image = images.get_mut(&walls.image).unwrap();
    if image.texture_descriptor.array_layer_count() == 1 {
        let layers = image.height() / image.width();
        image.reinterpret_stacked_2d_as_array(layers).unwrap();
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }
    commands.entity(*camera).insert((
        Skybox {
            image: Some(walls.image.clone()),
            brightness: 700.0,
            ..default()
        },
        GeneratedEnvironmentMapLight {
            environment_map: walls.image.clone(),
            intensity: 1_000.0,
            ..default()
        },
    ));
    walls.hung = true;
}

// ANCHOR: orbit
/// 左键拖动转台（23.11 的手法）：圆心与半径听 Tour 的
fn orbit_camera(
    window: Single<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    tour: Res<Tour>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    mut last_cursor: Local<Option<Vec2>>,
    mut yaw: Local<f32>,
) {
    if mouse.pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            if let Some(prev) = *last_cursor {
                *yaw -= (pos.x - prev.x) * 0.008;
            }
            *last_cursor = Some(pos);
        }
    } else {
        *last_cursor = None;
    }
    let (center, radius, height) = if tour.current == 0 {
        (Vec3::new(0.0, 0.9, 0.0), 7.6, 1.5)
    } else {
        let exhibit = &tour.exhibits[tour.current - 1];
        (exhibit.center, exhibit.radius, 0.45)
    };
    let seat = center + Vec3::new(radius * yaw.sin(), height, radius * yaw.cos());
    **camera = Transform::from_translation(seat).looking_at(center, Vec3::Y);
}
// ANCHOR_END: orbit

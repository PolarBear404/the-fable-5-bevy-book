//! Listing 24-8：材质球画廊——一排材质球，各显一手；空格 / 点画面给车漆球拨清漆。
//! 桌面 `cargo run` 开窗口；编成 wasm 后渲染进网页 <canvas>，可点可看（球自转着展示）。

use bevy::{
    image::ImageLoaderSettings,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        view::Hdr,
    },
};

fn main() {
    App::new()
        // ANCHOR: web_window
        // 仅 Web 生效：把渲染塞进页面里 id="bevy-ch24" 的 <canvas>，并随容器缩放。
        // 这几个字段在桌面平台无效——同一份代码，桌面开窗口、网页进画布，两头通吃。
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-ch24".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        // ANCHOR_END: web_window
        .insert_resource(ClearColor(Color::srgb(0.05, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, (assemble_surroundings, spin, toggle_clearcoat))
        .run();
}

/// 转台上的展品：自转——立体的东西要转着看，高光才随视角扫过
#[derive(Component)]
struct Spin;

/// 能拨清漆的展品（车漆球）：空格 / 点画面时在它身上开关 clearcoat
#[derive(Component)]
struct Coatable;

/// 攥着待装配成立方体贴图的 skybox 提货单（同第 22 章）
#[derive(Resource)]
struct Surroundings {
    image: Handle<Image>,
    assembled: bool,
}

// ANCHOR: gallery
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball = meshes.add(Sphere::new(0.62));
    // 带切线的同款球，专给法线贴图用（没切线，法线图使不上劲）
    let studded = meshes.add(
        Sphere::new(0.62)
            .mesh()
            .build()
            .with_generated_tangents()
            .unwrap(),
    );
    let normal = asset_server.load_with_settings(
        "textures/studs-normal.png",
        |s: &mut ImageLoaderSettings| s.is_srgb = false,
    );

    // 六样漆一字排开，各挑本章一手：素胎 / 鎏金 / 自发光 / 法线 / 玻璃 / 车漆。
    // 末位标 true 的车漆球，可以拨清漆
    let gallery: [(Handle<Mesh>, StandardMaterial, bool); 6] = [
        (
            ball.clone(),
            StandardMaterial {
                base_color: Color::srgb(0.82, 0.74, 0.62),
                perceptual_roughness: 0.9,
                ..default()
            },
            false,
        ),
        (
            ball.clone(),
            StandardMaterial {
                base_color: Color::srgb(0.94, 0.74, 0.36),
                metallic: 1.0,
                perceptual_roughness: 0.2,
                ..default()
            },
            false,
        ),
        (
            ball.clone(),
            StandardMaterial {
                base_color: Color::BLACK,
                emissive: LinearRgba::rgb(0.1, 1.6, 1.5),
                ..default()
            },
            false,
        ),
        (
            studded.clone(),
            StandardMaterial {
                base_color: Color::srgb(0.55, 0.57, 0.62),
                perceptual_roughness: 0.45,
                metallic: 0.2,
                normal_map_texture: Some(normal.clone()),
                ..default()
            },
            false,
        ),
        (
            ball.clone(),
            StandardMaterial {
                base_color: Color::srgba(0.55, 0.85, 0.95, 0.35),
                perceptual_roughness: 0.05,
                alpha_mode: AlphaMode::Blend,
                ..default()
            },
            false,
        ),
        (
            ball.clone(),
            StandardMaterial {
                base_color: Color::srgb(0.10, 0.12, 0.45),
                metallic: 0.6,
                perceptual_roughness: 0.4,
                clearcoat: 1.0,
                clearcoat_perceptual_roughness: 0.08,
                ..default()
            },
            true,
        ),
    ];

    let n = gallery.len();
    let pedestal = meshes.add(Cylinder::new(0.5, 0.5));
    let wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.22, 0.16),
        perceptual_roughness: 0.8,
        ..default()
    });
    for (i, (mesh, material, coatable)) in gallery.into_iter().enumerate() {
        let x = (i as f32 - (n as f32 - 1.0) / 2.0) * 1.5;
        commands.spawn((
            Mesh3d(pedestal.clone()),
            MeshMaterial3d(wood.clone()),
            Transform::from_xyz(x, 0.25, 0.0),
        ));
        let mut showpiece = commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(material)),
            Transform::from_xyz(x, 1.12, 0.0),
            Spin,
        ));
        if coatable {
            showpiece.insert(Coatable);
        }
    }
    // ANCHOR_END: gallery

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(40.0, 40.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.13, 0.15),
            perceptual_roughness: 0.7,
            ..default()
        })),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 6000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.4, -0.7, -0.5), Vec3::Y),
    ));

    // skybox 先当普通图加载，等 assemble_surroundings 装配成立方体贴图再挂环境光照
    let skybox = asset_server.load("textures/skybox.png");
    commands.insert_resource(Surroundings {
        image: skybox,
        assembled: false,
    });

    // 相机开 Hdr——金属 / 清漆 / 玻璃要靠环境光照才照得出周遭的质感
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Transform::from_xyz(0.0, 2.4, 8.5).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    println!("小棠：六样漆一字排开——素胎、鎏金、自发光、点子皮、玻璃、车漆。");
    println!("场记：空格（网页里点画面）给车漆球开关那层清漆。");
}

// ANCHOR: assemble
/// 把竖摞六面的 skybox PNG 装配成立方体贴图，再交给相机当环境光照（同第 22 章的两步）
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
    image.reinterpret_stacked_2d_as_array(layers).unwrap();
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
    commands.entity(*camera).insert(GeneratedEnvironmentMapLight {
        environment_map: surroundings.image.clone(),
        intensity: 1400.0,
        ..default()
    });
    surroundings.assembled = true;
}
// ANCHOR_END: assemble

/// 转台：每颗球绕竖轴慢慢自转，高光随之扫过表面
fn spin(mut balls: Query<&mut Transform, With<Spin>>, time: Res<Time>) {
    for mut transform in &mut balls {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

// ANCHOR: toggle
/// 空格键，或网页里按下鼠标左键：给车漆球开关那层清漆，亲眼看高光多 / 少一层
fn toggle_clearcoat(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    coatable: Query<&MeshMaterial3d<StandardMaterial>, With<Coatable>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) && !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    for handle in &coatable {
        if let Some(material) = materials.get_mut(&handle.0) {
            material.clearcoat = if material.clearcoat > 0.5 { 0.0 } else { 1.0 };
        }
    }
}
// ANCHOR_END: toggle

//! Listing 22-8：环境光照——A 键开环境光兜底，E 键换三色天光，看镜面球照见世界

use bevy::prelude::*;

#[derive(Resource)]
struct Toggles {
    ambient: bool,
    env: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 开场把总闸拉死：没有灯，也没有兜底
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.013, 0.022, 0.040)))
        .insert_resource(Toggles {
            ambient: false,
            env: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_ambient, toggle_env))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.2, 10.5).looking_at(Vec3::new(0.0, 1.2, 0.0), Vec3::Y),
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
    // 左：素坯绣球——靠漫反射吃光
    commands.spawn((
        Mesh3d(plinth.clone()),
        MeshMaterial3d(wood.clone()),
        Transform::from_xyz(-2.2, 0.25, 1.6),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.62))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.82, 0.74, 0.62),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(-2.2, 1.12, 1.6),
    ));
    // 右：21 章材质墙上那颗黑着的镜面金球——它在等一个值得照的世界
    commands.spawn((
        Mesh3d(plinth.clone()),
        MeshMaterial3d(wood.clone()),
        Transform::from_xyz(2.2, 0.25, 1.6),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.62))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.93, 0.88),
            metallic: 1.0,
            perceptual_roughness: 0.05,
            ..default()
        })),
        Transform::from_xyz(2.2, 1.12, 1.6),
    ));

    println!("老烛：一盏灯都没点，兜底也拉了。A 上环境光，E 上三色天光。");
}

// ANCHOR: ambient
/// A 键：环境光兜底开/关——四面八方等量的一层亮，无向、无影
fn toggle_ambient(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut toggles: ResMut<Toggles>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    if keyboard.just_pressed(KeyCode::KeyA) {
        toggles.ambient = !toggles.ambient;
        *ambient = if toggles.ambient {
            GlobalAmbientLight {
                brightness: 120.0,
                ..default()
            }
        } else {
            GlobalAmbientLight::NONE
        };
        println!(
            "老烛：环境光{}。",
            if toggles.ambient {
                "上了——满场一层平光；镜球照出一团死灰，四面八方全一样，等于什么都没照见"
            } else {
                "撤了"
            }
        );
    }
}
// ANCHOR_END: ambient

// ANCHOR: env
/// E 键：三色天光开/关——一张一像素见方的环境贴图，上蓝、中白、下褐
fn toggle_env(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut toggles: ResMut<Toggles>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    camera: Single<Entity, With<Camera3d>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        toggles.env = !toggles.env;
        if toggles.env {
            let mut env = EnvironmentMapLight::hemispherical_gradient(
                &mut images,
                Color::srgb(0.35, 0.55, 0.95), // 天顶蓝
                Color::srgb(0.85, 0.83, 0.78), // 地平线乳白
                Color::srgb(0.30, 0.24, 0.18), // 台面褐
            );
            env.intensity = 900.0; // 环境光照的亮度单位也是 cd/m²
            commands.entity(*camera).insert(env);
            println!("老烛：三色天光上了——镜球照出天与地，素坯有了上下脸，台面染了层天青。");
        } else {
            commands.entity(*camera).remove::<EnvironmentMapLight>();
            println!("老烛：天光撤了。");
        }
    }
}
// ANCHOR_END: env

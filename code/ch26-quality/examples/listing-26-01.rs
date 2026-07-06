//! Listing 26-1：冲印配方——空格轮换九种 Tonemapping，H 键抽/插 HDR 底片

use bevy::camera::Hdr;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;

// ANCHOR: recipes
/// 九种冲印配方，顺序照 vendor 源码里的枚举定义
const RECIPES: [(&str, Tonemapping); 9] = [
    ("None——不冲印，原样硬塞", Tonemapping::None),
    ("Reinhard", Tonemapping::Reinhard),
    ("ReinhardLuminance", Tonemapping::ReinhardLuminance),
    ("AcesFitted", Tonemapping::AcesFitted),
    ("AgX", Tonemapping::AgX),
    (
        "SomewhatBoringDisplayTransform",
        Tonemapping::SomewhatBoringDisplayTransform,
    ),
    ("TonyMcMapface——默认", Tonemapping::TonyMcMapface),
    ("BlenderFilmic", Tonemapping::BlenderFilmic),
    ("KhronosPbrNeutral", Tonemapping::KhronosPbrNeutral),
];

#[derive(Resource)]
struct Darkroom {
    recipe: usize, // 当前配方在 RECIPES 里的下标
}
// ANCHOR_END: recipes

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.035)))
        // 开场就站在默认配方上（TonyMcMapface 是 RECIPES[6]）
        .insert_resource(Darkroom { recipe: 6 })
        .add_systems(Startup, setup)
        .add_systems(Update, (swap_recipe, pull_film))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    // Hdr 是个不带字段的标记组件：挂上它，这台相机的中间画布就换成高动态范围
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Transform::from_xyz(0.0, 2.5, 6.8).looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y),
    ));
    // ANCHOR_END: camera

    // 台面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(22.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.23, 0.24, 0.27),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // 三匹绸缎：大红、翠绿、宝蓝——高饱和色是冲印配方最好的试纸
    let silks = [
        (Color::srgb(0.92, 0.06, 0.05), -2.6),
        (Color::srgb(0.10, 0.90, 0.12), 0.0),
        (Color::srgb(0.08, 0.25, 0.95), 2.6),
    ];
    for (color, x) in silks {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.5, 2.6, 0.08))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                perceptual_roughness: 0.55,
                ..default()
            })),
            Transform::from_xyz(x, 1.3, -1.0),
        ));
    }

    // 白瓷瓶：中性参照物，配方偏不偏色看它最清楚
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.42, 0.9))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.90, 0.87),
            perceptual_roughness: 0.25,
            ..default()
        })),
        Transform::from_xyz(2.2, 0.87, 2.6),
    ));

    // 一盏大红灯笼：自发光值远超 1.0，就是给高光留的“考题”
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.2, 0.1),
            emissive: LinearRgba::new(18.0, 3.5, 1.2, 1.0),
            ..default()
        })),
        Transform::from_xyz(-4.2, 3.0, -0.5),
    ));

    // 顶灯打得很足：故意让绸缎的受光面冲出 1.0，看各配方怎么收场
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.95, 0.88),
            intensity: 6_000_000.0,
            range: 30.0,
            ..default()
        },
        Transform::from_xyz(0.0, 4.2, 2.2),
    ));
    // 机位旁一盏补光：把瓷瓶和绸缎的正面托出来，别让参照物黑成剪影
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.95, 0.88),
            intensity: 600_000.0,
            range: 30.0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.2, 8.0),
    ));

    println!("盛师傅：三匹绸子一只瓶，一盏灯笼当考题。");
    println!("盛师傅：空格换冲印配方，H 抽插 HDR 底片。眼下是 TonyMcMapface——默认。");
}

// ANCHOR: swap
/// 空格轮换冲印配方：Tonemapping 就是相机实体上的一个组件，改字段即换算法
fn swap_recipe(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut darkroom: ResMut<Darkroom>,
    mut tonemapping: Single<&mut Tonemapping>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        darkroom.recipe = (darkroom.recipe + 1) % RECIPES.len();
        let (name, recipe) = RECIPES[darkroom.recipe];
        **tonemapping = recipe;
        println!("盛师傅：换配方——{name}。");
    }
}
// ANCHOR_END: swap

// ANCHOR: film
/// H 键抽/插底片：Hdr 只是标记组件，insert/remove 就是换底片
fn pull_film(
    keyboard: Res<ButtonInput<KeyCode>>,
    camera: Single<(Entity, Has<Hdr>), With<Camera3d>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        let (entity, has_hdr) = *camera;
        if has_hdr {
            commands.entity(entity).remove::<Hdr>();
            println!("盛师傅：底片抽了——回标准动态范围，亮部全靠 1.0 封顶。");
        } else {
            commands.entity(entity).insert(Hdr);
            println!("盛师傅：HDR 底片插回去，亮部又有账可算了。");
        }
    }
}
// ANCHOR_END: film

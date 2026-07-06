//! Listing 26-2：灯笼上晕——Bloom 入门；↑↓ 拨总强度，+/- 单给金灯加亮

use bevy::camera::Exposure;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

/// 金灯材质的把手，存起来好在运行时改它的 emissive
#[derive(Resource)]
struct GoldLantern {
    material: Handle<StandardMaterial>,
    boost: f32, // 自发光倍率，1.0 = 原亮度
}

/// 金灯出厂的自发光值：往上加亮全在这个基数上乘
const GOLD_EMISSIVE: LinearRgba = LinearRgba::new(7.0, 5.0, 1.6, 1.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.035)))
        .add_systems(Startup, setup)
        .add_systems(Update, (bloom_knob, lantern_knob))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    // 只写 Bloom，不写 Hdr——Bloom 声明了 #[require(Hdr)]，底片自动换成高动态范围
    commands.spawn((
        Camera3d::default(),
        Exposure::INDOOR, // 夜戏进光口径——22.2 的室内档
        Bloom::NATURAL,
        Transform::from_xyz(0.0, 3.0, 8.5).looking_at(Vec3::new(0.0, 1.8, 0.0), Vec3::Y),
    ));
    // ANCHOR_END: camera

    // 台面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(22.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.31, 0.34),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // ANCHOR: lanterns
    // 三盏灯笼：红纱、金纱、青纱。发晕的本钱是 emissive 远超 1.0——
    // 灯罩是自发光网格，照亮台面的活儿交给挂在同一实体上的 PointLight
    let lanterns = [
        (
            "红纱",
            -3.0,
            LinearRgba::new(9.0, 1.6, 0.8, 1.0),
            Color::srgb(1.0, 0.45, 0.30),
        ),
        ("金纱", 0.0, GOLD_EMISSIVE, Color::srgb(1.0, 0.85, 0.45)),
        (
            "青纱",
            3.0,
            LinearRgba::new(0.9, 5.5, 6.5, 1.0),
            Color::srgb(0.45, 0.9, 1.0),
        ),
    ];
    for (name, x, emissive, light_color) in lanterns {
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.1),
            emissive,
            ..default()
        });
        if name == "金纱" {
            commands.insert_resource(GoldLantern {
                material: material.clone(),
                boost: 1.0,
            });
        }
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.45))),
            MeshMaterial3d(material),
            Transform::from_xyz(x, 2.6, 0.0),
            children![(
                PointLight {
                    color: light_color,
                    intensity: 25_000.0,
                    range: 20.0,
                    ..default()
                },
                Transform::IDENTITY,
            )],
        ));
    }
    // ANCHOR_END: lanterns

    println!("盛师傅：三盏灯笼挂齐，辉光按自然档走。");
    println!("盛师傅：上下键拨总强度；想单让金灯出彩，+/- 直接加它的自发光。");
}

// ANCHOR: bloom_knob
/// ↑↓ 拨 Bloom 总强度：这是全画面的散射量，不是哪一盏灯的亮度
fn bloom_knob(keyboard: Res<ButtonInput<KeyCode>>, mut bloom: Single<&mut Bloom>) {
    let step = 0.05;
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        bloom.intensity = (bloom.intensity + step).min(1.0);
        println!("盛师傅：辉光总强度 {:.2}。", bloom.intensity);
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        bloom.intensity = (bloom.intensity - step).max(0.0);
        println!("盛师傅：辉光总强度 {:.2}。", bloom.intensity);
    }
}
// ANCHOR_END: bloom_knob

// ANCHOR: lantern_knob
/// +/- 只动金灯的 emissive：想让谁更亮，加它的自发光，而不是加全场的辉光
fn lantern_knob(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut gold: ResMut<GoldLantern>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut changed = false;
    if keyboard.just_pressed(KeyCode::Equal) {
        gold.boost = (gold.boost * 1.6).min(20.0);
        changed = true;
    }
    if keyboard.just_pressed(KeyCode::Minus) {
        gold.boost = (gold.boost / 1.6).max(0.2);
        changed = true;
    }
    if changed && let Some(mut material) = materials.get_mut(&gold.material) {
        material.emissive = GOLD_EMISSIVE * gold.boost;
        println!("盛师傅：金灯自发光 ×{:.1}——只有它的晕在长。", gold.boost);
    }
}
// ANCHOR_END: lantern_knob

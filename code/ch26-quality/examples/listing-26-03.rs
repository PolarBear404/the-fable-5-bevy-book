//! Listing 26-3：四套晕法——空格轮换 Bloom 的四个预设

use bevy::camera::Exposure;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

// ANCHOR: presets
/// 四个预设全是 Bloom 上的关联常量，一个常量一整套参数
const PRESETS: [(&str, Bloom); 4] = [
    ("NATURAL——自然档，默认", Bloom::NATURAL),
    ("OLD_SCHOOL——复古断档", Bloom::OLD_SCHOOL),
    ("ANAMORPHIC——横拉宽银幕", Bloom::ANAMORPHIC),
    ("SCREEN_BLUR——满屏蒙纱", Bloom::SCREEN_BLUR),
];

#[derive(Resource)]
struct BloomBooth {
    preset: usize,
}
// ANCHOR_END: presets

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.035)))
        .insert_resource(BloomBooth { preset: 0 })
        .add_systems(Startup, setup)
        .add_systems(Update, swap_preset)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Exposure::INDOOR, // 夜戏进光口径——22.2 的室内档
        Bloom::NATURAL,
        Transform::from_xyz(0.0, 3.0, 8.5).looking_at(Vec3::new(0.0, 1.8, 0.0), Vec3::Y),
    ));

    // 台面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(22.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.23, 0.24, 0.27),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // 主灯笼一盏（强发光）+ 檐下一串珠灯（亮度递减的小亮点）——
    // 珠灯是给 OLD_SCHOOL 的门槛和 ANAMORPHIC 的横拉当试纸的
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.45))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.1),
            emissive: LinearRgba::new(9.0, 6.5, 2.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 2.8, 0.0),
        children![(
            PointLight {
                color: Color::srgb(1.0, 0.85, 0.45),
                intensity: 25_000.0,
                range: 20.0,
                ..default()
            },
            Transform::IDENTITY,
        )],
    ));
    // ANCHOR: beads
    for i in 0..9 {
        let x = -4.0 + i as f32;
        // 珠灯亮度对半递减：8.0、4.0、2.0、1.0、0.5……最右一颗只剩 0.03——
        // 后半截连 OLD_SCHOOL 的 0.6 门槛都过不了
        let brightness = 8.0 * 0.5_f32.powi(i);
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.11))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.05, 0.02),
                emissive: LinearRgba::new(brightness, brightness * 0.55, brightness * 0.14, 1.0),
                ..default()
            })),
            Transform::from_xyz(x, 4.1, -1.5),
        ));
    }
    // ANCHOR_END: beads

    println!("盛师傅：一盏主灯九颗珠，珠灯亮度从左到右对半砍。");
    println!("盛师傅：空格换晕法。眼下是 NATURAL——自然档，默认。");
}

// ANCHOR: swap
/// 空格轮换预设：预设之间换的不是一个 intensity，是一整套参数
fn swap_preset(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut booth: ResMut<BloomBooth>,
    mut bloom: Single<&mut Bloom>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        booth.preset = (booth.preset + 1) % PRESETS.len();
        let (name, preset) = &PRESETS[booth.preset];
        **bloom = preset.clone();
        println!("盛师傅：换{name}。");
    }
}
// ANCHOR_END: swap

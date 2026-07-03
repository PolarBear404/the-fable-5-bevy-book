//! Listing 22-5：灯箱——RectLight 面光源，方向键改宽高，R 换台面粗糙度

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.013, 0.022, 0.040)))
        .add_systems(Startup, setup)
        .add_systems(Update, (resize, roughen))
        .run();
}

/// 标记：上了大漆的镜面台面
#[derive(Component)]
struct GlossyDeck;

/// 标记：匾额的可见面板（发光的皮，光本身是 RectLight 发的）
#[derive(Component)]
struct Panel;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.8, 10.5).looking_at(Vec3::new(0.0, 1.0, -1.0), Vec3::Y),
    ));

    // 台面这回上的是大漆：金属打底、越光越像一面昏暗的镜子
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 16.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.33, 0.30),
            metallic: 1.0,
            perceptual_roughness: 0.12,
            ..default()
        })),
        GlossyDeck,
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.62))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.82, 0.74, 0.62),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(-2.6, 0.62, 1.0),
    ));
    // 灯箱正下方立一根杆：矩形光不投影子，杆下不会有影
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.12, 3.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.40, 0.24),
            perceptual_roughness: 0.85,
            ..default()
        })),
        Transform::from_xyz(2.4, 1.5, 0.2),
    ));

    // ANCHOR: rect
    // 灯箱：RectLight 是一整块发光的面，躺在自己 XY 平面里、朝自己的 -Z 发光。
    // 立在台后当匾——转个身让 -Z 冲着观众席。匾额那层可见的“发光皮”
    // 是子实体上的自发光矩形网格，跟点光的灯泡一个道理
    commands.spawn((
        RectLight {
            color: Color::srgb(1.0, 0.62, 0.36),
            intensity: 600_000.0, // 还是流明：整块面往外泼的总量
            width: 3.0,
            height: 1.2,
            range: 30.0,
        },
        Transform::from_xyz(0.0, 2.4, -5.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        children![(
            Mesh3d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.72, 0.42),
                // 匾面就贴在光源平面上，别让它再吃自己发的光——unlit 直出本色
                unlit: true,
                ..default()
            })),
            // 父实体已整体转身，把发光皮再翻回来对着观众（背面是剔除的，21.4 节）
            Transform::from_scale(Vec3::new(3.0, 1.2, 1.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            Panel,
        )],
    ));
    // ANCHOR_END: rect

    println!("老烛：灯箱挂上了。方向键改宽高，R 换台面的粗细。");
}

// ANCHOR: resize
/// 方向键改灯箱的宽与高——台面倒影里的形状跟着变；发光皮同步缩放
fn resize(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut rect: Single<&mut RectLight>,
    mut panel: Single<&mut Transform, With<Panel>>,
) {
    let mut changed = false;
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        rect.width = (rect.width - 0.8).max(0.6);
        changed = true;
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        rect.width = (rect.width + 0.8).min(8.0);
        changed = true;
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        rect.height = (rect.height - 0.6).max(0.4);
        changed = true;
    }
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        rect.height = (rect.height + 0.6).min(5.0);
        changed = true;
    }
    if changed {
        panel.scale = Vec3::new(rect.width, rect.height, 1.0);
        println!("老烛：灯箱改到 {:.1} × {:.1} 米。", rect.width, rect.height);
    }
}
// ANCHOR_END: resize

/// R 键在光亮与磨砂之间换台面——看灯箱的倒影糊成什么样
fn roughen(
    keyboard: Res<ButtonInput<KeyCode>>,
    deck: Single<&MeshMaterial3d<StandardMaterial>, With<GlossyDeck>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        let Some(mut mat) = materials.get_mut(&deck.0) else {
            return;
        };
        mat.perceptual_roughness = if mat.perceptual_roughness < 0.3 { 0.5 } else { 0.12 };
        println!("老烛：台面粗糙度 {:.2}。", mat.perceptual_roughness);
    }
}

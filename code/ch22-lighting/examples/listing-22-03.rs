//! Listing 22-3：追光——SpotLight 盯着角儿走，左右键走圆场，[ ] 收放光锥

use bevy::prelude::*;

/// 标记：台上的角儿
#[derive(Component)]
struct Actor;

/// 标记：追光灯
#[derive(Component)]
struct FollowSpot;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 留一点点环境光垫底：出了追光的人也得隐约可见
        .insert_resource(GlobalAmbientLight {
            brightness: 8.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.013, 0.022, 0.040)))
        .add_systems(Startup, setup)
        .add_systems(Update, (walk, aim, tune).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.6, 11.0).looking_at(Vec3::new(0.0, 1.2, 0.0), Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.31, 0.30),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    // 台角的大鼓，给光锥边缘一个可以扫过的参照物
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.7, 0.9))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.52, 0.16, 0.12),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(4.2, 0.45, -0.6),
    ));

    // 角儿：一身武生打扮——胶囊身子顶个头
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.32, 1.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.68, 0.58),
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.87, 1.2),
        Actor,
        children![(
            Mesh3d(meshes.add(Sphere::new(0.24))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.86, 0.72, 0.60),
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_xyz(0.0, 1.02, 0.0),
        )],
    ));

    // ANCHOR: spot
    // 追光：吊在台口上方。SpotLight 照向自己的 -Z——跟相机、平行光一个规矩
    commands.spawn((
        SpotLight {
            color: Color::srgb(1.0, 0.97, 0.88),
            intensity: 3_000_000.0, // 追光是全场最亮的家伙：光都拢在一个小锥里
            range: 40.0,
            outer_angle: 0.32, // 光锥的外沿：出了这个角度一点光没有
            inner_angle: 0.22, // 内沿：从内沿到外沿，光顺滑地衰减出去
            ..default()
        },
        Transform::from_xyz(0.0, 7.0, 9.0),
        FollowSpot,
    ));
    // ANCHOR_END: spot

    println!("老烛：追光就位。左右键请角儿走圆场，光自己跟。");
    println!("老烛：[ ] 收放光圈，I 换硬边软边。");
}

/// 左右键走圆场
fn walk(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut actor: Single<&mut Transform, With<Actor>>,
) {
    let mut dir = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) {
        dir -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        dir += 1.0;
    }
    actor.translation.x = (actor.translation.x + dir * 2.0 * time.delta_secs()).clamp(-5.0, 5.0);
}

// ANCHOR: aim
/// 追光跟人：每帧把灯的 -Z 对准角儿——瞄准就是摆 Transform（第 12 章的老手艺）
fn aim(
    actor: Single<&Transform, With<Actor>>,
    mut spot: Single<&mut Transform, (With<FollowSpot>, Without<Actor>)>,
) {
    let target = actor.translation;
    spot.look_at(target, Vec3::Y);
}
// ANCHOR_END: aim

// ANCHOR: tune
/// [ ] 收放外沿；I 在“硬边”（内沿贴着外沿）与“软边”（内沿收进去）之间切换
fn tune(keyboard: Res<ButtonInput<KeyCode>>, mut spot: Single<&mut SpotLight>) {
    if keyboard.just_pressed(KeyCode::BracketLeft) {
        spot.outer_angle = (spot.outer_angle - 0.06).max(0.10);
        spot.inner_angle = spot.inner_angle.min(spot.outer_angle);
        println!("老烛：光圈收到 {:.2} 弧度。", spot.outer_angle);
    }
    if keyboard.just_pressed(KeyCode::BracketRight) {
        spot.outer_angle = (spot.outer_angle + 0.06).min(1.2);
        println!("老烛：光圈放到 {:.2} 弧度。", spot.outer_angle);
    }
    if keyboard.just_pressed(KeyCode::KeyI) {
        if spot.inner_angle < spot.outer_angle * 0.95 {
            spot.inner_angle = spot.outer_angle;
            println!("老烛：内沿贴上外沿——硬边光，圈口像刀裁的。");
        } else {
            spot.inner_angle = spot.outer_angle * 0.6;
            println!("老烛：内沿收进去——软边光，圈口一圈羽化。");
        }
    }
}
// ANCHOR_END: tune

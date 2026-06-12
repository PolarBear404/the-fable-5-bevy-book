//! Listing 13-3：跟拍第一版——镜头死死钉住阿燕

use bevy::prelude::*;

/// 标记：侠客阿燕
#[derive(Component)]
struct Hero;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.15)))
        .add_systems(Startup, setup)
        // 先走位、后跟拍：镜头读的必须是本帧的新位置
        .add_systems(Update, (walk_hero, follow_hero).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.13, 0.11), Vec2::new(1000.0, 560.0)),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
    for i in -3..=3 {
        for y in [-240.0, 240.0] {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.95, 0.75, 0.25), Vec2::splat(22.0)),
                Transform::from_xyz(i as f32 * 160.0, y, -5.0),
            ));
        }
    }
    commands.spawn((
        Hero,
        Sprite::from_color(Color::srgb(0.85, 0.2, 0.2), Vec2::splat(30.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    println!("老雷：阿燕走起来，1 号机跟住他！");
}

// ANCHOR: walk
/// 阿燕的走位：满场跑一个大“8”字
fn walk_hero(time: Res<Time>, mut hero: Single<&mut Transform, With<Hero>>) {
    let t = time.elapsed_secs() * 0.5;
    hero.translation.x = 420.0 * t.sin();
    hero.translation.y = 210.0 * (2.0 * t).sin();
}
// ANCHOR_END: walk

// ANCHOR: follow
/// 1 号机：阿燕在哪，镜头中心就在哪
fn follow_hero(
    mut lens: Single<&mut Transform, (With<Camera2d>, Without<Hero>)>,
    hero: Single<&Transform, With<Hero>>,
) {
    lens.translation.x = hero.translation.x;
    lens.translation.y = hero.translation.y;
    // z 留在原地：2D 相机的 z 不参与构图，但别让它乱跑
}
// ANCHOR_END: follow

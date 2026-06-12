//! Listing 13-4：跟拍第二版——smooth_nudge，镜头像人扛的

use bevy::prelude::*;

/// 标记：侠客阿燕
#[derive(Component)]
struct Hero;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.15)))
        .add_systems(Startup, setup)
        .add_systems(Update, (walk_hero, follow_hero).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.13, 0.11), Vec2::new(1400.0, 900.0)),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
    for i in -3..=3 {
        for y in [-350.0, 350.0] {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.95, 0.75, 0.25), Vec2::splat(22.0)),
                Transform::from_xyz(i as f32 * 200.0, y, -5.0),
            ));
        }
    }
    commands.spawn((
        Hero,
        Sprite::from_color(Color::srgb(0.85, 0.2, 0.2), Vec2::splat(30.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    println!("老雷：镜头柔一点——像人扛的，不是钉死的。");
}

fn walk_hero(time: Res<Time>, mut hero: Single<&mut Transform, With<Hero>>) {
    let t = time.elapsed_secs() * 0.5;
    hero.translation.x = 500.0 * t.sin();
    hero.translation.y = 250.0 * (2.0 * t).sin();
}

// ANCHOR: follow
/// 1 号机：每帧向阿燕“追近一段”，永远追不死——镜头便有了延迟感
fn follow_hero(
    time: Res<Time>,
    mut lens: Single<&mut Transform, (With<Camera2d>, Without<Hero>)>,
    hero: Single<&Transform, With<Hero>>,
) {
    let target = hero.translation.with_z(lens.translation.z);
    lens.translation.smooth_nudge(&target, 2.0, time.delta_secs());
}
// ANCHOR_END: follow

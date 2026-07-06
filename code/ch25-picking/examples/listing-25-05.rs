//! Listing 25-5：按压三部曲与连击计数——Press、Release、Click 和那只 500 毫秒的秒表

use std::time::Duration;

use bevy::picking::PickingSettings;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, tune_stopwatch)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.8, 6.4).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.0, 6.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.40, 0.42),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // ANCHOR: observers
    // 今日主角只有漆盒一件，三段戏都挂它身上
    commands
        .spawn((
            Name::new("剔红漆盒"),
            Mesh3d(meshes.add(Cuboid::new(1.4, 1.4, 1.4))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.62, 0.11, 0.08),
                perceptual_roughness: 0.35,
                ..default()
            })),
            Transform::from_xyz(0.0, 1.2, 0.0),
        ))
        .observe(|press: On<Pointer<Press>>| {
            println!("场记：按下（{:?}，连按第 {} 回）。", press.button, press.count);
        })
        .observe(|release: On<Pointer<Release>>| {
            println!("场记：抬手（{:?}）。", release.button);
        })
        .observe(|click: On<Pointer<Click>>| {
            println!(
                "场记：成交一击——连击第 {} 回，按了 {} 毫秒。",
                click.count,
                click.duration.as_millis()
            );
        });
    // ANCHOR_END: observers
    println!("老雷：陆掌柜验漆盒——单点看成色，双击开盖，按 T 换连击秒表。");
}

// ANCHOR: tune
/// 连击秒表：两击间隔小于 multi_click_interval 才算连上
fn tune_stopwatch(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<PickingSettings>) {
    if keys.just_pressed(KeyCode::KeyT) {
        settings.multi_click_interval =
            if settings.multi_click_interval == Duration::from_millis(120) {
                Duration::from_millis(500)
            } else {
                Duration::from_millis(120)
            };
        println!(
            "小棠：连击秒表拨到 {} 毫秒。",
            settings.multi_click_interval.as_millis()
        );
    }
}
// ANCHOR_END: tune

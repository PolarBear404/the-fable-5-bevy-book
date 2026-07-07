//! Listing 27-6：慢时钟下的粉线——同一只箱子，Update 与 FixedUpdate 各描一框。
//! 固定时钟拨到每秒 4 拍：绿框跟手，橙框一步一顿却不闪不灭。

use bevy::color::palettes::css::{LIME, ORANGE_RED};
use bevy::prelude::*;

const CRATE_SIZE: Vec2 = Vec2::new(120.0, 90.0);
const TRACK_HALF: f32 = 320.0;

#[derive(Component)]
struct PropCrate {
    speed: f32,
}

// ANCHOR: app
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 固定时钟放慢到每秒 4 拍，两种粉线的差距一眼可见
        .insert_resource(Time::<Fixed>::from_hz(4.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (slide_crate, chalk_every_frame).chain())
        .add_systems(FixedUpdate, chalk_every_tick)
        .run();
}
// ANCHOR_END: app

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        PropCrate { speed: 240.0 },
        Sprite::from_color(Color::srgb(0.52, 0.42, 0.30), CRATE_SIZE),
        Transform::from_xyz(-TRACK_HALF, 0.0, 0.0),
    ));
    println!("检场：固定时钟每秒 4 拍。绿框每帧描，橙框每拍描。");
}

fn slide_crate(mut crates: Query<(&mut PropCrate, &mut Transform)>, time: Res<Time>) {
    for (mut prop, mut transform) in &mut crates {
        transform.translation.x += prop.speed * time.delta_secs();
        if transform.translation.x.abs() > TRACK_HALF {
            transform.translation.x = transform.translation.x.clamp(-TRACK_HALF, TRACK_HALF);
            prop.speed = -prop.speed;
        }
    }
}

// ANCHOR: two_clocks
/// 渲染时钟的粉线：每帧重画，紧贴箱子
fn chalk_every_frame(mut gizmos: Gizmos, crates: Query<&Transform, With<PropCrate>>) {
    for transform in &crates {
        gizmos.rect_2d(transform.translation.truncate(), CRATE_SIZE, LIME);
    }
}

/// 固定时钟的粉线：每拍才画一次——可它在两拍之间一直挂在屏上
fn chalk_every_tick(mut gizmos: Gizmos, crates: Query<&Transform, With<PropCrate>>) {
    for transform in &crates {
        gizmos.rect_2d(
            transform.translation.truncate(),
            CRATE_SIZE * 1.25, // 画大一圈，别跟绿框叠死
            ORANGE_RED,
        );
    }
}
// ANCHOR_END: two_clocks

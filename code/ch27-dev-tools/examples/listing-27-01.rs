//! Listing 27-1：第一道粉线——道具箱滑过舞台，检场人给它描框、画速度箭头。
//! 粉线是即时模式：一帧一画，不画就没有。按住空格让检场歇手，粉线当场消失。

use bevy::color::palettes::css::{GOLD, ORANGE_RED};
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;

const CRATE_SIZE: Vec2 = Vec2::new(120.0, 90.0); // 道具箱外框
const TRACK_HALF: f32 = 320.0; // 滑轨半程
const CRATE_COLOR: Color = Color::srgb(0.52, 0.42, 0.30);

// ANCHOR: crate_type
/// 会自己在台上来回滑的道具箱
#[derive(Component)]
struct PropCrate {
    /// 眼下的横向速度（像素/秒，带方向）
    speed: f32,
}
// ANCHOR_END: crate_type

// ANCHOR: app
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                slide_crate,
                // 按住空格 = 检场歇手：这一帧不画，粉线就没有
                chalk_marks.run_if(not(input_pressed(KeyCode::Space))),
            )
                .chain(),
        )
        .run();
}
// ANCHOR_END: app

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        PropCrate { speed: 240.0 },
        Sprite::from_color(CRATE_COLOR, CRATE_SIZE),
        Transform::from_xyz(-TRACK_HALF, 0.0, 0.0),
    ));
    println!("检场：箱子上道。粉线伺候——按住空格我就歇手。");
}

/// 道具箱沿滑轨往返：碰到头就掉头
fn slide_crate(mut crates: Query<(&mut PropCrate, &mut Transform)>, time: Res<Time>) {
    for (mut prop, mut transform) in &mut crates {
        transform.translation.x += prop.speed * time.delta_secs();
        if transform.translation.x.abs() > TRACK_HALF {
            transform.translation.x = transform.translation.x.clamp(-TRACK_HALF, TRACK_HALF);
            prop.speed = -prop.speed;
        }
    }
}

// ANCHOR: chalk
/// 检场的粉线：描外框、画速度箭头。每帧都得重画一遍
fn chalk_marks(mut gizmos: Gizmos, crates: Query<(&PropCrate, &Transform)>) {
    for (prop, transform) in &crates {
        let center = transform.translation.truncate();
        // 外框：把道具箱的占地描出来
        gizmos.rect_2d(center, CRATE_SIZE, GOLD);
        // 速度箭头：从中心指向去处，长短对应快慢（半秒的路程）
        gizmos.arrow_2d(center, center + Vec2::X * prop.speed * 0.5, ORANGE_RED);
    }
}
// ANCHOR_END: chalk

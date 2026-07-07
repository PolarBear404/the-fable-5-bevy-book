//! Listing 27-9：场记的账本——三本内置账 + 一位每秒念账的播报员。
//! F 只听 fps 一条，G 恢复全念；T 把播报间隔改成 0.25 秒。

use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    LogDiagnosticsState, SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::*;
use core::time::Duration;

const CRATE_SIZE: Vec2 = Vec2::new(120.0, 90.0);
const TRACK_HALF: f32 = 320.0;

#[derive(Component)]
struct PropCrate {
    speed: f32,
}

// ANCHOR: app
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // 三本账：帧账（fps/frame_time/frame_count）、口数账（entity_count）、
            // 机器账（system/process 的 cpu_usage 与 mem_usage）
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
            SystemInformationDiagnosticsPlugin,
            // 播报员：每秒把在册的账念一遍到终端
            LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (slide_crate, tune_log))
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
    println!("场记：开账。F 只念 fps，G 恢复全念，T 改成每 0.25 秒念一轮。");
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

// ANCHOR: tune
/// 播报口径运行期可拨：LogDiagnosticsState 管过滤与节奏
fn tune_log(keyboard: Res<ButtonInput<KeyCode>>, mut log_state: ResMut<LogDiagnosticsState>) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        log_state.enable_filtering(); // 先立一张空名单：谁都不念
        log_state.add_filter(FrameTimeDiagnosticsPlugin::FPS); // 再把 fps 添进名单
        println!("场记：只念 fps。");
    }
    if keyboard.just_pressed(KeyCode::KeyG) {
        log_state.disable_filtering(); // 撤掉名单：在册的全念
        println!("场记：恢复全念。");
    }
    if keyboard.just_pressed(KeyCode::KeyT) {
        log_state.set_timer_duration(Duration::from_millis(250));
        println!("场记：改成每 0.25 秒念一轮。");
    }
}
// ANCHOR_END: tune

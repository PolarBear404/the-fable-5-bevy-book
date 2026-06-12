//! Listing 12-10：慢半拍的实测账——GlobalTransform 的更新时机
//! 籍册（Transform）当场改，实测（GlobalTransform）要等 PostUpdate 的传播

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::transform::TransformPlugin;
use std::time::Duration;

/// 探针卫星：实验对象
#[derive(Component)]
struct Probe;

// ANCHOR: main
fn main() {
    App::new()
        // 无窗口实验台：MinimalPlugins 不含 TransformPlugin，手动补上传播系统
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(200))),
            TransformPlugin,
        ))
        .add_systems(Startup, launch)
        .add_systems(Update, (banner, push, observe).chain())
        .run();
}
// ANCHOR_END: main

fn launch(mut commands: Commands) {
    commands.spawn((Probe, Transform::from_xyz(0.0, 0.0, 0.0)));
}

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

// ANCHOR: push
/// 装配工：第 1 帧把探针推到 x = 100，之后收手
fn push(mut probe: Single<&mut Transform, With<Probe>>, mut done: Local<bool>) {
    if !*done {
        probe.translation.x = 100.0;
        println!("  装配工：探针推到 x = 100，籍册改讫。");
        *done = true;
    }
}
// ANCHOR_END: push

// ANCHOR: observe
/// 观测站：同一帧紧跟着读两本账
fn observe(
    probe: Single<(&Transform, &GlobalTransform), With<Probe>>,
    mut frames: Local<u32>,
    mut exit: MessageWriter<AppExit>,
) {
    let (local, global) = *probe;
    println!(
        "  观测站：籍册 x = {}，实测 x = {}",
        local.translation.x,
        global.translation().x
    );
    *frames += 1;
    if *frames >= 2 {
        exit.write(AppExit::Success);
    }
}
// ANCHOR_END: observe

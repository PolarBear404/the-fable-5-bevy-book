//! Listing 11-12：谁动了黄油——track_location 与 changed_by()
//! 需要 Cargo feature：bevy = { features = ["track_location"] }

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

/// 黄油（桶）
#[derive(Component)]
struct Butter(u32);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((Name::new("库房"), Butter(12)));
        })
        .add_systems(Update, (banner, kitchen, tavern, detective).chain())
        .run();
}

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

// ANCHOR: suspects
/// 嫌疑人甲：后厨，第 2 帧领走 1 桶
fn kitchen(mut butter: Query<&mut Butter>, mut frame: Local<u32>) {
    *frame += 1;
    if *frame == 2 {
        butter.single_mut().unwrap().0 -= 1;
    }
}

/// 嫌疑人乙：酒馆，第 3 帧领走 3 桶
fn tavern(
    mut butter: Query<&mut Butter>,
    mut frame: Local<u32>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    if *frame == 3 {
        butter.single_mut().unwrap().0 -= 3;
        exit.write(AppExit::Success);
    }
}
// ANCHOR_END: suspects

// ANCHOR: detective
/// 侦探：changed_by() 直接报出修改发生的源码位置
fn detective(butters: Query<(&Name, Ref<Butter>), Changed<Butter>>) {
    for (name, butter) in &butters {
        println!(
            "  侦探：{name}的黄油余 {} 桶——经手处：{}",
            butter.0,
            butter.changed_by()
        );
    }
}
// ANCHOR_END: detective

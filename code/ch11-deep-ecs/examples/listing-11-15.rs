//! Listing 11-15：给镇库上警报——资源上的 observer

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

/// 镇库银两
#[derive(Resource)]
struct TownFunds(u32);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_observer(alarm) // 先挂警报，再入库
        .add_systems(Startup, |mut commands: Commands| {
            commands.insert_resource(TownFunds(73));
        })
        .add_systems(Update, script)
        .run();
}

// ANCHOR: alarm
/// 警报：资源也是组件，第 8 章的生命周期事件原样适用
fn alarm(on: On<Insert, TownFunds>, funds: Query<&TownFunds>) {
    // 事件的目标就是资源实体——顺手用查询按组件读它
    let TownFunds(coins) = funds.get(on.entity).unwrap();
    println!("  警报：镇库进账！现银 {coins} 枚（响铃的是 {}）", on.entity);
}
// ANCHOR_END: alarm

// ANCHOR: script
/// 剧本：改值不惊动警报，insert_resource 才是“入库”
fn script(
    mut funds: ResMut<TownFunds>,
    mut commands: Commands,
    mut frame: Local<u32>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        2 => {
            funds.0 += 2;
            println!("艾达收了 2 枚人头税，记在账上。（ResMut 改值，警报不响）");
        }
        3 => {
            println!("商队巨款入库！（insert_resource 覆盖写入）");
            commands.insert_resource(TownFunds(500));
        }
        4 => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
// ANCHOR_END: script

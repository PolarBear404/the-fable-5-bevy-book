//! Listing 11-11：变更检测的发条——Tick、Ref 与三种“沉默的写法”

use bevy::app::ScheduleRunnerPlugin;
use bevy::ecs::system::SystemChangeTick;
use bevy::prelude::*;
use std::time::Duration;

/// 库房账面（袋）
#[derive(Component, PartialEq)]
struct Stock(u32);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((Name::new("镇仓"), Stock(50)));
        })
        .add_systems(Update, (banner, script, audit).chain())
        .run();
}

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

// ANCHOR: script
/// 剧本：四种写法，账房只听得见一种
fn script(mut stocks: Query<(&Name, &mut Stock)>, mut frame: Local<u32>, mut exit: MessageWriter<AppExit>) {
    *frame += 1;
    let (name, mut stock) = stocks.single_mut().unwrap();
    match *frame {
        2 => {
            println!("  小工掸灰，把账目原样抄了一遍（解引用了 &mut，值没变）。");
            let v = stock.0;
            stock.0 = v;
        }
        3 => {
            println!("  小工长记性了：set_if_neq——值不变就不惊动账房。");
            stock.set_if_neq(Stock(50));
        }
        4 => {
            println!("  掌柜的悄悄补了 3 袋陈账（bypass_change_detection）。");
            stock.bypass_change_detection().0 += 3;
        }
        5 => {
            println!("  {name}入库 10 袋，正大光明记一笔。");
            stock.0 += 10;
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
// ANCHOR_END: script

// ANCHOR: audit
/// 账房：用 Ref 看变更档案，用 SystemChangeTick 看自己的窗口
fn audit(stocks: Query<(&Name, Ref<Stock>)>, tick: SystemChangeTick) {
    for (name, stock) in &stocks {
        if stock.is_changed() {
            println!(
                "  账房：{name}的账动了！现存 {} 袋（added={}，last_changed={}，窗口 ({}, {}]）",
                stock.0,
                stock.is_added(),
                stock.last_changed().get(),
                tick.last_run().get(),
                tick.this_run().get(),
            );
        } else {
            println!("  账房：无事。（账面 {} 袋）", stock.0);
        }
    }
}
// ANCHOR_END: audit

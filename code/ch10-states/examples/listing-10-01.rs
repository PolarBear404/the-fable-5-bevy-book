//! Listing 10-1：第一个状态机——灰岩镇杂货铺的街机《勇者斗史莱姆》
//! Menu 时滚动待机字幕，Playing 时打史莱姆；投币切换状态

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use std::time::Duration;

// ANCHOR: state
/// 街机的两个阶段：待机画面，或一局游戏中
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
}
// ANCHOR_END: state

// ANCHOR: main
fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .add_systems(
            Update,
            (
                banner,
                script,
                attract_screen.run_if(in_state(GameState::Menu)),
                battle.run_if(in_state(GameState::Playing)),
            )
                .chain(),
        )
        .run();
}
// ANCHOR_END: main

/// 报幕：让"帧"在输出里看得见
fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

// ANCHOR: script
/// 剧本：第 3 帧罗兰投币，第 6 帧打烊
fn script(
    mut frame: Local<u32>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        3 => {
            println!("  罗兰：守了一路商队，也轮到我冒险一回。（投入硬币，叮）");
            next.set(GameState::Playing);
        }
        6 => {
            println!("  老板：打烊喽。（拉闸）");
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
// ANCHOR_END: script

// ANCHOR: systems
/// 待机画面：轮播两条字幕
fn attract_screen(mut tick: Local<u32>) {
    *tick += 1;
    if *tick % 2 == 1 {
        println!("  屏幕：《勇者斗史莱姆》——投币开始");
    } else {
        println!("  屏幕：最高纪录 9999 分，保持者“老蔫儿”");
    }
}

/// 战斗画面：每帧打一回合，打完定格
fn battle(mut slime_hp: Local<Option<i32>>) {
    let hp = slime_hp.get_or_insert(20);
    match *hp {
        20 => println!("  屏幕：勇者挥剑！史莱姆 HP 剩 10"),
        10 => println!("  屏幕：勇者挥剑！史莱姆倒下——通关！"),
        _ => {} // 通关画面定格
    }
    *hp -= 10;
}
// ANCHOR_END: systems

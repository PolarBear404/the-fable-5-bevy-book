//! Listing 10-8：SubStates——暂停只在游戏中才有意义，
//! 把 IsPaused 挂在 GameState::Playing 名下

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use std::time::Duration;

// ANCHOR: states
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

/// 子状态：只在 GameState::Playing 期间存在
#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::Playing)]
enum IsPaused {
    #[default]
    Running,
    Paused,
}
// ANCHOR_END: states

// ANCHOR: main
fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .add_sub_state::<IsPaused>()
        .add_systems(OnEnter(IsPaused::Running), || {
            println!("  [OnEnter(IsPaused::Running)] 机内时钟走字")
        })
        .add_systems(OnEnter(IsPaused::Paused), || {
            println!("  [OnEnter(IsPaused::Paused)] “PAUSED”压上画面")
        })
        .add_systems(OnExit(IsPaused::Paused), || {
            println!("  [OnExit(IsPaused::Paused)] “PAUSED”字样消失")
        })
        .add_systems(
            Update,
            (
                banner,
                script,
                attract_screen.run_if(in_state(GameState::Menu)),
                battle.run_if(in_state(IsPaused::Running)),
                paused_screen.run_if(in_state(IsPaused::Paused)),
            )
                .chain(),
        )
        .run();
}
// ANCHOR_END: main

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

// ANCHOR: script
/// 剧本：开局打两回合，中途暂停取汽水，带着暂停退菜单，再开新局
fn script(
    mut frame: Local<u32>,
    paused: Option<Res<State<IsPaused>>>,
    mut next_game: ResMut<NextState<GameState>>,
    mut next_paused: ResMut<NextState<IsPaused>>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        2 => {
            println!("  罗兰投币。（叮）");
            next_game.set(GameState::Playing);
        }
        4 => {
            println!("  老板：罗兰，你的汽水！——罗兰按下暂停去拿。");
            next_paused.set(IsPaused::Paused);
        }
        6 => {
            println!("  小芙：门口有杂耍！罗兰顾不上恢复，直接退到待机画面。");
            next_game.set(GameState::Menu);
        }
        7 => {
            println!(
                "  （此刻 State<IsPaused> 资源：{}）",
                if paused.is_some() { "还在" } else { "已经不存在" }
            );
        }
        8 => {
            println!("  罗兰回来了，再投一币。（叮）");
            next_game.set(GameState::Playing);
        }
        10 => {
            println!("  老板：打烊喽。");
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
// ANCHOR_END: script

/// 待机画面
fn attract_screen() {
    println!("  屏幕：《勇者斗史莱姆》——投币开始");
}

/// 战斗：只在"游戏中且未暂停"时推进
fn battle(mut tick: Local<u32>) {
    *tick += 1;
    if *tick % 2 == 1 {
        println!("  屏幕：勇者挥剑，史莱姆向后弹开");
    } else {
        println!("  屏幕：史莱姆鼓起来，撞向勇者");
    }
}

/// 暂停画面：一切定格
fn paused_screen() {
    println!("  屏幕：PAUSED（史莱姆保持着挨打的姿势）");
}

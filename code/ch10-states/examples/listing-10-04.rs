//! Listing 10-4：OnExit → OnTransition → OnEnter——换幕时刻的三个调度，
//! 以及开机那次最早的"启动转换"

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use std::time::Duration;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

// ANCHOR: main
fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, || println!("  老板：搬个凳子守摊。（Startup）"))
        .add_systems(OnEnter(GameState::Menu), || {
            println!("  [OnEnter(Menu)] 屏幕亮起，待机字幕滚动")
        })
        .add_systems(OnExit(GameState::Menu), || {
            println!("  [OnExit(Menu)] 字幕收起")
        })
        .add_systems(
            OnTransition {
                exited: GameState::Menu,
                entered: GameState::Playing,
            },
            || println!("  [OnTransition] 读盘画面：LOADING……"),
        )
        .add_systems(OnEnter(GameState::Playing), || {
            println!("  [OnEnter(Playing)] 锣响，勇者登场！")
        })
        .add_systems(OnExit(GameState::Playing), || {
            println!("  [OnExit(Playing)] 结算画面一闪")
        })
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

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

/// 剧本：第 2 帧投币，第 4 帧通关回菜单，第 5 帧打烊
fn script(
    mut frame: Local<u32>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        2 => {
            println!("  罗兰投币。（叮）");
            next.set(GameState::Playing);
        }
        4 => {
            println!("  罗兰：通关！回待机画面吧。");
            next.set(GameState::Menu);
        }
        5 => {
            println!("  老板：打烊喽。");
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}

/// 待机画面：字幕滚动
fn attract_screen() {
    println!("  屏幕：《勇者斗史莱姆》——投币开始");
}

/// 战斗：一回合定胜负，之后画面定格
fn battle(mut round: Local<u32>) {
    *round += 1;
    match *round {
        1 => println!("  屏幕：勇者一剑劈倒史莱姆！"),
        _ => println!("  屏幕：通关结算画面定格"),
    }
}

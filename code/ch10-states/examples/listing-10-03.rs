//! Listing 10-3：set 不是搬闸刀——投币的这一帧，State 还是旧值，
//! 下一帧帧首 StateTransition 调度才真正切换

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

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .add_systems(Update, (banner, coin_slot, screen).chain())
        .run();
}

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

// ANCHOR: coin_slot
/// 投币口：第 2 帧投币，并立刻回头看一眼状态资源
fn coin_slot(
    mut frame: Local<u32>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        2 => {
            next.set(GameState::Playing);
            println!(
                "  罗兰投币。又凑近看了一眼：画面是 {:?}——硬币进去了，怎么没反应？",
                state.get()
            );
        }
        4 => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
// ANCHOR_END: coin_slot

// ANCHOR: screen
/// 屏幕：每帧报告自己处在哪个状态
fn screen(state: Res<State<GameState>>) {
    match state.get() {
        GameState::Menu => println!("  屏幕：待机画面（state = Menu）"),
        GameState::Playing => println!("  屏幕：战斗画面（state = Playing）"),
    }
}
// ANCHOR_END: screen

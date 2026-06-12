//! Listing 10-2：行不通——MinimalPlugins 里没有 StatesPlugin，
//! 没有 StateTransition 调度，init_state 直接 panic

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    #[expect(dead_code, reason = "演示用：还没轮到它出场，程序就 panic 了")]
    Playing,
}

// ANCHOR: main
fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        // 忘了 .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .run();
}
// ANCHOR_END: main

//! Listing 10-9：ComputedStates——老街机的演示模式。
//! "画面上有没有人在过招"由 GameState 推导而来，不能手动设置

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use std::time::Duration;

// ANCHOR: states
/// 源状态：Playing 带一个字段——这一局是机器自己玩的演示，还是真人局
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing {
        demo: bool,
    },
}

/// 计算状态：画面上正在过招（不管是谁在打）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InAction;

impl ComputedStates for InAction {
    type SourceStates = GameState;

    // 源状态同值重设、或换值后算出同样结果时，不重跑 OnExit/OnEnter
    const ALLOW_SAME_STATE_TRANSITIONS: bool = false;

    fn compute(source: GameState) -> Option<Self> {
        matches!(source, GameState::Playing { .. }).then_some(InAction)
    }
}
// ANCHOR_END: states

/// 本局得分
#[derive(Resource, Default)]
struct Score(u32);

// ANCHOR: main
fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .add_computed_state::<InAction>()
        .init_resource::<Score>()
        .add_systems(OnEnter(InAction), || {
            println!("  [OnEnter(InAction)] 机台风扇呼呼转起来")
        })
        .add_systems(OnExit(InAction), || {
            println!("  [OnExit(InAction)] 风扇停转，机台歇了")
        })
        .add_systems(
            Update,
            (
                banner,
                script,
                attract_screen.run_if(in_state(GameState::Menu)),
                battle.run_if(in_state(InAction)),
                scoring.run_if(in_state(GameState::Playing { demo: false })),
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
/// 剧本：机器自己开演示局揽客，罗兰投币接管，通关后回待机
fn script(
    mut frame: Local<u32>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        2 => {
            println!("  机器等得无聊，自顾自开了一局演示揽客。");
            next.set(GameState::Playing { demo: true });
        }
        4 => {
            println!("  罗兰看得手痒：让我来！（叮——切到真人局）");
            next.set(GameState::Playing { demo: false });
        }
        6 => {
            println!("  屏幕：通关！退回待机画面。");
            next.set(GameState::Menu);
        }
        7 => {
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

/// 战斗画面：演示局和真人局都要演
fn battle() {
    println!("  屏幕：勇者与史莱姆你来我往");
}

/// 记分牌：只有真人局才计分
fn scoring(mut score: ResMut<Score>) {
    score.0 += 100;
    println!("  记分牌：+100，共 {} 分", score.0);
}

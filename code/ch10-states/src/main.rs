//! 第 10 章综合示例：灰岩镇街机厅的一天
//! 三层状态各司其职：GameState 管"谁在玩"（菜单/演示局/真人局），
//! IsPaused 子状态管"暂停"（只在游戏中存在），
//! InAction 计算状态管"画面上是否正在过招"（推导而来，负责搭台拆台）

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use std::time::Duration;

// ANCHOR: states
/// 源状态：待机，或一局游戏（演示局 / 真人局）
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing {
        demo: bool,
    },
}

/// 子状态：暂停——只要在游戏中就存在，不管是演示还是真人
#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::Playing { .. })]
enum IsPaused {
    #[default]
    Running,
    Paused,
}

/// 计算状态：画面上正在过招
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InAction;

impl ComputedStates for InAction {
    type SourceStates = GameState;
    const ALLOW_SAME_STATE_TRANSITIONS: bool = false;

    fn compute(source: GameState) -> Option<Self> {
        matches!(source, GameState::Playing { .. }).then_some(InAction)
    }
}
// ANCHOR_END: states

/// 标记：一局游戏里的登场角色
#[derive(Component)]
struct Actor;

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
        .add_sub_state::<IsPaused>()
        .add_computed_state::<InAction>()
        .init_resource::<Score>()
        // 搭台拆台跟着 InAction 走：demo ↔ 真人切换不折腾舞台
        .add_systems(OnEnter(InAction), setup_stage)
        .add_systems(OnExit(InAction), || {
            println!("  [OnExit(InAction)] 风扇停转，机台歇了")
        })
        .add_systems(OnEnter(IsPaused::Paused), || {
            println!("  [OnEnter(Paused)] “PAUSED”压上画面")
        })
        .add_systems(OnExit(IsPaused::Paused), || {
            println!("  [OnExit(Paused)] “PAUSED”字样消失")
        })
        .add_systems(
            Update,
            (
                banner,
                script,
                attract_screen.run_if(in_state(GameState::Menu)),
                battle.run_if(in_state(IsPaused::Running)),
                scoring.run_if(
                    in_state(GameState::Playing { demo: false })
                        .and_then(in_state(IsPaused::Running)),
                ),
                paused_screen.run_if(in_state(IsPaused::Paused)),
            )
                .chain(),
        )
        .run();

    println!("（run() 返回，街机厅的一天结束了）");
}
// ANCHOR_END: main

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

// ANCHOR: setup
/// 开台：机台运转，角色登场；离开 InAction 时自动清场
fn setup_stage(mut commands: Commands) {
    println!("  [OnEnter(InAction)] 风扇转起来，勇者与史莱姆登场");
    commands.spawn((Actor, Name::new("勇者"), DespawnOnExit(InAction)));
    commands.spawn((Actor, Name::new("史莱姆"), DespawnOnExit(InAction)));
}
// ANCHOR_END: setup

// ANCHOR: script
/// 剧本：演示局揽客 → 罗兰接管 → 汽水暂停 → 通关散场
fn script(
    mut frame: Local<u32>,
    mut next_game: ResMut<NextState<GameState>>,
    mut next_paused: ResMut<NextState<IsPaused>>,
    actors: Query<&Name, With<Actor>>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        2 => {
            println!("  机器等得无聊，自顾自开了一局演示揽客。");
            next_game.set(GameState::Playing { demo: true });
        }
        4 => {
            println!("  罗兰看得手痒：让我来！（叮——切到真人局）");
            next_game.set(GameState::Playing { demo: false });
        }
        6 => {
            println!("  老板：汽水好了！——罗兰按下暂停。");
            next_paused.set(IsPaused::Paused);
        }
        8 => {
            println!("  罗兰一抹嘴：继续！");
            next_paused.set(IsPaused::Running);
        }
        10 => {
            println!("  屏幕：通关！罗兰心满意足，机器退回待机画面。");
            next_game.set(GameState::Menu);
        }
        11 => {
            println!("  老板清点：场上还剩 {} 个角色。打烊喽。", actors.iter().count());
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

/// 战斗画面：演示局与真人局共用，台词不同
fn battle(state: Res<State<GameState>>) {
    match state.get() {
        GameState::Playing { demo: true } => {
            println!("  屏幕：（演示局）勇者与史莱姆你来我往");
        }
        _ => println!("  屏幕：罗兰操刀，勇者大杀四方"),
    }
}

/// 记分牌：真人局且未暂停才计分
fn scoring(mut score: ResMut<Score>) {
    score.0 += 100;
    println!("  记分牌：+100，共 {} 分", score.0);
}

/// 暂停画面
fn paused_screen() {
    println!("  屏幕：PAUSED（史莱姆保持着挨打的姿势）");
}

//! Listing 10-7：修复——角色挂上 DespawnOnExit(Playing)，
//! 离开 Playing 的瞬间由引擎自动清场；
//! 结尾的手肘证明同值转换也清场——先收旧的一套，OnEnter 再摆新的一套

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

/// 标记：一局游戏里的登场角色
#[derive(Component)]
struct Actor;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Playing), spawn_actors)
        .add_systems(Update, (banner, script, stage_report).chain())
        .run();
}

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

// ANCHOR: spawn
/// 开局：生成本局的登场角色，离开 Playing 时自动注销
fn spawn_actors(mut commands: Commands) {
    println!("  [OnEnter(Playing)] 勇者与史莱姆登场");
    commands.spawn((Actor, Name::new("勇者"), DespawnOnExit(GameState::Playing)));
    commands.spawn((
        Actor,
        Name::new("史莱姆"),
        DespawnOnExit(GameState::Playing),
    ));
}
// ANCHOR_END: spawn

/// 剧本：投币 → Game Over 回菜单 → 不服气再投币 → 手肘撞出同值转换 → 打烊
fn script(
    mut frame: Local<u32>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        1 => {
            println!("  罗兰投币。（叮）");
            next.set(GameState::Playing);
        }
        3 => {
            println!("  屏幕：史莱姆扑倒勇者——GAME OVER，退回待机画面。");
            next.set(GameState::Menu);
        }
        5 => {
            println!("  罗兰：不服，再来！（叮）");
            next.set(GameState::Playing);
        }
        7 => {
            println!("  罗兰探身够汽水，手肘又撞上了开始键——又一次 set(Playing)！");
            next.set(GameState::Playing);
        }
        9 => {
            println!("  老板：打烊喽。");
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}

// ANCHOR: report
/// 清点画面上的角色
fn stage_report(state: Res<State<GameState>>, actors: Query<&Name, With<Actor>>) {
    let roster: Vec<_> = actors.iter().map(Name::as_str).collect();
    if roster.is_empty() {
        println!("  画面（{:?}）上空无一人", state.get());
    } else {
        println!(
            "  画面（{:?}）上站着 {} 个角色：{}",
            state.get(),
            roster.len(),
            roster.join("、")
        );
    }
}
// ANCHOR_END: report

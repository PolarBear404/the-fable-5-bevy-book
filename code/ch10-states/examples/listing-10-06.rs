//! Listing 10-6：有 bug——OnEnter 时生成的角色，回菜单后没人收，
//! 赖在待机画面上；再开一局又生成一套，场面翻倍

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
/// 开局：生成本局的登场角色
fn spawn_actors(mut commands: Commands) {
    println!("  [OnEnter(Playing)] 勇者与史莱姆登场");
    commands.spawn((Actor, Name::new("勇者")));
    commands.spawn((Actor, Name::new("史莱姆")));
}
// ANCHOR_END: spawn

/// 剧本：投币 → Game Over 回菜单 → 不服气再投币 → 打烊
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

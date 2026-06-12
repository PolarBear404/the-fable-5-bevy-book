//! Listing 10-5：同值转换——在 Playing 里再次 set(Playing)，
//! OnExit/OnEnter 照跑一轮（一局莫名重开）；set_if_neq 则按兵不动

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

/// 本局连击数
#[derive(Resource, Default)]
struct Combo(u32);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .init_resource::<Combo>()
        .add_systems(OnEnter(GameState::Playing), new_round)
        .add_systems(OnExit(GameState::Playing), || {
            println!("  [OnExit(Playing)] 结算画面一闪")
        })
        .add_systems(
            Update,
            (banner, script, battle.run_if(in_state(GameState::Playing))).chain(),
        )
        .run();
}

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

/// 开局：锣响，连击清零
fn new_round(mut combo: ResMut<Combo>) {
    combo.0 = 0;
    println!("  [OnEnter(Playing)] 锣响，新的一局！连击清零");
}

// ANCHOR: script
/// 剧本：投币开局；第 3 帧手肘撞上开始键；修好后第 5 帧又撞了一次
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
            println!("  罗兰探身够汽水，手肘撞上开始键——又一次 set(Playing)！");
            next.set(GameState::Playing);
        }
        4 => {
            println!("  老板边嘟囔边拆开按键，给它加了层防抖垫。");
        }
        5 => {
            println!("  罗兰的手肘又撞上去——这次是 set_if_neq(Playing)。");
            // 注意 (*next)：ResMut 自己也有个 set_if_neq（第 5 章），得先解引用
            (*next).set_if_neq(GameState::Playing);
        }
        7 => {
            println!("  老板：打烊喽。");
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
// ANCHOR_END: script

/// 战斗：连击数一帧帧涨
fn battle(mut combo: ResMut<Combo>) {
    combo.0 += 1;
    println!("  屏幕：勇者进攻！连击 ×{}", combo.0);
}

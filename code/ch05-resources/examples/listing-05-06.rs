//! Listing 5-6：FromWorld——初始值由 World 里已有的资源算出

use bevy::prelude::*;

// ANCHOR: difficulty
/// 场地难度：enum 一样能当 Resource
#[derive(Resource)]
enum Difficulty {
    Casual,
    Pro,
}
// ANCHOR_END: difficulty

// ANCHOR: rules
/// 记分规则：红心一枪多少分，由难度决定
#[derive(Resource)]
struct ScoreRules {
    bullseye: u32,
}

impl FromWorld for ScoreRules {
    fn from_world(world: &mut World) -> Self {
        match world.resource::<Difficulty>() {
            Difficulty::Casual => ScoreRules { bullseye: 10 },
            Difficulty::Pro => ScoreRules { bullseye: 25 },
        }
    }
}
// ANCHOR_END: rules

// ANCHOR: main
fn main() {
    // 休闲场
    let mut app = App::new();
    app.insert_resource(Difficulty::Casual)
        .init_resource::<ScoreRules>() // from_world 此刻运行：Difficulty 必须已就位
        .add_systems(Update, report);
    app.update();

    // 职业场：难度一换，算出的规则跟着变
    let mut app = App::new();
    app.insert_resource(Difficulty::Pro)
        .init_resource::<ScoreRules>()
        .add_systems(Update, report);
    app.update();
}
// ANCHOR_END: main

// ANCHOR: report
fn report(difficulty: Res<Difficulty>, rules: Res<ScoreRules>) {
    let name = match *difficulty {
        Difficulty::Casual => "休闲场",
        Difficulty::Pro => "职业场",
    };
    println!("{name}：红心一枪 {} 分", rules.bullseye);
}
// ANCHOR_END: report

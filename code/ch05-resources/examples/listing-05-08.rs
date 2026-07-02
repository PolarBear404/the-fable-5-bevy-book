//! Listing 5-8：资源的变更检测——is_changed、is_added 与 set_if_neq

use bevy::prelude::*;

// set_if_neq 靠 PartialEq 判断值是否真的变了
#[derive(Resource, PartialEq)]
struct Score(u32);

fn main() {
    let mut app = App::new();
    app.insert_resource(Score(0))
        .add_systems(Update, (shoot, scoreboard).chain());

    app.update(); // 第 1 枪：命中
    app.update(); // 第 2 枪：脱靶
    app.update(); // 第 3 枪：命中
}

// ANCHOR: shoot
/// 射手：第 2 枪脱靶（+0 分）；set_if_neq 让"没变"不留变更记录
fn shoot(mut score: ResMut<Score>, mut round: Local<u32>) {
    *round += 1;
    let hit = if *round == 2 { 0 } else { 10 };
    println!("第 {} 枪：{}", *round, if hit > 0 { "命中" } else { "脱靶" });
    let new_total = score.0 + hit;
    score.set_if_neq(Score(new_total));
}
// ANCHOR_END: shoot

// ANCHOR: scoreboard
/// 记分牌：开机打一次招呼，之后只在分数真的变了时刷新
fn scoreboard(score: Res<Score>) {
    if score.is_added() {
        println!("记分牌通电，开始计分");
    }
    if score.is_changed() {
        println!("记分牌刷新 → {} 分", score.0);
    }
}
// ANCHOR_END: scoreboard

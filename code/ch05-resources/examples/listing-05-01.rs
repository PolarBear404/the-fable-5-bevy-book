//! Listing 5-1：计分板——两个系统共享同一个 Resource

use bevy::prelude::*;

// ANCHOR: score
/// 计分板：全场唯一，不属于任何实体
#[derive(Resource)]
struct Score(u32);
// ANCHOR_END: score

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.insert_resource(Score(0))
        .add_systems(Update, (shoot, announce).chain());

    app.update(); // 第 1 枪
    app.update(); // 第 2 枪
    app.update(); // 第 3 枪
}
// ANCHOR_END: main

// ANCHOR: systems
/// 射手：每枪 10 分，写计分板
fn shoot(mut score: ResMut<Score>) {
    score.0 += 10;
    println!("砰！+10 分");
}

/// 报靶员：读计分板，播报总分
fn announce(score: Res<Score>) {
    println!("报靶员：累计 {} 分", score.0);
}
// ANCHOR_END: systems

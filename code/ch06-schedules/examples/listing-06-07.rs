//! Listing 6-7：run_if——条件不满足，系统整帧不跑

use bevy::prelude::*;

// set_if_neq 靠 PartialEq 判断值是否真的变了
#[derive(Resource, Default, PartialEq)]
struct Score(u32);

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.init_resource::<Score>().add_systems(
        Update,
        (
            shoot,
            // 兑现第 5 章的预告：分数没变的帧，记分牌根本不运行
            scoreboard.run_if(resource_changed::<Score>),
            // not() 取反：没动静的帧，轮到观众起哄
            heckle.run_if(not(resource_changed::<Score>)),
            // 组合条件：.and_then() 要求两边都成立——刚变过 且 满 30 分，才颁奖
            ceremony.run_if(resource_changed::<Score>.and_then(|score: Res<Score>| score.0 >= 30)),
        )
            .chain(),
    );

    for _ in 1..=4 {
        app.update();
    }
}
// ANCHOR_END: main

/// 射手：四枪剧本——命中、脱靶、命中、脱靶
fn shoot(mut score: ResMut<Score>, mut round: Local<u32>) {
    *round += 1;
    let hit = [10, 0, 20, 0][*round as usize - 1];
    println!("第 {} 枪：{}", *round, if hit > 0 { "命中" } else { "脱靶" });
    let new_total = score.0 + hit;
    score.set_if_neq(Score(new_total));
}

/// 记分牌：运行即刷新——"该不该刷新"已经由 run_if 替它把关
fn scoreboard(score: Res<Score>) {
    println!("  记分牌 → {} 分", score.0);
}

/// 观众：嘘声
fn heckle() {
    println!("  观众：嘘——");
}

/// 颁奖台
fn ceremony(score: Res<Score>) {
    println!("  颁奖台：{} 分达标，金靶奖章！", score.0);
}

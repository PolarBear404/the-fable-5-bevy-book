//! Listing 5-4：道具资源——Option 探测有无，Commands 运行期插拔

use bevy::prelude::*;

#[derive(Resource)]
struct Score(u32);

// ANCHOR: card
/// 双倍卡：在场即生效，本身不携带数据
#[derive(Resource)]
struct DoubleCard;
// ANCHOR_END: card

fn main() {
    let mut app = App::new();
    app.insert_resource(Score(0))
        .add_systems(Update, (shoot, stall_keeper).chain());

    app.update(); // 第 1 枪
    app.update(); // 第 2 枪
    app.update(); // 第 3 枪
}

// ANCHOR: systems
/// 射手：手里有双倍卡就翻倍
fn shoot(mut score: ResMut<Score>, card: Option<Res<DoubleCard>>) {
    let points = if card.is_some() { 20 } else { 10 };
    score.0 += points;
    println!("砰！+{points} 分（累计 {}）", score.0);
}

/// 摊主：第 1 枪后递来双倍卡，第 2 枪后收走
fn stall_keeper(mut commands: Commands, mut round: Local<u32>) {
    *round += 1;
    if *round == 1 {
        println!("摊主：这张双倍卡送你，下一枪生效！");
        commands.insert_resource(DoubleCard);
    }
    if *round == 2 {
        println!("摊主：双倍卡到期，收回了。");
        commands.remove_resource::<DoubleCard>();
    }
}
// ANCHOR_END: systems

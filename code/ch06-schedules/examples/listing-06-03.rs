//! Listing 6-3：.chain()——流水线按书写顺序执行

use bevy::prelude::*;

/// 矿场仓库：矿石、金属锭、金币
#[derive(Resource, Default)]
struct Stockpile {
    ore: u32,
    ingots: u32,
    coins: u32,
}

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.init_resource::<Stockpile>()
        // 挖矿 → 冶炼 → 铸币，一道工序都不能乱
        .add_systems(Update, (dig, smelt, mint).chain());

    app.update();
    app.update();
}
// ANCHOR_END: main

// ANCHOR: systems
/// 矿工：每帧挖 3 块矿石
fn dig(mut pile: ResMut<Stockpile>) {
    pile.ore += 3;
    println!("矿工：+3 矿石（库存 {}）", pile.ore);
}

/// 冶炼炉：把库存矿石全部炼成锭
fn smelt(mut pile: ResMut<Stockpile>) {
    let melted = pile.ore;
    pile.ore = 0;
    pile.ingots += melted;
    println!("冶炼炉：出锭 {melted} 根（库存 {}）", pile.ingots);
}

/// 铸币机：把库存锭全部铸成金币
fn mint(mut pile: ResMut<Stockpile>) {
    let minted = pile.ingots;
    pile.ingots = 0;
    pile.coins += minted;
    println!("铸币机：+{minted} 金币（金库 {}）", pile.coins);
}
// ANCHOR_END: systems

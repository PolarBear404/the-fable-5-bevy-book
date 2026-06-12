//! Listing 6-4：before/after——把自己嵌进别人的流水线

use bevy::prelude::*;

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
        // 假设这行是"别人写的"：只注册了挖矿和铸币，没有冶炼
        .add_systems(Update, (dig, mint))
        // 我们的冶炼炉单独注册，用 after/before 把自己嵌进两者中间
        .add_systems(Update, smelt.after(dig).before(mint));

    app.update();
}
// ANCHOR_END: main

fn dig(mut pile: ResMut<Stockpile>) {
    pile.ore += 3;
    println!("矿工：+3 矿石（库存 {}）", pile.ore);
}

fn smelt(mut pile: ResMut<Stockpile>) {
    let melted = pile.ore;
    pile.ore = 0;
    pile.ingots += melted;
    println!("冶炼炉：出锭 {melted} 根（库存 {}）", pile.ingots);
}

fn mint(mut pile: ResMut<Stockpile>) {
    let minted = pile.ingots;
    pile.ingots = 0;
    pile.coins += minted;
    println!("铸币机：+{minted} 金币（金库 {}）", pile.coins);
}

//! Listing 6-6：SystemSet——给系统分组，按组排序

use bevy::prelude::*;

// ANCHOR: set
/// 铸币厂的三道工序，先后分明
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MintStage {
    /// 备料：挖矿、砍柴
    Produce,
    /// 加工：冶炼
    Process,
    /// 结算：铸币、记账
    Settle,
}
// ANCHOR_END: set

#[derive(Resource, Default)]
struct OrePile(u32);

#[derive(Resource, Default)]
struct WoodPile(u32);

#[derive(Resource, Default)]
struct Treasury {
    ingots: u32,
    coins: u32,
}

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.init_resource::<OrePile>()
        .init_resource::<WoodPile>()
        .init_resource::<Treasury>()
        // 先给三道工序排好顺序——此刻一个系统都还没注册
        .configure_sets(
            Update,
            (MintStage::Produce, MintStage::Process, MintStage::Settle).chain(),
        )
        // 各系统只声明自己属于哪道工序，不点任何同事的名
        .add_systems(
            Update,
            (dig.in_set(MintStage::Produce), chop.in_set(MintStage::Produce)),
        )
        .add_systems(Update, smelt.in_set(MintStage::Process))
        .add_systems(
            Update,
            (mint_coins, report.after(mint_coins)).in_set(MintStage::Settle),
        );

    app.update();
    app.update();
}
// ANCHOR_END: main

// ANCHOR: produce
/// 矿工：每帧 +2 矿石。和砍柴互不相干，同一集合内不排序，引擎可以并行跑
fn dig(mut ore: ResMut<OrePile>) {
    ore.0 += 2;
}

/// 樵夫：每帧 +1 木柴
fn chop(mut wood: ResMut<WoodPile>) {
    wood.0 += 1;
}
// ANCHOR_END: produce

/// 冶炼炉：2 矿石 + 1 木柴 = 1 根锭
fn smelt(mut ore: ResMut<OrePile>, mut wood: ResMut<WoodPile>, mut treasury: ResMut<Treasury>) {
    let batches = (ore.0 / 2).min(wood.0);
    ore.0 -= batches * 2;
    wood.0 -= batches;
    treasury.ingots += batches;
    println!("冶炼炉：出锭 {batches} 根");
}

/// 铸币机：把锭全部铸成金币
fn mint_coins(mut treasury: ResMut<Treasury>) {
    let minted = treasury.ingots;
    treasury.ingots = 0;
    treasury.coins += minted;
    println!("铸币机：+{minted} 金币");
}

/// 账房：报告金库
fn report(treasury: Res<Treasury>) {
    println!("账房：金库共 {} 枚金币", treasury.coins);
}

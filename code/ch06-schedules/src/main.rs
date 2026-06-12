//! 第 6 章综合示例：皇家铸币厂的六天
//! 工序集合排顺序，run_if 把关，王令走 Commands——当天生效靠自动同步点

use bevy::prelude::*;

// —— 集合定义 ——

/// 铸币厂的三道工序
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MintStage {
    Produce,
    Process,
    Settle,
}

// —— 资源定义 ——

/// 厂房库存：矿石与锭
#[derive(Resource, Default)]
struct Stockpile {
    ore: u32,
    ingots: u32,
}

/// 金库
#[derive(Resource, Default)]
struct Treasury(u32);

/// 王室赶工令：在场时矿工加倍干活
#[derive(Resource)]
struct RushOrder;

fn main() {
    let mut app = App::new();
    app.init_resource::<Stockpile>()
        .init_resource::<Treasury>()
        .add_systems(Startup, opening)
        // 三道工序按 chain 排定先后；系统随后各自入伙
        .configure_sets(
            Update,
            (MintStage::Produce, MintStage::Process, MintStage::Settle).chain(),
        )
        // 国王先于全部工序发话；他的 Commands 会在排序边上触发自动同步点——王令当天生效
        .add_systems(Update, royal_decree.before(MintStage::Produce))
        .add_systems(Update, dig.in_set(MintStage::Produce))
        // 凑满 3 块矿石才开炉，否则整帧歇着
        .add_systems(
            Update,
            smelt
                .in_set(MintStage::Process)
                .run_if(|pile: Res<Stockpile>| pile.ore >= 3),
        )
        .add_systems(
            Update,
            (
                mint_coins,
                // 金库没动静的天，账房不出声
                report.after(mint_coins).run_if(resource_changed::<Treasury>),
            )
                .in_set(MintStage::Settle),
        );

    for day in 1..=6 {
        println!("—— 第 {day} 天 ——");
        app.update();
    }
}

// —— Startup：开张 ——

fn opening() {
    println!("皇家铸币厂开张！");
}

// —— Update：一天一轮 ——

/// 国王：第 2 天颁布赶工令，第 4 天收回
fn royal_decree(mut commands: Commands, mut day: Local<u32>) {
    *day += 1;
    if *day == 2 {
        println!("国王：颁布赶工令！");
        commands.insert_resource(RushOrder);
    }
    if *day == 4 {
        println!("国王：赶工令收回。");
        commands.remove_resource::<RushOrder>();
    }
}

/// 矿工：平日 +1 矿石，赶工 +3
fn dig(mut pile: ResMut<Stockpile>, rush: Option<Res<RushOrder>>) {
    let mined = if rush.is_some() { 3 } else { 1 };
    pile.ore += mined;
    println!("矿工：+{mined} 矿石（存 {}）", pile.ore);
}

/// 冶炼炉：3 块矿石出 1 根锭——run_if 保证开炉时矿石必然够数
fn smelt(mut pile: ResMut<Stockpile>) {
    pile.ore -= 3;
    pile.ingots += 1;
    println!("冶炼炉：出锭 1 根（余矿 {}）", pile.ore);
}

/// 铸币机：有锭才开机，金库才被写
fn mint_coins(mut pile: ResMut<Stockpile>, mut treasury: ResMut<Treasury>) {
    if pile.ingots > 0 {
        treasury.0 += pile.ingots;
        println!("铸币机：+{} 金币", pile.ingots);
        pile.ingots = 0;
    }
}

/// 账房：金库变了才记一笔
fn report(treasury: Res<Treasury>) {
    println!("账房：金库 {} 枚金币", treasury.0);
}

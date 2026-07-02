//! Listing 11-16：年终结算——contiguous_iter 一次借一册

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

/// 存粮（袋）
#[derive(Component, PartialEq)]
struct Stock(u32);

/// 铺面
#[derive(Component)]
struct Shop;

/// 盘点记号（SparseSet 存储，11-3 节的老相识）
#[derive(Component)]
#[component(storage = "SparseSet")]
struct Flagged;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_systems(Startup, settle_in)
        .add_systems(Update, (banner, bulk, audit, flagged_corner).chain())
        .run();
}

fn settle_in(mut commands: Commands) {
    commands.spawn((Name::new("罗兰"), Stock(3)));
    commands.spawn((Name::new("老蔫儿"), Stock(7), Flagged)); // 挂了盘点记号
    commands.spawn((Name::new("货郎"), Stock(20)));
    commands.spawn((Name::new("杂货铺"), Stock(40), Shop));
    commands.spawn((Name::new("铁匠铺"), Stock(2), Shop));
}

fn banner(mut frame: Local<u32>, mut exit: MessageWriter<AppExit>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
    if *frame == 3 {
        exit.write(AppExit::Success);
    }
}

// ANCHOR: bulk
/// 结算：不再一行一行递，一次借出一张表的整列
fn bulk(mut ledger: Query<(&Name, &mut Stock)>, mut frame: Local<u32>) {
    *frame += 1;
    match *frame {
        2 => {
            println!("  [结算] 一张表一张表地翻：");
            for (names, mut stocks) in ledger.contiguous_iter_mut().unwrap() {
                // names、stocks 是两条等长切片：这张表的整列
                println!("    这一张表 {} 行：{:?}", names.len(),
                    names.iter().map(Name::as_str).collect::<Vec<_>>());
                if stocks.len() == 3 {
                    stocks[0].0 += 100; // 只动下标 0 一行，但走的是 DerefMut——听账房的
                    println!("    只给{}补了 100 袋。", names[0]);
                }
            }
        }
        3 => {
            println!("  [结算] 全镇每户折损 1 袋，bypass_change_detection 悄悄记：");
            for (_, mut stocks) in ledger.contiguous_iter_mut().unwrap() {
                for stock in stocks.bypass_change_detection() {
                    stock.0 -= 1;
                }
            }
        }
        _ => {}
    }
}
// ANCHOR_END: bulk

/// 账房：第 11-5 节的老班底，Changed 一个都不放过
fn audit(ledger: Query<(&Name, &Stock), Changed<Stock>>) {
    let heard: Vec<String> = ledger.iter().map(|(n, s)| format!("{n} {} 袋", s.0)).collect();
    println!("  账房听到：{heard:?}");
}

// ANCHOR: sparse
/// 记号户专场：过滤器点到了 SparseSet 组件——这样的查询借不出整列
fn flagged_corner(mut marked: Query<(&Name, &mut Stock), With<Flagged>>, mut frame: Local<u32>) {
    *frame += 1;
    if *frame != 3 {
        return;
    }
    match marked.contiguous_iter_mut() {
        Ok(_) => println!("  记号户专场：竟然借成了？"),
        Err(e) => println!("  记号户专场借不出整列：{e}"),
    }
    for (name, stock) in &marked {
        println!("  普通 iter 不受限，照常一行行来：{name}现存 {} 袋", stock.0);
    }
}
// ANCHOR_END: sparse

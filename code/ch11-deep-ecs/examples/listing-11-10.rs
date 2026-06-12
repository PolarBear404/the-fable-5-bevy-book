//! Listing 11-10：盘点表的一行——#[derive(QueryData)]

use bevy::app::ScheduleRunnerPlugin;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use std::time::Duration;

/// 存粮（袋）
#[derive(Component)]
struct Stock(u32);

/// 借宿者：寄宿在谁家
#[derive(Component)]
struct Lodger {
    host: &'static str,
}

/// 盖过章：今年已盘点
#[derive(Component)]
struct Registered;

// ANCHOR: row
/// 盘点表的一行（只读）：字段名就是列名
#[derive(QueryData)]
struct CensusRow {
    entity: Entity,
    name: &'static Name,
    stock: &'static Stock,
    lodger: Option<&'static Lodger>, // 不是每户都有的列
    registered: Has<Registered>,
}
// ANCHOR_END: row

// ANCHOR: read
/// 念名册：按字段名取数，不再数元组位置
fn read_table(table: Query<CensusRow>) {
    println!("盘点表：");
    for row in &table {
        print!("  {} {}：存粮 {} 袋", row.entity, row.name, row.stock.0);
        if let Some(lodger) = row.lodger {
            print!("，借宿于{}家", lodger.host);
        }
        println!("{}", if row.registered { "（已盖章）" } else { "（未盖章）" });
    }
}
// ANCHOR_END: read

// ANCHOR: tax_row
/// 可写的一行：标 mutable，&'static mut 才放行
#[derive(QueryData)]
#[query_data(mutable)]
struct TaxRow {
    name: &'static Name,
    stock: &'static mut Stock,
}

/// 收粮税：每户 1 袋
fn collect_grain_tax(mut rows: Query<TaxRow>, mut exit: MessageWriter<AppExit>) {
    for mut row in &mut rows {
        row.stock.0 -= 1;
        println!("收税：{}缴 1 袋，余 {} 袋", row.name, row.stock.0);
    }
    exit.write(AppExit::Success);
}
// ANCHOR_END: tax_row

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_systems(Startup, settle_in)
        .add_systems(Update, (read_table, collect_grain_tax).chain())
        .run();
}

fn settle_in(mut commands: Commands) {
    commands.spawn((Name::new("罗兰"), Stock(3), Lodger { host: "杂货铺老板" }));
    commands.spawn((Name::new("老蔫儿"), Stock(7), Registered));
    commands.spawn((Name::new("杂货铺老板"), Stock(40)));
}

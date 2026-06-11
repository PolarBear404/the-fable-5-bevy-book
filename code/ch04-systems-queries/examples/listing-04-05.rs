//! Listing 4-5：Changed 与 Added——变更检测的初次见面，以及"写访问即变更"的陷阱

use bevy::prelude::*;

#[derive(Component)]
struct Hunger(i32);

/// 贪吃标记：小黑每天都要加餐
#[derive(Component)]
struct Greedy;

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_flock).add_systems(
        Update,
        (feed_greedy, recount, monitor, register).chain(),
    );

    for day in 1..=3 {
        println!("—— 第 {day} 帧 ——");
        app.update();
    }
}
// ANCHOR_END: main

fn spawn_flock(mut commands: Commands) {
    commands.spawn((Name::new("小白"), Hunger(10)));
    commands.spawn((Name::new("小黑"), Hunger(10), Greedy));
}

// ANCHOR: writers
/// 给贪吃的羊喂食——真实的修改
fn feed_greedy(mut greedy: Query<&mut Hunger, With<Greedy>>) {
    for mut hunger in &mut greedy {
        hunger.0 -= 2;
    }
}

/// 盘点员：第 2 帧把其余羊的饥饿值"重新登记"一遍——写回原值
fn recount(mut others: Query<&mut Hunger, Without<Greedy>>, mut day: Local<u32>) {
    *day += 1;
    if *day == 2 {
        for mut hunger in &mut others {
            let value = hunger.0;
            hunger.0 = value; // 值没变，但这是一次写访问
        }
        println!("〔盘点员把没加餐的羊重新登记了一遍〕");
    }
}
// ANCHOR_END: writers

// ANCHOR: watchers
/// 哨兵：只报告 Hunger 发生过变更的羊
fn monitor(changed: Query<(&Name, &Hunger), Changed<Hunger>>) {
    for (name, hunger) in &changed {
        println!("{name} 的饥饿值有变动：{}", hunger.0);
    }
}

/// 登记员：只报告 Hunger 是新挂上的羊
fn register(added: Query<&Name, Added<Hunger>>) {
    for name in &added {
        println!("名册新增：{name}");
    }
}
// ANCHOR_END: watchers

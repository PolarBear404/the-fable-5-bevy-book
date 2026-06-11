//! Listing 4-1：系统的形态——多参数系统、Local 私有状态、手动驱动三帧

use bevy::prelude::*;

#[derive(Component)]
struct Sheep;

#[derive(Component)]
struct Hunger(i32);

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_flock)
        .add_systems(Update, (graze, headcount).chain());

    app.update(); // 第 1 帧
    app.update(); // 第 2 帧
    app.update(); // 第 3 帧
}
// ANCHOR_END: main

// ANCHOR: systems
fn spawn_flock(mut commands: Commands) {
    commands.spawn_batch([
        (Name::new("小白"), Sheep, Hunger(10)),
        (Name::new("小黑"), Sheep, Hunger(8)),
        (Name::new("卷卷"), Sheep, Hunger(6)),
    ]);
}

/// 吃草：每帧每只羊饥饿 -2
fn graze(mut flock: Query<&mut Hunger, With<Sheep>>) {
    for mut hunger in &mut flock {
        hunger.0 -= 2;
    }
}

/// 点名：Local<u32> 记着这是第几天
fn headcount(flock: Query<&Hunger, With<Sheep>>, mut day: Local<u32>) {
    *day += 1;
    let total: i32 = flock.iter().map(|hunger| hunger.0).sum();
    println!("第 {} 天：{} 只羊，饥饿总和 {}", *day, flock.iter().count(), total);
}
// ANCHOR_END: systems

//! Listing 6-8：after_ignore_deferred——只要顺序，不要同步点

use bevy::prelude::*;

#[derive(Component)]
struct Soldier;

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_systems(
        Update,
        (
            recruit,
            // 点名保证在征兵之后运行，但拒绝为这条边插同步点
            headcount.after_ignore_deferred(recruit),
        ),
    );

    for day in 1..=3 {
        println!("—— 第 {day} 天 ——");
        app.update();
    }
}
// ANCHOR_END: main

/// 征兵处：每天征召 1 名新兵（Commands 排队）
fn recruit(mut commands: Commands) {
    commands.spawn(Soldier);
    println!("征兵处：征召 1 名新兵");
}

/// 点名官：清点现役士兵
fn headcount(soldiers: Query<&Soldier>) {
    println!("点名官：现役 {} 人", soldiers.iter().count());
}


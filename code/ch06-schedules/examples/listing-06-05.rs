//! Listing 6-5：歧义检测——让调度器报告"顺序不定还共用数据"的系统对

use bevy::ecs::schedule::{LogLevel, ScheduleBuildSettings};
use bevy::log::LogPlugin;
use bevy::prelude::*;

#[derive(Resource, Default)]
struct Stockpile {
    ore: u32,
}

// ANCHOR: main
fn main() {
    let mut app = App::new();
    // 歧义报告走日志通道，需要 LogPlugin（DefaultPlugins 自带，裸 App 手动加）
    app.add_plugins(LogPlugin::default())
        // 让 Update 调度在构建时检查歧义并发出警告
        .edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .init_resource::<Stockpile>()
        // 一写一读，却没有任何顺序约束——审计员看见的数字全凭运气
        .add_systems(Update, (dig, audit));

    app.update();
}
// ANCHOR_END: main

fn dig(mut pile: ResMut<Stockpile>) {
    pile.ore += 3;
}

fn audit(pile: Res<Stockpile>) {
    println!("审计员记下库存：{}", pile.ore);
}

//! Listing 11-5：盘点前夜的预检——普通系统里的 EntityRef 与 &World
//! 看起来全程只读……运行起来见分晓

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

/// 常住户
#[derive(Component)]
struct Resident;

/// 存粮（袋）
#[derive(Component)]
struct Stock(u32);

/// 镇库银两
#[derive(Resource)]
struct TownFunds(u32);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .insert_resource(TownFunds(73))
        .add_systems(Startup, settle_in)
        .add_systems(Update, precheck)
        .run();
}

fn settle_in(mut commands: Commands) {
    commands.spawn((Resident, Name::new("罗兰"), Stock(3)));
    commands.spawn((Resident, Name::new("老蔫儿"), Stock(7)));
    commands.spawn((Name::new("过路货郎"), Stock(20)));
}

// ANCHOR: precheck
/// 预检：EntityRef 逐户翻看，&World 看全局，最后顺手收个工
fn precheck(
    houses: Query<(Entity, EntityRef)>,
    world: &World,
    mut exit: MessageWriter<AppExit>,
) {
    println!("预检官挨家挨户翻名册：");
    for (id, house) in &houses {
        let name = house.get::<Name>().map(Name::as_str).unwrap_or("无名");
        let stock = house.get::<Stock>().map(|s| s.0).unwrap_or(0);
        let kind = if house.contains::<Resident>() { "常住" } else { "过路" };
        println!(
            "  {id} {name}：{kind}，存粮 {stock} 袋，共 {} 个组件",
            house.archetype().component_count()
        );
    }
    println!(
        "合计：全镇 {} 个实体；镇库 {} 枚银币。",
        world.entities().count_spawned(),
        world.resource::<TownFunds>().0
    );
    exit.write(AppExit::Success);
}
// ANCHOR_END: precheck

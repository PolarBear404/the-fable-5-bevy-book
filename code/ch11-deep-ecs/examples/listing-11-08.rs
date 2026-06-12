//! Listing 11-8：在独占系统里开柜台——SystemState

use bevy::app::ScheduleRunnerPlugin;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use std::time::Duration;

/// 常住户
#[derive(Component)]
struct Resident;

/// 公告牌
#[derive(Component)]
struct Notice;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_systems(Startup, settle_in)
        .add_systems(Update, (banner, patrol, take_census).chain())
        .run();
}

fn settle_in(mut commands: Commands) {
    commands.spawn((Resident, Name::new("罗兰")));
    commands.spawn((Resident, Name::new("老蔫儿")));
    commands.spawn((Resident, Name::new("杂货铺老板")));
}

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

/// 巡逻队：顺带数公告牌
fn patrol(
    residents: Query<(), With<Resident>>,
    notices: Query<&Name, With<Notice>>,
    mut exit: MessageWriter<AppExit>,
) {
    println!(
        "  巡逻队：住户 {} 人，公告牌 {} 块。",
        residents.iter().count(),
        notices.iter().count()
    );
    if !notices.is_empty() {
        exit.write(AppExit::Success);
    }
}

// ANCHOR: census
/// 盘点：独占系统里用 SystemState 开出惯用的柜台
fn take_census(
    world: &mut World,
    counter: &mut SystemState<(Query<&Name, With<Resident>>, Commands)>,
    mut frame: Local<u32>,
) {
    *frame += 1;
    if *frame != 2 {
        return;
    }

    // get_mut 借出参数——用起来和普通系统一模一样
    let (residents, mut commands) = counter.get_mut(world);
    let roll: Vec<&str> = residents.iter().map(Name::as_str).collect();
    println!("  艾达点名：{}。", roll.join("、"));
    commands.spawn((Notice, Name::new("年度盘点完毕")));

    // SystemState 借出的 Commands 同样是缓冲的——还没落地
    let before = world.query_filtered::<(), With<Notice>>().iter(world).count();
    counter.apply(world); // 手动结账，命令立刻执行
    let after = world.query_filtered::<(), With<Notice>>().iter(world).count();
    println!("  公告牌：apply 之前 {before} 块，之后 {after} 块。");
}
// ANCHOR_END: census

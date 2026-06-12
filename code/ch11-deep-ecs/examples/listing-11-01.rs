//! Listing 11-1：第一个独占系统——盘点日，镇务官艾达接管全镇
//! 普通系统隔着柜台按单取数据；独占系统拿到整个 World 的 &mut

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

/// 标记：灰岩镇的住户
#[derive(Component)]
struct Resident;

/// 标记：镇公所立的公告牌
#[derive(Component)]
struct Notice;

/// 归档：盘点结果
#[derive(Resource)]
struct CensusRecord {
    households: usize,
}

// ANCHOR: main
fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_systems(Startup, settle_in)
        // 独占系统的注册方式毫无特殊：add_systems 照旧
        .add_systems(Update, (banner, patrol, take_census, town_crier).chain())
        .run();
}
// ANCHOR_END: main

fn settle_in(mut commands: Commands) {
    commands.spawn((Resident, Name::new("罗兰")));
    commands.spawn((Resident, Name::new("老蔫儿")));
    commands.spawn((Resident, Name::new("杂货铺老板")));
}

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

/// 巡逻队：普通系统，照常报平安
fn patrol(residents: Query<&Name, With<Resident>>, notices: Query<(), With<Notice>>) {
    println!(
        "  巡逻队：在册住户 {} 人，公告牌 {} 块。",
        residents.iter().count(),
        notices.iter().count()
    );
}

// ANCHOR: census
/// 盘点：独占系统——参数是整个世界
fn take_census(world: &mut World, mut frame: Local<u32>) {
    *frame += 1;
    if *frame != 2 {
        return;
    }
    println!("  艾达：盘点日。全镇静止！（接管 World）");

    // 清点：连“住户”之外的实体也一并在内
    let total = world.entities().count_spawned();
    let households = world
        .query_filtered::<(), With<Resident>>()
        .iter(world)
        .count();
    println!("  艾达：全镇实体 {total} 个，其中在册住户 {households} 人。");

    // 当场动工：立一块公告牌——没有命令队列，立等可见
    world.spawn((Notice, Name::new("年度盘点完毕")));
    let notices = world.query_filtered::<(), With<Notice>>().iter(world).count();
    println!("  艾达：公告牌立讫，复核：{notices} 块。（spawn 当场生效）");

    // 归档成资源，同样立等可见
    world.insert_resource(CensusRecord { households });
}
// ANCHOR_END: census

// ANCHOR: crier
/// 喇叭：普通系统——同一帧的下一个系统已经读到归档
fn town_crier(record: Option<Res<CensusRecord>>, mut exit: MessageWriter<AppExit>) {
    let Some(record) = record else { return };
    if record.is_added() {
        // 独占系统在同一帧插入的资源，链上的下一个系统立刻读到
        println!("  喇叭：镇公所归档——在册住户 {} 人！", record.households);
    } else {
        // 多跑了一帧，巡逻队已看见公告牌——收工
        exit.write(AppExit::Success);
    }
}
// ANCHOR_END: crier

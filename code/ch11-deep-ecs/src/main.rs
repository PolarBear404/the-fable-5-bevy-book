//! 第 11 章综合示例：灰岩镇的盘点日
//! 普通系统打理日常（只读预检、巡逻），独占系统接管全镇（点名、存档、立牌），
//! 存档用 EntityRef + inspect_entity 逐户列出组件清单——一个最小的检查器

use bevy::app::ScheduleRunnerPlugin;
use bevy::ecs::entity_disabling::Disabled;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use std::time::Duration;

/// 常住户
#[derive(Component)]
struct Resident;

/// 借宿者
#[derive(Component)]
struct Lodger;

/// 铺面
#[derive(Component)]
struct Shop;

/// 存粮（袋）
#[derive(Component)]
struct Stock(u32);

/// 公告牌
#[derive(Component)]
struct Notice;

/// 镇公所总册：存档正文
#[derive(Resource, Default)]
struct TownLedger {
    lines: Vec<String>,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .init_resource::<TownLedger>()
        .add_systems(Startup, settle_in)
        .add_systems(
            Update,
            (banner, patrol, precheck, script, census_day, town_crier).chain(),
        )
        .run();

    println!("（run() 返回，盘点日结束了）");
}

fn settle_in(mut commands: Commands) {
    commands.spawn((Resident, Name::new("杂货铺老板"), Stock(40), Shop));
    commands.spawn((Resident, Name::new("老蔫儿"), Stock(7)));
    commands.spawn((Shop, Name::new("铁匠铺"), Stock(2)));
}

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

/// 巡逻队：普通查询，挂了 Disabled 的铺子自动从眼皮底下消失
fn patrol(folks: Query<&Name, Or<(With<Resident>, With<Lodger>, With<Shop>)>>) {
    let roll: Vec<&str> = folks.iter().map(Name::as_str).collect();
    println!("  巡逻队（{} 处亮灯）：{}", roll.len(), roll.join("、"));
}

/// 助手的预检：&World 只读看全局——Disabled 只瞒得过查询，瞒不过直接清点
fn precheck(world: &World, mut frame: Local<u32>) {
    *frame += 1;
    if *frame == 1 || *frame == 3 {
        println!(
            "  助手：预检——全镇实体 {} 个，待盘点核对。",
            world.entities().count_spawned()
        );
    }
}

/// 剧本：第 2 帧，罗兰投宿、铁匠铺冬歇
fn script(
    shops: Query<(Entity, &Name), With<Shop>>,
    mut commands: Commands,
    mut frame: Local<u32>,
) {
    *frame += 1;
    if *frame != 2 {
        return;
    }
    println!("  罗兰背着行李进镇：在杂货铺后屋搭个铺。（spawn）");
    commands.spawn((Lodger, Name::new("罗兰"), Stock(3)));
    let (smithy, _) = shops.iter().find(|(_, n)| n.as_str() == "铁匠铺").unwrap();
    println!("  铁匠铺：入冬封炉，明春再会。（挂上 Disabled）");
    commands.entity(smithy).insert(Disabled);
}

/// 盘点日：独占系统接管全镇——点名、逐户验看、写总册、立公告
fn census_day(
    world: &mut World,
    desk: &mut SystemState<Query<&Name, With<Resident>>>,
    mut frame: Local<u32>,
) {
    *frame += 1;
    if *frame != 3 {
        return;
    }
    println!("  艾达：盘点日，全镇静止！（接管 World）");

    // 柜台姿势没丢：SystemState 借出常住名册
    let residents = desk.get(world).unwrap();
    let roll: Vec<&str> = residents.iter().map(Name::as_str).collect();
    println!("  艾达点名（常住）：{}。", roll.join("、"));

    // 写总册要 &mut 资源，逐户验看要查询——resource_scope 两不耽误
    world.resource_scope(|world, mut ledger: Mut<TownLedger>| {
        ledger.lines.clear();
        // Allow<Disabled>：冬歇的铺子也得入册
        let mut everyone =
            world.query_filtered::<(Entity, EntityRef), (With<Name>, Allow<Disabled>)>();
        for (id, house) in everyone.iter(world) {
            let name = house.get::<Name>().unwrap();
            let stock = house.get::<Stock>().map(|s| s.0).unwrap_or(0);
            // 检查器的核心一招：不知道有什么组件？问档案
            let parts: Vec<String> = world
                .inspect_entity(id)
                .unwrap()
                .map(|info| format!("[{}]", info.name().shortname()))
                .collect();
            let dormant = if house.contains::<Disabled>() { "（冬歇）" } else { "" };
            ledger
                .lines
                .push(format!("{id} {name}{dormant}：存粮 {stock} 袋 {}", parts.join("")));
        }
    });

    // 立公告——独占系统里当场生效
    world.spawn((Notice, Name::new("年度盘点完毕")));
    let archived = world.resource::<TownLedger>().lines.len();
    println!("  艾达：归档 {archived} 户，公告牌立讫。");
}

/// 喇叭：同一帧里宣读总册——独占系统写下的东西立等可读
fn town_crier(
    notices: Query<(), With<Notice>>,
    ledger: Res<TownLedger>,
    mut exit: MessageWriter<AppExit>,
) {
    if notices.is_empty() {
        return;
    }
    println!("  喇叭：宣读总册——");
    for line in &ledger.lines {
        println!("    {line}");
    }
    exit.write(AppExit::Success);
}

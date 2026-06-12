//! Listing 11-13：冬歇的铁匠铺——Disabled 实体

use bevy::app::ScheduleRunnerPlugin;
use bevy::ecs::entity_disabling::Disabled;
use bevy::prelude::*;
use std::time::Duration;

/// 铺面
#[derive(Component)]
struct Shop;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_systems(Startup, open_street)
        .add_systems(Update, (banner, script, reopen, patrol, census).chain())
        .run();
}

fn open_street(mut commands: Commands) {
    commands.spawn((Shop, Name::new("杂货铺")));
    commands.spawn((Shop, Name::new("铁匠铺")));
    commands.spawn((Shop, Name::new("面包房")));
}

fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

// ANCHOR: script
/// 剧本：入冬挂牌，开春摘牌
fn script(
    shops: Query<(Entity, &Name), With<Shop>>,
    mut commands: Commands,
    mut frame: Local<u32>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        2 => {
            let (smithy, name) = shops.iter().find(|(_, n)| n.as_str() == "铁匠铺").unwrap();
            println!("  {name}：入冬封炉，明春再会。（挂上 Disabled）");
            commands.entity(smithy).insert(Disabled);
        }
        3 => {
            // 此刻铁匠铺已带 Disabled，上面的 shops 查询找不到它——
            // 摘牌的活只能交给下面的 reopen
            println!("  开春了，摘牌复工。（摘下 Disabled）");
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}

/// 摘牌：只看得见隐身实体的查询
fn reopen(disabled: Query<Entity, With<Disabled>>, mut commands: Commands, mut frame: Local<u32>) {
    *frame += 1;
    if *frame == 3 {
        for shop in &disabled {
            commands.entity(shop).remove::<Disabled>();
        }
    }
}
// ANCHOR_END: script

// ANCHOR: patrol
/// 巡逻队：什么都没改，却“看不见”冬歇的铺子
fn patrol(shops: Query<&Name, With<Shop>>) {
    let roll: Vec<&str> = shops.iter().map(Name::as_str).collect();
    println!("  巡逻队（{} 家亮灯）：{}", roll.len(), roll.join("、"));
}
// ANCHOR_END: patrol

// ANCHOR: census
/// 盘点：显式提到 Disabled，隐身的也得入册
fn census(shops: Query<(&Name, Has<Disabled>), With<Shop>>) {
    let total = shops.iter().count();
    let resting: Vec<&str> = shops
        .iter()
        .filter(|(_, dormant)| *dormant)
        .map(|(n, _)| n.as_str())
        .collect();
    println!(
        "  盘点册：在册 {total} 家，其中冬歇 {} 家{}",
        resting.len(),
        if resting.is_empty() {
            String::new()
        } else {
            format!("（{}）", resting.join("、"))
        }
    );
}
// ANCHOR_END: census

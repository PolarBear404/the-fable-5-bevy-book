//! 第 9 章综合示例：商队启程
//! 父子树管"谁在哪辆车上"，配发装备走 EquippedBy，认主之物走 SoulboundTo（linked_spawn）；
//! 坠崖一幕是三级连锁：车带走乘员（ChildOf），乘员带走认主之物（SoulboundTo），
//! 配发的灯笼无人认领，掉落在地

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

/// 标记：商队大旗（整棵树的根）
#[derive(Component)]
struct Caravan;

/// 标记：这是一辆车
#[derive(Component)]
struct Wagon;

/// 标记：这是一件物品
#[derive(Component)]
struct Item;

/// 商队配发的装备：主人注销后留在原地
#[derive(Component)]
#[relationship(relationship_target = Equipment)]
struct EquippedBy(Entity);

#[derive(Component)]
#[relationship_target(relationship = EquippedBy)]
struct Equipment(Vec<Entity>);

/// 认主的随身物：主人注销，它也一起注销
#[derive(Component)]
#[relationship(relationship_target = SoulboundGear)]
struct SoulboundTo(Entity);

#[derive(Component)]
#[relationship_target(relationship = SoulboundTo, linked_spawn)]
struct SoulboundGear(Vec<Entity>);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_systems(Update, (banner, script, roll_call).chain())
        .run();

    println!("（run() 返回，商队的故事讲完了）");
}

/// 报幕：让"帧"在输出里看得见
fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

// ANCHOR: script
/// 商队五幕剧，每帧推进一幕
fn script(
    everything: Query<(Entity, &Name)>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    mut frame: Local<u32>,
) {
    *frame += 1;
    let find = |target: &str| {
        everything
            .iter()
            .find(|(_, name)| name.as_str() == target)
            .map(|(entity, _)| entity)
            .unwrap()
    };
    match *frame {
        1 => {
            println!("队长老姜：人齐了，出发！");
            // 父子树：大旗 → 两辆车 → 乘员，一个表达式声明完
            commands.spawn((
                Name::new("商队大旗"),
                Caravan,
                children![
                    (
                        Name::new("青篷车"),
                        Wagon,
                        children![Name::new("老姜"), Name::new("小芙")],
                    ),
                    (
                        Name::new("铁皮货车"),
                        Wagon,
                        children![
                            Name::new("货物箱"),
                            Name::new("罗兰"),
                            Name::new("护卫老蔫儿"),
                        ],
                    ),
                ],
            ));
        }
        2 => {
            // children! 里拿不到乘员的 Entity，装备等下一帧按名字补发
            println!("账房：配发装备，认主之物自行登记。");
            commands.spawn((Name::new("长戟"), Item, EquippedBy(find("小芙"))));
            commands.spawn((
                Name::new("祖传腰刀"),
                Item,
                SoulboundTo(find("护卫老蔫儿")),
            ));
            commands.spawn((
                Name::new("公家灯笼"),
                Item,
                EquippedBy(find("护卫老蔫儿")),
            ));
        }
        3 => {
            println!("罗兰：货车颠得慌，我去青篷车给小芙讲掌灯人的故事。");
            commands.entity(find("罗兰")).insert(ChildOf(find("青篷车")));
        }
        4 => {
            println!("山道塌方！铁皮货车连人带货坠下山崖——");
            commands.entity(find("铁皮货车")).despawn();
        }
        5 => {
            println!("小芙：路边有盏灯笼……我捡走了。");
            commands.entity(find("公家灯笼")).insert(EquippedBy(find("小芙")));
        }
        6 => {
            println!("老姜：灰岩镇到了，收旗散队！");
            commands.entity(find("商队大旗")).despawn();
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
// ANCHOR_END: script

// ANCHOR: roll_call
/// 点名：树形打印整支商队，再报装备归属与地上的遗落物
fn roll_call(
    roots: Query<Entity, With<Caravan>>,
    names: Query<&Name>,
    children_query: Query<&Children>,
    items: Query<(&Name, Option<&EquippedBy>, Option<&SoulboundTo>), With<Item>>,
    ground: Query<&Name, (With<Item>, Without<EquippedBy>, Without<SoulboundTo>)>,
    in_roster: Query<Entity, With<Name>>,
) {
    for root in &roots {
        print_subtree(root, 1, &names, &children_query);
    }
    for (item_name, equipped, soulbound) in &items {
        match (equipped, soulbound) {
            (Some(equipped), _) => println!(
                "  随身：{item_name} → {}（配发）",
                names.get(equipped.0).unwrap()
            ),
            (_, Some(soulbound)) => println!(
                "  随身：{item_name} → {}（认主）",
                names.get(soulbound.0).unwrap()
            ),
            _ => {}
        }
    }
    for item_name in &ground {
        println!("  地上：{item_name}");
    }
    if roots.is_empty() {
        println!("  （在册实体：{} 个）", in_roster.iter().count());
    }
}

fn print_subtree(
    entity: Entity,
    depth: usize,
    names: &Query<&Name>,
    children_query: &Query<&Children>,
) {
    println!("{}{}", "  ".repeat(depth), names.get(entity).unwrap());
    if let Ok(children) = children_query.get(entity) {
        for &child in children {
            print_subtree(child, depth + 1, names, children_query);
        }
    }
}
// ANCHOR_END: roll_call

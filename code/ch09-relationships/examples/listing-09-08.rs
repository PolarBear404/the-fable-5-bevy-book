//! Listing 9-8：自定义关系——装备槽 EquippedBy / Equipment，与父子树并存

use bevy::prelude::*;

/// 标记：这是一辆车
#[derive(Component)]
struct Wagon;

// ANCHOR: derive
/// 关系源：这件物品装备在谁身上（戴在物品实体上）
#[derive(Component)]
#[relationship(relationship_target = Equipment)]
struct EquippedBy(Entity);

/// 关系目标：这个人身上的装备清单（引擎自动维护，字段不公开）
#[derive(Component)]
#[relationship_target(relationship = EquippedBy)]
struct Equipment(Vec<Entity>);
// ANCHOR_END: derive

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (inspect, handover, inspect, drop_charm, inspect).chain(),
        )
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    let wagon = commands.spawn((Name::new("青篷车"), Wagon)).id();

    // 小芙：人在车上（ChildOf），装备在身上（Equipment）——两种关系互不相干
    commands.spawn((
        Name::new("小芙"),
        ChildOf(wagon),
        related!(Equipment[Name::new("长戟"), Name::new("护身符")]),
    ));
    commands.spawn((Name::new("罗兰"), ChildOf(wagon)));
}
// ANCHOR_END: setup

// ANCHOR: handover
/// 转手：给长戟换一个新的 EquippedBy——跟换车是同一个动作
fn handover(everything: Query<(Entity, &Name)>, mut commands: Commands) {
    let spear = find(&everything, "长戟");
    let roland = find(&everything, "罗兰");
    println!("【小芙把长戟交给罗兰】");
    commands.entity(spear).insert(EquippedBy(roland));
}
// ANCHOR_END: handover

/// 颠簸：护身符摔碎——despawn 之后，主人的清单自动除名
fn drop_charm(everything: Query<(Entity, &Name)>, mut commands: Commands) {
    let charm = find(&everything, "护身符");
    println!("【山路颠簸，护身符摔得粉碎】");
    commands.entity(charm).despawn();
}

fn find(query: &Query<(Entity, &Name)>, target: &str) -> Entity {
    query
        .iter()
        .find(|(_, name)| name.as_str() == target)
        .map(|(entity, _)| entity)
        .unwrap()
}

// ANCHOR: inspect
/// 行装清点：每个人报所在的车和身上的装备
fn inspect(crew: Query<(&Name, &ChildOf, Option<&Equipment>)>, names: Query<&Name>) {
    for (name, child_of, equipment) in &crew {
        let wagon = names.get(child_of.parent()).unwrap();
        let gear = match equipment {
            Some(equipment) => {
                let list: Vec<&str> = equipment
                    .iter()
                    .map(|item| names.get(item).unwrap().as_str())
                    .collect();
                list.join("、")
            }
            None => "（空手）".to_string(),
        };
        println!("{name} │ 在{wagon}上 │ 装备：{gear}");
    }
    println!();
}
// ANCHOR_END: inspect

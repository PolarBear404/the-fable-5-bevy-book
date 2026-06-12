//! Listing 11-7：档案室——Archetype、Table 与“搬家”

use bevy::prelude::*;

/// 常住户
#[derive(Component)]
struct Resident;

/// 铺面
#[derive(Component)]
struct Shop;

/// 盖过章：今年已盘点（默认 Table 存储）
#[derive(Component)]
struct Registered;

// ANCHOR: sparse
/// 盘点期间的临时记号：频繁贴撕，声明用 SparseSet 存储
#[derive(Component)]
#[component(storage = "SparseSet")]
struct Flagged;
// ANCHOR_END: sparse

fn main() {
    let mut world = World::new();

    // ANCHOR: archetypes
    println!("空世界的档案册数：{}", world.archetypes().len());

    let roland = world.spawn((Name::new("罗兰"), Resident)).id();
    world.spawn((Name::new("老蔫儿"), Resident));
    println!("住进两户（组件组合相同）：{} 册", world.archetypes().len());

    world.spawn((Name::new("杂货铺老板"), Resident, Shop));
    println!("再住进一户（多一个组件）：{} 册", world.archetypes().len());
    // ANCHOR_END: archetypes

    // ANCHOR: move
    let before = world.entities().get_spawned(roland).unwrap();
    world.entity_mut(roland).insert(Registered); // 盖章：组件组合变了
    let after = world.entities().get_spawned(roland).unwrap();
    println!(
        "盖章（Table 组件）：archetype {:?} → {:?}，table {:?} → {:?}",
        before.archetype_id, after.archetype_id, before.table_id, after.table_id,
    );

    let before = after;
    world.entity_mut(roland).insert(Flagged); // 贴记号：SparseSet 组件
    let after = world.entities().get_spawned(roland).unwrap();
    println!(
        "贴记号（SparseSet 组件）：archetype {:?} → {:?}，table {:?} → {:?}",
        before.archetype_id, after.archetype_id, before.table_id, after.table_id,
    );
    // ANCHOR_END: move

    // ANCHOR: inspect
    print!("罗兰户的档案页：");
    for info in world.inspect_entity(roland).unwrap() {
        print!("[{}] ", info.name().shortname());
    }
    println!();
    // ANCHOR_END: inspect
}

//! Listing 11-4：上门盘点——EntityRef、EntityWorldMut 与 EntityMut

use bevy::prelude::*;

/// 存粮（袋）
#[derive(Component)]
struct Stock(u32);

/// 盖过章：今年已盘点
#[derive(Component)]
struct Registered;

/// 私酿酒（违禁品）
#[derive(Component)]
struct Moonshine;

fn main() {
    let mut world = World::new();
    let roland = world.spawn((Name::new("罗兰"), Stock(3))).id();
    let lao = world.spawn((Name::new("老蔫儿"), Stock(7), Moonshine)).id();
    let shed = world.spawn(Name::new("废弃棚屋")).id();

    // ANCHOR: entity_ref
    // 只读句柄 EntityRef：站在门口看，看什么临场决定
    let house = world.entity(lao);
    println!(
        "{}家：存粮 {} 袋，私酿酒：{}",
        house.get::<Name>().unwrap(),
        house.get::<Stock>().unwrap().0,
        if house.contains::<Moonshine>() { "有！" } else { "无" },
    );
    // ANCHOR_END: entity_ref

    // ANCHOR: entity_world_mut
    // 全权句柄 EntityWorldMut：改数据、增删组件、乃至拆房
    let mut house = world.entity_mut(lao);
    house.get_mut::<Stock>().unwrap().0 -= 1; // 改数据：抽一袋作税粮
    house.insert(Registered); // 加组件：盖章
    if let Some(_jar) = house.take::<Moonshine>() {
        // 摘下组件、拿到它的值：没收
        println!("艾达：私酿酒没收。（老蔫儿：哎——）");
    }
    world.entity_mut(shed).despawn(); // 危房：当场拆除
    // ANCHOR_END: entity_world_mut

    // ANCHOR: get_entity
    // 门牌可能已注销——entity() 直接 panic，get_entity() 给 Result
    match world.get_entity(shed) {
        Ok(_) => println!("棚屋还在"),
        Err(e) => println!("再访棚屋：{e}"),
    }
    // ANCHOR_END: get_entity

    // ANCHOR: many
    // 同时借两户：拿到的是 EntityMut——能读写数据，做不了结构变更
    let [mut rich, mut poor] = world.entity_mut([lao, roland]);
    rich.get_mut::<Stock>().unwrap().0 -= 2;
    poor.get_mut::<Stock>().unwrap().0 += 2; // 济贫：匀两袋
    // rich.insert(Registered); // ← EntityMut 根本没有 insert 这个方法
    // ANCHOR_END: many

    let mut ledger = world.query::<(&Name, &Stock, Has<Registered>)>();
    for (name, stock, registered) in ledger.iter(&world) {
        println!(
            "台账：{name} 存粮 {} 袋{}",
            stock.0,
            if registered { "（已盖章）" } else { "" }
        );
    }
}

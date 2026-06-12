//! Listing 11-3：镇公所的沙盘——World 就是一个普通的 Rust 值
//! 不要 App、不要调度，直接在 main 里摆弄一个世界

use bevy::prelude::*;

/// 存粮（袋）
#[derive(Component)]
struct Stock(u32);

/// 镇库银两
#[derive(Resource)]
struct TownFunds(u32);

/// 敲门（事件）
#[derive(Event)]
struct Knock;

fn main() {
    // ANCHOR: new
    let mut world = World::new();

    let roland = world.spawn((Name::new("罗兰"), Stock(3))).id();
    world.spawn((Name::new("老蔫儿"), Stock(7)));

    // 读：立等可取，没有任何延迟
    let name = world.entity(roland).get::<Name>().unwrap();
    println!("沙盘上第一个小人：{name}");
    // ANCHOR_END: new

    // ANCHOR: resource
    world.insert_resource(TownFunds(100));
    world.resource_mut::<TownFunds>().0 -= 30; // 修桥支出
    println!("修桥之后，镇库还剩 {} 枚银币", world.resource::<TownFunds>().0);
    // ANCHOR_END: resource

    // ANCHOR: query
    // World 上没有现成的 Query 参数，要先向它要一个查询
    let mut stocks = world.query::<(&Name, &Stock)>();
    for (name, stock) in stocks.iter(&world) {
        println!("{name}家存粮 {} 袋", stock.0);
    }
    // ANCHOR_END: query

    // ANCHOR: scope
    // 既要镇库（资源的 &mut）又要跑查询（World 的 &mut）？
    // resource_scope 把资源暂时“摘下来”，两不耽误
    world.resource_scope(|world, mut funds: Mut<TownFunds>| {
        let households = world.query::<&Stock>().iter(world).count();
        funds.0 += households as u32; // 每户上缴 1 枚
        println!("收讫人头税 {households} 枚，镇库现银 {} 枚", funds.0);
    });
    // ANCHOR_END: scope

    // ANCHOR: trigger
    world.add_observer(|_: On<Knock>| println!("  （沙盘小人探出头：谁呀？）"));
    println!("敲门——");
    world.trigger(Knock);
    println!("——话音未落，门里已经应了。"); // trigger 当场执行完毕才返回
    // ANCHOR_END: trigger

    // ANCHOR: despawn
    world.despawn(roland);
    match world.get_entity(roland) {
        Ok(_) => println!("罗兰还在"),
        Err(e) => println!("再访罗兰家：{e}"),
    }
    // ANCHOR_END: despawn
}

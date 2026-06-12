//! Listing 8-3：EntityEvent——鉴定室里指名道姓

use bevy::prelude::*;

/// 实体事件：某件物品被鉴定了——entity 字段就是事件的目标
#[derive(EntityEvent)]
struct Identified {
    entity: Entity,
}

#[derive(Component)]
struct Item {
    name: &'static str,
}

fn main() {
    let mut app = App::new();
    app.add_observer(record)
        .add_systems(Startup, stock_items)
        .add_systems(Update, identify_all);

    println!("—— 第 1 帧 ——");
    app.update();
}

fn stock_items(mut commands: Commands) {
    commands.spawn(Item { name: "铁剑" });
    // 给这把剑挂一个只属于它的 observer
    commands
        .spawn(Item { name: "诅咒之剑" })
        .observe(|_identified: On<Identified>| {
            println!("诅咒之剑：（剑身震颤）谁准你看穿我的底细！");
        });
}

/// 鉴定师：把架上的物品挨个鉴定一遍
fn identify_all(items: Query<Entity, With<Item>>, mut commands: Commands) {
    for entity in &items {
        commands.trigger(Identified { entity });
    }
}

/// 全局 observer：每一次鉴定都记录在案，不论目标是谁
fn record(identified: On<Identified>, items: Query<&Item>) {
    let item = items.get(identified.entity).unwrap();
    println!("鉴定师：{} 鉴定完毕，记录在案。", item.name);
}

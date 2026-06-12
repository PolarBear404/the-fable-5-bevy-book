//! Listing 8-4：鉴定单追进了熔炉——事件送达时，目标可能已经没了

use bevy::prelude::*;

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
        .add_systems(Update, melt_then_identify);

    println!("—— 第 1 帧 ——");
    app.update();
}

/// 学徒：先把剑丢进熔炉，再递交对它的鉴定单
fn melt_then_identify(mut commands: Commands) {
    let sword = commands.spawn(Item { name: "铁剑" }).id();
    commands.entity(sword).despawn();
    println!("学徒：剑已经熔了，但鉴定单还是递了上去……");
    commands.trigger(Identified { entity: sword });
}

/// 鉴定师：查不到目标就退单，而不是 unwrap
fn record(identified: On<Identified>, items: Query<&Item>) {
    let Ok(item) = items.get(identified.entity) else {
        println!("鉴定师：单子上的 {} 已经不在了，退单。", identified.entity);
        return;
    };
    println!("鉴定师：{} 鉴定完毕。", item.name);
}

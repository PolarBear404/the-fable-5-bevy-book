//! Listing 9-1：第一次父子树——乘员挂上 ChildOf，车自动长出 Children

use bevy::prelude::*;

/// 标记：这是一辆车
#[derive(Component)]
struct Wagon;

fn main() {
    App::new()
        .add_systems(Startup, board)
        .add_systems(Update, (roster_from_wagon, whereami_from_crew).chain())
        .run();
}

// ANCHOR: board
/// 发车前：乘员逐个上车——只写 ChildOf，不碰 Children
fn board(mut commands: Commands) {
    let wagon = commands.spawn((Name::new("青篷车"), Wagon)).id();

    commands.spawn((Name::new("老姜"), ChildOf(wagon)));
    commands.spawn((Name::new("小芙"), ChildOf(wagon)));
    commands.spawn((Name::new("罗兰"), ChildOf(wagon)));
}
// ANCHOR_END: board

// ANCHOR: roster
/// 从车往下看：Children 是引擎替我们记好的名单
fn roster_from_wagon(wagons: Query<(&Name, &Children), With<Wagon>>, names: Query<&Name>) {
    println!("== 从车往下看 ==");
    for (wagon_name, children) in &wagons {
        println!("{wagon_name} 载着 {} 人：", children.len());
        for &crew in children {
            println!("  - {}", names.get(crew).unwrap());
        }
    }
}
// ANCHOR_END: roster

// ANCHOR: whereami
/// 从人往上看：ChildOf 记着自己的父实体
fn whereami_from_crew(crew: Query<(&Name, &ChildOf)>, names: Query<&Name>) {
    println!("== 从人往上看 ==");
    for (crew_name, child_of) in &crew {
        let wagon_name = names.get(child_of.parent()).unwrap();
        println!("{crew_name} 在 {wagon_name} 上");
    }
}
// ANCHOR_END: whereami

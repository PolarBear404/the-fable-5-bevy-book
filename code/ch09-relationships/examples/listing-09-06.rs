//! Listing 9-6：换车的正确姿势——insert 一个新的 ChildOf，旧关系自动了断

use bevy::prelude::*;

/// 标记：这是一辆车
#[derive(Component)]
struct Wagon;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (muster, transfer, muster).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("青篷车"),
        Wagon,
        children![Name::new("老姜"), Name::new("小芙")],
    ));
    commands.spawn((Name::new("铁皮货车"), Wagon, children![Name::new("罗兰")]));
}

// ANCHOR: transfer
/// 换乘：给小芙 insert 新的 ChildOf——旧车那边自动除名
fn transfer(
    crew: Query<(Entity, &Name), With<ChildOf>>,
    wagons: Query<(Entity, &Name), With<Wagon>>,
    mut commands: Commands,
) {
    let xiaofu = find(&crew, "小芙");
    let truck = find(&wagons, "铁皮货车");
    println!("【小芙晕车，换乘铁皮货车】");
    commands.entity(xiaofu).insert(ChildOf(truck));
}
// ANCHOR_END: transfer

fn find<F: bevy::ecs::query::QueryFilter>(
    query: &Query<(Entity, &Name), F>,
    target: &str,
) -> Entity {
    query
        .iter()
        .find(|(_, name)| name.as_str() == target)
        .map(|(entity, _)| entity)
        .unwrap()
}

fn muster(wagons: Query<(&Name, Option<&Children>), With<Wagon>>, names: Query<&Name>) {
    for (wagon_name, children) in &wagons {
        match children {
            Some(children) => {
                let list: Vec<&str> = children
                    .iter()
                    .map(|child| names.get(child).unwrap().as_str())
                    .collect();
                println!("{wagon_name} → {}", list.join("、"));
            }
            None => println!("{wagon_name} →（空车）"),
        }
    }
    println!();
}

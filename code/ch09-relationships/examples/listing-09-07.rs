//! Listing 9-7：商队点名——iter_descendants、iter_descendants_depth_first、
//! iter_ancestors 与 root_ancestor

use bevy::prelude::*;

/// 标记：商队大旗（整棵树的根）
#[derive(Component)]
struct Caravan;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (roll_call, trace_up).chain())
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("商队大旗"),
        Caravan,
        children![
            (
                Name::new("青篷车"),
                children![
                    Name::new("老姜"),
                    (Name::new("小芙"), children![Name::new("长戟")]),
                ],
            ),
            (
                Name::new("铁皮货车"),
                children![(Name::new("罗兰"), children![Name::new("铜灯")])],
            ),
        ],
    ));
}
// ANCHOR_END: setup

// ANCHOR: roll_call
/// 全队点名：同一棵树，两种走法
fn roll_call(
    caravan: Single<Entity, With<Caravan>>,
    children_query: Query<&Children>,
    names: Query<&Name>,
) {
    let breadth: Vec<&str> = children_query
        .iter_descendants(*caravan)
        .map(|entity| names.get(entity).unwrap().as_str())
        .collect();
    println!("广度优先：{}", breadth.join("、"));

    let depth: Vec<&str> = children_query
        .iter_descendants_depth_first(*caravan)
        .map(|entity| names.get(entity).unwrap().as_str())
        .collect();
    println!("深度优先：{}", depth.join("、"));
}
// ANCHOR_END: roll_call

// ANCHOR: trace_up
/// 失物招领：从长戟一路向上找到树根
fn trace_up(
    everything: Query<(Entity, &Name)>,
    parents: Query<&ChildOf>,
    names: Query<&Name>,
) {
    let (spear, _) = everything
        .iter()
        .find(|(_, name)| name.as_str() == "长戟")
        .unwrap();

    print!("长戟在谁手里：长戟");
    for ancestor in parents.iter_ancestors(spear) {
        print!(" ← {}", names.get(ancestor).unwrap());
    }
    println!();

    let root = parents.root_ancestor(spear);
    println!("它属于哪支商队：{}", names.get(root).unwrap());
}
// ANCHOR_END: trace_up

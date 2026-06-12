//! Listing 9-2：建树三种写法——children! 宏、with_children 闭包、add_child 后补

use bevy::prelude::*;

/// 标记：这是一辆车
#[derive(Component)]
struct Wagon;

fn main() {
    App::new()
        .add_systems(Startup, assemble)
        .add_systems(Update, print_trees)
        .run();
}

// ANCHOR: assemble
fn assemble(mut commands: Commands) {
    // 写法一：children! 宏——树长什么样，代码就长什么样
    commands.spawn((
        Name::new("铁皮货车"),
        Wagon,
        children![
            (Name::new("罗兰"), children![Name::new("铜灯")]),
            Name::new("货物箱"),
        ],
    ));

    // 写法二：with_children 闭包——适合循环生成，或继续拿子实体的 id
    let wagon = commands
        .spawn((Name::new("青篷车"), Wagon))
        .with_children(|car| {
            car.spawn(Name::new("老姜"));
            for i in 1..=2 {
                car.spawn(Name::new(format!("木桶 {i} 号")));
            }
        })
        .id();

    // 写法三：add_child——实体早已存在，事后认亲
    let pole = commands.spawn(Name::new("扁担")).id();
    commands.entity(wagon).add_child(pole);
}
// ANCHOR_END: assemble

// ANCHOR: print
/// 从每辆车出发，递归打印整棵树
fn print_trees(
    wagons: Query<Entity, With<Wagon>>,
    names: Query<&Name>,
    children_query: Query<&Children>,
) {
    for wagon in &wagons {
        print_subtree(wagon, 0, &names, &children_query);
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
// ANCHOR_END: print

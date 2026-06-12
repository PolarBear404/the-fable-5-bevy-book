//! Listing 9-4：三种告别——despawn 全家、despawn_children 清舱、remove::<ChildOf> 下车

use bevy::prelude::*;

/// 标记：这是一辆车
#[derive(Component)]
struct Wagon;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (muster, act_fall, muster, act_burn, muster, act_walk, muster).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    println!("== 出发点名 ==");
    commands.spawn((
        Name::new("铁皮货车"),
        Wagon,
        children![Name::new("货物箱一"), Name::new("货物箱二")],
    ));
    commands.spawn((
        Name::new("柴车"),
        Wagon,
        children![Name::new("湿柴一捆"), Name::new("湿柴二捆")],
    ));
    commands.spawn((
        Name::new("青篷车"),
        Wagon,
        children![Name::new("老姜"), Name::new("小芙")],
    ));
}

// ANCHOR: acts
/// 第一刀：despawn——连车带货整树消失
fn act_fall(wagons: Query<(Entity, &Name), With<Wagon>>, mut commands: Commands) {
    let truck = find(&wagons, "铁皮货车");
    println!("【塌方！铁皮货车坠下山崖】");
    commands.entity(truck).despawn();
}

/// 第二刀：despawn_children——只销毁全部子实体，车留下
fn act_burn(wagons: Query<(Entity, &Name), With<Wagon>>, mut commands: Commands) {
    let cart = find(&wagons, "柴车");
    println!("【湿柴发了霉，连捆烧掉——车留下】");
    commands.entity(cart).despawn_children();
}

/// 第三刀：remove::<ChildOf>——解除关系，人和车都还在
fn act_walk(crew: Query<(Entity, &Name), With<ChildOf>>, mut commands: Commands) {
    let jiang = find(&crew, "老姜");
    println!("【到家门口了，老姜下车】");
    commands.entity(jiang).remove::<ChildOf>();
}
// ANCHOR_END: acts

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

// ANCHOR: muster
/// 清点：每辆车报名单；空车与下车的人也要点到
fn muster(
    wagons: Query<(&Name, Option<&Children>), With<Wagon>>,
    names: Query<&Name>,
    loose: Query<&Name, (Without<ChildOf>, Without<Wagon>)>,
) {
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
    for name in &loose {
        println!("地面上站着：{name}");
    }
    println!();
}
// ANCHOR_END: muster

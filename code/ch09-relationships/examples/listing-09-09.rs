//! Listing 9-9：linked_spawn——公家灯笼留下，祖传腰刀随主人走

use bevy::prelude::*;

// ANCHOR: derive
/// 商队配发的装备：主人注销后留在原地，等下一任来领
#[derive(Component)]
#[relationship(relationship_target = Equipment)]
struct EquippedBy(Entity);

#[derive(Component)]
#[relationship_target(relationship = EquippedBy)]
struct Equipment(Vec<Entity>);

/// 认主的随身物：linked_spawn——主人注销，它也一起注销
#[derive(Component)]
#[relationship(relationship_target = SoulboundGear)]
struct SoulboundTo(Entity);

#[derive(Component)]
#[relationship_target(relationship = SoulboundTo, linked_spawn)]
struct SoulboundGear(Vec<Entity>);
// ANCHOR_END: derive

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (roster, dismiss, roster).chain())
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    let guard = commands.spawn(Name::new("护卫老蔫儿")).id();
    commands.spawn((Name::new("公家灯笼"), EquippedBy(guard)));
    commands.spawn((Name::new("祖传腰刀"), SoulboundTo(guard)));
}

/// 合同到期：注销护卫实体
fn dismiss(guards: Query<Entity, With<SoulboundGear>>, mut commands: Commands) {
    println!("【这趟跑完，老蔫儿领钱走人】");
    for guard in &guards {
        commands.entity(guard).despawn();
    }
}
// ANCHOR_END: setup

/// 清点在册实体，并报出每件物品的归属
fn roster(
    everything: Query<(&Name, Option<&EquippedBy>, Option<&SoulboundTo>)>,
    names: Query<&Name>,
) {
    println!("== 清点 ==");
    for (name, equipped, soulbound) in &everything {
        match (equipped, soulbound) {
            (Some(equipped), _) => {
                println!("{name}（配发）→ 在 {} 手里", names.get(equipped.0).unwrap());
            }
            (_, Some(soulbound)) => {
                println!("{name}（认主）→ 绑定 {}", names.get(soulbound.0).unwrap());
            }
            _ => println!("{name}"),
        }
    }
    println!();
}

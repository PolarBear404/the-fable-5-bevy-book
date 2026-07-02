//! Listing 11-14：镇公所的内账——混合查询与 IsResource

use bevy::app::ScheduleRunnerPlugin;
use bevy::ecs::{component::Components, resource::IsResource};
use bevy::prelude::*;

/// 常住户
#[derive(Component)]
struct Resident;

/// 存粮（袋）
#[derive(Component)]
struct Stock(u32);

/// 镇库银两
#[derive(Resource)]
struct TownFunds(u32);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .insert_resource(TownFunds(73))
        .add_systems(Startup, settle_in)
        .add_systems(Update, (roll_call, tally).chain())
        .run();
}

fn settle_in(mut commands: Commands) {
    commands.spawn((Resident, Name::new("罗兰"), Stock(3)));
    commands.spawn((Resident, Name::new("老蔫儿"), Stock(7)));
    commands.spawn((Resident, Name::new("杂货铺老板"), Stock(40)));
}

// ANCHOR: roll_call
/// 全场点名：住户和资源实体在同一个查询里过审
fn roll_call(
    everyone: Query<(Entity, Option<&Name>, Option<&IsResource>)>,
    components: &Components,
    world: &World,
) {
    let mut rows: Vec<_> = everyone.iter().collect();
    rows.sort_by_key(|(id, ..)| id.index()); // 按行号排序，只为看着整齐
    println!("艾达翻开全镇的账（count_spawned = {}）：", world.entities().count_spawned());
    for (id, name, is_resource) in rows {
        // 这一行上到底放着几件东西？问档案（11-3 节的 inspect_entity）
        let held = world.inspect_entity(id).unwrap().count();
        if let Some(marker) = is_resource {
            // IsResource 里记着这行装的是哪个资源，借 Components 查出类型名
            let type_name = components.get_name(marker.resource_component_id()).unwrap();
            println!("  {id}  资源  {}（{held} 件）", type_name.shortname());
        } else if let Some(name) = name {
            println!("  {id}  住户  {name}（{held} 件）");
        }
    }
}
// ANCHOR_END: roll_call

// ANCHOR: tally
/// 分账：广查询靠 With/Without<IsResource> 划界；窄查询天生不沾内账
fn tally(
    inner: Query<(), With<IsResource>>,      // 广查询圈内：只看资源实体
    town: Query<Entity, Without<IsResource>>, // 广查询圈外：只看普通实体
    granary: Query<&Stock>,                   // 窄查询：点了名就不用划界
    funds: Res<TownFunds>,
) {
    let grain: u32 = granary.iter().map(|s| s.0).sum();
    println!(
        "内账 {} 行，民册 {} 行；存粮共 {grain} 袋，镇库 {} 枚（Res 照常直达）。",
        inner.iter().count(),
        town.iter().count(),
        funds.0
    );
}
// ANCHOR_END: tally

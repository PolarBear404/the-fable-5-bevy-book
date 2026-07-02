//! Listing 5-7：全场点名——资源实体现出真身

// ANCHOR: setup
use bevy::ecs::{component::Components, resource::IsResource};
use bevy::prelude::*;

/// 计分板：本章用到现在的普通资源
#[derive(Resource)]
struct Score(u32);

fn main() {
    let mut app = App::new();
    app.insert_resource(Score(0))
        .add_systems(Startup, spawn_targets)
        .add_systems(Update, roll_call);
    app.update();
}

/// 生成两个普通实体，与资源实体同台对照
fn spawn_targets(mut commands: Commands) {
    commands.spawn(Name::new("外环"));
    commands.spawn(Name::new("红心"));
}
// ANCHOR_END: setup

// ANCHOR: roll_call
/// 全场点名：不点名任何具体组件的广查询，World 里每一行都会到场
fn roll_call(
    everyone: Query<(Entity, Option<&IsResource>, Option<&Name>)>,
    components: &Components,
    score: Res<Score>,
) {
    let mut rows: Vec<_> = everyone.iter().collect();
    rows.sort_by_key(|(entity, ..)| entity.index()); // 按行号排序，只为看着整齐
    for (entity, is_resource, name) in rows {
        if let Some(marker) = is_resource {
            // IsResource 里记着这行装的是哪个资源，借 Components 查出类型名
            let resource_name = components.get_name(marker.resource_component_id()).unwrap();
            println!("{entity}  资源实体  {}", resource_name.shortname());
        } else if let Some(name) = name {
            println!("{entity}  普通实体  {name}");
        }
    }
    println!("Res<Score> 照常直达：当前 {} 分", score.0);
}
// ANCHOR_END: roll_call

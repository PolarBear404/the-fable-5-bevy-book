//! Listing 4-2：一条查询里的四种取法——必有（&T）、可选（Option）、在场与否（Has）、行号（Entity）

use bevy::prelude::*;

#[derive(Component)]
struct Hunger(i32);

/// 铃铛标记
#[derive(Component)]
struct Bell;

fn main() {
    App::new()
        .add_systems(Startup, spawn_farm)
        .add_systems(Update, roster)
        .run();
}

// ANCHOR: spawn
fn spawn_farm(mut commands: Commands) {
    commands.spawn((Name::new("小白"), Hunger(6), Bell));
    commands.spawn((Name::new("小黑"), Hunger(9)));
    commands.spawn((Name::new("阿黄"), Bell));
    commands.spawn(Name::new("灰背"));
}
// ANCHOR_END: spawn

// ANCHOR: roster
fn roster(animals: Query<(Entity, &Name, Option<&Hunger>, Has<Bell>)>) {
    println!("=== 农场花名册 ===");
    for (entity, name, hunger, has_bell) in &animals {
        let hunger_text = match hunger {
            Some(hunger) => format!("饥饿 {}", hunger.0),
            None => "饥饿 —".to_string(),
        };
        let bell_text = if has_bell { "戴铃铛" } else { "无铃铛" };
        println!("{entity}  {name}  {hunger_text}  {bell_text}");
    }
}
// ANCHOR_END: roster

//! Listing 4-4：过滤器的组合——元组是"且"，Or 是"或"，Without 是"非"

use bevy::prelude::*;

#[derive(Component)]
struct Sheep;

#[derive(Component)]
struct Wolf;

#[derive(Component)]
struct Sheepdog;

#[derive(Component)]
struct Bell;

fn main() {
    App::new()
        .add_systems(Startup, spawn_farm)
        .add_systems(Update, night_census)
        .run();
}

fn spawn_farm(mut commands: Commands) {
    commands.spawn((Name::new("小白"), Sheep, Bell));
    commands.spawn((Name::new("小黑"), Sheep));
    commands.spawn((Name::new("卷卷"), Sheep));
    commands.spawn((Name::new("阿黄"), Sheepdog, Bell));
    commands.spawn((Name::new("灰背"), Wolf));
}

// ANCHOR: census
fn night_census(
    // 羊，或牧羊犬——在牧场过夜的住户
    residents: Query<&Name, Or<(With<Sheep>, With<Sheepdog>)>>,
    // 羊，且没戴铃铛
    unbelled_sheep: Query<&Name, (With<Sheep>, Without<Bell>)>,
    // （羊或牧羊犬），且没戴铃铛——过滤器可以任意嵌套
    unbelled_residents: Query<&Name, (Or<(With<Sheep>, With<Sheepdog>)>, Without<Bell>)>,
) {
    let list = |names: Vec<&str>| names.join("、");
    println!("过夜的住户：{}", list(residents.iter().map(Name::as_str).collect()));
    println!("没铃铛的羊：{}", list(unbelled_sheep.iter().map(Name::as_str).collect()));
    println!("没铃铛的住户：{}", list(unbelled_residents.iter().map(Name::as_str).collect()));
}
// ANCHOR_END: census

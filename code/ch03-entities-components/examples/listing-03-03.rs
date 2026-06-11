use bevy::prelude::*;

#[derive(Component)]
struct Monster;

fn main() {
    App::new()
        .add_systems(Update, (call_reinforcements, recount).chain())
        .run();
}

fn call_reinforcements(mut commands: Commands, monsters: Query<&Monster>) {
    println!("下令前，查询到的怪物数：{}", monsters.iter().count());

    let id = commands.spawn(Monster).id();
    println!("已下令生成增援，预分配的 Entity：{id}");

    println!("下令后，查询到的怪物数：{}", monsters.iter().count());
}

fn recount(monsters: Query<&Monster>) {
    println!("下一个系统查询到的怪物数：{}", monsters.iter().count());
}

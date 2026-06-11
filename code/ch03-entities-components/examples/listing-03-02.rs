use bevy::prelude::*;

#[derive(Component)]
struct Health(i32);

// ANCHOR: markers
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Monster;
// ANCHOR_END: markers

fn main() {
    App::new()
        .add_systems(Startup, spawn_party)
        .add_systems(Update, print_roster)
        .run();
}

// ANCHOR: spawn_party
fn spawn_party(mut commands: Commands) {
    commands.spawn((Name::new("罗兰"), Player, Health(100)));
    commands.spawn((Name::new("史莱姆"), Monster, Health(30)));
    commands.spawn((Name::new("骷髅兵"), Monster, Health(45)));

    // 批量生成：组件组合相同、数据各异的一批实体
    commands.spawn_batch(
        (1..=3).map(|i| (Name::new(format!("蝙蝠 {i} 号")), Monster, Health(10))),
    );
}
// ANCHOR_END: spawn_party

// ANCHOR: roster
fn print_roster(creatures: Query<(Entity, &Name, &Health)>) {
    println!("=== 实体清单 ===");
    for (entity, name, health) in &creatures {
        println!("{entity}  {name}  HP {}", health.0);
    }
}
// ANCHOR_END: roster

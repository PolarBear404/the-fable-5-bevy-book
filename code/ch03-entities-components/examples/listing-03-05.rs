use bevy::prelude::*;

// ANCHOR: definitions
#[derive(Component)]
struct Health(i32);

impl Default for Health {
    fn default() -> Self {
        Health(30)
    }
}

#[derive(Component, Default)]
#[require(Health)]
struct Monster;

#[derive(Component)]
#[require(Monster, Health(120))]
struct Golem;
// ANCHOR_END: definitions

fn main() {
    App::new()
        .add_systems(Startup, spawn_creatures)
        .add_systems(Update, print_roster)
        .run();
}

// ANCHOR: spawn
fn spawn_creatures(mut commands: Commands) {
    // 只给出 Monster：缺的 Health 由引擎用 Default 值补齐
    commands.spawn((Name::new("史莱姆"), Monster));
    // 手动给出的组件优先，required 构造器让位
    commands.spawn((Name::new("史莱姆王"), Monster, Health(99)));
    // Golem：递归补上 Monster，Health 用它声明的 120
    commands.spawn((Name::new("石巨人"), Golem));
}
// ANCHOR_END: spawn

// ANCHOR: roster
fn print_roster(monsters: Query<(&Name, &Health), With<Monster>>) {
    println!("=== 怪物清单 ===");
    for (name, health) in &monsters {
        println!("{name}  HP {}", health.0);
    }
}
// ANCHOR_END: roster

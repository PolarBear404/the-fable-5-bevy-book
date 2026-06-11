use bevy::prelude::*;

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct Wounded;

fn main() {
    App::new()
        .add_systems(Startup, spawn_monsters)
        .add_systems(Update, (sweep, triage, reinforce, print_roster).chain())
        .run();
}

// ANCHOR: spawn
fn spawn_monsters(mut commands: Commands) {
    commands.spawn((Name::new("史莱姆"), Health(30)));
    commands.spawn((Name::new("骷髅兵"), Health(45)));
    commands.spawn((Name::new("石巨人"), Health(80)));
}
// ANCHOR_END: spawn

// ANCHOR: sweep
fn sweep(mut commands: Commands, mut monsters: Query<(Entity, &Name, &mut Health)>) {
    for (entity, name, mut health) in &mut monsters {
        health.0 -= 50;
        if health.0 <= 0 {
            println!("{name} 倒下了");
            commands.entity(entity).despawn();
        } else {
            println!("{name} 负伤，剩 {} 点生命", health.0);
            commands.entity(entity).insert(Wounded);
        }
    }
}
// ANCHOR_END: sweep

// ANCHOR: triage
fn triage(mut commands: Commands, wounded: Query<(Entity, &Name), With<Wounded>>) {
    for (entity, name) in &wounded {
        println!("牧师治疗了 {name}");
        commands.entity(entity).remove::<Wounded>();
    }
}
// ANCHOR_END: triage

// ANCHOR: reinforce
fn reinforce(mut commands: Commands) {
    let id = commands.spawn((Name::new("增援史莱姆"), Health(30))).id();
    println!("增援抵达：{id}");
}
// ANCHOR_END: reinforce

// ANCHOR: roster
fn print_roster(survivors: Query<(Entity, &Name, &Health)>) {
    println!("=== 战后清点 ===");
    for (entity, name, health) in &survivors {
        println!("{entity}  {name}  HP {}", health.0);
    }
}
// ANCHOR_END: roster

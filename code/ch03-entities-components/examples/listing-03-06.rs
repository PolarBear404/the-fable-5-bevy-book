use bevy::prelude::*;

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct Monster;

// ANCHOR: bundle
#[derive(Bundle)]
struct MonsterBundle {
    name: Name,
    marker: Monster,
    health: Health,
}
// ANCHOR_END: bundle

fn main() {
    App::new()
        .add_systems(Startup, spawn_wave)
        .add_systems(Update, print_roster)
        .run();
}

// ANCHOR: spawn
fn spawn_wave(mut commands: Commands) {
    commands.spawn(MonsterBundle {
        name: Name::new("史莱姆"),
        marker: Monster,
        health: Health(30),
    });
}
// ANCHOR_END: spawn

fn print_roster(monsters: Query<(&Name, &Health), With<Monster>>) {
    for (name, health) in &monsters {
        println!("{name}  HP {}", health.0);
    }
}

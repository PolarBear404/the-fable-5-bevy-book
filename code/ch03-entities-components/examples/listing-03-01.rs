use bevy::prelude::*;

#[derive(Component)]
struct Health(i32);

fn main() {
    App::new()
        .add_systems(Startup, spawn_slime)
        .add_systems(Update, census)
        .run();
}

fn spawn_slime(mut commands: Commands) {
    commands.spawn(Health(30));
}

fn census(creatures: Query<&Health>) {
    for health in &creatures {
        println!("发现一个实体，生命值：{}", health.0);
    }
}

use bevy::prelude::*;

fn hello() {
    println!("Hello, Bevy!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, hello)
        .run();
}

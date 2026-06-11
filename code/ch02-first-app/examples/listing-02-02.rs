use bevy::prelude::*;

fn hello() {
    println!("Hello, Bevy!");
}

fn main() {
    App::new().add_systems(Update, hello).run();
}

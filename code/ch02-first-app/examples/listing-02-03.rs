use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::time::Duration;

fn hello() {
    println!("Hello, Bevy!");
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs(1),
        )))
        .add_systems(Update, hello)
        .run();
}

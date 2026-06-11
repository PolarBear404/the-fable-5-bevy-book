use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::time::Duration;

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hello_once)
            .add_systems(Update, hello_every_update);
    }
}

fn hello_once() {
    println!("[Startup] HelloPlugin is on!");
}

fn hello_every_update() {
    println!("[Update] hello again");
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs(1),
        )))
        .add_plugins(HelloPlugin)
        .run();
}

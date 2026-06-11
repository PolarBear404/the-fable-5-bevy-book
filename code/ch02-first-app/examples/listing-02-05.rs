use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "我的第一个 Bevy 窗口".into(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}

//! Listing 13-6：行不通——OrthographicProjection 没有 Default

use bevy::prelude::*;

// ANCHOR: no_default
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.5,
            ..default() // 错误：OrthographicProjection 没实现 Default
        }),
    ));
}
// ANCHOR_END: no_default

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

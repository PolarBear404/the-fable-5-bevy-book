//! Listing 28-10：格线从一数起——数组的习惯在这儿害人。
//! 把包厢放到「第 0 列」，程序当场倒在 Startup 里。

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            display: Display::Grid,
            width: percent(100),
            height: percent(100),
            grid_template_columns: RepeatedGridTrack::flex(8, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
            ..default()
        },
        children![(
            Node {
                // 想放最左一列，顺手写了个 0——格线是从 1 数起的
                grid_column: GridPlacement::start(0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.83, 0.69, 0.36)),
        )],
    ));
}
// ANCHOR_END: setup

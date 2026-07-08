//! Listing 28-11：地契写了没换衙门——grid_template 配齐了，
//! display 还是默认的 Flex，格子字段整套被静默无视。按 D 换成 Grid 立正。

use bevy::prelude::*;

/// 那块「画了格子」的地
#[derive(Component)]
struct Board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, switch_display)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|stage| {
            stage
                .spawn((
                    Board,
                    // 图纸上画了 4 列 2 行——可 display 没写，还是默认的 Flex。
                    // Flex 不认识 grid_* 字段：不用、不警告、不报错
                    Node {
                        width: px(560),
                        border: UiRect::all(px(3)),
                        padding: UiRect::all(px(8)),
                        row_gap: px(8),
                        column_gap: px(8),
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.83, 0.69, 0.36)),
                ))
                .with_children(|board| {
                    for i in 0..8 {
                        let color = if i % 2 == 0 {
                            Color::srgb(0.55, 0.17, 0.12)
                        } else {
                            Color::srgb(0.38, 0.65, 0.66)
                        };
                        board.spawn((
                            Node {
                                width: px(120),
                                height: px(64),
                                ..default()
                            },
                            BackgroundColor(color),
                        ));
                    }
                });
        });
    println!("水牌师傅：格子明明画了 4×2，怎么全挤一排？——按 D 换衙门。");
}
// ANCHOR_END: setup

// ANCHOR: switch
/// D：在 Flex（默认）和 Grid 之间拨 display
fn switch_display(keys: Res<ButtonInput<KeyCode>>, mut board: Single<&mut Node, With<Board>>) {
    if keys.just_pressed(KeyCode::KeyD) {
        board.display = match board.display {
            Display::Grid => Display::Flex,
            _ => Display::Grid,
        };
        println!("  display 拨到 {:?}", board.display);
    }
}
// ANCHOR_END: switch

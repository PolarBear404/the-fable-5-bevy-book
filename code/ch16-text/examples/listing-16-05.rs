//! Listing 16-5：行高与磨边——LineHeight 与 FontSmoothing

use bevy::prelude::*;
use bevy::text::{FontSmoothing, LineHeight};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let zh_font = asset_server.load("fonts/book-sans-sc-regular.otf");
    let verse = "雁背驮霜\n橹声欸乃\n一篙点破满江星";

    // 三种行高：默认 1.2 倍字号 / 宽松 1.8 倍 / 用像素写死
    for (x, line_height) in [
        (-380.0, LineHeight::RelativeToFont(1.2)),
        (0.0, LineHeight::RelativeToFont(1.8)),
        (380.0, LineHeight::Px(34.0)),
    ] {
        commands.spawn((
            Text2d::new(verse),
            TextFont {
                font: zh_font.clone(),
                font_size: 32.0,
                ..default()
            },
            line_height,
            Transform::from_xyz(x, 110.0, 0.0),
        ));
    }

    // 磨边对比：默认灰度抗锯齿 vs 关掉抗锯齿（像素字）
    commands.spawn((
        Text2d::new("磨边的字"),
        TextFont {
            font: zh_font.clone(),
            font_size: 56.0,
            ..default()
        },
        Transform::from_xyz(-220.0, -220.0, 0.0),
    ));
    commands.spawn((
        Text2d::new("不磨的字"),
        TextFont {
            font: zh_font.clone(),
            font_size: 56.0,
            font_smoothing: FontSmoothing::None,
            ..default()
        },
        Transform::from_xyz(220.0, -220.0, 0.0),
    ));
}
// ANCHOR_END: setup

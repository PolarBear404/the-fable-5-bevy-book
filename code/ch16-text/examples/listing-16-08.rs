//! Listing 16-8：行高、字距与磨边——LineHeight、LetterSpacing 与 FontSmoothing

use bevy::prelude::*;
use bevy::text::{LetterSpacing, LineHeight};

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
                font: zh_font.clone().into(),
                font_size: FontSize::Px(32.0),
                ..default()
            },
            line_height,
            Transform::from_xyz(x, 150.0, 0.0),
        ));
    }

    // 字距对比：同样四个字，一块匾额把字距拉开
    for (y, label, spacing) in [
        (-40.0, "默认字距", LetterSpacing::Px(0.0)),
        (-110.0, "Px(16)", LetterSpacing::Px(16.0)),
    ] {
        commands.spawn((
            Text2d::new(label),
            TextFont {
                font: zh_font.clone().into(),
                font_size: FontSize::Px(18.0),
                ..default()
            },
            Transform::from_xyz(-360.0, y, 0.0),
        ));
        commands.spawn((
            Text2d::new("渡口夜话"),
            TextFont {
                font: zh_font.clone().into(),
                font_size: FontSize::Px(40.0),
                ..default()
            },
            spacing,
            Transform::from_xyz(0.0, y, 0.0),
        ));
    }

    // 磨边对比：默认灰度抗锯齿 vs 关掉抗锯齿（像素字）
    commands.spawn((
        Text2d::new("磨边的字"),
        TextFont {
            font: zh_font.clone().into(),
            font_size: FontSize::Px(56.0),
            ..default()
        },
        Transform::from_xyz(-220.0, -250.0, 0.0),
    ));
    commands.spawn((
        Text2d::new("不磨的字"),
        TextFont {
            font: zh_font.clone().into(),
            font_size: FontSize::Px(56.0),
            font_smoothing: FontSmoothing::None,
            ..default()
        },
        Transform::from_xyz(220.0, -250.0, 0.0),
    ));
}
// ANCHOR_END: setup

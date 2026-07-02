//! Listing 16-9：秋白的改词手稿——TextSpan 富文本与装饰

use bevy::prelude::*;
use bevy::sprite::Text2dShadow;
use bevy::text::TextBackgroundColor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.13, 0.14, 0.18)))
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let regular = asset_server.load("fonts/book-sans-sc-regular.otf");
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");

    // 一块文本 = 根 Text2d + 一串 TextSpan 子实体。
    // 排版按整块算，但每一段各管各的字体、字号、颜色、装饰
    commands.spawn((
        Text2d::new("阿燕"),
        TextFont {
            font: bold.clone().into(),
            font_size: FontSize::Px(36.0),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.72, 0.29)),
        TextLayout::justify(Justify::Center),
        // 阴影是整块字的事，加在根上
        Text2dShadow::default(),
        children![
            (
                TextSpan::new("（提灯，望江）\n"),
                TextFont {
                    font: regular.clone().into(),
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
                TextColor(Color::srgb(0.55, 0.57, 0.62)),
            ),
            (
                TextSpan::new("夜渡无人，"),
                TextFont {
                    font: regular.clone().into(),
                    font_size: FontSize::Px(36.0),
                    ..default()
                },
                TextColor::WHITE,
            ),
            (
                // 改掉的旧词：划线作废，颜色压暗
                TextSpan::new("孤舟"),
                TextFont {
                    font: regular.clone().into(),
                    font_size: FontSize::Px(36.0),
                    ..default()
                },
                TextColor(Color::srgb(0.45, 0.45, 0.48)),
                Strikethrough,
            ),
            (
                // 换上的新词：加粗、标红、底下画金线
                TextSpan::new("秋水"),
                TextFont {
                    font: bold.clone().into(),
                    font_size: FontSize::Px(36.0),
                    ..default()
                },
                TextColor(Color::srgb(0.86, 0.32, 0.28)),
                Underline,
                UnderlineColor(Color::srgb(0.91, 0.72, 0.29)),
            ),
            (
                TextSpan::new("自横。"),
                TextFont {
                    font: regular.clone().into(),
                    font_size: FontSize::Px(36.0),
                    ..default()
                },
                TextColor::WHITE,
            ),
            (
                // 落款用默认字体（ASCII 够用），加一块底色当签条
                TextSpan::new("\nACT II - draft 3"),
                TextFont::from_font_size(18.0),
                TextColor(Color::srgb(0.75, 0.86, 0.92)),
                TextBackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
            ),
        ],
    ));

    println!("秋白：‘孤舟’划了，换‘秋水’——水比船有戏。");
}
// ANCHOR_END: setup

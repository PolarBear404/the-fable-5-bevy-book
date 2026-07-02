//! Listing 16-5：向系统借字模——system_font_discovery
//! 运行要带上门票：cargo run -p ch16-text --example listing-16-05 --features system_font_discovery

use bevy::prelude::*;
use bevy::text::FontCx;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, mut font_cx: ResMut<FontCx>) {
    commands.spawn(Camera2d);

    // FontCx 是排版引擎的字体登记簿：数数系统交来了多少家族
    let count = font_cx.collection.family_names().count();
    println!("场记：清点库房——字体家族共 {count} 个（因机而异）。");
    for source in [
        FontSource::SansSerif,
        FontSource::Serif,
        FontSource::Monospace,
        FontSource::FangSong,
        FontSource::SystemUi,
    ] {
        let family = font_cx.get_family(&source).unwrap_or("（无着落）").to_string();
        println!("  {source:?} -> {family}");
    }

    // 语义类别现在有了着落——三段文字，一个字体文件都没带
    commands.spawn((
        Text2d::new("Monospace 0123 ilI1"),
        TextFont {
            font: FontSource::Monospace,
            font_size: FontSize::Px(40.0),
            ..default()
        },
        Transform::from_xyz(0.0, 120.0, 0.0),
    ));
    commands.spawn((
        Text2d::new("仿宋体的夜渡无人"),
        TextFont {
            font: FontSource::FangSong,
            font_size: FontSize::Px(40.0),
            ..default()
        },
    ));
    // 第一节那行豆腐块的原班配置：默认字体 + 中文
    commands.spawn((
        Text2d::new("夜渡无人，秋水自横。"),
        TextFont::from_font_size(40.0),
        Transform::from_xyz(0.0, -120.0, 0.0),
    ));
}
// ANCHOR_END: setup

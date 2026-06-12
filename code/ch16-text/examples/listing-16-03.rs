//! Listing 16-3：字模也是资产——加载中文字体，词上屏

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 字体也是资产：load 拿到一张 Handle<Font> 提货单
    let zh_font = asset_server.load("fonts/book-sans-sc-regular.otf");

    commands.spawn((
        Text2d::new("夜渡无人，秋水自横。"),
        TextFont {
            font: zh_font,
            font_size: 48.0,
            ..default()
        },
    ));

    // 对照行：不指定 TextFont 就用内置默认字体（只有 95 个 ASCII 字形）
    commands.spawn((
        Text2d::new("THE NIGHT FERRY"),
        Transform::from_xyz(0.0, -80.0, 0.0),
    ));

    println!("老雷：字模进库房了，跟道具一个走法——先开提货单。");
}
// ANCHOR_END: setup

//! Listing 16-14：台上的字与玻璃上的字——Text2d vs UI 的 Text

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, drift_camera)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let zh_font = asset_server.load("fonts/book-sans-sc-regular.otf");

    // 台上的字：Text2d 活在世界坐标里，镜头一动它就跟着画面走
    commands.spawn((
        Text2d::new("渡口——台上的字"),
        TextFont {
            font: zh_font.clone().into(),
            font_size: FontSize::Px(40.0),
            ..default()
        },
    ));

    // 玻璃上的字：UI 的 Text 钉在屏幕坐标里，镜头怎么晃都不动。
    // 样式三件套（TextFont/TextColor/TextLayout）原封不动，
    // 只是定位从 Transform + Anchor 换成了 Node（第 28 章的主角）
    commands.spawn((
        Text::new("第二幕 · 夜战——玻璃上的字"),
        TextFont {
            font: zh_font.clone().into(),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}
// ANCHOR_END: setup

/// 镜头慢慢来回摇——看两种字谁跟着动
fn drift_camera(time: Res<Time>, mut camera: Single<&mut Transform, With<Camera2d>>) {
    camera.translation.x = 120.0 * (time.elapsed_secs() * 0.7).sin();
}

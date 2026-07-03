//! Listing 16-7：字号阶梯与"放大照片"——font_size 不是缩放

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
    let zh_font = asset_server.load("fonts/book-sans-sc-regular.otf");

    // 字号阶梯：font_size 是"按多大字号刻字模"
    let mut y = 240.0;
    for size in [16.0, 24.0, 40.0, 64.0] {
        commands.spawn((
            Text2d::new(format!("渡口夜话 {size} 号")),
            TextFont {
                font: zh_font.clone().into(),
                font_size: FontSize::Px(size),
                ..default()
            },
            Transform::from_xyz(0.0, y, 0.0),
        ));
        y -= 30.0 + size;
    }

    // 对照组：同样的视觉大小，左边用 64 号字模，右边用 16 号字模放大 4 倍
    commands.spawn((
        Text2d::new("会心"),
        TextFont {
            font: zh_font.clone().into(),
            font_size: FontSize::Px(64.0),
            ..default()
        },
        Transform::from_xyz(-160.0, -240.0, 0.0),
    ));
    commands.spawn((
        Text2d::new("会心"),
        TextFont {
            font: zh_font.clone().into(),
            font_size: FontSize::Px(16.0),
            ..default()
        },
        // 把 16 号小字模硬放大 4 倍——像把小照片拉成海报
        Transform::from_xyz(160.0, -240.0, 0.0).with_scale(Vec3::splat(4.0)),
    ));

    println!("小棠：左边是 64 号字模，右边是 16 号字模放大四倍——凑近看。");
}
// ANCHOR_END: setup

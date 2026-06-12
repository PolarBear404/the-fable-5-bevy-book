//! Listing 15-2：试装台——Nearest 采样救回像素，再把 Sprite 的字段挨个试一遍

use bevy::prelude::*;

fn main() {
    App::new()
        // ANCHOR: nearest
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // ANCHOR_END: nearest
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.13)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let still = asset_server.load("actors/ayan-still.png");
    let six_times = Some(Vec2::new(32.0, 40.0) * 6.0);

    // ANCHOR: variants
    // 一号位：原样放大六倍
    commands.spawn((
        Sprite {
            image: still.clone(),
            custom_size: six_times,
            ..default()
        },
        Transform::from_xyz(-440.0, 0.0, 0.0),
    ));

    // 二号位：color 染色——整张画与这块颜色逐像素相乘
    commands.spawn((
        Sprite {
            image: still.clone(),
            custom_size: six_times,
            color: Color::srgb(0.55, 0.70, 1.00),
            ..default()
        },
        Transform::from_xyz(-220.0, 0.0, 0.0),
    ));

    // 三号位：flip_x 水平翻转——剑柄换了边
    commands.spawn((
        Sprite {
            image: still.clone(),
            custom_size: six_times,
            flip_x: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 四号位：rect 只取原图的一块——裁一张头像（单位是原图像素）
    commands.spawn((
        Sprite {
            image: still.clone(),
            custom_size: Some(Vec2::new(14.0, 15.0) * 8.0),
            rect: Some(Rect::new(9.0, 0.0, 23.0, 15.0)),
            ..default()
        },
        Transform::from_xyz(220.0, 0.0, 0.0),
    ));

    // 五号位：custom_size 不按比例硬拉——人被抻成了瘦高个
    commands.spawn((
        Sprite {
            image: still,
            custom_size: Some(Vec2::new(110.0, 330.0)),
            ..default()
        },
        Transform::from_xyz(440.0, 0.0, 0.0),
    ));
    // ANCHOR_END: variants

    println!("小棠：一号位原样放大，这回锐了。");
    println!("小棠：二号位罩了层月光色，三号位翻了个面。");
    println!("小棠：四号位裁头像，五号位硬抻——最后这张是反面教材。");
}

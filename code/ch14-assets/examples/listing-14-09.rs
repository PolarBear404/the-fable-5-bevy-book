//! Listing 14-9：像素海报糊了——默认线性采样、.meta 档案与 load_builder

use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.08, 0.1)))
        .add_systems(Startup, print_posters)
        .run();
}

// ANCHOR: posters
fn print_posters(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 左：16×16 像素原稿，默认设置（线性采样）放大十六倍——糊成一团
    commands.spawn((
        Sprite {
            image: asset_server.load("props/sword-16.png"),
            custom_size: Some(Vec2::splat(256.0)),
            ..default()
        },
        Transform::from_xyz(-300.0, 0.0, 0.0),
    ));

    // 中：同一张稿的副本，旁边躺着一份同名 .meta 档案，里面把采样写成 Nearest
    commands.spawn((
        Sprite {
            image: asset_server.load("props/sword-16-meta.png"),
            custom_size: Some(Vec2::splat(256.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 右：下单时现场改设置——load_builder 挂上 with_settings，只动要动的字段
    commands.spawn((
        Sprite {
            image: asset_server
                .load_builder()
                .with_settings(|settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                })
                .load("props/sword-16-settings.png"),
            custom_size: Some(Vec2::splat(256.0)),
            ..default()
        },
        Transform::from_xyz(300.0, 0.0, 0.0),
    ));

    println!("老顾：同一张十六格的剑稿放大十六倍，三种洗法——");
    println!("老顾：左边默认线性，糊；中间走 .meta 档案，利；右边下单现改设置，一样利。");
}
// ANCHOR_END: posters

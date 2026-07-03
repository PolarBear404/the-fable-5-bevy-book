//! Listing 19-2：循环 BGM——PlaybackSettings 决定一段声音怎么播

use bevy::audio::Volume;
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
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/changfeng-overture.wav")),
        // 不再用默认的 ONCE：循环播放，音量压到六成给台词留地方
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.6)),
    ));
    println!("琴师：序曲上循环——尾音接得上头一拍，奏到散场为止。");
    // ANCHOR_END: setup

    commands.spawn((
        Text2d::new("《长风渡》序曲 · 循环中"),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-bold.otf").into(),
            font_size: FontSize::Px(36.0),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.88, 0.80)),
    ));
}

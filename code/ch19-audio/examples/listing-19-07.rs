//! Listing 19-7：音量台——sink 的旋钮与 GlobalVolume 总闸
//! +/- 调 BGM 音量；G 敲锣；V 拧总闸（看清它只管新开播的声音）

use bevy::audio::Volume;
use bevy::prelude::*;

#[derive(Component)]
struct Bgm;

#[derive(Resource)]
struct GongSound(Handle<AudioSource>);

#[derive(Component)]
struct Hud;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .add_systems(Startup, setup)
        .add_systems(Update, (tune_strings, strike_gong, master_dial, hud))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Bgm,
        AudioPlayer::new(asset_server.load("music/changfeng-overture.wav")),
        PlaybackSettings::LOOP,
    ));
    commands.insert_resource(GongSound(asset_server.load("sfx/gong.wav")));
    commands.spawn((
        Hud,
        Text2d::new(""),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-bold.otf").into(),
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.88, 0.80)),
        Transform::from_xyz(0.0, 40.0, 5.0),
    ));
    commands.spawn((
        Text2d::new("+/- 调曲子音量　　G 敲锣　　V 拧总闸"),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(0.55, 0.57, 0.62)),
        Transform::from_xyz(0.0, -40.0, 5.0),
    ));
    println!("老雷：合成开始——曲子的音量琴师管，全场的总闸我管。");
}

// ANCHOR: tune
/// 琴师的旋钮：在播的声音，音量拧在 AudioSink 上
fn tune_strings(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut strings: Query<&mut AudioSink, With<Bgm>>,
) {
    let Ok(mut sink) = strings.single_mut() else {
        return;
    };
    let step = i32::from(keyboard.just_pressed(KeyCode::Equal))
        - i32::from(keyboard.just_pressed(KeyCode::Minus));
    if step != 0 {
        let turned = sink.volume().increase_by_percentage(step as f32 * 25.0);
        sink.set_volume(turned);
        println!(
            "琴师：拧到线性 {:.2}（{:+.1} dB）。",
            turned.to_linear(),
            turned.to_decibels(),
        );
    }
}
// ANCHOR_END: tune

// ANCHOR: master
/// 老雷的总闸：GlobalVolume 只在“开播那一刻”乘进去——在播的纹丝不动
fn master_dial(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut master: ResMut<GlobalVolume>,
    strings: Query<&AudioSink, With<Bgm>>,
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        master.volume = if master.volume == Volume::Linear(1.0) {
            Volume::Linear(0.25)
        } else {
            Volume::Linear(1.0)
        };
        println!("老雷：总闸拧到 {:.2}。", master.volume.to_linear());
        if let Ok(sink) = strings.single() {
            println!(
                "场记：在播的曲子 sink.volume() 还是 {:.2}——总闸没碰它，只管下一声。",
                sink.volume().to_linear(),
            );
        }
    }
}
// ANCHOR_END: master

fn strike_gong(
    keyboard: Res<ButtonInput<KeyCode>>,
    gong: Res<GongSound>,
    master: Res<GlobalVolume>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        commands.spawn((
            AudioPlayer::new(gong.0.clone()),
            PlaybackSettings::DESPAWN,
        ));
        println!(
            "鼓师：哐——（开播音量 = 1.00 × 总闸 {:.2} = {:.2}）",
            master.volume.to_linear(),
            master.volume.to_linear(),
        );
    }
}

fn hud(
    master: Res<GlobalVolume>,
    strings: Query<&AudioSink, With<Bgm>>,
    mut text: Single<&mut Text2d, With<Hud>>,
) {
    let volume = strings
        .single()
        .map(|sink| sink.volume())
        .unwrap_or(Volume::SILENT);
    text.0 = format!(
        "曲子音量 线性 {:.2} / {:+.1} dB　　总闸 {:.2}",
        volume.to_linear(),
        volume.to_decibels(),
        master.volume.to_linear(),
    );
}

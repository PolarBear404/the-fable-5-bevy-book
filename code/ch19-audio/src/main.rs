//! Listing 19-9：《长风渡》首演——文武场全编制
//! 琴师奏循环序曲，阿燕提梆子巡台（空间音频），台口亮相与剑花走消息敲锣打鼓；
//! 空格 = 中场/开戏（戏台钟与全部声音一起停），J = 剑花，+/- = 曲子音量。

use bevy::audio::{AudioPlugin, SpatialScale, Volume};
use bevy::prelude::*;
use bevy::sprite::Anchor;

const AUDIO_SCALE: f32 = 1.0 / 100.0;
const EAR_GAP: f32 = 400.0;
const LIMIT: f32 = 420.0;

// ANCHOR: cue
/// 武场的戏单：谁要锣鼓，投一条消息——第 7 章的解耦，原样照搬
#[derive(Message)]
enum Cue {
    /// 台口亮相：锣
    Flourish,
    /// 剑花：鼓
    Sword,
}
// ANCHOR_END: cue

#[derive(Component)]
struct Bgm;

#[derive(Component)]
struct Patrol {
    velocity: f32,
}

#[derive(Component)]
struct FrameClock(Timer);

#[derive(Component)]
struct Hud;

#[derive(Resource)]
struct Percussion {
    gong: Handle<AudioSource>,
    drum: Handle<AudioSource>,
}

// ANCHOR: app
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AudioPlugin {
                    default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .add_message::<Cue>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (patrol, call_sword, play_percussion).chain(),
                intermission,
                tune_strings,
                animate,
                hud,
            ),
        )
        .run();
}
// ANCHOR_END: app

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // 文场：琴师的循环序曲，音量先压到六成
    commands.spawn((
        Bgm,
        AudioPlayer::new(asset_server.load("music/changfeng-overture.wav")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.6)),
    ));
    // 武场的家伙什：锣与鼓的提货单
    commands.insert_resource(Percussion {
        gong: asset_server.load("sfx/gong.wav"),
        drum: asset_server.load("sfx/drum.wav"),
    });

    // 台板
    commands.spawn((
        Sprite {
            image: asset_server.load("props/dock-plank.png"),
            custom_size: Some(Vec2::new(1400.0, 56.0)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 4.0,
            },
            ..default()
        },
        Transform::from_xyz(0.0, -148.0, 0.5),
    ));

    // 阿燕：巡台的发声体——梆子是开了 spatial 的循环声
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        6,
        2,
        None,
        None,
    ));
    commands.spawn((
        Patrol { velocity: 120.0 },
        FrameClock(Timer::from_seconds(0.15, TimerMode::Repeating)),
        Sprite::from_atlas_image(
            asset_server.load("actors/ayan-sheet.png"),
            TextureAtlas { layout, index: 6 },
        ),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(-LIMIT, -120.0, 2.0).with_scale(Vec3::splat(4.0)),
        AudioPlayer::new(asset_server.load("sfx/bangzi-loop.wav")),
        PlaybackSettings::LOOP.with_spatial(true),
    ));

    // 观众席正中的一对耳朵
    commands.spawn((
        SpatialListener::new(EAR_GAP),
        Transform::from_xyz(0.0, -260.0, 0.0),
        Visibility::default(),
        children![
            (
                Sprite::from_color(Color::srgb(0.86, 0.45, 0.40), Vec2::splat(30.0)),
                Transform::from_xyz(-EAR_GAP / 2.0, 0.0, 1.0),
            ),
            (
                Sprite::from_color(Color::srgb(0.42, 0.76, 0.70), Vec2::splat(30.0)),
                Transform::from_xyz(EAR_GAP / 2.0, 0.0, 1.0),
            ),
        ],
    ));

    commands.spawn((
        Hud,
        Text2d::new(""),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-bold.otf"),
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.88, 0.80)),
        Transform::from_xyz(0.0, 300.0, 5.0),
    ));
    commands.spawn((
        Text2d::new("空格 中场/开戏　　J 剑花　　+/- 曲子音量"),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-regular.otf"),
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::srgb(0.55, 0.57, 0.62)),
        Transform::from_xyz(0.0, 244.0, 5.0),
    ));

    println!("老雷：《长风渡》首演。文场起曲，武场听 cue，阿燕巡台。");
}

// ANCHOR: writers
/// 写者一：阿燕的台步——走到台口亮相，向武场投一条 cue
fn patrol(
    time: Res<Time>,
    mut patrols: Query<(&mut Transform, &mut Patrol)>,
    mut cues: MessageWriter<Cue>,
) {
    for (mut transform, mut patrol) in &mut patrols {
        transform.translation.x += patrol.velocity * time.delta_secs();
        if transform.translation.x.abs() > LIMIT {
            transform.translation.x = transform.translation.x.clamp(-LIMIT, LIMIT);
            patrol.velocity = -patrol.velocity;
            cues.write(Cue::Flourish);
        }
    }
}

/// 写者二：剑花的 cue 从键盘来——写者彼此不相识，读者只有一个
fn call_sword(keyboard: Res<ButtonInput<KeyCode>>, mut cues: MessageWriter<Cue>) {
    if keyboard.just_pressed(KeyCode::KeyJ) {
        cues.write(Cue::Sword);
    }
}

/// 读者：武场。听见什么 cue，敲什么家伙——一声一个 DESPAWN 实体
fn play_percussion(
    mut cues: MessageReader<Cue>,
    percussion: Res<Percussion>,
    mut commands: Commands,
) {
    for cue in cues.read() {
        let (sound, line) = match cue {
            Cue::Flourish => (&percussion.gong, "武场：台口亮相——哐！"),
            Cue::Sword => (&percussion.drum, "武场：剑花——咚！"),
        };
        commands.spawn((
            AudioPlayer::new(sound.clone()),
            PlaybackSettings::DESPAWN,
        ));
        println!("{line}");
    }
}
// ANCHOR_END: writers

// ANCHOR: intermission
/// 中场：戏台钟与台上所有声音一起停——两套开关在同一个系统里各拧各的
fn intermission(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
    plain_sinks: Query<&AudioSink>,
    spatial_sinks: Query<&SpatialAudioSink>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if time.is_paused() {
            time.unpause();
            plain_sinks.iter().for_each(AudioSink::play);
            spatial_sinks.iter().for_each(SpatialAudioSink::play);
            println!("老雷：开戏——钟、曲、更声一起回来。");
        } else {
            time.pause();
            plain_sinks.iter().for_each(AudioSink::pause);
            spatial_sinks.iter().for_each(SpatialAudioSink::pause);
            println!("老雷：中场——这回连琴带梆子全歇。");
        }
    }
}
// ANCHOR_END: intermission

/// 琴师的旋钮（与 Listing 19-7 同款）
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
        println!("琴师：曲子拧到线性 {:.2}。", turned.to_linear());
    }
}

fn animate(time: Res<Time>, mut patrols: Query<(&mut Sprite, &mut FrameClock, &Patrol)>) {
    for (mut sprite, mut clock, patrol) in &mut patrols {
        if clock.0.tick(time.delta()).just_finished()
            && let Some(atlas) = sprite.texture_atlas.as_mut()
        {
            atlas.index = if atlas.index >= 11 { 6 } else { atlas.index + 1 };
        }
        sprite.flip_x = patrol.velocity < 0.0;
    }
}

fn hud(
    stage: Res<Time<Virtual>>,
    strings: Query<&AudioSink, With<Bgm>>,
    patrols: Query<&Transform, With<Patrol>>,
    mut text: Single<&mut Text2d, With<Hud>>,
) {
    let (position, volume) = match strings.single() {
        Ok(sink) => (sink.position().as_secs_f32(), sink.volume().to_linear()),
        Err(_) => (0.0, 0.0),
    };
    let where_is_ayan = patrols
        .single()
        .map(|t| t.translation.x)
        .unwrap_or_default();
    text.0 = format!(
        "{}　　曲子 {position:5.1} 秒 / 音量 {volume:.2}　　阿燕 x = {where_is_ayan:>4.0}",
        if stage.is_paused() { "—— 中场 ——" } else { "—— 开演 ——" },
    );
}

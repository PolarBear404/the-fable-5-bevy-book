//! Listing 19-6：中场两停对照——戏台钟一停人定格，曲子却照奏
//! 空格 = 中场/开戏（拧 Time<Virtual>）；P = 琴师压弦/续奏（拧 AudioSink）

use bevy::prelude::*;
use bevy::sprite::Anchor;

const LIMIT: f32 = 420.0;

#[derive(Component)]
struct Bgm;

#[derive(Component)]
struct Walker {
    velocity: f32,
}

#[derive(Component)]
struct FrameClock(Timer);

#[derive(Component)]
struct Hud;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .add_systems(Startup, setup)
        .add_systems(Update, (intermission, silence_strings, walk, animate, hud))
        .run();
}

// ANCHOR: two_pauses
/// 老雷的中场：拧的是戏台钟——台上的人应声定格
fn intermission(keyboard: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        if time.is_paused() {
            time.unpause();
            println!("老雷：开戏。");
        } else {
            time.pause();
            println!("老雷：中场——台上都歇了。听听，琴怎么还在响？");
        }
    }
}

/// 琴师的中场：拧的是 AudioSink——曲子才真正停下
fn silence_strings(
    keyboard: Res<ButtonInput<KeyCode>>,
    strings: Query<&AudioSink, With<Bgm>>,
) {
    let Ok(sink) = strings.single() else {
        return; // 开播之前还没有 sink，等它上岗
    };
    if keyboard.just_pressed(KeyCode::KeyP) {
        sink.toggle_playback();
        if sink.is_paused() {
            println!("琴师：压弦。（sink.is_paused = {}）", sink.is_paused());
        } else {
            println!("琴师：续上，进度一秒没丢。");
        }
    }
}
// ANCHOR_END: two_pauses

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Bgm,
        AudioPlayer::new(asset_server.load("music/changfeng-overture.wav")),
        PlaybackSettings::LOOP,
    ));

    // 台板与走台的阿燕（速度乘 delta——戏台钟一停她就定格）
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
        Transform::from_xyz(0.0, -208.0, 0.5),
    ));
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        6,
        2,
        None,
        None,
    ));
    commands.spawn((
        Walker { velocity: 180.0 },
        FrameClock(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Sprite::from_atlas_image(
            asset_server.load("actors/ayan-sheet.png"),
            TextureAtlas { layout, index: 6 },
        ),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(0.0, -180.0, 2.0).with_scale(Vec3::splat(4.0)),
    ));

    commands.spawn((
        Hud,
        Text2d::new(""),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-bold.otf").into(),
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.88, 0.80)),
        Transform::from_xyz(0.0, 270.0, 5.0),
    ));
    commands.spawn((
        Text2d::new("空格 中场/开戏（戏台钟）　　P 压弦/续奏（AudioSink）"),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(0.55, 0.57, 0.62)),
        Transform::from_xyz(0.0, 210.0, 5.0),
    ));
    println!("老雷：空格中场。P 是琴师的活——两个开关，各管各的。");
}

fn walk(time: Res<Time>, mut walkers: Query<(&mut Transform, &mut Walker)>) {
    for (mut transform, mut walker) in &mut walkers {
        transform.translation.x += walker.velocity * time.delta_secs();
        if transform.translation.x.abs() > LIMIT {
            transform.translation.x = transform.translation.x.clamp(-LIMIT, LIMIT);
            walker.velocity = -walker.velocity;
        }
    }
}

fn animate(time: Res<Time>, mut walkers: Query<(&mut Sprite, &mut FrameClock, &Walker)>) {
    for (mut sprite, mut clock, walker) in &mut walkers {
        if clock.0.tick(time.delta()).just_finished()
            && let Some(atlas) = sprite.texture_atlas.as_mut()
        {
            atlas.index = if atlas.index >= 11 { 6 } else { atlas.index + 1 };
        }
        sprite.flip_x = walker.velocity < 0.0;
    }
}

// ANCHOR: hud
/// 读数牌：戏台钟与曲子进度并排走——中场后谁停谁不停，一眼便知
fn hud(
    stage: Res<Time<Virtual>>,
    strings: Query<&AudioSink, With<Bgm>>,
    mut text: Single<&mut Text2d, With<Hud>>,
) {
    let (position, paused) = match strings.single() {
        Ok(sink) => (sink.position().as_secs_f32(), sink.is_paused()),
        Err(_) => (0.0, false),
    };
    text.0 = format!(
        "戏台钟 {:5.1} 秒（{}）　　曲子 {:5.1} 秒（{}）",
        stage.elapsed_secs(),
        if stage.is_paused() { "停" } else { "走" },
        position,
        if paused { "停" } else { "奏" },
    );
}
// ANCHOR_END: hud

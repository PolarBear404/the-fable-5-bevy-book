//! Listing 19-8：巡夜——空间音频：双耳听者、移动声源与距离衰减
//! 阿燕提着梆子左右巡台；你坐在台中，左耳红、右耳青

use bevy::audio::{AudioPlugin, SpatialScale};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

/// 100 像素折合 1 个“音频单位”——衰减公式里的距离用的是换算后的数
const AUDIO_SCALE: f32 = 1.0 / 100.0;
/// 双耳间距（像素）：换算后 4 个音频单位
const EAR_GAP: f32 = 400.0;
const LIMIT: f32 = 420.0;

#[derive(Component)]
struct Patrol {
    velocity: f32,
}

#[derive(Component)]
struct FrameClock(Timer);

#[derive(Component)]
struct Hud;

// ANCHOR: app
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AudioPlugin {
            // 全局换算尺：2D 里 1 像素 = 1 单位太大，按 100 像素 1 单位缩
            default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                patrol,
                animate,
                hud,
                night_report.run_if(on_timer(Duration::from_secs(2))),
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

    // ANCHOR: spatial_cast
    // 发声体：阿燕与梆子——开了 spatial 的循环声源，挂在会动的实体上
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
        Transform::from_xyz(-LIMIT, -20.0, 2.0).with_scale(Vec3::splat(4.0)),
        AudioPlayer::new(asset_server.load("sfx/bangzi-loop.wav")),
        PlaybackSettings::LOOP.with_spatial(true),
    ));

    // 听者：一对耳朵，左红右青——空间音频只认这对耳朵的位置
    commands.spawn((
        SpatialListener::new(EAR_GAP),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::default(),
        children![
            (
                Sprite::from_color(Color::srgb(0.86, 0.45, 0.40), Vec2::splat(36.0)),
                Transform::from_xyz(-EAR_GAP / 2.0, 0.0, 1.0),
            ),
            (
                Sprite::from_color(Color::srgb(0.42, 0.76, 0.70), Vec2::splat(36.0)),
                Transform::from_xyz(EAR_GAP / 2.0, 0.0, 1.0),
            ),
        ],
    ));
    // ANCHOR_END: spatial_cast

    for (text, y, size) in [
        ("巡夜：梆子声跟着阿燕走——过左耳偏左响，走远了轻", 300.0, 26.0),
        ("左红右青是你的两只耳朵", 254.0, 22.0),
    ] {
        commands.spawn((
            Text2d::new(text),
            TextFont {
                font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
                font_size: FontSize::Px(size),
                ..default()
            },
            TextColor(Color::srgb(0.55, 0.57, 0.62)),
            Transform::from_xyz(0.0, y, 5.0),
        ));
    }
    commands.spawn((
        Hud,
        Text2d::new(""),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-bold.otf").into(),
            font_size: FontSize::Px(26.0),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.88, 0.80)),
        Transform::from_xyz(0.0, -200.0, 5.0),
    ));
    println!("老雷：熄灯，巡夜。阿燕打更，你们坐台中听远近。");
}

fn patrol(time: Res<Time>, mut patrols: Query<(&mut Transform, &mut Patrol)>) {
    for (mut transform, mut patrol) in &mut patrols {
        transform.translation.x += patrol.velocity * time.delta_secs();
        if transform.translation.x.abs() > LIMIT {
            transform.translation.x = transform.translation.x.clamp(-LIMIT, LIMIT);
            patrol.velocity = -patrol.velocity;
        }
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

// ANCHOR: gains
/// 按 rodio 源码的公式复算双耳增益：声像系数 × min(1/距离², 1)
fn ear_gains(emitter: Vec2, listener: &SpatialListener, origin: Vec2) -> (f32, f32) {
    let left = (origin + listener.left_ear_offset.truncate()) * AUDIO_SCALE;
    let right = (origin + listener.right_ear_offset.truncate()) * AUDIO_SCALE;
    let emitter = emitter * AUDIO_SCALE;
    let (dl, dr) = (left.distance(emitter), right.distance(emitter));
    let gap = left.distance(right);
    let pan = |near: f32, far: f32| (((near - far) / gap + 1.0) / 4.0 + 0.5).min(1.0);
    let dist = |d: f32| (1.0 / (d * d)).min(1.0);
    (pan(dl, dr) * dist(dl), pan(dr, dl) * dist(dr))
}
// ANCHOR_END: gains

// ANCHOR: report
/// 场记的更声台账：每两秒一笔——方位、距离、双耳各自的衰减
fn night_report(
    patrols: Query<(&Transform, &SpatialAudioSink), With<Patrol>>,
    listener: Single<(&Transform, &SpatialListener)>,
) {
    let Ok((walker, sink)) = patrols.single() else {
        return; // SpatialAudioSink 同样要等开播才上岗
    };
    let (listener_pos, ears) = *listener;
    let emitter = walker.translation.truncate();
    let (left, right) = ear_gains(emitter, ears, listener_pos.translation.truncate());
    let side = if emitter.x < -40.0 {
        "左舷"
    } else if emitter.x > 40.0 {
        "右舷"
    } else {
        "正中"
    };
    println!(
        "场记：阿燕在{side} {:>4.0} 像素处，左耳 {:.2}、右耳 {:.2}（sink 暂停 {}）。",
        emitter.x.abs(),
        left,
        right,
        sink.is_paused(),
    );
}
// ANCHOR_END: report

fn hud(
    patrols: Query<&Transform, With<Patrol>>,
    listener: Single<&Transform, With<SpatialListener>>,
    mut text: Single<&mut Text2d, With<Hud>>,
) {
    let Ok(walker) = patrols.single() else {
        return;
    };
    let pixels = walker.translation.truncate().distance(listener.translation.truncate());
    text.0 = format!(
        "声源距台心 {pixels:>4.0} 像素 = {:.1} 音频单位",
        pixels * AUDIO_SCALE,
    );
}

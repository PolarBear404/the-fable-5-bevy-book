//! Listing 15-4：走马灯——0.1 秒一帧循环走路格，阿燕在台上来回走

use bevy::prelude::*;

/// 走路动画占图集的第 6..=11 格
const WALK_FIRST: usize = 6;
const WALK_LAST: usize = 11;
/// 台口：走到 ±420 就转身
const STAGE_EDGE: f32 = 420.0;

// ANCHOR: components
/// 帧动画的节拍与范围：每隔 interval 进一帧，走完 last 回到 first
#[derive(Component)]
struct FrameClock {
    timer: Timer,
    first: usize,
    last: usize,
}

/// 台上的走位：速度为正朝右、为负朝左
#[derive(Component)]
struct Pacing {
    speed: f32,
}
// ANCHOR_END: components

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.13)))
        .add_systems(Startup, setup)
        .add_systems(Update, (advance_frames, pace_the_stage))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // 台板：一条供脚底参照的木色长带
    commands.spawn((
        Sprite::from_color(Color::srgb(0.30, 0.24, 0.18), Vec2::new(1100.0, 8.0)),
        Transform::from_xyz(0.0, -124.0, 0.0),
    ));

    // ANCHOR: spawn
    let sheet = asset_server.load("actors/ayan-sheet.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        6,
        2,
        None,
        None,
    ));

    commands.spawn((
        Sprite::from_atlas_image(
            sheet,
            TextureAtlas {
                layout,
                index: WALK_FIRST,
            },
        ),
        Transform::from_xyz(-STAGE_EDGE, -40.0, 1.0).with_scale(Vec3::splat(4.0)),
        FrameClock {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            first: WALK_FIRST,
            last: WALK_LAST,
        },
        Pacing { speed: 180.0 },
    ));
    // ANCHOR_END: spawn

    println!("老雷：合上原稿，走两步。");
}

// ANCHOR: advance
/// 节拍一到就拨下一帧：first → … → last → first，周而复始
fn advance_frames(time: Res<Time>, mut query: Query<(&mut FrameClock, &mut Sprite)>) {
    for (mut clock, mut sprite) in &mut query {
        if !clock.timer.tick(time.delta()).just_finished() {
            continue;
        }
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index == clock.last {
                clock.first
            } else {
                atlas.index + 1
            };
        }
    }
}
// ANCHOR_END: advance

// ANCHOR: pace
/// 来回走位：撞到台口就调头，转身就是把 flip_x 翻个面
fn pace_the_stage(
    time: Res<Time>,
    mut query: Query<(&mut Pacing, &mut Transform, &mut Sprite)>,
    mut turns: Local<u32>,
) {
    for (mut pacing, mut transform, mut sprite) in &mut query {
        transform.translation.x += pacing.speed * time.delta_secs();

        if transform.translation.x.abs() > STAGE_EDGE {
            transform.translation.x = STAGE_EDGE.copysign(transform.translation.x);
            pacing.speed = -pacing.speed;
            // 原画朝右；向左走时翻面
            sprite.flip_x = pacing.speed < 0.0;

            if *turns < 2 {
                *turns += 1;
                let side = if sprite.flip_x { "东" } else { "西" };
                println!("场记：到{}头，转身。", side);
            }
        }
    }
}
// ANCHOR_END: pace

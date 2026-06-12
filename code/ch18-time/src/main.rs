//! Listing 18-10：《赶月》——同一套鼓点结算，替身直写 Transform，阿燕在两拍之间插值
//! 空格中场/开戏，↑↓ 给戏台钟换挡（慢放时替身顿得更凶，阿燕照样丝滑）。

use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::time::common_conditions::on_real_timer;

const FLOOR: f32 = -270.0;
const BACK_LANE: f32 = -40.0;
const LIMIT: f32 = 420.0;
const SPEED: f32 = 180.0;

// ANCHOR: components
/// 巡台的人：速度与“账上的位置”——上一拍在哪、这一拍在哪
#[derive(Component)]
struct Walker {
    velocity: f32,
}

#[derive(Component)]
struct StagePos {
    current: f32,
    previous: f32,
}

/// 替身：鼓点上挪一次（直写 Transform）
#[derive(Component)]
struct Direct;
/// 阿燕：每帧在两拍之间找步子（插值）
#[derive(Component)]
struct Interpolated;
// ANCHOR_END: components

#[derive(Component)]
struct FrameClock(Timer);

#[derive(Component)]
struct Hud;

/// 折返的趟数（场记的台账）
#[derive(Resource, Default)]
struct Laps(u32);

// ANCHOR: app
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .insert_resource(Time::<Fixed>::from_hz(4.0)) // 慢板鼓点，顿挫与丝滑一目了然
        .init_resource::<Laps>()
        .add_systems(Startup, setup)
        // 鼓点上：结算所有人的账，替身随手把账写上画面
        .add_systems(FixedUpdate, (advance, snap_direct).chain())
        // 鼓点之后、每帧一次：阿燕按 overstep 在两拍之间找步子
        .add_systems(
            RunFixedMainLoop,
            glide.in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
        )
        .add_systems(
            Update,
            (
                conduct,
                animate,
                hud.run_if(on_real_timer(Duration::from_millis(100))),
            ),
        )
        .run();
}
// ANCHOR_END: app

// ANCHOR: advance
/// 鼓点上的结算：先把“这一拍”挪进“上一拍”，再走新的一步——两本账都是给插值留的
fn advance(
    time: Res<Time>,
    mut walkers: Query<(&mut StagePos, &mut Walker, Option<&Interpolated>)>,
    mut laps: ResMut<Laps>,
) {
    for (mut pos, mut walker, interpolated) in &mut walkers {
        pos.previous = pos.current;
        pos.current += walker.velocity * time.delta_secs();
        if pos.current.abs() > LIMIT {
            pos.current = pos.current.clamp(-LIMIT, LIMIT);
            walker.velocity = -walker.velocity;
            if interpolated.is_some() {
                laps.0 += 1;
                println!("场记：第 {} 趟走完，折返。", laps.0);
            }
        }
    }
}

/// 替身：账上是多少，画面就是多少——一秒只动四次
fn snap_direct(mut walkers: Query<(&mut Transform, &StagePos), With<Direct>>) {
    for (mut transform, pos) in &mut walkers {
        transform.translation.x = pos.current;
    }
}

/// 阿燕：离下一拍还差几分（overstep_fraction），步子就迈到两拍之间的几分处
fn glide(
    fixed: Res<Time<Fixed>>,
    mut walkers: Query<(&mut Transform, &StagePos), With<Interpolated>>,
) {
    let alpha = fixed.overstep_fraction();
    for (mut transform, pos) in &mut walkers {
        transform.translation.x = pos.previous.lerp(pos.current, alpha);
    }
}
// ANCHOR_END: advance

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");
    let regular = asset_server.load("fonts/book-sans-sc-regular.otf");

    // 前后两条台板
    for (y, z) in [(FLOOR, 1.0), (BACK_LANE, 0.5)] {
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
            Transform::from_xyz(0.0, y - 28.0, z),
        ));
    }

    // ANCHOR: cast
    // 同一本连环画：后排替身（直写，灰青调），前排阿燕（插值，本色）
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        6,
        2,
        None,
        None,
    ));
    let sheet = asset_server.load("actors/ayan-sheet.png");
    commands.spawn((
        Direct,
        Walker { velocity: SPEED },
        StagePos {
            current: -LIMIT,
            previous: -LIMIT,
        },
        FrameClock(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Sprite {
            color: Color::srgb(0.62, 0.72, 0.86),
            ..Sprite::from_atlas_image(
                sheet.clone(),
                TextureAtlas {
                    layout: layout.clone(),
                    index: 6,
                },
            )
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(-LIMIT, BACK_LANE, 2.0).with_scale(Vec3::splat(4.0)),
    ));
    commands.spawn((
        Interpolated,
        Walker { velocity: SPEED },
        StagePos {
            current: -LIMIT,
            previous: -LIMIT,
        },
        FrameClock(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Sprite::from_atlas_image(sheet, TextureAtlas { layout, index: 6 }),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(-LIMIT, FLOOR, 3.0).with_scale(Vec3::splat(4.0)),
    ));
    // ANCHOR_END: cast

    // 台口名牌与 HUD
    for (y, label, color) in [
        (BACK_LANE + 180.0, "替身（直写）", Color::srgb(0.62, 0.72, 0.86)),
        (FLOOR + 180.0, "阿燕（插值）", Color::srgb(0.91, 0.72, 0.29)),
    ] {
        commands.spawn((
            Text2d::new(label),
            TextFont {
                font: bold.clone(),
                font_size: 26.0,
                ..default()
            },
            TextColor(color),
            Anchor::CENTER_LEFT,
            Transform::from_xyz(-620.0, y, 5.0),
        ));
    }
    commands.spawn((
        Hud,
        Text2d::new(""),
        TextFont {
            font: bold,
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.88, 0.80)),
        Transform::from_xyz(0.0, 320.0, 5.0),
    ));
    commands.spawn((
        Text2d::new("空格 中场/开戏　　↑↓ 换挡　　鼓点 4 拍/秒"),
        TextFont {
            font: regular,
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::srgb(0.55, 0.57, 0.62)),
        Transform::from_xyz(0.0, 252.0, 5.0),
    ));

    println!("老雷：《赶月》走台——替身踩着鼓点挪，阿燕在鼓点之间找步子。");
    println!("场记：空格中场/开戏，↑↓ 换挡。慢放时替身顿得更凶，阿燕照样丝滑。");
}

const SPEEDS: [f32; 5] = [0.25, 0.5, 1.0, 2.0, 4.0];

/// 指挥席（与 Listing 18-3 同款）：暂停与变速拧在 Time<Virtual> 上，鼓点跟着停
fn conduct(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
    mut gear: Local<Option<usize>>,
) {
    let gear = gear.get_or_insert(2);
    if keyboard.just_pressed(KeyCode::Space) {
        if time.is_paused() {
            time.unpause();
            println!("老雷：开戏。");
        } else {
            time.pause();
            println!("老雷：中场——鼓也歇了，人也定了。");
        }
    }
    let step: isize = i32::from(keyboard.just_pressed(KeyCode::ArrowUp)) as isize
        - i32::from(keyboard.just_pressed(KeyCode::ArrowDown)) as isize;
    if step != 0 {
        let next = gear.saturating_add_signed(step).min(SPEEDS.len() - 1);
        if next != *gear {
            *gear = next;
            time.set_relative_speed(SPEEDS[next]);
            println!("鼓师：换挡——戏台钟 ×{}。", SPEEDS[next]);
        }
    }
}

/// 帧动画与朝向：腿脚每帧都顺
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

/// 读数牌（与 Listing 18-3 同款）
fn hud(
    real: Res<Time<Real>>,
    stage: Res<Time<Virtual>>,
    mut text: Single<&mut Text2d, With<Hud>>,
) {
    let status = if stage.is_paused() { "中场" } else { "开戏" };
    text.0 = format!(
        "怀表 {:6.1} 秒　　戏台钟 {:6.1} 秒　　速度 ×{:.2}　　{status}",
        real.elapsed_secs(),
        stage.elapsed_secs(),
        stage.relative_speed(),
    );
}

//! Listing 18-9：台步上鼓点——结算搬进 FixedUpdate，画面跟着鼓点一顿一顿
//! 鼓点放慢到 4 拍/秒，顿挫看得清清楚楚；就算用默认 64 Hz，毛病也只是变小没变没。

use bevy::prelude::*;
use bevy::sprite::Anchor;

const FLOOR: f32 = -200.0;
const LIMIT: f32 = 420.0;
const SPEED: f32 = 180.0;

/// 巡台的人：只有速度，位置就用 Transform 本身
#[derive(Component)]
struct Walker {
    velocity: f32,
}

#[derive(Component)]
struct FrameClock(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .insert_resource(Time::<Fixed>::from_hz(4.0)) // 慢板：一秒四拍
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, advance)
        .add_systems(Update, animate)
        .run();
}

// ANCHOR: advance
/// 鼓点上结算走位：Res<Time> 在这里是固定钟，delta 恒为 0.25 秒
/// 位置直接写进 Transform——逻辑没错，错的是观感：一秒只挪四次
fn advance(time: Res<Time>, mut walkers: Query<(&mut Transform, &mut Walker, &mut Sprite)>) {
    for (mut transform, mut walker, mut sprite) in &mut walkers {
        transform.translation.x += walker.velocity * time.delta_secs();
        if transform.translation.x.abs() > LIMIT {
            transform.translation.x = transform.translation.x.clamp(-LIMIT, LIMIT);
            walker.velocity = -walker.velocity;
        }
        sprite.flip_x = walker.velocity < 0.0;
    }
}
// ANCHOR_END: advance

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
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
        Transform::from_xyz(0.0, FLOOR - 28.0, 0.0),
    ));
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        6,
        2,
        None,
        None,
    ));
    commands.spawn((
        Walker { velocity: SPEED },
        FrameClock(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Sprite::from_atlas_image(
            asset_server.load("actors/ayan-sheet.png"),
            TextureAtlas { layout, index: 6 },
        ),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(-LIMIT, FLOOR, 3.0).with_scale(Vec3::splat(4.0)),
    ));
    println!("老雷：把走位的结算搬上鼓点——慢板，一秒四拍。看出毛病没有？");
}

/// 帧动画照常在 Update 走——腿是顺的，人却一顿一顿
fn animate(time: Res<Time>, mut ayan: Single<(&mut Sprite, &mut FrameClock)>) {
    let (sprite, clock) = &mut *ayan;
    if clock.0.tick(time.delta()).just_finished()
        && let Some(atlas) = sprite.texture_atlas.as_mut()
    {
        atlas.index = if atlas.index >= 11 { 6 } else { atlas.index + 1 };
    }
}

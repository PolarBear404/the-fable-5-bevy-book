//! Listing 15-5：多一格的事故——把 last 写成 12，整张连环画每轮闪现一次

use bevy::prelude::*;

// ANCHOR: bug
/// 故意写错的帧范围：图集一共 12 格，编号 0..=11，可这里让它走到 12
const WALK_FIRST: usize = 6;
const WALK_LAST: usize = 12; // 错！最后一格是 11
// ANCHOR_END: bug

#[derive(Component)]
struct FrameClock(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.13)))
        .add_systems(Startup, setup)
        .add_systems(Update, advance_frames)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let sheet = asset_server.load("actors/ayan-sheet.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        6,
        2,
        None,
        None,
    ));

    // 原地踏步（不走位），节拍放慢到 0.8 秒一帧，方便看清事故现场
    commands.spawn((
        Sprite::from_atlas_image(
            sheet,
            TextureAtlas {
                layout,
                index: WALK_FIRST,
            },
        ),
        Transform::from_scale(Vec3::splat(4.0)),
        FrameClock(Timer::from_seconds(0.8, TimerMode::Repeating)),
    ));

    println!("小棠：还是那套走路帧，我把收尾格改成了 12——应该没差吧？");
}

// ANCHOR: advance
fn advance_frames(
    time: Res<Time>,
    mut query: Query<(&mut FrameClock, &mut Sprite)>,
    mut alarmed: Local<bool>,
) {
    for (mut clock, mut sprite) in &mut query {
        if !clock.0.tick(time.delta()).just_finished() {
            continue;
        }
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index == WALK_LAST {
                WALK_FIRST
            } else {
                atlas.index + 1
            };
            println!("现在亮的是第 {} 格", atlas.index);
            if atlas.index == 12 && !*alarmed {
                *alarmed = true;
                println!("小棠：等等——第 12 格是哪来的？！整张原稿都上去了！");
            }
        }
    }
}
// ANCHOR_END: advance

//! Listing 15-3：连环画与取景框——左边挂整张图集原稿，右边用 TextureAtlas 一格一格翻

use bevy::prelude::*;

/// 翻页节拍器：每 0.6 秒进一格
#[derive(Resource)]
struct PageTimer(Timer);

/// 标记：右侧的取景框视图（带图集的 Sprite）
#[derive(Component)]
struct FramedView;

/// 标记：左侧原稿上的金色高亮罩，指出当前格
#[derive(Component)]
struct CellHighlight;

/// 原稿在舞台上的摆放：中心位置与放大倍数
const SHEET_POS: Vec2 = Vec2::new(-280.0, 60.0);
const SHEET_SCALE: f32 = 3.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.13)))
        .insert_resource(PageTimer(Timer::from_seconds(0.6, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, turn_pages)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // ANCHOR: atlas
    // 一张 192×80 的连环画：每格 32×40，6 列 2 行
    let sheet = asset_server.load("actors/ayan-sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 40), 6, 2, None, None);
    let layout_handle = layouts.add(layout);
    // ANCHOR_END: atlas

    // 左：整张原稿直接上墙——Sprite 眼里它就是一张普通图片
    commands.spawn((
        Sprite {
            image: sheet.clone(),
            custom_size: Some(Vec2::new(192.0, 80.0) * SHEET_SCALE),
            ..default()
        },
        Transform::from_translation(SHEET_POS.extend(0.0)),
    ));

    // 原稿上的取景框：盖在当前格上的一层金色薄纱
    commands.spawn((
        CellHighlight,
        Sprite::from_color(
            Color::srgba(1.0, 0.84, 0.30, 0.35),
            Vec2::new(32.0, 40.0) * SHEET_SCALE,
        ),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));

    // ANCHOR: framed
    // 右：同一张图 + 取景框说明书，只露出第 0 格
    commands.spawn((
        FramedView,
        Sprite::from_atlas_image(
            sheet,
            TextureAtlas {
                layout: layout_handle,
                index: 0,
            },
        ),
        Transform::from_xyz(300.0, 0.0, 0.0).with_scale(Vec3::splat(6.0)),
    ));
    // ANCHOR_END: framed

    println!("小棠：原稿十二格挂左边，右边的取景框一次只放一格。");
}

// ANCHOR: turn
/// 每 0.6 秒把 index 拨到下一格，并把高亮罩挪到原稿的对应格子上
fn turn_pages(
    time: Res<Time>,
    mut timer: ResMut<PageTimer>,
    mut framed: Single<&mut Sprite, With<FramedView>>,
    mut highlight: Single<&mut Transform, With<CellHighlight>>,
    mut pages_turned: Local<u32>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let Some(atlas) = &mut framed.texture_atlas else {
        return;
    };

    atlas.index = (atlas.index + 1) % 12;

    // 高亮罩对准第 index 格：列 = index % 6，行 = index / 6
    let (col, row) = (atlas.index % 6, atlas.index / 6);
    let cell = Vec2::new(32.0, 40.0) * SHEET_SCALE;
    let top_left = SHEET_POS + Vec2::new(-2.5 * cell.x, 0.5 * cell.y);
    let offset = Vec2::new(col as f32 * cell.x, -(row as f32) * cell.y);
    highlight.translation = (top_left + offset).extend(1.0);

    // 头一轮逐格报幕，之后安静翻页
    if *pages_turned < 12 {
        *pages_turned += 1;
        let act = if atlas.index < 6 { "正面" } else { "走路" };
        println!("小棠：第 {} 格——{}第 {} 帧。", atlas.index, act, atlas.index % 6 + 1);
        if *pages_turned == 12 {
            println!("小棠：十二格翻完一轮，后面不报了。");
        }
    }
}
// ANCHOR_END: turn

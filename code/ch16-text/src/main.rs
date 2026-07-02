//! Listing 16-11：《夜战》练功房——伤害飘字
//! 阿燕按节拍劈木人桩：每记一击触发一个 Event（第 8 章），Observer 当场
//! 铸一条会上飘、会淡出的 Text2d；会心一击换大字号、金色与"会心！"小签。
//! 连击牌用 Text2dWriter 改 TextSpan 的数字。

use bevy::prelude::*;
use bevy::sprite::{Anchor, Text2dShadow};
use bevy::text::TextBounds;

/// 桥板地面的高度（演员与木桩都站在它上面）
const FLOOR: f32 = -180.0;
/// 木人桩的站位
const DUMMY_X: f32 = 210.0;

// ANCHOR: events
/// 一记劈中：伤害多少、是不是会心
#[derive(Event)]
struct StrikeLanded {
    damage: u32,
    crit: bool,
}

/// 歇手：一轮打完，连击归零
#[derive(Event)]
struct BreatherTaken;
// ANCHOR_END: events

// ANCHOR: floating
/// 飘字的飞行参数：往上飘多快、活多久
#[derive(Component)]
struct FloatingText {
    rise: f32,
    life: Timer,
}
// ANCHOR_END: floating

/// 阿燕的练功节拍：预备 → 抬剑 → 劈落，六剑一歇
#[derive(Component)]
enum DrillPhase {
    Ready { until: f32 },
    Raise { until: f32 },
    Slash { until: f32 },
    Rest { until: f32 },
}

/// 连击计数
#[derive(Resource, Default)]
struct Combo {
    count: u32,
    best: u32,
}

/// 标记：阿燕 / 连击牌的文本根 / 木人桩
#[derive(Component)]
struct Ayan;
#[derive(Component)]
struct ComboBoard;
#[derive(Component)]
struct Dummy {
    wobble: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .init_resource::<Combo>()
        .add_systems(Startup, setup)
        .add_systems(Update, (drill, float_and_fade, wobble_dummy))
        .add_observer(on_strike)
        .add_observer(on_breather)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    let regular = asset_server.load("fonts/book-sans-sc-regular.otf");
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");

    // —— 练功房：桥板地面 + 木人桩 ——
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
    commands.spawn((
        Dummy {
            wobble: Timer::from_seconds(0.25, TimerMode::Once),
        },
        Sprite::from_image(asset_server.load("props/wooden-dummy.png")),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(DUMMY_X, FLOOR, 2.0).with_scale(Vec3::splat(4.0)),
    ));

    // —— 阿燕：四帧练剑图集，从预备式起拍 ——
    let drill_sheet = asset_server.load("actors/ayan-drill-sheet.png");
    let drill_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        4,
        1,
        None,
        None,
    ));
    commands.spawn((
        Ayan,
        Sprite::from_atlas_image(
            drill_sheet,
            TextureAtlas {
                layout: drill_layout,
                index: 0,
            },
        ),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(120.0, FLOOR, 3.0).with_scale(Vec3::splat(4.0)),
        DrillPhase::Ready { until: 1.2 },
    ));

    // ANCHOR: combo_board
    // —— 连击牌：根管"连击 ×"，两个 TextSpan 各管数字与最高纪录 ——
    commands.spawn((
        ComboBoard,
        Text2d::new("连击 ×"),
        TextFont {
            font: bold.clone().into(),
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.72, 0.29)),
        Anchor::CENTER_LEFT,
        Transform::from_xyz(-600.0, 300.0, 5.0),
        children![
            (
                TextSpan::new("0"),
                TextFont {
                    font: bold.clone().into(),
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                TextColor::WHITE,
            ),
            (
                TextSpan::new("　最高 ×0"),
                TextFont {
                    font: regular.clone().into(),
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.55, 0.57, 0.62)),
            ),
        ],
    ));
    // ANCHOR_END: combo_board

    // —— 台口字幕框：本场的题板 ——
    commands.spawn((
        Sprite {
            image: asset_server.load("props/scroll-panel.png"),
            custom_size: Some(Vec2::new(700.0, 84.0)),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(12.0),
                max_corner_scale: 4.0,
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, -310.0, 4.0),
        children![(
            Text2d::new("第二幕《夜战》排练——木人桩上见真章"),
            TextFont {
                font: regular.clone().into(),
                font_size: FontSize::Px(26.0),
                ..default()
            },
            TextColor(Color::srgb(0.24, 0.16, 0.08)),
            TextBounds::new_horizontal(640.0),
            Transform::from_translation(Vec3::Z),
        )],
    ));

    println!("老雷：《夜战》排练。小棠的木人桩，秋白的飘字——都上。");
    println!("场记：连击牌挂好，看招吧。");
}

// ANCHOR: drill
/// 练功节拍机：预备 → 抬剑 → 劈落（触发 StrikeLanded）→ 六剑一歇
fn drill(
    time: Res<Time>,
    mut ayan: Single<(&mut DrillPhase, &mut Sprite), With<Ayan>>,
    mut swings: Local<u32>,
    mut commands: Commands,
) {
    let now = time.elapsed_secs();
    let (phase, sprite) = &mut *ayan;
    let frame = match **phase {
        DrillPhase::Ready { until } => {
            if now >= until {
                **phase = DrillPhase::Raise { until: now + 0.18 };
                2
            } else if (now * 2.5) as u32 % 2 == 0 {
                0
            } else {
                1
            }
        }
        DrillPhase::Raise { until } => {
            if now >= until {
                // 劈落即命中：第 n 剑的伤害走一条固定数列，每四剑出一记会心
                *swings += 1;
                let n = *swings;
                let base = 14 + (n * 7) % 9;
                let crit = n % 4 == 3;
                commands.trigger(StrikeLanded {
                    damage: if crit { base * 2 } else { base },
                    crit,
                });
                **phase = DrillPhase::Slash { until: now + 0.22 };
                3
            } else {
                2
            }
        }
        DrillPhase::Slash { until } => {
            if now >= until {
                **phase = if *swings % 6 == 0 {
                    commands.trigger(BreatherTaken);
                    DrillPhase::Rest { until: now + 2.2 }
                } else {
                    DrillPhase::Ready { until: now + 0.9 }
                };
            }
            3
        }
        DrillPhase::Rest { until } => {
            if now >= until {
                **phase = DrillPhase::Ready { until: now + 0.9 };
            }
            0
        }
    };
    sprite.texture_atlas.as_mut().unwrap().index = frame;
}
// ANCHOR_END: drill

// ANCHOR: on_strike
/// 每一记命中：当场铸一条飘字，更新连击牌，再让木桩晃两下
fn on_strike(
    strike: On<StrikeLanded>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut combo: ResMut<Combo>,
    board: Single<Entity, With<ComboBoard>>,
    mut writer: Text2dWriter,
    mut dummy: Single<&mut Dummy>,
    mut said: Local<bool>,
) {
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");

    // 连击牌：数字与最高纪录各是一个 TextSpan，按序号改字符串
    combo.count += 1;
    combo.best = combo.best.max(combo.count);
    *writer.text(*board, 1) = combo.count.to_string();
    *writer.text(*board, 2) = format!("　最高 ×{}", combo.best);

    // 飘字落点：木桩头顶偏一点，免得叠成一摞
    let jitter = (combo.count * 53 % 61) as f32 - 30.0;
    let spot = Vec3::new(DUMMY_X + jitter, FLOOR + 210.0, 6.0);

    if strike.crit {
        // 会心：大一号、金色、带"会心！"小签
        commands.spawn((
            Text2d::new(strike.damage.to_string()),
            TextFont {
                font: bold.clone().into(),
                font_size: FontSize::Px(46.0),
                ..default()
            },
            TextColor(Color::srgb(0.95, 0.78, 0.22)),
            Text2dShadow::default(),
            Transform::from_translation(spot),
            FloatingText {
                rise: 130.0,
                life: Timer::from_seconds(1.1, TimerMode::Once),
            },
            children![(
                TextSpan::new(" 会心！"),
                TextFont {
                    font: bold.into(),
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::srgb(0.86, 0.32, 0.28)),
            )],
        ));
        if !*said {
            *said = true;
            println!("场记：会心！双倍，记上。");
        }
    } else {
        commands.spawn((
            Text2d::new(strike.damage.to_string()),
            TextFont {
                font: bold.into(),
                font_size: FontSize::Px(30.0),
                ..default()
            },
            TextColor(Color::srgb(0.92, 0.93, 0.96)),
            Text2dShadow::default(),
            Transform::from_translation(spot),
            FloatingText {
                rise: 110.0,
                life: Timer::from_seconds(0.9, TimerMode::Once),
            },
        ));
    }

    dummy.wobble.reset();
}
// ANCHOR_END: on_strike

/// 歇手：连击归零（最高纪录留着）
fn on_breather(
    _breather: On<BreatherTaken>,
    mut combo: ResMut<Combo>,
    board: Single<Entity, With<ComboBoard>>,
    mut writer: Text2dWriter,
    mut said: Local<bool>,
) {
    combo.count = 0;
    *writer.text(*board, 1) = "0".to_string();
    if !*said {
        *said = true;
        println!("阿燕：喘口气，连击重头数。");
    }
}

// ANCHOR: float_and_fade
/// 飘字的一生：匀速上飘，按寿命比例淡出，到点谢幕。
/// 字色淡出用 Text2dWriter 扫整块文本——根和"会心！"小签一起变透明；
/// 阴影是另一个组件，别忘了一起淡，否则后半程只剩一团黑影
fn float_and_fade(
    time: Res<Time>,
    mut floats: Query<(Entity, &mut Transform, &mut FloatingText, &mut Text2dShadow)>,
    mut writer: Text2dWriter,
    mut commands: Commands,
) {
    for (entity, mut transform, mut float, mut shadow) in &mut floats {
        float.life.tick(time.delta());
        transform.translation.y += float.rise * time.delta_secs();
        let alpha = 1.0 - float.life.fraction();
        writer.for_each_color(entity, |mut color| color.set_alpha(alpha));
        shadow.color.set_alpha(alpha * 0.8);
        if float.life.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
// ANCHOR_END: float_and_fade

/// 挨了打的木桩晃两下（纯装饰）
fn wobble_dummy(time: Res<Time>, mut dummy: Single<(&mut Dummy, &mut Transform)>) {
    let (dummy, transform) = &mut *dummy;
    dummy.wobble.tick(time.delta());
    let strength = 1.0 - dummy.wobble.fraction();
    transform.rotation =
        Quat::from_rotation_z(0.05 * strength * (time.elapsed_secs() * 40.0).sin());
}

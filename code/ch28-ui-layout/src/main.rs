//! Listing 28-15《前厅》：给《打瓦》公演装一面响应式 HUD。
//! 台上（sprite 世界）镜头缓摇，玻璃上（UI）纹丝不动——第 16 章的比方在这兑现。
//!
//! 键位：空格=碎一片瓦（+50 分，战利品架亮一枚）　H=折一条凳腿（掉命）
//!       P=中场横幅　R=重开锣　F3=UI 透视镜

use bevy::prelude::*;
use bevy::ui_render::GlobalUiDebugOptions;

// ---- 戏班配色 ----
const LACQUER: Color = Color::srgb(0.55, 0.17, 0.12); // 朱漆
const GOLD: Color = Color::srgb(0.83, 0.69, 0.36); // 描金
const PAPER: Color = Color::srgb(0.96, 0.93, 0.86); // 纸底
const INK: Color = Color::srgb(0.16, 0.13, 0.10); // 墨
const GLAZE: Color = Color::srgb(0.38, 0.65, 0.66); // 釉青
const PLAIN: Color = Color::srgb(0.66, 0.62, 0.55); // 素瓦灰

/// 前台的账：分数与剩凳（假账本，键盘拨着玩）
#[derive(Resource)]
struct FrontDesk {
    score: u32,
    balls: u8,
}

impl Default for FrontDesk {
    fn default() -> Self {
        Self { score: 0, balls: 3 }
    }
}

/// 比分牌上的数字
#[derive(Component)]
struct ScoreText;

/// 命图标：第几颗球
#[derive(Component)]
struct BallIcon(u8);

/// 战利品架上的第几枚瓦签
#[derive(Component)]
struct LootTile(u32);

/// 中场横幅
#[derive(Component)]
struct Intermission;

/// 台上的布景相机（缓摇的那台）
#[derive(Component)]
struct StageCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "前厅——《打瓦》公演".into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<FrontDesk>()
        .add_systems(Startup, (setup_stage, setup_hud))
        .add_systems(Update, (sway_camera, take_keys, refresh_hud))
        .run();
}

// ANCHOR: stage
/// 台上：几排瓦、一条凳、一颗球——sprite 世界，归摇着的相机拍
fn setup_stage(mut commands: Commands) {
    commands.spawn((StageCamera, Camera2d));

    for row in 0..4 {
        for col in 0..10 {
            let color = if row < 2 { GLAZE } else { PLAIN };
            commands.spawn((
                Sprite::from_color(color, Vec2::new(96.0, 26.0)),
                Transform::from_xyz(
                    -486.0 + col as f32 * 108.0,
                    250.0 - row as f32 * 38.0,
                    0.0,
                ),
            ));
        }
    }
    // 凳与球
    commands.spawn((
        Sprite::from_color(Color::srgb(0.42, 0.30, 0.20), Vec2::new(150.0, 20.0)),
        Transform::from_xyz(0.0, -280.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.78, 0.31, 0.24), Vec2::splat(20.0)),
        Transform::from_xyz(0.0, -256.0, 0.0),
    ));
}

/// 镜头缓摇：台上的一切跟着晃，玻璃上的字不为所动
fn sway_camera(time: Res<Time>, mut camera: Single<&mut Transform, With<StageCamera>>) {
    camera.translation.x = (time.elapsed_secs() * 0.4).sin() * 60.0;
}
// ANCHOR_END: stage

// ANCHOR: hud_root
/// 玻璃上：整面 HUD。尺寸全用 Vw/VMin/百分比——拖窗自己跟
fn setup_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");
    let regular = asset_server.load("fonts/book-sans-sc-regular.otf");
    let skin = asset_server.load("ui/panel-board.png");
    let icons = asset_server.load("ui/icons-sheet.png");
    let icon_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(48),
        4,
        1,
        None,
        None,
    ));

    // 九宫格皮的绷法，比分牌和战利品架共用
    let paneling = NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect::all(28.0),
        ..default()
    });

    // HUD 根：铺满视口，上下两栏用 SpaceBetween 撑开；
    // 战利品架也挂在它名下——绝对定位出列，不占 Column 的队
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(vmin(2)),
            ..default()
        })
        .with_children(|hud| {
            spawn_top_bar(hud, &bold, &skin, &paneling, &icons, &icon_layout);
            spawn_hint_bar(hud, &regular);
            spawn_loot_rack(hud, &skin, &paneling, &icons, &icon_layout);
        });

    spawn_intermission(&mut commands, &bold);

    println!("水牌师傅：前厅开张。空格碎瓦，H 折凳，P 中场，R 重开，F3 透视。");
}
// ANCHOR_END: hud_root

// ANCHOR: top_bar
/// 顶栏：比分牌（左）、匾额（中）、命图标（右）——SpaceBetween 三分天下
fn spawn_top_bar(
    hud: &mut ChildSpawnerCommands,
    bold: &Handle<Font>,
    skin: &Handle<Image>,
    paneling: &NodeImageMode,
    icons: &Handle<Image>,
    icon_layout: &Handle<TextureAtlasLayout>,
) {
    hud.spawn(Node {
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        column_gap: vw(2),
        ..default()
    })
    .with_children(|bar| {
        // 比分牌：九宫格皮做底，皮要画满整个外框（BorderBox），字排在 padding 里
        bar.spawn((
            ImageNode {
                image: skin.clone(),
                image_mode: paneling.clone(),
                visual_box: VisualBox::BorderBox,
                ..default()
            },
            Node {
                padding: UiRect::axes(vw(2), vmin(1.8)),
                ..default()
            },
            children![(
                ScoreText,
                Text::new("记分 0"),
                TextFont {
                    font: bold.clone().into(),
                    font_size: FontSize::Vw(2.0),
                    ..default()
                },
                TextColor(INK),
            )],
        ));

        // 匾额：圆角墨底描金字，字号跟窗宽走
        bar.spawn((
            Node {
                padding: UiRect::axes(vw(3), vmin(1.5)),
                border: UiRect::all(px(3)),
                border_radius: BorderRadius::all(vmin(1.5)),
                ..default()
            },
            BackgroundColor(INK),
            BorderColor::all(GOLD),
            children![(
                Text::new("打瓦 · 首演"),
                TextFont {
                    font: bold.clone().into(),
                    font_size: FontSize::Vw(2.6),
                    ..default()
                },
                TextColor(GOLD),
                TextShadow {
                    offset: Vec2::splat(2.0),
                    color: LACQUER.with_alpha(0.8),
                },
            )],
        ));

        // 命图标：三颗球排一行，谁没了谁褪色
        bar.spawn(Node {
            column_gap: vw(0.8),
            ..default()
        })
        .with_children(|row| {
            for i in 0..3 {
                row.spawn((
                    BallIcon(i),
                    ImageNode::from_atlas_image(
                        icons.clone(),
                        TextureAtlas {
                            layout: icon_layout.clone(),
                            index: 0,
                        },
                    ),
                    Node {
                        width: vmin(5),
                        height: vmin(5),
                        ..default()
                    },
                ));
            }
        });
    });
}
// ANCHOR_END: top_bar

/// 底部提示条：居中一行小字
fn spawn_hint_bar(hud: &mut ChildSpawnerCommands, regular: &Handle<Font>) {
    hud.spawn(Node {
        justify_content: JustifyContent::Center,
        ..default()
    })
    .with_children(|bar| {
        bar.spawn((
            Text::new("空格碎瓦 · H 折凳 · P 中场 · R 重开 · F3 透视"),
            TextFont {
                font: regular.clone().into(),
                font_size: FontSize::VMin(2.2),
                ..default()
            },
            TextColor(PAPER.with_alpha(0.75)),
        ));
    });
}

// ANCHOR: loot_rack
/// 战利品架：右侧竖着一面 3×4 的 Grid，碎一片瓦亮一枚签
fn spawn_loot_rack(
    hud: &mut ChildSpawnerCommands,
    skin: &Handle<Image>,
    paneling: &NodeImageMode,
    icons: &Handle<Image>,
    icon_layout: &Handle<TextureAtlasLayout>,
) {
    hud.spawn((
            // 出列钉在右缘；top/bottom 拉满 + 上下外距 auto = 竖向居中
            // （28.7 的组合拳竖过来使——所以架子挂在 HUD 根下，不自立门户）
            ImageNode {
                image: skin.clone(),
                image_mode: paneling.clone(),
                visual_box: VisualBox::BorderBox,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                right: vmin(2),
                top: px(0),
                bottom: px(0),
                margin: UiRect::vertical(auto()),
                height: percent(56),
                aspect_ratio: Some(0.78),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                padding: UiRect::all(vmin(2.2)),
                row_gap: vmin(1),
                column_gap: vmin(1),
                ..default()
            },
        ))
        .with_children(|rack| {
            for i in 0..12 {
                // 前八枚是上过釉的，最后四枚是金瓦
                let frame = if i < 8 { 1 } else { 3 };
                rack.spawn((
                    LootTile(i),
                    ImageNode::from_atlas_image(
                        icons.clone(),
                        TextureAtlas {
                            layout: icon_layout.clone(),
                            index: frame,
                        },
                    )
                    .with_color(Color::srgba(1.0, 1.0, 1.0, 0.18)),
                    Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                ));
            }
        });
}
// ANCHOR_END: loot_rack

/// 中场横幅：又一个根节点，GlobalZIndex(2) 保证盖过一切；平时 Display::None
fn spawn_intermission(commands: &mut Commands, bold: &Handle<Font>) {
    commands.spawn((
        Intermission,
        Node {
            display: Display::None,
            width: percent(100),
            height: vmin(12),
            top: percent(42),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(GOLD),
        GlobalZIndex(2),
        children![(
            Text::new("中 场 休 息"),
            TextFont {
                font: bold.clone().into(),
                font_size: FontSize::Vw(4.0),
                ..default()
            },
            TextColor(INK),
        )],
    ));
}

// ANCHOR: keys
/// 前台记账：键盘拨假数据，顺带拨横幅和透视镜
fn take_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut desk: ResMut<FrontDesk>,
    mut banner: Single<&mut Node, With<Intermission>>,
    mut xray: ResMut<GlobalUiDebugOptions>,
) {
    if keys.just_pressed(KeyCode::Space) && desk.score < 600 {
        desk.score += 50;
        println!("水牌师傅：又碎一片，记 {} 分。", desk.score);
    }
    if keys.just_pressed(KeyCode::KeyH) && desk.balls > 0 {
        desk.balls -= 1;
        println!("水牌师傅：折了条凳腿，还剩 {} 条。", desk.balls);
    }
    if keys.just_pressed(KeyCode::KeyR) {
        *desk = FrontDesk::default();
        println!("水牌师傅：重开锣。");
    }
    if keys.just_pressed(KeyCode::KeyP) {
        banner.display = match banner.display {
            Display::None => Display::Flex,
            _ => Display::None,
        };
    }
    if keys.just_pressed(KeyCode::F3) {
        xray.enabled = !xray.enabled;
    }
}
// ANCHOR_END: keys

// ANCHOR: refresh
/// 账一动，玻璃跟着换字换色——UI 的日常就是「数据变了我重写」
fn refresh_hud(
    desk: Res<FrontDesk>,
    mut score_text: Single<&mut Text, With<ScoreText>>,
    mut balls: Query<(&BallIcon, &mut ImageNode), Without<LootTile>>,
    mut loot: Query<(&LootTile, &mut ImageNode), Without<BallIcon>>,
) {
    if !desk.is_changed() {
        return;
    }
    score_text.0 = format!("记分 {}", desk.score);
    for (icon, mut image) in &mut balls {
        image.color = if icon.0 < desk.balls {
            Color::WHITE
        } else {
            Color::srgba(1.0, 1.0, 1.0, 0.15)
        };
    }
    for (tile, mut image) in &mut loot {
        image.color = if desk.score >= (tile.0 + 1) * 50 {
            Color::WHITE
        } else {
            Color::srgba(1.0, 1.0, 1.0, 0.18)
        };
    }
}
// ANCHOR_END: refresh

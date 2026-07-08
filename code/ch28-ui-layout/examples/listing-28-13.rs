//! Listing 28-13：玻璃上的字——UI 文本是会自己量尺寸的内容。
//! 左板自动换行，字越多板越高（按 T 加一句台词试试）；
//! 右板不许换行，字捅出框外（按 O 给框子上裁刀）。

use bevy::prelude::*;

/// 左板：会长高的那块
#[derive(Component)]
struct WrapBoard;

/// 右板（外框）：字往外捅的那块
#[derive(Component)]
struct NoWrapBoard;

const OPENING_LINE: &str = "打瓦首演，今晚开锣。";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (add_line, toggle_clip, report_heights))
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let font = asset_server.load("fonts/book-sans-sc-regular.otf");
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: px(28),
            padding: UiRect::all(px(24)),
            ..default()
        })
        .with_children(|stage| {
            // 匾额：UI 文本 + 影子。样式组件全是第 16 章的老相识
            stage.spawn((
                Text::new("打瓦 · 首演"),
                TextFont {
                    font: bold.into(),
                    font_size: FontSize::Px(44.0),
                    ..default()
                },
                TextColor(Color::srgb(0.83, 0.69, 0.36)),
                TextShadow {
                    offset: Vec2::splat(3.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.6),
                },
            ));

            // 两块板并排
            stage
                .spawn(Node {
                    column_gap: px(24),
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|row| {
                    // 左板：宽度钉死 300，高度不写——由字说了算
                    row.spawn((
                        WrapBoard,
                        Node {
                            width: px(300),
                            padding: UiRect::all(px(14)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.96, 0.93, 0.86)),
                        children![(
                            Text::new(OPENING_LINE),
                            TextFont {
                                font: font.clone().into(),
                                font_size: FontSize::Px(22.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.2, 0.16, 0.12)),
                        )],
                    ));

                    // 右板：同样 300 宽，但字不许换行——LineBreak::NoWrap
                    row.spawn((
                        NoWrapBoard,
                        Node {
                            width: px(300),
                            padding: UiRect::all(px(14)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.96, 0.93, 0.86)),
                        children![(
                            Text::new("这行字说什么也不肯换行，只好从板子右边捅出去。"),
                            TextLayout {
                                linebreak: LineBreak::NoWrap,
                                ..default()
                            },
                            TextFont {
                                font: font.clone().into(),
                                font_size: FontSize::Px(22.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.2, 0.16, 0.12)),
                        )],
                    ));
                });
        });

    println!("水牌师傅：两块字板挂好。T 加台词，O 上裁刀，空格报板高。");
}
// ANCHOR_END: setup

// ANCHOR: add_line
/// T：往左板的 Text 里再续一句——字一变，测量、布局、板高全自动跟上
fn add_line(
    keys: Res<ButtonInput<KeyCode>>,
    board: Single<&Children, With<WrapBoard>>,
    mut texts: Query<&mut Text>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        for child in board.iter() {
            if let Ok(mut text) = texts.get_mut(child) {
                text.push_str("锣鼓一响，瓦片纷飞。");
                println!("  台词续到 {} 个字", text.chars().count());
            }
        }
    }
}
// ANCHOR_END: add_line

// ANCHOR: clip
/// O：给右板拨 overflow——Visible 任它捅出去，clip() 裁齐板边
fn toggle_clip(keys: Res<ButtonInput<KeyCode>>, mut board: Single<&mut Node, With<NoWrapBoard>>) {
    if keys.just_pressed(KeyCode::KeyO) {
        board.overflow = if board.overflow.is_visible() {
            Overflow::clip()
        } else {
            Overflow::visible()
        };
        println!("  右板 overflow 拨到 {:?}", board.overflow);
    }
}
// ANCHOR_END: clip

/// 空格：报左板实测高度
fn report_heights(
    keys: Res<ButtonInput<KeyCode>>,
    board: Single<&ComputedNode, With<WrapBoard>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let size = board.size() * board.inverse_scale_factor();
        println!("  左板实测 {:.0} × {:.0} 逻辑像素", size.x, size.y);
    }
}

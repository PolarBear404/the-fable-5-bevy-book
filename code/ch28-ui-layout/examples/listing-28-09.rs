//! Listing 28-9：首演座位表——CSS Grid 的地界一次画完，住户按格入座。
//! 按 B 拨包厢的跨度（1→2→3），看散座怎么让位；空格报格子实测尺寸。

use bevy::prelude::*;

/// 包厢
#[derive(Component)]
struct BoxSeat;

/// 散座
#[derive(Component)]
struct Seat;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (dial_box_span, report_cells))
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 居中的展台
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|stage| {
            stage
                .spawn((
                    // 座位表：8 列 4 行全用 flex(1.0)——等分且允许缩到零；
                    // 行列间留 8 像素过道；万一有人挤出界，补的行一律 28 像素高
                    Node {
                        display: Display::Grid,
                        width: px(680),
                        height: px(360),
                        padding: UiRect::all(px(12)),
                        row_gap: px(8),
                        column_gap: px(8),
                        grid_template_columns: RepeatedGridTrack::flex(8, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                        grid_auto_rows: GridTrack::px(28.0),
                        border: UiRect::all(px(3)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.83, 0.69, 0.36)),
                ))
                .with_children(|chart| {
                    // 包厢：从第 4 条竖格线起跨 2 列，从第 1 条横格线起跨 2 行
                    chart.spawn((
                        BoxSeat,
                        Node {
                            grid_column: GridPlacement::start_span(4, 2),
                            grid_row: GridPlacement::start_span(1, 2),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.83, 0.69, 0.36)),
                    ));
                    // 乐池：钉死在第 4 行，从第 1 条竖格线横到最后一条（-1 从右往左数）
                    chart.spawn((
                        Node {
                            grid_row: GridPlacement::start(4),
                            grid_column: GridPlacement::start_end(1, -1),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.20, 0.28, 0.37)),
                    ));
                    // 散座二十席：不写位置，自动填进剩下的格子
                    for i in 0..20 {
                        let color = if i % 2 == 0 {
                            Color::srgb(0.55, 0.17, 0.12)
                        } else {
                            Color::srgb(0.62, 0.28, 0.22)
                        };
                        chart.spawn((Seat, Node::default(), BackgroundColor(color)));
                    }
                });
        });

    println!("水牌师傅：座位表贴出来了。B 拨包厢跨度，空格报格子尺寸。");
}
// ANCHOR_END: setup

// ANCHOR: dial
/// B：包厢跨度 1→2→3 循环。跨度一改，二十席散座全体自动重新入座
fn dial_box_span(
    keys: Res<ButtonInput<KeyCode>>,
    mut box_seat: Single<&mut Node, With<BoxSeat>>,
    mut span: Local<u16>,
) {
    if keys.just_pressed(KeyCode::KeyB) {
        if *span == 0 {
            *span = 2; // 开场时包厢是 2×2
        }
        *span = *span % 3 + 1;
        box_seat.grid_column = GridPlacement::start_span(4, *span);
        box_seat.grid_row = GridPlacement::start_span(1, *span);
        println!("  包厢改成 {0}×{0}", *span);
    }
}
// ANCHOR_END: dial

/// 空格：报包厢和头一张散座的实测尺寸
fn report_cells(
    keys: Res<ButtonInput<KeyCode>>,
    box_seat: Single<&ComputedNode, With<BoxSeat>>,
    seats: Query<&ComputedNode, With<Seat>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    let b = box_seat.size() * box_seat.inverse_scale_factor();
    println!("  包厢 {:.0} × {:.0}", b.x, b.y);
    if let Some(seat) = seats.iter().next() {
        let s = seat.size() * seat.inverse_scale_factor();
        println!("  散座 {:.0} × {:.0}", s.x, s.y);
    }
}

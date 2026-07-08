//! Listing 28-3：单位阅兵——同一排牌子，五种量法。
//! 拖窗看谁跟着动；按 U 拨 UiScale，看谁独自长个；按空格让水牌师傅报尺寸。

use bevy::prelude::*;

/// 受阅的牌子：第几号、用的什么刻度
#[derive(Component)]
struct UnitBar {
    index: usize,
    label: &'static str,
}

// ANCHOR: bars
/// 五种刻度各出一块牌：宽度写法不同，高度一律 44 逻辑像素
const BARS: [(&str, Val, Color); 5] = [
    ("Px(300)", Val::Px(300.0), Color::srgb(0.55, 0.17, 0.12)), // 朱漆
    ("Percent(50)", Val::Percent(50.0), Color::srgb(0.83, 0.69, 0.36)), // 描金
    ("Vw(50)", Val::Vw(50.0), Color::srgb(0.38, 0.65, 0.66)),   // 釉青
    ("Vh(50)", Val::Vh(50.0), Color::srgb(0.20, 0.28, 0.37)),   // 黛蓝
    ("VMin(50)", Val::VMin(50.0), Color::srgb(0.30, 0.42, 0.31)), // 苔绿
];
// ANCHOR_END: bars

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (dial_ui_scale, report_widths))
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 校场：铺满整个视口的一根立柱，牌子从上往下一块块排
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(12),
            padding: UiRect::all(px(16)),
            ..default()
        })
        .with_children(|parade| {
            for (index, (label, width, color)) in BARS.into_iter().enumerate() {
                parade.spawn((
                    UnitBar { index, label },
                    Node {
                        width,
                        height: px(44),
                        ..default()
                    },
                    BackgroundColor(color),
                ));
            }
        });

    println!("水牌师傅：五块牌列队。拖拖窗户，按空格报数，按 U 拨 UiScale。");
}
// ANCHOR_END: setup

// ANCHOR: ui_scale
/// U 键在 1.0 → 1.5 → 2.0 之间拨全局 UiScale
fn dial_ui_scale(keys: Res<ButtonInput<KeyCode>>, mut ui_scale: ResMut<UiScale>) {
    if keys.just_pressed(KeyCode::KeyU) {
        ui_scale.0 = match ui_scale.0 {
            s if s < 1.25 => 1.5,
            s if s < 1.75 => 2.0,
            _ => 1.0,
        };
        println!("水牌师傅：UiScale 拨到 {:.1}。", ui_scale.0);
    }
}
// ANCHOR_END: ui_scale

/// 空格：按队列顺序报每块牌的实测宽度（换算回逻辑像素）
fn report_widths(keys: Res<ButtonInput<KeyCode>>, bars: Query<(&UnitBar, &ComputedNode)>) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    let mut rows: Vec<_> = bars.iter().collect();
    rows.sort_by_key(|(bar, _)| bar.index);
    println!("报数——");
    for (bar, computed) in rows {
        let width = computed.size().x * computed.inverse_scale_factor();
        println!("  {:>11} 宽 {:.0} 逻辑像素", bar.label, width);
    }
}

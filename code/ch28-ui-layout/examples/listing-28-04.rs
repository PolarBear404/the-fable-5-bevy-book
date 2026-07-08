//! Listing 28-4：三层皮——margin、border、padding 各占一层。
//! 两块牌参数完全相同，只有 box_sizing 不同；按空格报实测尺寸，按 F3 开透视镜。

use bevy::prelude::*;
use bevy::ui_render::GlobalUiDebugOptions;

/// 展台上的牌子：记着自己的量法
#[derive(Component)]
struct SizedBoard(&'static str);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (report_sizes, toggle_xray))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    // 展台居中，两块牌并排
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            board(BoxSizing::BorderBox, "BorderBox（默认）"),
            board(BoxSizing::ContentBox, "ContentBox"),
        ],
    ));
    println!("水牌师傅：左右两块牌，图纸都写 240×150。空格报数，F3 透视。");
}

// ANCHOR: board
/// 同一张图纸：宽 240 高 150，padding 24、border 8、margin 16——只有 box_sizing 不同
fn board(box_sizing: BoxSizing, label: &'static str) -> impl Bundle {
    (
        SizedBoard(label),
        Node {
            box_sizing,
            width: px(240),
            height: px(150),
            padding: UiRect::all(px(24)),
            border: UiRect::all(px(8)),
            margin: UiRect::all(px(16)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.96, 0.93, 0.86)), // 纸底
        BorderColor::all(Color::srgb(0.83, 0.69, 0.36)), // 描金框
        // 内容物：一块填满内容区的青布，衬出 padding 留出的边
        children![(
            Node {
                width: percent(100),
                height: percent(100),
                ..default()
            },
            BackgroundColor(Color::srgb(0.38, 0.65, 0.66)),
        )],
    )
}
// ANCHOR_END: board

/// 空格：报两块牌的实测外框尺寸（按画面左右排）
fn report_sizes(
    keys: Res<ButtonInput<KeyCode>>,
    boards: Query<(&SizedBoard, &ComputedNode, &UiGlobalTransform)>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    let mut rows: Vec<_> = boards.iter().collect();
    rows.sort_by(|(_, _, a), (_, _, b)| a.translation.x.total_cmp(&b.translation.x));
    for (board, computed, _) in rows {
        let size = computed.size() * computed.inverse_scale_factor();
        println!("  {}：实测 {:.0} × {:.0} 逻辑像素", board.0, size.x, size.y);
    }
}

// ANCHOR: xray
/// F3：透视镜——给每个节点描出 border/padding/content 三层框
fn toggle_xray(keys: Res<ButtonInput<KeyCode>>, mut options: ResMut<GlobalUiDebugOptions>) {
    if keys.just_pressed(KeyCode::F3) {
        options.enabled = !options.enabled;
        options.outline_border_box = true;
        options.outline_padding_box = true;
        options.outline_content_box = true;
        println!(
            "水牌师傅：透视镜{}。",
            if options.enabled { "开" } else { "关" }
        );
    }
}
// ANCHOR_END: xray

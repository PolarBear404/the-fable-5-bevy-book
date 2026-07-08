//! Listing 28-5：九宫对齐板——justify_content 管主轴，align_items 管交叉轴。
//! J 拨主轴档位，A 拨交叉轴档位，D 换主轴方向，S 让二号牌单独站队，空格报当前档。

use bevy::prelude::*;

/// 对齐板本板
#[derive(Component)]
struct Stage;

/// 二号牌：被 S 键单独调遣的那块
#[derive(Component)]
struct SecondTile;

// ANCHOR: dials
/// 主轴上的六种分法
const JUSTIFY_DIAL: [(JustifyContent, &str); 6] = [
    (JustifyContent::FlexStart, "FlexStart"),
    (JustifyContent::Center, "Center"),
    (JustifyContent::FlexEnd, "FlexEnd"),
    (JustifyContent::SpaceBetween, "SpaceBetween"),
    (JustifyContent::SpaceAround, "SpaceAround"),
    (JustifyContent::SpaceEvenly, "SpaceEvenly"),
];

/// 交叉轴上的四种站法
const ALIGN_DIAL: [(AlignItems, &str); 4] = [
    (AlignItems::FlexStart, "FlexStart"),
    (AlignItems::Center, "Center"),
    (AlignItems::FlexEnd, "FlexEnd"),
    (AlignItems::Stretch, "Stretch"),
];

/// 主轴的四个朝向
const DIRECTION_DIAL: [(FlexDirection, &str); 4] = [
    (FlexDirection::Row, "Row"),
    (FlexDirection::RowReverse, "RowReverse"),
    (FlexDirection::Column, "Column"),
    (FlexDirection::ColumnReverse, "ColumnReverse"),
];
// ANCHOR_END: dials

/// 三个旋钮各自拨到第几档
#[derive(Resource, Default)]
struct DialState {
    justify: usize,
    align: usize,
    direction: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<DialState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (turn_dials, dial_align_self))
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        // 外层只干一件事：把对齐板挪到屏幕中央
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Stage,
            // 对齐板：640×360 的金框空场，三块高矮不一的牌子归它调度。
            // 两个旋钮都从 FlexStart 起步，跟下面 DialState 的初始档对齐
            Node {
                width: px(640),
                height: px(360),
                border: UiRect::all(px(3)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BorderColor::all(Color::srgb(0.83, 0.69, 0.36)),
            children![
                tile(60.0, Color::srgb(0.55, 0.17, 0.12)),
                (SecondTile, tile(120.0, Color::srgb(0.38, 0.65, 0.66))),
                tile(180.0, Color::srgb(0.30, 0.42, 0.31)),
            ],
        )],
    ));
    println!("水牌师傅：对齐板立好了。J 拨主轴，A 拨交叉轴，D 换方向，S 单独站队。");
}

/// 一块 90 宽、高矮给定的牌。高度写 min_height 而不是 height，
/// 给 AlignItems::Stretch 留出拉伸的余地
fn tile(min_height: f32, color: Color) -> impl Bundle {
    (
        Node {
            width: px(90),
            min_height: px(min_height),
            ..default()
        },
        BackgroundColor(color),
    )
}
// ANCHOR_END: setup

// ANCHOR: turn
/// 三个键各转一个旋钮，直接改对齐板的 Node 字段
fn turn_dials(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DialState>,
    mut stage: Single<&mut Node, With<Stage>>,
) {
    let mut turned = false;
    if keys.just_pressed(KeyCode::KeyJ) {
        state.justify = (state.justify + 1) % JUSTIFY_DIAL.len();
        stage.justify_content = JUSTIFY_DIAL[state.justify].0;
        turned = true;
    }
    if keys.just_pressed(KeyCode::KeyA) {
        state.align = (state.align + 1) % ALIGN_DIAL.len();
        stage.align_items = ALIGN_DIAL[state.align].0;
        turned = true;
    }
    if keys.just_pressed(KeyCode::KeyD) {
        state.direction = (state.direction + 1) % DIRECTION_DIAL.len();
        stage.flex_direction = DIRECTION_DIAL[state.direction].0;
        turned = true;
    }
    if turned || keys.just_pressed(KeyCode::Space) {
        println!(
            "  主轴 {} ｜ justify_content: {} ｜ align_items: {}",
            DIRECTION_DIAL[state.direction].1,
            JUSTIFY_DIAL[state.justify].1,
            ALIGN_DIAL[state.align].1,
        );
    }
}
// ANCHOR_END: turn

// ANCHOR: self_dial
/// 交叉轴的单独站法：Auto（默认）是「听父级 align_items 的」
const SELF_DIAL: [(AlignSelf, &str); 4] = [
    (AlignSelf::Auto, "Auto"),
    (AlignSelf::Center, "Center"),
    (AlignSelf::FlexEnd, "FlexEnd"),
    (AlignSelf::Stretch, "Stretch"),
];

/// S：只拨二号牌的 align_self——全队听 align_items 的，它一个人另站
fn dial_align_self(
    keys: Res<ButtonInput<KeyCode>>,
    mut tile: Single<&mut Node, With<SecondTile>>,
    mut dial: Local<usize>,
) {
    if keys.just_pressed(KeyCode::KeyS) {
        *dial = (*dial + 1) % SELF_DIAL.len();
        tile.align_self = SELF_DIAL[*dial].0;
        println!("  二号牌 align_self 拨到 {}", SELF_DIAL[*dial].1);
    }
}
// ANCHOR_END: self_dial

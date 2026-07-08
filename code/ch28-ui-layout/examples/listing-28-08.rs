//! Listing 28-8：谁贴在上面——同级看 ZIndex，跨树看 GlobalZIndex，
//! 都不写就按「后来者居上」。Z 拨告示·甲的 ZIndex，G 拨横幅的 GlobalZIndex，
//! 空格把整摞玻璃从垫底到封面念一遍。

use bevy::prelude::*;
use bevy::ui::UiStack;

/// 告示·甲：被 Z 键拨来拨去的那张
#[derive(Component)]
struct PosterA;

/// 中场横幅：另一棵 UI 树上的住户
#[derive(Component)]
struct Banner;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (turn_dials, report_stack))
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 一面墙，三张告示错位叠贴。它们是亲兄弟：spawn 顺序甲、乙、丙
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![
            (
                PosterA,
                Name::new("告示·甲"),
                poster(60.0, 40.0, Color::srgb(0.55, 0.17, 0.12)),
            ),
            (Name::new("告示·乙"), poster(150.0, 100.0, Color::srgb(0.38, 0.65, 0.66))),
            (Name::new("告示·丙"), poster(240.0, 160.0, Color::srgb(0.30, 0.42, 0.31))),
        ],
    ));

    // 中场横幅：自立门户的另一棵树（另一个根节点），横贯半腰。
    // GlobalZIndex(-1)：先垫在所有人后面
    commands.spawn((
        Banner,
        Name::new("中场横幅"),
        Node {
            width: percent(100),
            height: px(64),
            top: percent(42),
            ..default()
        },
        BackgroundColor(Color::srgb(0.83, 0.69, 0.36)),
        GlobalZIndex(-1),
    ));

    println!("水牌师傅：三张告示一条横幅。Z 拨甲，G 拨横幅，空格念座次。");
}

/// 一张 220×150 的告示，错位贴在墙上
fn poster(left: f32, top: f32, color: Color) -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            left: px(left),
            top: px(top),
            width: px(220),
            height: px(150),
            ..default()
        },
        BackgroundColor(color),
    )
}
// ANCHOR_END: setup

// ANCHOR: dials
/// Z：告示·甲的 ZIndex 在 0 和 2 之间拨——只跟亲兄弟比；
/// G：横幅的 GlobalZIndex 在 -1 和 1 之间拨——跟全世界比
fn turn_dials(
    keys: Res<ButtonInput<KeyCode>>,
    mut poster_a: Single<&mut ZIndex, With<PosterA>>,
    mut banner: Single<&mut GlobalZIndex, With<Banner>>,
) {
    if keys.just_pressed(KeyCode::KeyZ) {
        poster_a.0 = if poster_a.0 == 0 { 2 } else { 0 };
        println!("  告示·甲 ZIndex 拨到 {}", poster_a.0);
    }
    if keys.just_pressed(KeyCode::KeyG) {
        banner.0 = -banner.0;
        println!("  中场横幅 GlobalZIndex 拨到 {}", banner.0);
    }
}
// ANCHOR_END: dials

// ANCHOR: stack
/// 空格：UiStack 是引擎排好的画序清单（从垫底到封面），拿 Name 念出来
fn report_stack(keys: Res<ButtonInput<KeyCode>>, stack: Res<UiStack>, names: Query<&Name>) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    let order: Vec<_> = stack
        .uinodes
        .iter()
        .filter_map(|entity| names.get(*entity).ok())
        .map(Name::as_str)
        .collect();
    println!("  垫底 → {} → 封面", order.join(" → "));
}
// ANCHOR_END: stack

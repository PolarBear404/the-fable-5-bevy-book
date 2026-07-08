//! Listing 28-7：在场与离场——absolute 出列不占位；
//! Visibility::Hidden 人走席留，Display::None 连席位一起撤。
//! H 拨老二的 Visibility，N 拨老二的 Display，空格报三兄弟的横坐标。

use bevy::prelude::*;

/// 排排坐的三兄弟
#[derive(Component)]
struct Brother(&'static str);

/// 老二：被拨来拨去的那个
#[derive(Component)]
struct Second;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_second, report_positions))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // ANCHOR: corners
    // 四角的告示：position_type: Absolute——出列，不挤在场的任何人。
    // 各用一对 inset 字段钉住一个角
    let corners: [(&str, Val, Val, Val, Val); 4] = [
        //（名号,      left,      top,       right,     bottom）
        ("入口", px(12), px(12), auto(), auto()),
        ("出口", auto(), px(12), px(12), auto()),
        ("茶水", px(12), auto(), auto(), px(12)),
        ("杂役", auto(), auto(), px(12), px(12)),
    ];
    for (_name, left, top, right, bottom) in corners {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left,
                top,
                right,
                bottom,
                width: px(100),
                height: px(56),
                ..default()
            },
            BackgroundColor(Color::srgb(0.20, 0.28, 0.37)),
        ));
    }

    // ANCHOR_END: corners

    // ANCHOR: brothers
    // 在场的三兄弟：正常排队，占位靠布局
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: px(16),
            ..default()
        },
        children![
            brother("老大", Color::srgb(0.55, 0.17, 0.12)),
            (Second, brother("老二", Color::srgb(0.38, 0.65, 0.66))),
            brother("老三", Color::srgb(0.30, 0.42, 0.31)),
            // 居中的金幅：出列的第四个孩子。left/right 双双为 0 把可摆区拉满，
            // margin 左右 auto 让它居中。这手要像这样挂在 Flex 容器里使——
            // 挂成根节点 auto 会被当成 0，金幅贴左（28.7 正文有说法）
            (
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(0),
                    bottom: px(24),
                    width: px(280),
                    height: px(48),
                    margin: UiRect::horizontal(auto()),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.83, 0.69, 0.36)),
            ),
        ],
    ));
    // ANCHOR_END: brothers

    println!("水牌师傅：三兄弟入席，四角贴妥。H 拨隐身，N 拨撤席，空格报座次。");
}

/// 一位兄弟：120×120 的一块牌
fn brother(name: &'static str, color: Color) -> impl Bundle {
    (
        Brother(name),
        Node {
            width: px(120),
            height: px(120),
            ..default()
        },
        BackgroundColor(color),
    )
}

// ANCHOR: toggle
/// H：Visibility::Hidden——不画了，但座位还给他留着；
/// N：Display::None——布局里除名，两侧当场合拢
fn toggle_second(
    keys: Res<ButtonInput<KeyCode>>,
    second: Single<(&mut Node, &mut Visibility), With<Second>>,
) {
    let (mut node, mut visibility) = second.into_inner();
    if keys.just_pressed(KeyCode::KeyH) {
        *visibility = match *visibility {
            Visibility::Hidden => Visibility::Inherited,
            _ => Visibility::Hidden,
        };
        println!("  老二 Visibility 拨到 {:?}", *visibility);
    }
    if keys.just_pressed(KeyCode::KeyN) {
        node.display = match node.display {
            Display::None => Display::DEFAULT,
            _ => Display::None,
        };
        println!("  老二 Display 拨到 {:?}", node.display);
    }
}
// ANCHOR_END: toggle

/// 空格：报三兄弟的中心横坐标（逻辑像素）——看老三挪没挪窝
fn report_positions(
    keys: Res<ButtonInput<KeyCode>>,
    brothers: Query<(&Brother, &ComputedNode, &UiGlobalTransform)>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    for (brother, computed, transform) in &brothers {
        println!(
            "  {} 站在 x = {:.0}",
            brother.0,
            transform.translation.x * computed.inverse_scale_factor()
        );
    }
}

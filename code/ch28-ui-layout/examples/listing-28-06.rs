//! Listing 28-6：分地——flex_basis 定底数，flex_grow 分余粮，flex_shrink 摊亏空。
//! 拖宽拖窄窗口看分配；G 拨大户的 grow，S 拨老铺的 shrink，W 拨换行，空格报数。

use bevy::prelude::*;

/// 街面
#[derive(Component)]
struct Street;

/// 一户铺面：名号
#[derive(Component)]
struct Plot(&'static str);

/// 老铺（basis 220、grow 0、shrink 0——寸土不让的那家）
#[derive(Component)]
struct OldShop;

/// 大户（grow 2 的那家）
#[derive(Component)]
struct BigShop;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (turn_dials, report_widths))
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Street,
        // 街面：横排，铺面之间留 12 像素的巷子
        Node {
            width: percent(100),
            padding: UiRect::all(px(16)),
            column_gap: px(12),
            row_gap: px(12),
            ..default()
        },
        children![
            (
                OldShop,
                Plot("老铺 grow 0 shrink 0"),
                // 底数 220，余粮不要（grow 0），亏空不摊（shrink 0）
                Node {
                    flex_basis: px(220),
                    flex_grow: 0.0,
                    flex_shrink: 0.0,
                    height: px(90),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.55, 0.17, 0.12)),
            ),
            (
                Plot("佃户 grow 1"),
                // 同样底数 220，余粮按一股分，亏空照默认（shrink 1）摊
                Node {
                    flex_basis: px(220),
                    flex_grow: 1.0,
                    height: px(90),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.38, 0.65, 0.66)),
            ),
            (
                BigShop,
                Plot("大户 grow 2"),
                // 底数一样，余粮拿两股
                Node {
                    flex_basis: px(220),
                    flex_grow: 2.0,
                    height: px(90),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.30, 0.42, 0.31)),
            ),
        ],
    ));
    println!("水牌师傅：三户分一条街。拖窗看分法，空格报数，G/S/W 拨档。");
}
// ANCHOR_END: setup

// ANCHOR: dials
/// G：大户的 grow 在 2 和 0 之间拨；S：老铺的 shrink 在 0 和 1 之间拨；
/// W：整条街在「不许换行」和「挤不下就换行」之间拨
fn turn_dials(
    keys: Res<ButtonInput<KeyCode>>,
    mut street: Single<&mut Node, (With<Street>, Without<OldShop>, Without<BigShop>)>,
    mut old_shop: Single<&mut Node, (With<OldShop>, Without<BigShop>)>,
    mut big_shop: Single<&mut Node, With<BigShop>>,
) {
    if keys.just_pressed(KeyCode::KeyG) {
        big_shop.flex_grow = if big_shop.flex_grow > 0.0 { 0.0 } else { 2.0 };
        println!("  大户 flex_grow 拨到 {}", big_shop.flex_grow);
    }
    if keys.just_pressed(KeyCode::KeyS) {
        old_shop.flex_shrink = if old_shop.flex_shrink > 0.0 { 0.0 } else { 1.0 };
        println!("  老铺 flex_shrink 拨到 {}", old_shop.flex_shrink);
    }
    if keys.just_pressed(KeyCode::KeyW) {
        street.flex_wrap = match street.flex_wrap {
            FlexWrap::NoWrap => FlexWrap::Wrap,
            _ => FlexWrap::NoWrap,
        };
        println!("  街面 flex_wrap 拨到 {:?}", street.flex_wrap);
    }
}
// ANCHOR_END: dials

/// 空格：按街面从左到右报每户实测宽度
fn report_widths(
    keys: Res<ButtonInput<KeyCode>>,
    street: Single<&ComputedNode, With<Street>>,
    plots: Query<(&Plot, &ComputedNode, &UiGlobalTransform)>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    let street_width = street.size().x * street.inverse_scale_factor();
    println!("街面宽 {street_width:.0}——");
    // 先按行（y）再按列（x）排，换行之后报数顺序也对得上画面
    let mut rows: Vec<_> = plots.iter().collect();
    rows.sort_by(|(_, _, a), (_, _, b)| {
        (a.translation.y, a.translation.x).partial_cmp(&(b.translation.y, b.translation.x)).unwrap()
    });
    for (plot, computed, transform) in rows {
        let width = computed.size().x * computed.inverse_scale_factor();
        let row_y = transform.translation.y * computed.inverse_scale_factor();
        println!("  {}：{:.0} 逻辑像素（行心 y = {:.0}）", plot.0, width, row_y);
    }
}

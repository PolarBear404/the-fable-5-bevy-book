//! Listing 15-10：色卡墙——混色、转色相、提亮压暗、现成色票，一面墙看全

use bevy::color::palettes::{basic, css, tailwind};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.13)))
        .add_systems(Startup, setup)
        .run();
}

/// 在 (x, y) 处贴一张 72×72 的色卡
fn swatch(commands: &mut Commands, x: f32, y: f32, color: impl Into<Color>) {
    commands.spawn((
        Sprite::from_color(color, Vec2::splat(72.0)),
        Transform::from_xyz(x, y, 1.0),
    ));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // ANCHOR: mix
    // 第一排：在 sRGB 空间里从靛蓝直线走到金黄——中段又灰又脏
    // 第二排：换 Oklch 空间走同一趟——一路干净
    let indigo = Srgba::new(0.10, 0.25, 0.90, 1.0);
    let gold = Srgba::new(1.00, 0.84, 0.20, 1.0);
    for i in 0..11 {
        let f = i as f32 / 10.0;
        let x = i as f32 * 100.0 - 500.0;
        swatch(&mut commands, x, 250.0, indigo.mix(&gold, f));
        swatch(
            &mut commands,
            x,
            160.0,
            Oklcha::from(indigo).mix(&Oklcha::from(gold), f),
        );
    }
    // ANCHOR_END: mix

    // ANCHOR: hue
    // 第三排：一块基色转 12 次色相，一只调色轮
    let base = Hsla::new(0.0, 0.85, 0.55, 1.0);
    for i in 0..12 {
        let x = i as f32 * 100.0 - 550.0;
        swatch(&mut commands, x, 60.0, base.rotate_hue(i as f32 * 30.0));
    }
    // ANCHOR_END: hue

    // ANCHOR: luminance
    // 第四排：灯笼金往两头打——darker 四档、原色、lighter 四档
    let gilt = Color::srgb(0.93, 0.74, 0.29);
    for i in -4i32..=4 {
        let x = i as f32 * 100.0;
        let color = if i < 0 {
            gilt.darker(-i as f32 * 0.07)
        } else {
            gilt.lighter(i as f32 * 0.07)
        };
        swatch(&mut commands, x, -40.0, color);
    }
    // ANCHOR_END: luminance

    // ANCHOR: palettes
    // 第五排：三本现成色票——CSS 命名色、Tailwind 色阶、八只基本色
    let named: [Srgba; 10] = [
        basic::AQUA,
        basic::NAVY,
        css::GOLD,
        css::CRIMSON,
        css::REBECCA_PURPLE,
        css::SEA_GREEN,
        css::TOMATO,
        tailwind::SKY_400,
        tailwind::EMERALD_500,
        tailwind::AMBER_300,
    ];
    for (i, color) in named.into_iter().enumerate() {
        swatch(&mut commands, i as f32 * 100.0 - 450.0, -150.0, color);
    }
    // ANCHOR_END: palettes

    // ANCHOR: alpha
    // 第六排：同一块朱红逐档调透明度，后面垫一条金带衬出"透"
    commands.spawn((
        Sprite::from_color(Color::srgb(0.93, 0.74, 0.29), Vec2::new(1000.0, 36.0)),
        Transform::from_xyz(0.0, -260.0, 0.0),
    ));
    for i in 0..9 {
        let alpha = 1.0 - i as f32 / 8.0;
        swatch(
            &mut commands,
            i as f32 * 100.0 - 400.0,
            -260.0,
            css::CRIMSON.with_alpha(alpha),
        );
    }
    // ANCHOR_END: alpha

    // ANCHOR: report
    let mid_srgb = indigo.mix(&gold, 0.5);
    let mid_oklch = Srgba::from(Oklcha::from(indigo).mix(&Oklcha::from(gold), 0.5));
    println!("小棠：sRGB 直线的中点是 {}，又灰又闷。", mid_srgb.to_hex());
    println!("小棠：Oklch 路线的中点是 {}，干净多了。", mid_oklch.to_hex());

    let ayan_red = Srgba::hex("#C83A3A").unwrap();
    println!(
        "小棠：阿燕戏服的色号 {}，换算成色相是 {:.0}°。",
        ayan_red.to_hex(),
        Hsla::from(ayan_red).hue
    );
    // ANCHOR_END: report
}

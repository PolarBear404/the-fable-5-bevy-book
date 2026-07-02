//! Listing 16-6：可变字体——一副字模，千般字重

use bevy::prelude::*;
use bevy::text::{FontVariationTag, FontVariations};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    // Mona Sans：可变字体，带 wght（200~900）与 wdth（75~125）两根轴
    let mona: Handle<Font> = asset_server.load("fonts/MonaSans-VariableFont.ttf");

    // 左列：字重阶梯——同一个文件，weight 拨到哪儿就是哪儿
    for (i, w) in [200u16, 400, 550, 700, 900].into_iter().enumerate() {
        commands.spawn((
            Text2d::new(format!("NIGHT FERRY {w}")),
            TextFont {
                font: mona.clone().into(),
                font_size: FontSize::Px(40.0),
                weight: FontWeight(w),
                ..default()
            },
            Transform::from_xyz(-280.0, 240.0 - 80.0 * i as f32, 0.0),
        ));
    }

    // 右列上：字宽两档——压扁与撑开也是同一个文件
    for (i, (label, width)) in [
        ("Condensed", FontWidth::CONDENSED),
        ("Expanded", FontWidth::EXPANDED),
    ]
    .into_iter()
    .enumerate()
    {
        commands.spawn((
            Text2d::new(label),
            TextFont {
                font: mona.clone().into(),
                font_size: FontSize::Px(40.0),
                width,
                ..default()
            },
            Transform::from_xyz(300.0, 240.0 - 80.0 * i as f32, 0.0),
        ));
    }

    // 右列下：FontVariations 直接报轴名拨轴，weight/width 字段是它的省写
    commands.spawn((
        Text2d::new("wght 850, wdth 80"),
        TextFont {
            font: mona.clone().into(),
            font_size: FontSize::Px(40.0),
            font_variations: FontVariations::builder()
                .set(FontVariationTag::WEIGHT, 850.0)
                .set(FontVariationTag::WIDTH, 80.0)
                .build(),
            ..default()
        },
        Transform::from_xyz(300.0, 0.0, 0.0),
    ));

    // 对照：静态字体给 weight 拨轴——没轴可拨，纹丝不动
    commands.spawn((
        Text2d::new("静态字模拨字重 550"),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
            font_size: FontSize::Px(40.0),
            weight: FontWeight(550),
            ..default()
        },
        Transform::from_xyz(0.0, -260.0, 0.0),
    ));
}
// ANCHOR_END: setup

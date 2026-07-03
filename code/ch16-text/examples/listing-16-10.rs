//! Listing 16-10：把词排进字幕框——TextBounds 与 LineBreak

use bevy::prelude::*;
use bevy::text::TextBounds;

const LINE: &str = "夜渡无人，秋水自横。雁背驮霜，橹声欸乃，一篙点破满江星。";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.72, 0.80, 0.76)))
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let zh_font = asset_server.load("fonts/book-sans-sc-regular.otf");
    let text_font = TextFont {
        font: zh_font.clone().into(),
        font_size: FontSize::Px(28.0),
        ..default()
    };

    // 三只一模一样的字幕框，三种排版规矩
    let cases = [
        (220.0, "给宽度，自动换行", TextBounds::new_horizontal(560.0), LineBreak::WordBoundary),
        (0.0, "不许换行", TextBounds::new_horizontal(560.0), LineBreak::NoWrap),
        (-220.0, "宽高都给，装不下呢？", TextBounds::new(560.0, 30.0), LineBreak::WordBoundary),
    ];
    for (y, label, bounds, linebreak) in cases {
        // 字幕框：ch15 的九宫格画框
        commands.spawn((
            Sprite {
                image: asset_server.load("props/scroll-panel.png"),
                custom_size: Some(Vec2::new(620.0, 110.0)),
                image_mode: SpriteImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(12.0),
                    max_corner_scale: 4.0,
                    ..default()
                }),
                ..default()
            },
            Transform::from_xyz(80.0, y, 0.0),
            // 词是框的子实体：跟着框走，画在框上面一层
            children![(
                Text2d::new(LINE),
                text_font.clone(),
                TextColor(Color::srgb(0.24, 0.16, 0.08)),
                bounds,
                TextLayout::new(Justify::Left, linebreak),
                Transform::from_translation(Vec3::Z),
            )],
        ));
        // 旁注小签
        commands.spawn((
            Text2d::new(label),
            TextFont {
                font: zh_font.clone().into(),
                font_size: FontSize::Px(18.0),
                ..default()
            },
            Transform::from_xyz(-510.0, y, 0.0),
        ));
    }
}
// ANCHOR_END: setup

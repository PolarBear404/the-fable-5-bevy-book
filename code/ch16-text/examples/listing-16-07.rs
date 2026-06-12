//! Listing 16-7：Justify 管行与行，Anchor 管整块字钉哪

use bevy::prelude::*;
use bevy::sprite::Anchor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let zh_font = asset_server.load("fonts/book-sans-sc-regular.otf");
    let text_font = TextFont {
        font: zh_font.clone(),
        font_size: 26.0,
        ..default()
    };
    // 每个样本的 Transform 钉子位置都画一枚金色图钉作参照
    let pin = |commands: &mut Commands, x: f32, y: f32| {
        commands.spawn((
            Sprite::from_color(Color::srgb(0.91, 0.72, 0.29), Vec2::splat(8.0)),
            Transform::from_xyz(x, y, 1.0),
        ));
    };

    // 第一排：同一块两行字，三种 Justify——只挪行与行的相对位置
    let verse = "渡口\n夜话连台";
    for (x, justify) in [
        (-380.0, Justify::Left),
        (0.0, Justify::Center),
        (380.0, Justify::Right),
    ] {
        pin(&mut commands, x, 150.0);
        commands.spawn((
            Text2d::new(verse),
            text_font.clone(),
            TextLayout::new_with_justify(justify),
            Transform::from_xyz(x, 150.0, 0.0),
        ));
    }

    // 第二排：同一行字，三种 Anchor——挪的是整块字相对钉子的位置
    for (x, anchor) in [
        (-380.0, Anchor::TOP_LEFT),
        (0.0, Anchor::CENTER),
        (380.0, Anchor::BOTTOM_RIGHT),
    ] {
        pin(&mut commands, x, -150.0);
        commands.spawn((
            Text2d::new("梢公摇橹"),
            text_font.clone(),
            anchor,
            Transform::from_xyz(x, -150.0, 0.0),
        ));
    }
}
// ANCHOR_END: setup

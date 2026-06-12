//! Listing 15-6：锚点九宫——九张定位照，钉在同一排金十字上的九种钉法

use bevy::prelude::*;
use bevy::sprite::Anchor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.13)))
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let still = asset_server.load("actors/ayan-still.png");

    let anchors = [
        Anchor::TOP_LEFT,
        Anchor::TOP_CENTER,
        Anchor::TOP_RIGHT,
        Anchor::CENTER_LEFT,
        Anchor::CENTER,
        Anchor::CENTER_RIGHT,
        Anchor::BOTTOM_LEFT,
        Anchor::BOTTOM_CENTER,
        Anchor::BOTTOM_RIGHT,
    ];

    for (i, anchor) in anchors.into_iter().enumerate() {
        let pin = Vec3::new(
            (i % 3) as f32 * 330.0 - 330.0,
            110.0 - (i / 3) as f32 * 220.0,
            0.0,
        );

        // 金十字标出钉子的位置（即这一组的 Transform 平移量）
        commands.spawn((
            Sprite::from_color(Color::srgb(0.93, 0.74, 0.29), Vec2::new(26.0, 4.0)),
            Transform::from_translation(pin.with_z(1.0)),
        ));
        commands.spawn((
            Sprite::from_color(Color::srgb(0.93, 0.74, 0.29), Vec2::new(4.0, 26.0)),
            Transform::from_translation(pin.with_z(1.0)),
        ));

        // 同一张定位照，钉法各不相同
        commands.spawn((
            Sprite {
                image: still.clone(),
                custom_size: Some(Vec2::new(32.0, 40.0) * 3.0),
                color: Color::srgba(1.0, 1.0, 1.0, 0.88),
                ..default()
            },
            anchor,
            Transform::from_translation(pin),
        ));

    }

    println!("小棠：九张定位照，九种钉法，钉子全在金十字上——画各自让开。");
}
// ANCHOR_END: setup

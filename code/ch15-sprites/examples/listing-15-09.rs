//! Listing 15-9：装裱事故——切片线一人一半，帛心被切没了

use bevy::prelude::*;

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

    // 素材一共 48×48，四边各让 30 像素：30 + 30 = 60，比图还宽
    commands.spawn(Sprite {
        image: asset_server.load("props/scroll-panel.png"),
        custom_size: Some(Vec2::new(340.0, 150.0)),
        image_mode: SpriteImageMode::Sliced(TextureSlicer {
            border: BorderRect::all(30.0),
            max_corner_scale: 4.0,
            ..default()
        }),
        ..default()
    });

    println!("小棠：边框留宽点总没错吧——咦，框呢？");
}
// ANCHOR_END: setup

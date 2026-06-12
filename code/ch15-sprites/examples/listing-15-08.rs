//! Listing 15-8：装裱与平铺——同一块画框素材的三种摆法，外加两条铺出来的水面与桥板

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.13)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let panel = asset_server.load("props/scroll-panel.png");
    let panel_size = Some(Vec2::new(340.0, 150.0));

    // ANCHOR: sliced
    // 一号框：Auto——整张图跟着 custom_size 硬拉，角花全变形
    commands.spawn((
        Sprite {
            image: panel.clone(),
            custom_size: panel_size,
            ..default()
        },
        Transform::from_xyz(-400.0, 150.0, 0.0),
    ));

    // 二号框：九宫格切片——角花保形，但仍是原图里的 12 像素，细得几乎看不见
    commands.spawn((
        Sprite {
            image: panel.clone(),
            custom_size: panel_size,
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(12.0),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 150.0, 0.0),
    ));

    // 三号框：放开 max_corner_scale，让四角跟全场一样放大 4 倍
    commands.spawn((
        Sprite {
            image: panel.clone(),
            custom_size: panel_size,
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(12.0),
                max_corner_scale: 4.0,
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(400.0, 150.0, 0.0),
    ));
    // ANCHOR_END: sliced

    // ANCHOR: tiled
    // 16×16 的水纹与桥板贴片，各铺一条长带：贴片按 4 倍一块往外重复
    commands.spawn((
        Sprite {
            image: asset_server.load("props/water-tile.png"),
            custom_size: Some(Vec2::new(1240.0, 128.0)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 4.0,
            },
            ..default()
        },
        Transform::from_xyz(0.0, -240.0, 0.0),
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("props/dock-plank.png"),
            custom_size: Some(Vec2::new(1240.0, 64.0)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 4.0,
            },
            ..default()
        },
        Transform::from_xyz(0.0, -120.0, 0.0),
    ));
    // ANCHOR_END: tiled

    println!("小棠：一号框白送的反面教材，二号框守规矩，三号框才是像素戏该有的裱法。");
    println!("小棠：水和桥板不用画整条，一块贴片铺到头。");
}

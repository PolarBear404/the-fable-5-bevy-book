//! Listing 15-7：脚踩实地——镜头推近拉远时，锚点决定演员的脚还在不在台板上

use bevy::prelude::*;
use bevy::sprite::Anchor;

/// 台板的上沿（脚应该一直贴着这条线）
const FLOOR_TOP: f32 = -150.0;

/// 标记：参与推拉的演员
#[derive(Component)]
struct Zoomed;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.13)))
        .add_systems(Startup, setup)
        .add_systems(Update, push_and_pull)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let still = asset_server.load("actors/ayan-still.png");

    // 台板
    commands.spawn((
        Sprite::from_color(Color::srgb(0.30, 0.24, 0.18), Vec2::new(1100.0, 10.0)),
        Transform::from_xyz(0.0, FLOOR_TOP - 5.0, 0.0),
    ));

    // 左：默认锚点（CENTER）。先按 3 倍身高把腰挪到脚上方 60 像素处，脚正好落线
    commands.spawn((
        Zoomed,
        Sprite {
            image: still.clone(),
            custom_size: Some(Vec2::new(32.0, 40.0) * 3.0),
            ..default()
        },
        Transform::from_xyz(-220.0, FLOOR_TOP + 60.0, 1.0),
    ));

    // 右：锚点改到鞋底（BOTTOM_CENTER），钉子直接钉在台板线上
    commands.spawn((
        Zoomed,
        Sprite {
            image: still,
            custom_size: Some(Vec2::new(32.0, 40.0) * 3.0),
            ..default()
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(220.0, FLOOR_TOP, 1.0),
    ));

    println!("老雷：推近景再拉回来，看哪位的脚不老实。");
}
// ANCHOR_END: setup

// ANCHOR: zoom
/// 近景远景来回推拉：只改 custom_size，不动 Transform
fn push_and_pull(time: Res<Time>, mut query: Query<&mut Sprite, With<Zoomed>>) {
    let zoom = 3.0 + 1.4 * (time.elapsed_secs() * 1.2).sin();
    for mut sprite in &mut query {
        sprite.custom_size = Some(Vec2::new(32.0, 40.0) * zoom);
    }
}
// ANCHOR_END: zoom

//! Listing 16-9：会自己变的字号——FontSize 的 Vw/Vh 与 Rem

use bevy::prelude::*;
use bevy::text::RemSize;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, stage_hands)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let zh = asset_server.load("fonts/book-sans-sc-regular.otf");
    let rows = [
        (180.0, "Px(28)：铁打的二十八", FontSize::Px(28.0)),
        (60.0, "Vw(4)：窗宽的百分之四", FontSize::Vw(4.0)),
        (-60.0, "Vh(6)：窗高的百分之六", FontSize::Vh(6.0)),
        (-180.0, "Rem(1.4)：基准尺的一点四倍", FontSize::Rem(1.4)),
    ];
    for (y, text, size) in rows {
        commands.spawn((
            Text2d::new(text),
            TextFont {
                font: zh.clone().into(),
                font_size: size,
                ..default()
            },
            Transform::from_xyz(0.0, y, 0.0),
        ));
    }
}
// ANCHOR_END: setup

// ANCHOR: stage_hands
/// 检场：三秒时把窗户收窄，六秒时把 RemSize 基准尺从 20 拨到 30
fn stage_hands(
    time: Res<Time>,
    mut window: Single<&mut Window>,
    mut rem: ResMut<RemSize>,
    mut acted: Local<(bool, bool)>,
) {
    if time.elapsed_secs() > 3.0 && !acted.0 {
        acted.0 = true;
        window.resolution.set(880.0, 720.0);
        println!("检场：窗户收窄到 880——看 Vw 那行。");
    }
    if time.elapsed_secs() > 6.0 && !acted.1 {
        acted.1 = true;
        rem.0 = 30.0;
        println!("检场：基准尺 20 拨到 30——看 Rem 那行。");
    }
}
// ANCHOR_END: stage_hands

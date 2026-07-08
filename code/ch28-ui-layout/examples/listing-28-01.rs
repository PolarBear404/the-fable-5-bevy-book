//! Listing 28-1：第一块牌子——Node 是一块归布局系统管的矩形。
//! 拨一下：把 spawn(Camera2d) 那行注释掉再跑——满屏漆黑，日志一声不吭。

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    // UI 也得有相机才上屏。没挂过任何相机？整棵 UI 树静默不画，连警告都没有
    commands.spawn(Camera2d);

    // 一块 320×140 的朱漆牌。注意没给 Transform 也没给 Sprite——
    // Node 的位置和大小由布局系统说了算；它没有父节点，就贴着视口左上角
    commands.spawn((
        Node {
            width: px(320),
            height: px(140),
            ..default()
        },
        BackgroundColor(Color::srgb(0.55, 0.17, 0.12)),
    ));

    println!("水牌师傅：头一块牌，挂上了。");
}
// ANCHOR_END: setup

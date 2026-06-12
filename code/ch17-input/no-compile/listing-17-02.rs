//! Listing 17-2：行不通——KeyCode 里没有名叫 A 的变体
//! 旧教程（Bevy 0.12 之前）的键名照搬过来，编译器当场拦下。

use bevy::prelude::*;

/// 标记：阿燕
#[derive(Component)]
struct Ayan;

// ANCHOR: old_name
/// 照网上旧帖写的走位检查——KeyCode::A 是 0.12 之前的键名
fn walk(keyboard: Res<ButtonInput<KeyCode>>, mut ayan: Single<&mut Transform, With<Ayan>>) {
    if keyboard.pressed(KeyCode::A) {
        ayan.translation.x -= 4.0;
    }
}
// ANCHOR_END: old_name

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, walk)
        .run();
}

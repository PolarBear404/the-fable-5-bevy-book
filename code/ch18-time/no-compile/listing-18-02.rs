//! Listing 18-2：行不通——中场休息，给通用时钟按暂停
//! 时间既然是 Time，暂停想必就是 time.pause()？编译器另有说法。

use bevy::prelude::*;

// ANCHOR: intermission
/// 老雷的第一反应：按住空格，时间就该停
fn intermission(keyboard: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time>) {
    if keyboard.just_pressed(KeyCode::Space) {
        time.pause();
    }
}
// ANCHOR_END: intermission

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, intermission)
        .run();
}

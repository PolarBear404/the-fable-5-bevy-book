//! Listing 24-2（编译失败示例）：emissive 要的是 LinearRgba，塞 Color 进去编不过。
//! 期望错误：E0308 mismatched types — expected `LinearRgba`, found `Color`（`Color::srgb` 的返回类型）。
//!
//! 这个文件放在 no-compile/ 下，不参与 `cargo check`——它「就是要」编不过。
//! 改对的办法见正文：把 emissive 写成 `LinearRgba::rgb(...)`，或在末尾加 `.into()`。

use bevy::prelude::*;

fn setup(mut materials: ResMut<Assets<StandardMaterial>>) {
    // ANCHOR: bad
    materials.add(StandardMaterial {
        base_color: Color::BLACK,
        // ✗ emissive 是 LinearRgba 字段，Color::srgb(...) 是 Color，类型对不上
        emissive: Color::srgb(0.1, 2.4, 2.1),
        ..default()
    });
    // ANCHOR_END: bad
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

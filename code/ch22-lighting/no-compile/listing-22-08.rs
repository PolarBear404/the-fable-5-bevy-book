//! Listing 22-8（编译失败示例）：把 AmbientLight 当资源插——通不过编译
//!
//! AmbientLight 在 0.18 是「组件」（挂在相机上，只管那一台相机），
//! 全局那一份才叫 GlobalAmbientLight，是「资源」。名字像、字段同，但身份两样。
//! 把组件塞进 insert_resource，编译器当场拦下。

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // ANCHOR: wrong
        // 想调全局环境光，却把组件 AmbientLight 塞进了 insert_resource
        .insert_resource(AmbientLight {
            brightness: 220.0,
            ..default()
        })
        // ANCHOR_END: wrong
        .run();
}

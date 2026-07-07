//! Listing 27-11：没开门就想拿工具——bevy_dev_tools 不在默认 feature 集合里，
//! 编译期直接报 E0433。本文件不参与正常构建；把它临时挂进 examples 并用
//! `cargo check -p ch27-dev-tools --example listing-27-11 --no-default-features`
//! 即可复现正文里的报错。

use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FpsOverlayPlugin::default()))
        .run();
}

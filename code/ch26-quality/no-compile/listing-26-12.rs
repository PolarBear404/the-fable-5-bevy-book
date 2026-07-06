//! Listing 26-12：老黄历的写法——把 Msaa 当资源插，编译器不答应
//!
//! 本文件不在 Cargo targets 里，编译必失败，报错原文见正文。

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // ANCHOR: wrong
        // 网上老教程的抗锯齿开关：全局插一份资源——可这块门牌对不上，Msaa 是每台相机的组件
        .insert_resource(Msaa::Off)
        // ANCHOR_END: wrong
        .run();
}

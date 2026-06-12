//! Listing 5-5：init_resource——没有才创建，已有则让位

use bevy::prelude::*;

// ANCHOR: config
/// 打靶场配置：每局子弹数与单价
#[derive(Resource)]
struct RangeConfig {
    bullets: u32,
    price: u32,
}

impl Default for RangeConfig {
    fn default() -> Self {
        RangeConfig {
            bullets: 5,
            price: 2,
        }
    }
}
// ANCHOR_END: config

// ANCHOR: main
fn main() {
    // 场景一：只 init——资源不存在，用 Default 值创建
    let mut app = App::new();
    app.init_resource::<RangeConfig>().add_systems(Update, report);
    app.update();

    // 场景二：已经 insert 过自定义配置，init 发现资源已存在，什么都不做
    let mut app = App::new();
    app.insert_resource(RangeConfig {
        bullets: 10,
        price: 1,
    })
    .init_resource::<RangeConfig>()
    .add_systems(Update, report);
    app.update();
}
// ANCHOR_END: main

fn report(config: Res<RangeConfig>) {
    println!("本场配置：{} 发子弹，单价 {} 元", config.bullets, config.price);
}

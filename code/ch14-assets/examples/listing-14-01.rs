//! Listing 14-1：第一件道具——从 assets/ 加载青霜剑，亲眼看见“加载是异步的”

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.08, 0.1)))
        .add_systems(Startup, place_order)
        .add_systems(Update, watch_shelf)
        .run();
}

// ANCHOR: place_order
/// 开单：向 AssetServer 要青霜剑
fn place_order(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
) {
    commands.spawn(Camera2d);

    // 路径相对 assets/ 目录写；load 立刻返回一张 Handle——提货单
    let sword: Handle<Image> = asset_server.load("props/qingshuang-sword.png");

    // 提货单刚到手，马上去货架（Assets<Image>）上找找看
    let on_shelf = if images.get(&sword).is_some() { "有货" } else { "还空着" };
    println!("老顾：《长风渡》头一件道具，青霜剑，单子开出去了。");
    println!("老顾：（瞅一眼货架）提货单在手，货架上——{on_shelf}。");

    // 不等了：把提货单直接交给 Sprite，挂上片场
    commands.spawn(Sprite::from_image(sword));
    println!("老顾：不碍事，先挂上。到货那一刻它自己会亮出来。");
}
// ANCHOR_END: place_order

// ANCHOR: watch_shelf
/// 每帧瞅一眼货架，到货那一刻报一声
fn watch_shelf(
    sprite: Single<&Sprite>,
    images: Res<Assets<Image>>,
    mut frames: Local<u32>,
    mut reported: Local<bool>,
) {
    *frames += 1;
    if *reported {
        return;
    }
    if let Some(image) = images.get(&sprite.image) {
        println!(
            "老顾：到货！第 {} 帧，{} × {} 像素，已经挂在片场了。",
            *frames,
            image.size().x,
            image.size().y
        );
        *reported = true;
    }
}
// ANCHOR_END: watch_shelf

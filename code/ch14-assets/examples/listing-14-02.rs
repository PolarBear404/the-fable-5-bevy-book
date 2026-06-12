//! Listing 14-2：提货单的脾气——同路径同单、克隆、货号与货址，以及常驻的白布

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.08, 0.1)))
        .add_systems(Startup, inspect_tickets)
        .run();
}

// ANCHOR: tickets
fn inspect_tickets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
) {
    commands.spawn(Camera2d);

    // 同一条路径开两次单：拿到的是同一张提货单，库房不会进第二遍货
    let a: Handle<Image> = asset_server.load("props/lantern.png");
    let b: Handle<Image> = asset_server.load("props/lantern.png");
    println!("老顾：灯笼的单子开了两回，是同一张单吗？——{}", a == b);

    // 克隆提货单：复印件指向同一件货，想发给几个组就发几份
    let c = a.clone();
    println!("老顾：复印一份给布景组，指的还是那盏灯吗？——{}", c == a);

    // 单子背面印着货号与货址
    println!("老顾：货号——{:?}", a.id());
    println!("老顾：货址——{:?}", a.path());

    // 两张单、画同一件货：左右各挂一盏灯笼
    commands.spawn((Sprite::from_image(a), Transform::from_xyz(-100.0, 0.0, 0.0)));
    commands.spawn((Sprite::from_image(c), Transform::from_xyz(100.0, 0.0, 0.0)));
    // ANCHOR_END: tickets

    // ANCHOR: default_handle
    // Handle::<Image>::default() 是一张常驻提货单，指向引擎自带的纯白小图——
    // 第 2 章 Sprite::from_color 的色块，真身就是这块白布染了色
    let blank = Handle::<Image>::default();
    let white = images.get(&blank).expect("白布常年在架");
    println!(
        "老顾：另有一块 {} × {} 的百搭白布常年在架，染个色就能当色块使。",
        white.size().x,
        white.size().y
    );
    // ANCHOR_END: default_handle
}

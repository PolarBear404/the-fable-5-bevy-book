//! Listing 6-1：Main 调度全家——注册顺序打乱，执行顺序不乱

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    // 故意不按执行顺序注册：执行顺序由调度决定，与注册顺序无关
    app.add_systems(Update, || println!("  Update      —— 游戏逻辑"))
        .add_systems(Last, || println!("  Last        —— 帧末收尾"))
        .add_systems(Startup, || println!("  Startup     —— 搭建场景（只跑一次）"))
        .add_systems(First, || println!("  First       —— 一帧之始"))
        .add_systems(PostStartup, || println!("  PostStartup —— 开赛前检查（只跑一次）"))
        .add_systems(PreUpdate, || println!("  PreUpdate   —— 引擎备料"))
        .add_systems(FixedUpdate, || println!("  FixedUpdate —— （本例一声不吭）"))
        .add_systems(PreStartup, || println!("  PreStartup  —— 最早的准备（只跑一次）"))
        .add_systems(PostUpdate, || println!("  PostUpdate  —— 引擎善后"));

    println!("—— 第 1 帧 ——");
    app.update();
    println!("—— 第 2 帧 ——");
    app.update();
}

//! Listing 6-2：FixedUpdate 初见——固定时钟与帧率脱钩

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

fn main() {
    let mut app = App::new();
    // MinimalPlugins 里有 TimePlugin——没有时钟，固定时钟无从谈起
    app.add_plugins(MinimalPlugins)
        // ANCHOR: clock
        // 把固定时钟的步长设为 50 毫秒（默认 64 Hz，即 15.625 毫秒）
        .insert_resource(Time::<Fixed>::from_seconds(0.05))
        // 实验台专用：让每帧恰好"流逝"30 毫秒，输出完全可复现；真实游戏用默认的真实时间
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(30)))
        // ANCHOR_END: clock
        .add_systems(Update, || println!("  Update"))
        .add_systems(FixedUpdate, |time: Res<Time>| {
            println!("  FixedUpdate（本步 {} 毫秒）", time.delta().as_millis())
        });

    for frame in 1..=5 {
        println!("—— 第 {frame} 帧（流逝 30 毫秒）——");
        app.update();
    }
}



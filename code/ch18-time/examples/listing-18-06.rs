//! Listing 18-6：对表——同一只 Res<Time>，在 Update 与 FixedUpdate 里各报什么
//! 固定步长 50 毫秒，每帧流逝 30 毫秒（第 6 章的实验台），看鼓师怎么记账。

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

// ANCHOR: probes
/// 鼓点上的对表：三只钟各报各的
fn on_the_beat(time: Res<Time>, fixed: Res<Time<Fixed>>, stage: Res<Time<Virtual>>) {
    println!(
        "  [FixedUpdate] Res<Time> 报 {} 毫秒 = Time<Fixed> 的步长 {}；Time<Virtual> 报 {}",
        time.delta().as_millis(),
        fixed.timestep().as_millis(),
        stage.delta().as_millis(),
    );
}

/// 帧上的对表：Res<Time> 报本帧流逝，再看一眼鼓师攒下的零头
fn on_the_frame(time: Res<Time>, fixed: Res<Time<Fixed>>) {
    println!(
        "  [Update]      Res<Time> 报 {} 毫秒；鼓师攒着 {} 毫秒",
        time.delta().as_millis(),
        fixed.overstep().as_millis(),
    );
}
// ANCHOR_END: probes

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        // 鼓点：50 毫秒一拍（默认是 64 Hz，即 15.625 毫秒）
        .insert_resource(Time::<Fixed>::from_seconds(0.05))
        // 实验台旋钮：每帧固定流逝 30 毫秒
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(30)))
        .add_systems(FixedUpdate, on_the_beat)
        .add_systems(Update, on_the_frame);

    for frame in 1..=5 {
        println!("—— 第 {frame} 帧（流逝 30 毫秒）——");
        app.update();
    }
}
// ANCHOR_END: main

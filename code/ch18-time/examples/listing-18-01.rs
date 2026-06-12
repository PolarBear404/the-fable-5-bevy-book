//! Listing 18-1：一帧一步 vs 一秒一速——同一段台步，在快慢两台机器上各走一秒

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

/// 两人在台上走出的距离（单位）
#[derive(Resource, Default)]
struct Marks {
    standin: f32,
    ayan: f32,
}

/// 开拍令牌：它在场，台步系统才许动
#[derive(Resource)]
struct Action;

// ANCHOR: strides
/// 替身的走法：跑一帧迈一步，每步 4 个单位
fn stride_per_frame(mut marks: ResMut<Marks>) {
    marks.standin += 4.0;
}

/// 阿燕的走法：速度按“每秒 240 单位”定义，乘上这一帧实际流逝的时间
fn stride_per_second(time: Res<Time>, mut marks: ResMut<Marks>) {
    marks.ayan += 240.0 * time.delta_secs();
}
// ANCHOR_END: strides

// ANCHOR: rehearse
/// 在一台“每秒 fps 帧”的机器上排练一秒
fn rehearse(stage: &str, fps: u32) {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Marks>()
        // 实验台旋钮（第 6 章用过）：每帧固定流逝 1/fps 秒，输出可复现
        .insert_resource(TimeUpdateStrategy::ManualDuration(
            Duration::from_secs_f64(1.0 / fps as f64),
        ))
        .add_systems(
            Update,
            (stride_per_frame, stride_per_second).run_if(resource_exists::<Action>),
        );

    app.update(); // 对表帧：时钟刚起步，本帧 delta 按 0 计（第 6 章交过底）
    app.insert_resource(Action); // 开拍
    for _ in 0..fps {
        app.update(); // 跑满“一秒”应有的帧数
    }

    let marks = app.world().resource::<Marks>();
    println!("—— {stage}（每秒 {fps} 帧）走一秒 ——");
    println!("  替身走到 {:.1}（{fps} 帧 × 4 单位）", marks.standin);
    println!("  阿燕走到 {:.1}（240 单位/秒 × 实际时长）", marks.ayan);
}
// ANCHOR_END: rehearse

fn main() {
    rehearse("新戏台", 60);
    rehearse("老戏台", 30);
    println!("老雷：替身换台慢一半，阿燕走哪儿都是 240——往后谁的步子不乘 delta，谁加练。");
}

//! Listing 7-4：错过即丢——打瞌睡的记分员每三帧才醒一次

use bevy::prelude::*;

/// 消息带上碰撞序号，方便对账
#[derive(Message)]
struct WallHit {
    n: u32,
}

#[derive(Component)]
struct Ball {
    pos: i32,
    velocity: i32,
}

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_message::<WallHit>().add_systems(Startup, spawn_ball);
    app.add_systems(
        Update,
        (
            move_ball,
            // 记分员打瞌睡：每三帧才醒来读一次
            drowsy_scorer.run_if(|mut frame: Local<u32>| {
                *frame += 1;
                frame.is_multiple_of(3)
            }),
        )
            .chain(),
    );

    for frame in 1..=9 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}
// ANCHOR_END: main

fn spawn_ball(mut commands: Commands) {
    // 走廊只有 4 格，速度拉满：每一帧都在撞墙
    commands.spawn(Ball {
        pos: 0,
        velocity: 4,
    });
}

// ANCHOR: systems
/// 写者：球速失控，每帧撞一次墙，消息编号递增
fn move_ball(
    mut ball: Single<&mut Ball>,
    mut hits: MessageWriter<WallHit>,
    mut count: Local<u32>,
) {
    ball.pos += ball.velocity;
    if ball.pos == 0 || ball.pos == 4 {
        ball.velocity = -ball.velocity;
        *count += 1;
        println!("球：第 {} 次撞墙", *count);
        hits.write(WallHit { n: *count });
    }
}

/// 读者：醒来时把还在缓冲里的消息一次读完
fn drowsy_scorer(mut hits: MessageReader<WallHit>) {
    let heard: Vec<u32> = hits.read().map(|hit| hit.n).collect();
    println!("记分员醒来：听到第 {heard:?} 次");
}
// ANCHOR_END: systems

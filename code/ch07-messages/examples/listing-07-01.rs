//! Listing 7-1：最小消息闭环——球撞墙，音效响

use bevy::prelude::*;

/// 消息：球撞上了墙
#[derive(Message)]
struct WallHit;

/// 弹球：在 0..=12 的走廊里来回弹
#[derive(Component)]
struct Ball {
    pos: i32,
    velocity: i32,
}

fn main() {
    let mut app = App::new();
    app.add_message::<WallHit>()
        .add_systems(Startup, spawn_ball)
        .add_systems(Update, (move_ball, play_sound).chain());

    for frame in 1..=6 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn(Ball {
        pos: 0,
        velocity: 4,
    });
}

/// 写者：移动球，撞墙就反弹，并写一条消息
fn move_ball(mut ball: Single<&mut Ball>, mut hits: MessageWriter<WallHit>) {
    ball.pos += ball.velocity;
    println!("球：位置 {}", ball.pos);
    if ball.pos == 0 || ball.pos == 12 {
        ball.velocity = -ball.velocity;
        hits.write(WallHit);
    }
}

/// 读者：每读到一条碰撞消息，放一声音效
fn play_sound(mut hits: MessageReader<WallHit>) {
    for _ in hits.read() {
        println!("音效：砰！");
    }
}

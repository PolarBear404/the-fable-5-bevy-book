//! Listing 7-5：MessageMutator——消息在途中被修改

use bevy::prelude::*;

#[derive(Clone, Copy)]
enum Wall {
    Left,
    Right,
}

/// 消息带上撞击力道，供途中修改、供下游读取
#[derive(Message)]
struct WallHit {
    wall: Wall,
    force: i32,
}

#[derive(Component)]
struct Ball {
    pos: i32,
    velocity: i32,
}

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_message::<WallHit>()
        .add_systems(Startup, spawn_ball)
        // 写 → 改 → 读，三段流水线
        .add_systems(Update, (move_ball, cushion, play_sound).chain());

    for frame in 1..=6 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}
// ANCHOR_END: main

fn spawn_ball(mut commands: Commands) {
    commands.spawn(Ball {
        pos: 0,
        velocity: 4,
    });
}

fn move_ball(mut ball: Single<&mut Ball>, mut hits: MessageWriter<WallHit>) {
    ball.pos += ball.velocity;
    if ball.pos == 0 || ball.pos == 12 {
        let wall = if ball.pos == 0 { Wall::Left } else { Wall::Right };
        hits.write(WallHit {
            wall,
            force: ball.velocity.abs(),
        });
        ball.velocity = -ball.velocity;
    }
}

// ANCHOR: cushion
/// 右墙贴了海绵垫：途经的碰撞消息被吸掉 2 点力道
fn cushion(mut hits: MessageMutator<WallHit>) {
    for hit in hits.read() {
        if let Wall::Right = hit.wall {
            hit.force -= 2;
            println!("海绵垫：吸掉 2 点力道");
        }
    }
}
// ANCHOR_END: cushion

/// 下游读者拿到的是修改后的消息
fn play_sound(mut hits: MessageReader<WallHit>) {
    for hit in hits.read() {
        println!("音效：砰！（力道 {}）", hit.force);
    }
}

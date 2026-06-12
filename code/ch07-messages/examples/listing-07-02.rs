//! Listing 7-2：一写多读——音效与计分各自读到全部碰撞

use bevy::prelude::*;

// ANCHOR: message
/// 走廊两端的墙：左边是普通墙，右边是金墙
#[derive(Clone, Copy)]
enum Wall {
    Left,
    Right,
}

/// 消息升级：除了"撞了"，还带上"撞的是哪面墙"
#[derive(Message)]
struct WallHit {
    wall: Wall,
}
// ANCHOR_END: message

/// 总分
#[derive(Resource, Default)]
struct Score(u32);

#[derive(Component)]
struct Ball {
    pos: i32,
    velocity: i32,
}

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_message::<WallHit>()
        .init_resource::<Score>()
        .add_systems(Startup, spawn_ball)
        // 写者在前；两个读者之间不分先后，可以并行
        .add_systems(Update, (move_ball, (play_sound, update_score)).chain());

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

// ANCHOR: systems
/// 写者只管报告事实，不关心谁在听
fn move_ball(mut ball: Single<&mut Ball>, mut hits: MessageWriter<WallHit>) {
    ball.pos += ball.velocity;
    if ball.pos == 0 || ball.pos == 12 {
        ball.velocity = -ball.velocity;
        let wall = if ball.pos == 0 { Wall::Left } else { Wall::Right };
        hits.write(WallHit { wall });
    }
}

/// 读者一：放音效——不关心撞的是哪面墙
fn play_sound(mut hits: MessageReader<WallHit>) {
    for _ in hits.read() {
        println!("音效：砰！");
    }
}

/// 读者二：计分——按消息携带的数据区别对待
fn update_score(mut hits: MessageReader<WallHit>, mut score: ResMut<Score>) {
    for hit in hits.read() {
        let (name, points) = match hit.wall {
            Wall::Left => ("普通墙", 1),
            Wall::Right => ("金墙", 3),
        };
        score.0 += points;
        println!("记分牌：{name} +{points}，共 {} 分", score.0);
    }
}
// ANCHOR_END: systems

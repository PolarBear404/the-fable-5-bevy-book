//! Listing 7-3：读者排在写者之前——音效慢一帧，但一条不丢

use bevy::prelude::*;

#[derive(Message)]
struct WallHit;

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
        // 故意反过来：读者先跑，写者后跑
        .add_systems(Update, (play_sound, move_ball).chain());

    for frame in 1..=7 {
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
        ball.velocity = -ball.velocity;
        println!("球：撞墙！");
        hits.write(WallHit);
    }
}

fn play_sound(mut hits: MessageReader<WallHit>) {
    for _ in hits.read() {
        println!("音效：砰！");
    }
}

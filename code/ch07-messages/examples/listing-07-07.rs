//! Listing 7-7：MessageMutator——缓冲条在途中卸力

use bevy::prelude::*;

#[derive(Clone, Copy)]
enum Rail {
    Left,
    Right,
}

/// 消息带上撞击力道，供途中修改、供下游读取
#[derive(Message)]
struct RailHit {
    rail: Rail,
    force: i32,
}

#[derive(Component)]
struct Car {
    pos: i32,
    velocity: i32,
}

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_message::<RailHit>()
        .add_systems(Startup, spawn_car)
        // 写 → 改 → 读，三段流水线
        .add_systems(Update, (drive, cushion, play_sound).chain());

    for frame in 1..=4 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}
// ANCHOR_END: main

fn spawn_car(mut commands: Commands) {
    commands.spawn(Car {
        pos: 0,
        velocity: 4,
    });
}

fn drive(mut car: Single<&mut Car>, mut hits: MessageWriter<RailHit>) {
    car.pos += car.velocity;
    if car.pos == 0 || car.pos == 4 {
        let rail = if car.pos == 0 { Rail::Left } else { Rail::Right };
        hits.write(RailHit {
            rail,
            force: car.velocity.abs(),
        });
        car.velocity = -car.velocity;
    }
}

// ANCHOR: cushion
/// 维修工给右护栏装了缓冲条：途经的碰撞消息被吸掉 2 点力道
fn cushion(mut hits: MessageMutator<RailHit>) {
    for hit in hits.read() {
        if let Rail::Right = hit.rail {
            hit.force -= 2;
            println!("缓冲条：噗——卸掉 2 点力道");
        }
    }
}
// ANCHOR_END: cushion

/// DJ 拿到的是修改后的消息，音量跟着力道走
fn play_sound(mut hits: MessageReader<RailHit>) {
    for hit in hits.read() {
        println!("DJ：砰！（力道 {}）", hit.force);
    }
}

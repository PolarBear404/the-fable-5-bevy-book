//! Listing 7-1：最小消息闭环——阿莱撞护栏，DJ 放音效

use bevy::prelude::*;

/// 消息：有车撞上了护栏
#[derive(Message)]
struct RailHit;

/// 碰碰车：在一条小直道上来回冲撞
#[derive(Component)]
struct Car {
    pos: i32,
    velocity: i32,
}

fn main() {
    let mut app = App::new();
    app.add_message::<RailHit>()
        .add_systems(Startup, spawn_car)
        .add_systems(Update, (drive, play_sound).chain());

    for frame in 1..=3 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}

fn spawn_car(mut commands: Commands) {
    // 直道只有 4 格，车速也是 4：新手阿莱每一帧都结结实实撞在护栏上
    commands.spawn(Car {
        pos: 0,
        velocity: 4,
    });
}

/// 写者：开车，撞上护栏就反弹，并写一条消息
fn drive(mut car: Single<&mut Car>, mut hits: MessageWriter<RailHit>) {
    car.pos += car.velocity;
    if car.pos == 0 || car.pos == 4 {
        car.velocity = -car.velocity;
        println!("阿莱：撞上护栏！");
        hits.write(RailHit);
    }
}

/// 读者：每读到一条碰撞消息，放一声音效
fn play_sound(mut hits: MessageReader<RailHit>) {
    for _ in hits.read() {
        println!("DJ：砰！");
    }
}

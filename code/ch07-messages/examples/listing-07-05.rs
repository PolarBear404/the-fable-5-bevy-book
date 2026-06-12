//! Listing 7-5：错过即丢——打瞌睡的记分员每三帧才醒一次

use bevy::prelude::*;

/// 消息带上碰撞序号，丢没丢一对便知
#[derive(Message)]
struct RailHit {
    n: u32,
}

#[derive(Component)]
struct Car {
    pos: i32,
    velocity: i32,
}

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_message::<RailHit>().add_systems(Startup, spawn_car);
    app.add_systems(
        Update,
        (
            drive,
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

// ANCHOR: systems
fn spawn_car(mut commands: Commands) {
    // 还是那条 4 格直道：阿莱每一帧都在撞
    commands.spawn(Car {
        pos: 0,
        velocity: 4,
    });
}

/// 写者：每次撞护栏，消息编号递增
fn drive(mut car: Single<&mut Car>, mut hits: MessageWriter<RailHit>, mut count: Local<u32>) {
    car.pos += car.velocity;
    if car.pos == 0 || car.pos == 4 {
        car.velocity = -car.velocity;
        *count += 1;
        println!("阿莱：第 {} 次撞护栏", *count);
        hits.write(RailHit { n: *count });
    }
}

/// 读者：醒来时把缓冲里还剩的消息一次读完
fn drowsy_scorer(mut hits: MessageReader<RailHit>) {
    let heard: Vec<u32> = hits.read().map(|hit| hit.n).collect();
    println!("记分员醒来：记下第 {heard:?} 次");
}
// ANCHOR_END: systems

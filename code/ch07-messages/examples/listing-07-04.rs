//! Listing 7-4：DJ 排在车手之前——音效慢一拍，但一声不少

use bevy::prelude::*;

#[derive(Message)]
struct RailHit;

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
        // 故意接反：DJ 先跑，车手后跑
        .add_systems(Update, (play_sound, drive).chain());

    for frame in 1..=5 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}
// ANCHOR_END: main

fn spawn_car(mut commands: Commands) {
    // 直道加长到 8 格：隔一帧撞一次，错拍看得更清楚
    commands.spawn(Car {
        pos: 0,
        velocity: 4,
    });
}

fn drive(mut car: Single<&mut Car>, mut hits: MessageWriter<RailHit>) {
    car.pos += car.velocity;
    if car.pos == 0 || car.pos == 8 {
        car.velocity = -car.velocity;
        println!("阿莱：撞上护栏！");
        hits.write(RailHit);
    }
}

fn play_sound(mut hits: MessageReader<RailHit>) {
    for _ in hits.read() {
        println!("DJ：砰！");
    }
}

//! Listing 7-3：一写多读——DJ 与记分员各自读到全部碰撞

use bevy::prelude::*;

// ANCHOR: message
/// 直道两端的护栏：促销期间，撞右边的金护栏算 3 分
#[derive(Clone, Copy)]
enum Rail {
    Left,
    Right,
}

/// 消息升级：除了"撞了"，还带上撞的是哪边
#[derive(Message)]
struct RailHit {
    rail: Rail,
}
// ANCHOR_END: message

/// 阿莱的总分
#[derive(Resource, Default)]
struct Score(u32);

#[derive(Component)]
struct Car {
    pos: i32,
    velocity: i32,
}

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_message::<RailHit>()
        .init_resource::<Score>()
        .add_systems(Startup, spawn_car)
        // 写者在前；两个读者之间不分先后，可以并行
        .add_systems(Update, (drive, (play_sound, update_score)).chain());

    for frame in 1..=4 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}
// ANCHOR_END: main

fn spawn_car(mut commands: Commands) {
    // 还是 4 格直道：左右护栏轮流挨撞
    commands.spawn(Car {
        pos: 0,
        velocity: 4,
    });
}

// ANCHOR: systems
/// 写者只管报告事实，不关心谁在听
fn drive(mut car: Single<&mut Car>, mut hits: MessageWriter<RailHit>) {
    car.pos += car.velocity;
    if car.pos == 0 || car.pos == 4 {
        car.velocity = -car.velocity;
        let rail = if car.pos == 0 { Rail::Left } else { Rail::Right };
        hits.write(RailHit { rail });
    }
}

/// 读者一：DJ 放音效——不关心撞的是哪边
fn play_sound(mut hits: MessageReader<RailHit>) {
    for _ in hits.read() {
        println!("DJ：砰！");
    }
}

/// 读者二：记分员——按消息携带的数据区别对待
fn update_score(mut hits: MessageReader<RailHit>, mut score: ResMut<Score>) {
    for hit in hits.read() {
        let (name, points) = match hit.rail {
            Rail::Left => ("普通护栏", 1),
            Rail::Right => ("金护栏", 3),
        };
        score.0 += points;
        println!("记分员：{name} +{points}，阿莱共 {} 分", score.0);
    }
}
// ANCHOR_END: systems

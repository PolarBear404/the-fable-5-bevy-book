//! Listing 7-6：实习 DJ 与老 DJ——"有就响一声"的两种写法

use bevy::prelude::*;

/// 消息带上车手名字
#[derive(Message)]
struct RailHit {
    driver: &'static str,
}

#[derive(Component)]
struct Car {
    driver: &'static str,
    pos: i32,
    velocity: i32,
}

// ANCHOR: main
fn main() {
    let mut app = App::new();
    app.add_message::<RailHit>()
        .add_systems(Startup, spawn_cars)
        .add_systems(
            Update,
            (
                drive,
                rookie_dj,
                veteran_dj,
                // 没有新碰撞的帧，灯光师整帧不跑
                light_show.run_if(on_message::<RailHit>),
            )
                .chain(),
        );

    for frame in 1..=2 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}
// ANCHOR_END: main

fn spawn_cars(mut commands: Commands) {
    // 三辆车并排冲撞：每一帧三声巨响
    for driver in ["阿莱", "小柔", "老高"] {
        commands.spawn(Car {
            driver,
            pos: 0,
            velocity: 4,
        });
    }
}

fn drive(mut cars: Query<&mut Car>, mut hits: MessageWriter<RailHit>) {
    for mut car in &mut cars {
        car.pos += car.velocity;
        if car.pos == 0 || car.pos == 4 {
            car.velocity = -car.velocity;
            hits.write(RailHit { driver: car.driver });
        }
    }
}

// ANCHOR: djs
/// 实习 DJ：逐条响应——一帧三撞就放三声，吵翻天
fn rookie_dj(mut hits: MessageReader<RailHit>) {
    for hit in hits.read() {
        println!("实习 DJ：给{}来一声砰！", hit.driver);
    }
}

/// 老 DJ：有就响一声，几条不管——is_empty + clear 模式
fn veteran_dj(mut hits: MessageReader<RailHit>) {
    if !hits.is_empty() {
        hits.clear();
        println!("老 DJ：砰！一声就够。");
    }
}

/// 灯光师：函数体连消息都不碰，run_if(on_message) 替他把关
fn light_show() {
    println!("灯光师：全场闪一下");
}
// ANCHOR_END: djs

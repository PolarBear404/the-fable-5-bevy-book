//! 第 7 章综合示例：碰碰车场开业
//! 三位车手各自撞护栏写消息；DJ、灯光师、记分员互不相识地各读各的；
//! 撞满 10 次，老板写一条 AppExit，全场打烊

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

/// 消息：有车撞上了护栏，带上车手名字
#[derive(Message)]
struct RailHit {
    driver: &'static str,
}

/// 全场累计撞击数
#[derive(Resource, Default)]
struct HitCount(u32);

#[derive(Component)]
struct Car {
    driver: &'static str,
    pos: i32,
    velocity: i32,
}

fn main() {
    App::new()
        // 真正的主循环：每 100 毫秒跑一帧，直到读到 AppExit
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_message::<RailHit>()
        .init_resource::<HitCount>()
        .add_systems(Startup, opening)
        .add_systems(
            Update,
            (
                banner,
                drive,
                play_sound,
                // 没有新碰撞的帧，灯光师整帧不跑
                light_show.run_if(on_message::<RailHit>),
                update_score,
                close_up,
            )
                .chain(),
        )
        .run();

    println!("（run() 返回，进程结束）");
}

fn opening(mut commands: Commands) {
    println!("碰碰车场开业！三位车手上场。");
    // 12 格的大直道，三辆车速度各异，撞护栏的节奏就此岔开
    for (driver, velocity) in [("阿莱", 4), ("小柔", 6), ("老高", 3)] {
        commands.spawn(Car {
            driver,
            pos: 0,
            velocity,
        });
    }
}

/// 报幕：让"同一帧"在输出里看得见
fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

/// 唯一的写者：谁撞护栏就替谁写一条，同一帧可能写多条
fn drive(mut cars: Query<&mut Car>, mut hits: MessageWriter<RailHit>) {
    for mut car in &mut cars {
        car.pos += car.velocity;
        if car.pos == 0 || car.pos == 12 {
            car.velocity = -car.velocity;
            hits.write(RailHit { driver: car.driver });
        }
    }
}

/// DJ：同一帧几辆车齐撞也只响一声——is_empty + clear 模式
fn play_sound(mut hits: MessageReader<RailHit>) {
    if !hits.is_empty() {
        hits.clear();
        println!("DJ：砰！");
    }
}

/// 灯光师：函数体不用碰消息，run_if(on_message) 替他把关
fn light_show() {
    println!("灯光师：闪一下");
}

/// 记分员：一条消息记一笔，认得每位车手
fn update_score(mut hits: MessageReader<RailHit>, mut count: ResMut<HitCount>) {
    for hit in hits.read() {
        count.0 += 1;
        println!("记分员：{}撞护栏，全场第 {} 次", hit.driver, count.0);
    }
}

/// 老板：撞满 10 次该歇业了——写一条 AppExit
fn close_up(count: Res<HitCount>, mut exit: MessageWriter<AppExit>) {
    if count.0 >= 10 {
        println!("老板：撞满 10 次，打烊！");
        exit.write(AppExit::Success);
    }
}

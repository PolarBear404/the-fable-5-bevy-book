//! 第 7 章综合示例：弹球馆开业
//! 三颗球各自撞墙写消息；音效、灯光、记分牌互不相识地各读各的；
//! 满 10 分写一条 AppExit，弹球馆自己打烊

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

/// 消息：有球撞上了墙
#[derive(Message)]
struct WallHit;

/// 总分
#[derive(Resource, Default)]
struct Score(u32);

#[derive(Component)]
struct Ball {
    pos: i32,
    velocity: i32,
}

fn main() {
    App::new()
        // 真正的主循环：每 100 毫秒跑一帧，直到读到 AppExit
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .add_message::<WallHit>()
        .init_resource::<Score>()
        .add_systems(Startup, spawn_balls)
        .add_systems(
            Update,
            (
                banner,
                move_balls,
                play_sound,
                // ANCHOR: on_message
                // 没有新碰撞的帧，灯光系统整帧不跑
                flash_lights.run_if(on_message::<WallHit>),
                // ANCHOR_END: on_message
                update_score,
                check_exit,
            )
                .chain(),
        )
        .run();

    println!("（run() 返回，进程结束）");
}

fn spawn_balls(mut commands: Commands) {
    println!("弹球馆开业！三颗球上场。");
    commands.spawn(Ball {
        pos: 0,
        velocity: 3,
    });
    commands.spawn(Ball {
        pos: 0,
        velocity: 4,
    });
    commands.spawn(Ball {
        pos: 0,
        velocity: 6,
    });
}

/// 报幕：让"同一帧"在输出里看得见
fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

/// 唯一的写者：谁撞墙谁写消息，同一帧可能写多条
fn move_balls(mut balls: Query<&mut Ball>, mut hits: MessageWriter<WallHit>) {
    for mut ball in &mut balls {
        ball.pos += ball.velocity;
        if ball.pos == 0 || ball.pos == 12 {
            ball.velocity = -ball.velocity;
            hits.write(WallHit);
        }
    }
}

// ANCHOR: play_sound
/// 音效：同一帧撞几次也只响一声——is_empty + clear 模式
fn play_sound(mut hits: MessageReader<WallHit>) {
    if !hits.is_empty() {
        hits.clear();
        println!("音效：砰！");
    }
}
// ANCHOR_END: play_sound

/// 灯光：靠 run_if(on_message) 把关，函数体不用碰消息
fn flash_lights() {
    println!("灯光：闪烁");
}

/// 记分牌：一条消息记一分，逐条处理
fn update_score(mut hits: MessageReader<WallHit>, mut score: ResMut<Score>) {
    for _ in hits.read() {
        score.0 += 1;
        println!("记分牌：{} 分", score.0);
    }
}

/// 满 10 分：写一条 AppExit，下一次循环检查时 app 退出
fn check_exit(score: Res<Score>, mut exit: MessageWriter<AppExit>) {
    if score.0 >= 10 {
        println!("满 10 分，弹球馆打烊！");
        exit.write(AppExit::Success);
    }
}

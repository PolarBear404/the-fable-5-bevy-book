//! Listing 8-3：run_if——打烊之后，锣照样响，人不再集合

use bevy::prelude::*;

/// 事件：大厅的铜锣响了
#[derive(Event)]
struct GongStruck;

/// 资源：公会是否在营业
#[derive(Resource)]
struct GuildOpen(bool);

fn main() {
    let mut app = App::new();
    app.insert_resource(GuildOpen(true))
        // 守夜人全天在岗：有锣必记
        .add_observer(night_watch)
        // 公会成员只在营业时间响应：run_if 直接挂在 observer 系统上
        .add_observer(assemble.run_if(|open: Res<GuildOpen>| open.0))
        .add_systems(Update, gong_script);

    for frame in 1..=2 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}

/// 司仪：第 1 帧营业中敲锣，第 2 帧打烊后再敲一声
fn gong_script(mut open: ResMut<GuildOpen>, mut commands: Commands, mut frame: Local<u32>) {
    *frame += 1;
    match *frame {
        1 => {
            println!("司仪：公会开张，敲锣——");
            commands.trigger(GongStruck);
        }
        2 => {
            println!("司仪：打烊了，再敲一声——");
            open.0 = false;
            commands.trigger(GongStruck);
        }
        _ => {}
    }
}

/// 带条件的 observer：营业时间才集合
fn assemble(_gong: On<GongStruck>) {
    println!("公会成员：锣响了，集合！");
}

/// 无条件的 observer：守夜人什么时候都记账
fn night_watch(_gong: On<GongStruck>, mut count: Local<u32>) {
    *count += 1;
    println!("守夜人：记下了，今天第 {} 声锣。", *count);
}

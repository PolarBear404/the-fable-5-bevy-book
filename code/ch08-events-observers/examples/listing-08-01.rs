//! Listing 8-1：第一个 Observer——敲锣，全公会立刻听见

use bevy::prelude::*;

/// 事件：大厅的铜锣响了
#[derive(Event)]
struct GongStruck;

fn main() {
    let mut app = App::new();
    app.add_observer(hear_gong)
        .add_systems(Update, strike_gong);

    println!("—— 第 1 帧 ——");
    app.update();
}

/// 司仪：用 trigger 敲响铜锣
fn strike_gong(mut commands: Commands) {
    println!("司仪：各位安静——");
    commands.trigger(GongStruck);
    println!("司仪：（锣槌已经挥出去了）");
}

/// Observer：第一个参数必须是 On<事件类型>
fn hear_gong(_gong: On<GongStruck>) {
    println!("公会成员：锣响了，集合！");
}

//! Listing 7-8：同型消息一写一读挤进同一个系统——首帧 panic

use bevy::prelude::*;

#[derive(Message)]
struct RailHit;

fn main() {
    let mut app = App::new();
    app.add_message::<RailHit>();
    // 想在一个系统里既写又读同一种消息——借用冲突
    app.add_systems(Update, impossible);
    app.update();
}

/// MessageWriter 内部是 ResMut，MessageReader 内部是 Res——同一资源一写一读
fn impossible(_writer: MessageWriter<RailHit>, _reader: MessageReader<RailHit>) {}

//! Listing 7-2：忘记 add_message——写者就位，通道没开，首帧 panic

use bevy::prelude::*;

#[derive(Message)]
struct RailHit;

fn main() {
    let mut app = App::new();
    // 这里"忘了"调用 add_message::<RailHit>()
    app.add_systems(Update, drive);
    app.update();
}

fn drive(mut hits: MessageWriter<RailHit>) {
    hits.write(RailHit);
}

//! Listing 5-3：忘了 insert_resource——首帧 panic

use bevy::prelude::*;

#[derive(Resource)]
struct Score(u32);

// ANCHOR: main
fn main() {
    let mut app = App::new();
    // 这里本该有 app.insert_resource(Score(0))
    app.add_systems(Update, announce);
    app.update();
}

fn announce(score: Res<Score>) {
    println!("报靶员：累计 {} 分", score.0);
}
// ANCHOR_END: main

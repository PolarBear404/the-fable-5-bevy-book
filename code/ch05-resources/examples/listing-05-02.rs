//! Listing 5-2：同一系统对同一资源一读一写——B0002，首帧 panic

use bevy::prelude::*;

#[derive(Resource)]
struct Score(u32);

fn main() {
    let mut app = App::new();
    app.insert_resource(Score(0))
        .add_systems(Update, impossible);
    app.update();
}

// ANCHOR: conflict
/// 想一边翻倍一边读旧值——同一资源既要 Res 又要 ResMut
fn impossible(old: Res<Score>, mut score: ResMut<Score>) {
    score.0 += old.0;
}
// ANCHOR_END: conflict

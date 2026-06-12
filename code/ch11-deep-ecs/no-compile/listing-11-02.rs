//! Listing 11-2：行不通——&mut World 旁边坐不下普通参数

use bevy::prelude::*;

// ANCHOR: mixed
/// 既要整个世界，又要一份命令队列？
fn take_census(world: &mut World, mut commands: Commands) {
    let households = world.entities().count_spawned();
    commands.spawn(Name::new(format!("公告：全镇 {households} 个实体")));
}

fn main() {
    App::new().add_systems(Update, take_census).run();
}
// ANCHOR_END: mixed

//! Listing 9-5：行不通——关系组件不可变，&mut ChildOf 过不了编译

use bevy::prelude::*;

/// 标记：这是一辆车
#[derive(Component)]
struct Wagon;

fn main() {
    App::new().add_systems(Update, transfer_everyone).run();
}

// ANCHOR: transfer
/// 全员换乘：直接改写 ChildOf 的目标——行不通
fn transfer_everyone(mut crew: Query<&mut ChildOf>, new_wagon: Single<Entity, With<Wagon>>) {
    for mut child_of in &mut crew {
        child_of.0 = *new_wagon;
    }
}
// ANCHOR_END: transfer

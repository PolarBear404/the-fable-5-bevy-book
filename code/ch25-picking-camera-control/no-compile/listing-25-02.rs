//! Listing 25-2：这段故意编不过。`Pointer<Drag>::delta` 是屏幕像素 `Vec2`，不是世界坐标。

use bevy::prelude::*;

// ANCHOR: bad
fn bad_drag(event: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(event.entity).unwrap();
    transform.translation += event.delta;
}
// ANCHOR_END: bad

//! Listing 27-4：忘了 init_gizmo_group 的下场——自定义组没登记就用，运行期直接翻脸。

use bevy::color::palettes::css::GOLD;
use bevy::prelude::*;

// ANCHOR: all
/// 幽灵线：定义了组，却忘了在 App 上登记
#[derive(Default, Reflect, GizmoConfigGroup)]
struct GhostLines;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 这里本该有一行 .init_gizmo_group::<GhostLines>()
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .add_systems(Update, chalk_ghost)
        .run();
}

fn chalk_ghost(mut ghost: Gizmos<GhostLines>) {
    ghost.circle_2d(Vec2::ZERO, 100.0, GOLD);
}
// ANCHOR_END: all

//! Listing 28-2：量尺的什么时候来——ComputedNode 是布局系统的账本，
//! 布局在 PostUpdate 才跑，来早了账上全是零。

use bevy::prelude::*;
use bevy::ui::UiSystems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, measure_at_startup).chain())
        .add_systems(Update, measure_in_update)
        .add_systems(
            PostUpdate,
            // 排在布局之后，账本刚记完就来看
            measure_after_layout.after(UiSystems::Layout),
        )
        .run();
}

#[derive(Component)]
struct SignBoard;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        SignBoard,
        Node {
            width: px(320),
            height: px(140),
            ..default()
        },
        BackgroundColor(Color::srgb(0.55, 0.17, 0.12)),
    ));
}

// ANCHOR: measure
/// Startup 里 spawn 完立刻量：布局系统一次都还没跑
fn measure_at_startup(board: Single<&ComputedNode, With<SignBoard>>) {
    println!("开台（Startup）量一次：size = {:?}", board.size());
}

/// Update 里量：头一帧的 Update 也在布局之前——照样是零
fn measure_in_update(board: Single<&ComputedNode, With<SignBoard>>, mut tick: Local<u32>) {
    *tick += 1;
    if *tick <= 2 {
        println!("第 {} 帧 Update 量一次：size = {:?}", *tick, board.size());
    }
}

/// PostUpdate、排在 UiSystems::Layout 之后量：同一帧内账已经上了
fn measure_after_layout(board: Single<&ComputedNode, With<SignBoard>>, mut done: Local<bool>) {
    if !*done {
        *done = true;
        println!(
            "第 1 帧布局之后量一次：size = {:?}（物理像素）",
            board.size()
        );
        println!(
            "  乘 inverse_scale_factor({}) 换回逻辑像素 = {:?}",
            board.inverse_scale_factor(),
            board.size() * board.inverse_scale_factor()
        );
    }
}
// ANCHOR_END: measure

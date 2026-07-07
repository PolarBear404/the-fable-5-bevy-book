//! Listing 27-2：粉线字典——开演前的台面记号一屏画全。
//! ↑/↓ 拨站位圈的折线段数，亲眼看"圆"是几段直线拼的。

use bevy::color::palettes::css::*;
use bevy::prelude::*;

// ANCHOR: resolution_res
/// 站位圈的折线段数：检场拿它做实验
#[derive(Resource)]
struct CircleResolution(u32);

const RESOLUTION_STOPS: [u32; 6] = [4, 6, 8, 16, 32, 64];
// ANCHOR_END: resolution_res

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CircleResolution(32))
        .add_systems(Startup, setup)
        .add_systems(Update, (chalk_floor_plan, tune_resolution))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    println!("检场：台面记号画齐了。上下方向键拨站位圈的段数。");
}

// ANCHOR: floor_plan
/// 开演前的台面记号：一屏画全 2D 词汇表
fn chalk_floor_plan(mut gizmos: Gizmos, resolution: Res<CircleResolution>) {
    // 台面格线：16×9 格，每格 80 像素，外沿也描上
    gizmos
        .grid_2d(Isometry2d::IDENTITY, UVec2::new(16, 9), Vec2::splat(80.0), GRAY)
        .outer_edges();

    // 站位圈：主角站中央偏左；段数由资源拨——圆其实是折线拼的
    gizmos
        .circle_2d(Vec2::new(-240.0, 60.0), 110.0, GOLD)
        .resolution(resolution.0);

    // 走位弧线：绕着站位圈外一圈，扫过四分之一圆
    gizmos.arc_2d(
        Isometry2d::new(Vec2::new(-240.0, 60.0), Rot2::degrees(45.0)),
        std::f32::consts::FRAC_PI_2,
        170.0,
        ORANGE,
    );

    // 软垫落点：圆角矩形，翻跟头摔在这儿
    gizmos
        .rounded_rect_2d(Vec2::new(220.0, -120.0), Vec2::new(180.0, 120.0), SEA_GREEN)
        .corner_radius(24.0);

    // 道具落点：一个叉，东西搬回来照这儿放
    gizmos.cross_2d(Vec2::new(220.0, 120.0), 20.0, ORANGE_RED);

    // 椭圆场记桌：斜着摆
    gizmos.ellipse_2d(
        Isometry2d::new(Vec2::new(-240.0, -180.0), Rot2::degrees(20.0)),
        Vec2::new(90.0, 40.0),
        PLUM,
    );

    // 上场门方向：单头箭头；下场门：双头（两边都走）
    gizmos.arrow_2d(Vec2::new(-560.0, 0.0), Vec2::new(-430.0, 0.0), YELLOW);
    gizmos
        .arrow_2d(Vec2::new(560.0, 0.0), Vec2::new(430.0, 0.0), LIME)
        .with_double_end()
        .with_tip_length(16.0);

    // 幕布沿线：一条渐变线，从翼幕红到台心金
    gizmos.line_gradient_2d(Vec2::new(-640.0, 300.0), Vec2::new(640.0, 300.0), RED, GOLD);

    // 检场自己的巡台路线：折线一笔连过去，最后闭合成圈
    gizmos.linestrip_2d(
        [
            Vec2::new(-560.0, -260.0),
            Vec2::new(-240.0, -80.0),
            Vec2::new(60.0, -200.0),
            Vec2::new(340.0, -60.0),
        ],
        AQUA,
    );
}
// ANCHOR_END: floor_plan

// ANCHOR: tune
/// ↑/↓ 在档位表里换挡：4 段的"圆"和 64 段的圆是两种东西
fn tune_resolution(keyboard: Res<ButtonInput<KeyCode>>, mut resolution: ResMut<CircleResolution>) {
    let index = RESOLUTION_STOPS.iter().position(|&n| n == resolution.0);
    let Some(index) = index else { return };
    let next = if keyboard.just_pressed(KeyCode::ArrowUp) {
        (index + 1).min(RESOLUTION_STOPS.len() - 1)
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        index.saturating_sub(1)
    } else {
        return;
    };
    resolution.0 = RESOLUTION_STOPS[next];
    println!("检场：站位圈改成 {} 段折线。", resolution.0);
}
// ANCHOR_END: tune

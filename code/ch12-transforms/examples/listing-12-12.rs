//! Listing 12-12：观测站的几何课——Rect、Circle 与 Annulus
//! 几何原语是纯数据：描述形状、回答几何问题，渲染是后面章节才给它们的差事

use bevy::math::primitives::{Annulus, Circle, Measured2d};
use bevy::math::{Rect, Vec2};

fn main() {
    // ANCHOR: rect
    // 取景框：以原点为中心，宽 560、高 320 的矩形
    let viewfinder = Rect::from_center_size(Vec2::ZERO, Vec2::new(560.0, 320.0));
    println!("取景框中心 {}，半尺寸 {}", viewfinder.center(), viewfinder.half_size());

    let earth = Vec2::new(240.0, 0.0);
    let comet = Vec2::new(-120.0, 320.0);
    println!("地球 {earth} 在镜头里吗？{}", viewfinder.contains(earth));
    println!("彗星 {comet} 在镜头里吗？{}", viewfinder.contains(comet));
    // ANCHOR_END: rect

    // ANCHOR: circle
    // 地球轨道抽象成一个圆：能直接报直径、周长、面积
    let orbit = Circle::new(240.0);
    println!(
        "地球轨道：直径 {}，周长约 {:.0}，圈住面积约 {:.0}",
        orbit.diameter(),
        orbit.perimeter(),
        orbit.area()
    );
    // ANCHOR_END: circle

    // ANCHOR: annulus
    // 小行星带：内径 300、外径 380 的环带
    let belt = Annulus::new(300.0, 380.0);
    println!("小行星带厚度 {}", belt.thickness());

    // closest_point 把任何点拉回形状上：越界的小行星各自归位
    let outer_stray = Vec2::new(450.0, 0.0); // 飘出外缘
    let inner_stray = Vec2::new(120.0, 160.0); // 坠向内缘
    println!("外逃的小行星 {outer_stray} 押回 {}", belt.closest_point(outer_stray));
    println!("内坠的小行星 {inner_stray} 拉回 {}", belt.closest_point(inner_stray));
    // ANCHOR_END: annulus
}

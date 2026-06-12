//! Listing 12-7：方向的护照——Quat、Rot2 与 Dir2 的零碎规矩
//! 同样不需要 App，这些类型在 main 里就能把脾气摸清

use bevy::math::{Dir2, Quat, Rot2, Vec2, Vec3};
use std::f32::consts::FRAC_PI_2;

fn main() {
    // ANCHOR: quat
    // Quat 是 3D 旋转的通用货币：绕 +Z 转 90°，把 +X 送到 +Y
    let quarter = Quat::from_rotation_z(FRAC_PI_2);
    println!("Quat 把 +X 转到 {}", (quarter * Vec3::X).round());
    // ANCHOR_END: quat

    // ANCHOR: rot2
    // Rot2 是 2D 专用的旋转：能用角度直观构造，直接乘 Vec2
    let r = Rot2::degrees(45.0);
    println!("Rot2 把 (1, 0) 转到 {:.3}", r * Vec2::X);
    println!("再把角度读回来：{} 度", r.as_degrees());
    // ANCHOR_END: rot2

    // ANCHOR: dir
    // Dir2 是“归一化护照”：入境查验，零向量当场拒签
    match Dir2::new(Vec2::ZERO) {
        Ok(heading) => println!("方向 {heading}"),
        Err(err) => println!("Dir2 拒签零向量：{err:?}"),
    }

    // (3, 4) 这样的合法向量，入境后长度一律压成 1
    let heading = Dir2::new(Vec2::new(3.0, 4.0)).unwrap();
    println!("(3, 4) 入境后变成 {heading}，长度 {}", heading.length());

    // 方向之间用 slerp 匀速摆头：从正东转向正北，走三分之一
    let third = Dir2::EAST.slerp(Dir2::NORTH, 1.0 / 3.0);
    println!("从正东朝正北摆头三分之一：{:.3}（恰好 30°）", third.as_vec2());
    // ANCHOR_END: dir
}

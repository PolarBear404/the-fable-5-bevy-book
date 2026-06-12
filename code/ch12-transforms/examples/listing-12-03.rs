//! Listing 12-3：图纸上的演算——Vec2 既是位置，也是箭头
//! 不需要 App：bevy 的数学类型就是普通 Rust 值，main 里直接算

use bevy::math::{Vec2, Vec3};

fn main() {
    // ANCHOR: positions
    // 老盖摊开图纸：三个天体的位置，各用一个 Vec2 表示
    let sun = Vec2::new(0.0, 0.0); // 太阳钉在图纸原点
    let earth = Vec2::new(180.0, 0.0); // 地球在太阳正右方 180
    let comet = Vec2::new(-120.0, 160.0); // 彗星在左上方游荡
    // ANCHOR_END: positions

    // ANCHOR: length_distance
    // 位置相减得到“从 B 指向 A 的箭头”，箭头的长度就是距离
    let sun_to_comet = comet - sun;
    println!("老盖量彗星：离太阳 {}，离地球 {}", sun_to_comet.length(), comet.distance(earth));
    // ANCHOR_END: length_distance

    // ANCHOR: normalize
    // 归一化：把箭头压成长度 1，只留方向
    let comet_bearing = sun_to_comet.normalize();
    println!("彗星的方位（单位向量）：{comet_bearing:.1}，长度 {:.2}", comet_bearing.length());
    // ANCHOR_END: normalize

    // ANCHOR: dot
    // 点积：两个单位向量同向得 1，垂直得 0，反向得 -1
    let to_sun = (sun - comet).normalize();
    let dive = Vec2::new(0.6, -0.8); // 第一晚测得的彗星速度方向
    let graze = Vec2::new(0.8, 0.6); // 第二晚测得的速度方向
    println!("第一晚：速度·朝日方向 = {}（全速冲着太阳来！）", dive.dot(to_sun));
    println!("第二晚：速度·朝日方向 = {}（虚惊一场，正横着掠过）", graze.dot(to_sun));
    // ANCHOR_END: dot

    // ANCHOR: lerp_extend
    // lerp：在两点之间按比例取点，0.5 就是正中
    println!("日地连线的中点：{}", sun.lerp(earth, 0.5));

    // 2D 图纸钉进 3D 世界：补一个 z 就是 Vec3，截掉 z 又回到 Vec2
    let comet_3d: Vec3 = comet.extend(5.0);
    println!("彗星入册：{comet_3d}，截回平面：{}", comet_3d.truncate());
    // ANCHOR_END: lerp_extend
}

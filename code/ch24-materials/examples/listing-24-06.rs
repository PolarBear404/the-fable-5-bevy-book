//! Listing 24-6：给没画 UV 的素坯开纹——当场 panic 给你看

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

fn main() {
    // 老鲁 21.5 节风格的素坯：位置、法线、索引——他当年压根没打算贴图，UV 欠奉
    let mut plank = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.75, -0.75, 0.0],
            [0.75, -0.75, 0.0],
            [0.75, 0.75, 0.0],
            [-0.75, 0.75, 0.0],
        ],
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4])
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]));

    println!("老鲁：素坯一块，开纹！");
    plank.generate_tangents().unwrap(); // 💥 切线算法要按 UV 定方向，没 UV 没商量
    println!("老鲁：纹开好了。"); // 走不到这行
}

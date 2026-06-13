//! Listing 20-8：行不通——拆进模块的 Score 没标 pub，邻居用不了
//! （单文件重现拆分现场：mod 块等价于 src/score.rs 与 src/menu.rs 两个文件）

use bevy::prelude::*;

mod score {
    use bevy::prelude::*;

    /// 战果：碎了几片瓦——原样从 main.rs 搬进来，没动一个字
    #[derive(Resource, Default)]
    struct Score(u32);
}

mod menu {
    use bevy::prelude::*;

    use crate::score::Score; // 结算屏要读分数

    pub fn show_curtain(score: Res<Score>) {
        println!("这局砸下 {} 片瓦", score.0);
    }
}

fn main() {}

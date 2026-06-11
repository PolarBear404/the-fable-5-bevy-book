//! 烟雾测试：验证锁定的 Bevy 版本在本机可编译、可启动。
//! 不属于任何章节，只用于脚手架验证。

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

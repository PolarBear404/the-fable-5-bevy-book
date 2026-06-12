# 第一个 Bevy App

上一章把环境装好了，这一章正式写代码：从一个什么都不做的空程序开始，一步步加出主循环、窗口，最后让屏幕上出现一个会动的方块。代码总量不到五十行，但走完这条路，你会建立起贯穿全书的两个心智模型——**App 是 Plugin 的容器，逻辑都是 System**。

本章示例在配套仓库的 `code/ch02-first-app`。各阶段版本这样运行：

```console
cargo run -p ch02-first-app --example listing-02-01   # 运行 Listing 2-1
cargo run -p ch02-first-app                           # 运行最终版（Listing 2-7）
```

本章内容：

- [App 与 System](ch02-01-app-and-systems.md)：最小的 Bevy 程序长什么样，System 怎么注册，为什么它只跑了一次
- [Plugin：App 能力的来源](ch02-02-plugins.md)：主循环和窗口都是插件给的；MinimalPlugins、DefaultPlugins，以及写一个自己的 Plugin
- [一个会动的 Sprite](ch02-03-a-moving-sprite.md)：相机、方块、每帧改写位置——第 1 章那张“表”第一次跑起来

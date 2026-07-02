# Entity 与 Component

第 2 章结尾，World 那张“表”里你亲手添的只有相机和方块两行，而且每一列都是引擎自带的。这一章你会亲手扩建它：定义自己的 Component、成批生成和销毁 Entity、给类型声明“必需组件”，并搞清楚一件第 2 章悬而未决的事——`Commands` 为什么不当场生效，以及它到底什么时候生效。

本章的示例没有窗口、没有渲染，全部输出都打印在控制台上。这是有意的：ECS 的核心是数据的进出，而画面只是数据的一种消费方式。剥掉画面，剩下的正是 Bevy 的心脏。

本章示例在配套仓库的 `code/ch03-entities-components`。各阶段版本这样运行：

```console
cargo run -p ch03-entities-components --example listing-03-01   # 运行 Listing 3-1
cargo run -p ch03-entities-components                           # 运行最终版（Listing 3-7）
```

本章内容：

- [组件与实体](ch03-01-components-and-entities.md)：定义 Component、生成 Entity、打印实体清单；Entity 这个 ID 的真实构造
- [Commands：排队修改世界](ch03-02-commands.md)：为什么结构性修改要排队、队伍何时清空；增删组件与销毁实体
- [Required Components 与 Bundle](ch03-03-required-components-and-bundles.md)：让类型自带“标配组件”——第 2 章相机谜题的答案；以及打包组件的传统手段

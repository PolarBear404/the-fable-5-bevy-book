# Resource——全局唯一数据

第 4 章结尾留了一个缺口：`Local` 是系统私有的记忆，没法共享；可游戏里偏偏到处是**多个系统都要碰的全局数据**——比分、难度设置、随机数种子。它们不挂在任何游戏对象上：比分不属于某只靶子，难度不属于某个玩家。真要塞进第 1 章那张“实体表”，就得开一行“全局行”专门驮着它们，而麻烦在后头：那一行的 `Entity` 怎么传到每个系统手里？为一个全局值把行号、查询折腾一遍，不值当。

Bevy 的回答是把“全局唯一”升格为正式机制：**Resource（资源）——World 里按类型存放、全局唯一的数据**。一个类型在整个 World 里至多存在一份，任何系统报上类型名就能读写它，不用惦记它存在哪、怎么找到它。本章把 Resource 从定义讲到本质：怎么声明和注册、怎么在系统里读写、不存在时会发生什么、初始值从哪来按什么顺序算、它究竟存在 World 的哪个角落，以及它和组件共用的那套变更检测。第 2 章惊鸿一瞥的 `Res<Time>` 也将在本章正式归队。

舞台换到一个打靶场——计分板、场地难度、双倍得分卡，全是资源的主场。示例在配套仓库的 `code/ch05-resources`：

```console
cargo run -p ch05-resources --example listing-05-01   # 运行 Listing 5-1
cargo run -p ch05-resources                           # 运行最终版（Listing 5-9）
```

本章内容：

- [定义与访问 Resource](ch05-01-defining-and-accessing-resources.md)：`#[derive(Resource)]`、`insert_resource` 注册、`Res`/`ResMut` 读写，以及资源版的借用冲突 B0002
- [资源的有无](ch05-02-resource-presence.md)：缺失资源的 panic、`Option<Res<T>>` 探测、用 `Commands` 在运行期插拔资源
- [初始化：init_resource 与 FromWorld](ch05-03-initialization-order.md)：`init_resource` 的让位语义、`FromWorld` 按 World 现状算初始值、初始化顺序为什么重要
- [资源的本质](ch05-04-the-nature-of-resources.md)：资源到底存在哪——资源实体、`IsResource` 与 `insert_resource` 的幕后，兑现第 3 章 0 到 6 号行的悬案
- [资源的变更检测](ch05-05-resource-change-detection.md)：`is_changed`/`is_added`、`set_if_neq`，以及全章的综合示例

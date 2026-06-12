# System 与 Query

第 3 章结尾说过：Query 一直在打杂。其实 System 也一样——整本书写到现在，它顶着“普通函数”这个说法干了不少活，但参数到底能写什么、引擎怎么知道该喂什么、两个参数互相打架了怎么办，全都没有交代。本章把这两位主角扶正：先给 System 一个准确的定义，盘点函数签名里能放的东西；再穷尽 Query 的能力——取数据的全部姿势、过滤器全家、变更检测，以及借用冲突的化解。

读完本章，你看任何 Bevy 系统的函数签名，都应该能立刻说出：它读什么、写什么、作用于哪一类实体。**签名即文档**，这是阅读 Bevy 代码的第一技巧，也是引擎并行调度的全部依据。

本章延续第 3 章的纯逻辑风格：没有窗口，所有输出都在控制台。舞台从地下城搬到一座牧场——羊群、牧羊犬阿黄，还有围栏外的狼。示例在配套仓库的 `code/ch04-systems-queries`：

```console
cargo run -p ch04-systems-queries --example listing-04-01   # 运行 Listing 4-1
cargo run -p ch04-systems-queries                           # 运行最终版（Listing 4-8）
```

本章内容：

- [系统与系统参数](ch04-01-systems-and-system-params.md)：什么样的函数是 System、系统参数家族一览、`Local` 私有状态，以及手动驱动“一帧”的实验手法
- [Query：精确取数据](ch04-02-query-data.md)：可选组件、在场检测、按 `Entity` 直取某一行、“恰好一个”的两种写法
- [过滤器与变更检测](ch04-03-query-filters.md)：`With`/`Without`/`Or` 的组合代数；`Changed`/`Added` 只看“动过的”行
- [借用冲突与 ParamSet](ch04-04-borrow-conflicts-and-paramset.md)：两个查询盯上同一列时会发生什么，以及化解冲突的三板斧

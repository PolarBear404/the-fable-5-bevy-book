# 消息工具箱

写、读、寿命三大件都齐了，这一节把外围工具一次清点完。

## 写端与读端的变体

写端除了 `write`，还有两个变体：消息类型实现了 `Default` 时（空结构体最常见），`write_default()` 省去手写构造；一次写一批用 `write_batch(iter)`，比逐条 `write` 高效。

读端真正值得记住的是一个模式。`MessageReader` 提供 `len()` 和 `is_empty()`，**不消费消息**地查看有多少新货；`clear()` 则把游标一步推到底，已有消息全部作废。两者拼起来，就是"**有就办一次，有几条不重要**"的标准写法——弹球馆的音效系统正该如此，同一帧撞三次墙也只该响一声：

```rust
{{#include ../../code/ch07-messages/src/main.rs:play_sound}}
```

关键细节：`clear()` 清的是**自己的游标**，不是缓冲。消息本体还在 `Messages<M>` 里，其他读者照常逐条读到——音效偷的懒，不会少记记分牌一分钱。下一节的完整示例里两者并排运行，可以亲眼对账。

同一个判断还可以前移到调度层：运行条件 `on_message::<M>` 在有新消息时放行，否则整帧拦下——第 6 章条件清单里欠的那一项，现在补上了。它和 `run_if` 家族的其他成员一样组合自由；内部同样持有一个游标，每次评估顺手消费掉新消息，但游标是条件私有的，不影响任何真正的读者：

```rust
{{#include ../../code/ch07-messages/src/main.rs:on_message}}
```

（两段都截自下一节的完整示例，Listing 7-6 里看全貌。）

## MessageMutator：途中改写

第三种访问方式介于读写之间：`MessageMutator<M>` 像 `MessageReader` 一样带游标逐条遍历，但给出的是 `&mut M`——消息在送达下游之前可以被修改。在写者和读者之间插一道"加工工序"时用它，比如给右墙贴上海绵垫，吸掉一部分撞击力道：

```rust
{{#include ../../code/ch07-messages/examples/listing-07-05.rs:cushion}}
```

流水线照旧用 `chain` 排定：写 → 改 → 读：

```rust
{{#include ../../code/ch07-messages/examples/listing-07-05.rs:main}}
```

<span class="caption">Listing 7-5：MessageMutator——消息在途中被修改</span>

```console
cargo run -p ch07-messages --example listing-07-05
```

```text
—— 第 1 帧 ——
—— 第 2 帧 ——
—— 第 3 帧 ——
海绵垫：吸掉 2 点力道
音效：砰！（力道 2）
—— 第 4 帧 ——
—— 第 5 帧 ——
—— 第 6 帧 ——
音效：砰！（力道 4）
```

第 3 帧撞的是右墙（写者写入力道 4），下游音效拿到的已经是削过的 2；第 6 帧的左墙没贴垫子，原值送达。写者依旧不知道这道工序存在——加工序和加读者一样，是纯增量改动。

三种参数的并发性质各不相同，根源都是上一章见过的借用规则——它们不过是 `Messages<M>` 资源的三种访问姿势：

| 参数 | 对 `Messages<M>` 的访问 | 并发性质 |
|---|---|---|
| `MessageReader<M>` | `Res` + 私有游标 | 读者之间可并行 |
| `MessageWriter<M>` | `ResMut` | 与同型一切读写互斥 |
| `MessageMutator<M>` | `ResMut` + 私有游标 | 与同型一切读写互斥 |

这张表还顺手解释了一个常见的碰壁：同一个系统里同时声明 `MessageWriter<M>` 和 `MessageReader<M>`（同型），等于 `ResMut` 撞 `Res`——第 4 章的借用冲突，系统初始化时直接 panic。解法也还是第 4 章那个：`ParamSet` 把两者错开。不同类型之间随便混用，读 A 写 B 是消息流水线的常态。

## 别的写入口

最后两个入口，都来自"消息缓冲只是个 Resource"这一事实：

- `World` 的直接访问能写消息：`world.write_message(...)`（第 11 章展开 World API）。
- `Commands` 也能写：`commands.write_message(...)`。注意它走的是第 3 章的命令队列——消息**不会当场入仓**，要等命令在同步点应用时才真正写入，第 6 章的三条规则全部适用。它存在的意义是类型擦除的场合（运行期才知道写什么类型）；常规代码一律用 `MessageWriter`，更快也更直白。

> 还有一个为大批量消息准备的 `par_read()`，把读取分摊到多个线程，与第 4 章并行查询同族；等第 34 章谈并行时再见。

工具齐了。回到弹球馆，把整章拼成一个会自己打烊的程序。

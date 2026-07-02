# 消息工具箱

三辆车并排冲撞，每一帧三声巨响。实习 DJ 拿着 Listing 7-1 的剧本逐条放音效——砰、砰、砰，吵翻天。老板捂着耳朵提要求：音效这种事，**“有没有”比“有几条”重要**，一帧只准响一声。

这一节就从这个问题出发，把读端剩下的工具一次清点完。先看三位听众的不同做派：

```rust
{{#include ../../code/ch07-messages/examples/listing-07-06.rs:djs}}
```

接线照旧，唯一的新面孔挂在灯光师身上：

```rust
{{#include ../../code/ch07-messages/examples/listing-07-06.rs:main}}
```

<span class="caption">Listing 7-6：实习 DJ 与老 DJ——“有就响一声”的两种写法</span>

```console
cargo run -p ch07-messages --example listing-07-06
```

```text
—— 第 1 帧 ——
实习 DJ：给阿莱来一声砰！
实习 DJ：给小柔来一声砰！
实习 DJ：给老高来一声砰！
老 DJ：砰！一声就够。
灯光师：全场闪一下
—— 第 2 帧 ——
实习 DJ：给阿莱来一声砰！
实习 DJ：给小柔来一声砰！
实习 DJ：给老高来一声砰！
老 DJ：砰！一声就够。
灯光师：全场闪一下
```

三个人听的是同一条通道，各自的收成对账如下：

- **实习 DJ**：`read()` 逐条遍历——这是 Listing 7-1 的正确写法用错了场合，三撞三响。
- **老 DJ**：`is_empty()` 先**不消费**地探一眼有没有新货，有就 `clear()` 把游标一步推到底、响一声完事。这对组合就是“有就办一次，有几条不管”的标准写法。`len()` 是同族工具，不消费地数个数。
- **灯光师**：把同一个判断**前移到调度层**——运行条件 `on_message::<M>` 在有新消息时放行，否则整帧不跑。第 6 章条件清单里欠的那一项，今天补上。函数体因此干干净净，连 `MessageReader` 参数都不用声明。

关键细节藏在执行顺序里：老 DJ 的 `clear()` 排在灯光师**之前**，灯光师却照样每帧点亮——因为 `clear()` 清的是**老 DJ 自己的游标**，不是缓冲。消息本体还在 `Messages<M>` 里，实习 DJ、灯光师的条件、以及任何别的读者照常读到。`on_message` 同理：条件内部自带一个游标，每次评估顺手消费新消息，但那是条件私有的，不影响任何真正的读者。

写端顺带补全两个变体：消息类型实现了 `Default` 时（空结构体最常见），`write_default()` 省去手写构造；一次写一批用 `write_batch(iter)`，比逐条 `write` 高效。

## 途中改写：MessageMutator

维修工嫌右护栏挨撞太狠，给它装了一条缓冲垫。这道工序既不是写也不是读——它要在消息送达下游**之前**改掉里面的数：

```rust
{{#include ../../code/ch07-messages/examples/listing-07-07.rs:cushion}}
```

`MessageMutator<M>` 像 `MessageReader` 一样带游标逐条遍历，但给出的是 `&mut M`。流水线照旧用 `chain` 排定，写 → 改 → 读：

```rust
{{#include ../../code/ch07-messages/examples/listing-07-07.rs:main}}
```

<span class="caption">Listing 7-7：MessageMutator——缓冲条在途中卸力</span>

```console
cargo run -p ch07-messages --example listing-07-07
```

```text
—— 第 1 帧 ——
缓冲条：噗——卸掉 2 点力道
DJ：砰！（力道 2）
—— 第 2 帧 ——
DJ：砰！（力道 4）
—— 第 3 帧 ——
缓冲条：噗——卸掉 2 点力道
DJ：砰！（力道 2）
—— 第 4 帧 ——
DJ：砰！（力道 4）
```

撞右护栏（第 1、3 帧）的消息写入时力道是 4，DJ 拿到手已被削成 2；左护栏（第 2、4 帧）没装垫子，原值送达。车手照旧不知道这道工序存在——**加工序和加读者一样，是纯增量改动**。

## 一张表与一条红线

三种参数的并发性质各不相同，根源都是第 4 章的借用规则——它们不过是 `Messages<M>` 这个资源的三种访问姿势：

| 参数 | 对 `Messages<M>` 的访问 | 并发性质 |
|---|---|---|
| `MessageReader<M>` | `Res` + 私有游标 | 读者之间可并行 |
| `MessageWriter<M>` | `ResMut` | 与同型一切读写互斥 |
| `MessageMutator<M>` | `ResMut` + 私有游标 | 与同型一切读写互斥 |

这张表划出一条红线：同一个系统里同时声明同型的 `MessageWriter<M>` 和 `MessageReader<M>`，等于 `ResMut` 撞 `Res`。亲眼看一次：

```rust
{{#include ../../code/ch07-messages/examples/listing-07-08.rs}}
```

<span class="caption">Listing 7-8：同型消息一写一读挤进同一个系统——首帧 panic</span>

```console
cargo run -p ch07-messages --example listing-07-08
```

```text
error[B0002]: Res<bevy_ecs::message::messages::Messages<listing_07_08::RailHit>>
in system listing_07_08::impossible conflicts with a previous system parameter.
Consider removing the duplicate access using `Without<IsResource>` to create
disjoint Queries or merging conflicting Queries into a `ParamSet`.
See: https://bevy.org/learn/errors/b0002
```

剧目编号 **B0002**——和第 5 章资源冲突同款，因为消息缓冲**就是**资源。报错原文还替本章第一节的说法验明正身：`MessageReader` 现出了 `Res<Messages<RailHit>>` 的原形；和它冲突的“previous system parameter”没有点名，但这个系统一共就两个参数——正是 `MessageWriter` 里那个 `ResMut<Messages<RailHit>>`。报错开的两张药方里，`Without<IsResource>` 治的是查询撞上资源的场合（第 5 章“资源的本质”解释过），这里没有查询、用不上；对症的是第 4 章的老朋友 `ParamSet`，把两者错开取用。至于**不同类型**之间读写混搭（读 `RailHit` 写 `Combo`），那是消息流水线的常态，毫无问题。

## 别的写入口

最后两个写入口，都来自“消息缓冲只是个资源”这一事实：

- `World` 的直接访问能写消息：`world.write_message(...)`（World API 在第 11 章展开）。
- `Commands` 也能写：`commands.write_message(...)`。注意它走的是第 3 章的命令队列——消息**不会当场入仓**，要等命令在同步点应用时才真正写入，第 6 章的三条规则全部适用。它的用武之地是类型擦除的场合（运行期才决定写什么类型）；常规代码一律用 `MessageWriter`，更快也更直白。

> 大批量消息还有一个 `par_read()`，把读取分摊到多个线程，与第 4 章的并行查询同族，第 34 章谈并行时再见。

工具齐了。回到碰碰车场，把全章拼成一个会自己打烊的程序。

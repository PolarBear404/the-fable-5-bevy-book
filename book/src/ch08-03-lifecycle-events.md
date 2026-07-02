# 生命周期事件：引擎替你敲锣

到了附魔台，主菜上桌。先想清楚一件事：在 ECS 里，“装备”这个动作的本质是什么？第 3 章就有答案——往实体上 `insert` 组件；卸下就是 `remove`。那么“装上火焰附魔的瞬间点火”翻译过来就是：**`Flaming` 组件被插入的那一刻，运行一段逻辑**。

可是该由谁来 `trigger` 呢？谁都不用。组件的插入、移除、销毁，引擎全都看在眼里，并且替你敲锣——这就是**生命周期事件**（lifecycle events）：一组由引擎自动触发的内置 EntityEvent。盯住它们的 observer 写法和上一节别无二致：

```rust
{{#include ../../code/ch08-events-observers/examples/listing-08-06.rs}}
```

<span class="caption">Listing 8-6：生命周期事件——附魔台上，装上点火、卸下熄灭</span>

```console
cargo run -p ch08-events-observers --example listing-08-06
```

```text
—— 第 1 帧 ——
附魔师：上火焰附魔——
小芙的长戟 轰地烧了起来！
—— 第 2 帧 ——
附魔师：拆掉附魔——
小芙的长戟 上的火光熄灭了。
```

注意 `Flaming` 的定义：它就是个普通组件，全程没有任何人写过 `trigger`。新鲜的只有 observer 的参数 `On<Add, Flaming>`——`On` 的第二个泛型参数是**过滤器**：`Add` 事件每时每刻都在为各种组件触发，这个 observer 只认“`Flaming` 被加上”这一种。事件本身是 EntityEvent，`add.entity` 指向那把长戟，后面照常 `Query::get` 拿数据。

> 过滤器一次可以填多个组件，如 `On<Add, (Flaming, Frozen)>`——语义是**其中任一**组件被加上就触发，而不是“两个都加上”。和 Query 过滤器的 `With` 直觉相反，别在这里栽跟头。

## 五件套点名

`Add` 和 `Remove` 只是五分之二。组件的一生总共有五个节点，全部内置为生命周期事件：

| 事件 | 何时触发 |
|---|---|
| `Add` | 组件加到**原本没有它**的实体上 |
| `Insert` | 组件被写入，**不论先前有没有**（每次 `insert` 都算） |
| `Discard` | 组件值即将被清退——被新值顶替**或**被移除都算 |
| `Remove` | 组件从实体上**彻底离身**（含实体销毁） |
| `Despawn` | 实体销毁时，为它身上的每个组件触发 |

五个名字两两配对再加一个收尾：`Add`/`Remove` 盯的是**有无**的变化，`Insert`/`Discard` 盯的是**值**的写入与清退，`Despawn` 单管销毁。表格背不住没关系，让一把长戟把五幕全演一遍：

```rust
{{#include ../../code/ch08-events-observers/examples/listing-08-07.rs}}
```

<span class="caption">Listing 8-7：五个生命周期事件同台——一把长戟的五幕剧</span>

```console
cargo run -p ch08-events-observers --example listing-08-07
```

```text
—— 第 1 帧 ——
附魔师：第一次附魔，威力 3。
  [Add]     附魔初次上身，威力 3
  [Insert]  新值写入完毕，当前威力 3
—— 第 2 帧 ——
附魔师：重新附魔，威力升到 9。
  [Discard] 旧值即将清退，此刻还能读到威力 3
  [Insert]  新值写入完毕，当前威力 9
—— 第 3 帧 ——
附魔师：把附魔拆下来。
  [Discard] 旧值即将清退，此刻还能读到威力 9
  [Remove]  附魔彻底离身，临走前威力 9
—— 第 4 帧 ——
附魔师：再附一次，威力 6。
  [Add]     附魔初次上身，威力 6
  [Insert]  新值写入完毕，当前威力 6
—— 第 5 帧 ——
附魔师：这把回炉重造！
  [Despawn] 整把武器进了熔炉
  [Discard] 旧值即将清退，此刻还能读到威力 6
  [Remove]  附魔彻底离身，临走前威力 6
```

五幕逐一对账：

- **第 1 帧（首次附魔）**：`Add` 与 `Insert` 先后响起——从无到有，两类事件都算数。
- **第 2 帧（重新附魔）**：`Add` 沉默了，因为组件早就在身上；响的是 `Discard` + `Insert`。细看两行的威力值：`Discard` 读到的是**旧值 3**——它运行在新值写入之前，给你最后一次接触旧数据的机会；`Insert` 读到的已经是新值 9。
- **第 3 帧（拆除）**：`Discard` 先告别旧值，`Remove` 送它彻底离身。两位 observer 此刻都还查得到组件——移除类事件全都跑在数据真正消失**之前**。
- **第 4 帧（重新装上）**：`Add` 又响了。它和 `Remove` 是一对“有无”哨兵：彻底离身过，再上身就算初次。
- **第 5 帧（销毁）**：销毁走另一条流水线——`Despawn` 率先响起，随后 `Discard`、`Remove` 依次跟上。对 observer 而言“销毁也是一种移除”，所以 Listing 8-6 的熄火逻辑不用为熔炉单写一份。

还有一幕**没有**上演，值得专门点名：通过 `Query<&mut Flaming>` 修改威力数值，五个事件**一个都不触发**。生命周期事件盯的是组件的来去与写入，不是字段的变动——“给附魔调个数”不算“重新附魔”。要捕捉值的变化，工具是第 4 章的 `Changed<T>` 过滤器。

> **拉模式的同款备件**：如果你更想用消息的节奏处理移除——攒一帧、统一清算——系统参数 `RemovedComponents<T>` 提供 `Remove` 事件的缓冲版，用法和 `MessageReader` 一个模子。官方示例 `ecs/removal_detection.rs` 是现成参考。

## 联动：一次 insert，一串反应

生命周期事件配上“observer 也能发命令”，就解锁了大纲里写的那种玩法——**装上一个组件，即时触发一串联动**：

```rust
{{#include ../../code/ch08-events-observers/examples/listing-08-08.rs}}
```

<span class="caption">Listing 8-8：连锁联动——一次 insert，一串反应，全在同一帧</span>

```console
cargo run -p ch08-events-observers --example listing-08-08
```

```text
—— 第 1 帧 ——
附魔师：上火焰附魔——
  第一环：检测到火焰附魔，给它配上火光。
  第二环：小芙的长戟 亮起来了！
巡场员：本帧收工时，小芙的长戟 已经在发光了。
```

链条是这样咬合的：附魔师 `insert(Flaming)` → 同步点应用，`Add` 触发，第一环 observer 运行，它又排入 `insert(Glowing)` → 这条新命令在**同一个同步点**里接着被应用，再触发第二环。observer 排出的命令会被递归处理到队列见底为止，所以巡场员——一个排在附魔师之后的普通系统——上场时看到的已经是尘埃落定的世界。同样的三级联动换成 Message 来写，就是三个系统接力读写两条通道，最坏要三帧才能传完；这里从头到尾没出第 1 帧。

连锁的另一面是失控：observer 完全可以触发自己正在观察的事件，引擎**不做环路检测**，无限连锁的下场是栈溢出崩溃。本章末尾的练习 3 会让你和这条红线打个照面——以及看看它为什么比想象中难踩。

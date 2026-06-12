# 实体句柄：EntityRef 与 EntityWorldMut

`Query` 的工作方式是“按成分召集”：满足条件的实体列队走过来。盘点正相反——艾达拿着门牌号挨户上门，进了门要看什么、改什么，临场才决定。第 3 章的 `commands.entity(e)` 也算上门，但它只会留条子（排队的命令），看不了也问不了。当场上门的工具，是 World 发的三种**实体句柄**。

## 三档授权

只读的一档叫 `EntityRef`，全权的一档叫 `EntityWorldMut`，先看这两个。今天第一站，老蔫儿家：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-04.rs:entity_ref}}
```

<span class="caption">Listing 11-4（其一）：EntityRef——看什么临场决定</span>

`get::<T>()` 给 `Option<&T>`，`contains::<T>()` 答有没有——注意这里和 `Query` 的本质区别：**不需要预先声明组件清单**，站在门口想看哪件看哪件。这正是检查器要的能力。

看完该动手了。`world.entity_mut` 给出全权句柄：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-04.rs:entity_world_mut}}
```

<span class="caption">Listing 11-4（其二）：EntityWorldMut——读写、增删、乃至拆房</span>

`get_mut` 改数据、`insert` 加组件、`despawn` 拆房，全部当场生效。新面孔是 `take::<T>()`：把组件摘下来**并把值交到你手上**——`remove` 是扔掉，`take` 是没收，私酿酒就该用后者。

门牌号可能已经注销。`entity` / `entity_mut` 在查无此户时直接 panic，盘点这种“名单可能过期”的场合该用 `get_entity` / `get_entity_mut`，拿 `Result` 自己处理：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-04.rs:get_entity}}
```

<span class="caption">Listing 11-4（其三）：get_entity——拆过的房，礼貌地查</span>

## 第三档：EntityMut

一次借一户是 `EntityWorldMut`；想**同时**借两户——济贫，从富户匀两袋粮到穷户——传个数组进去：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-04.rs:many}}
```

<span class="caption">Listing 11-4（其四）：同时借两户，拿到的是降档的 EntityMut</span>

注意返回类型降了一档：不是两个 `EntityWorldMut`，而是两个 **`EntityMut`**——能 `get`/`get_mut` 读写数据，但 `insert`/`remove`/`despawn` 这些**结构变更**的方法根本不存在。原因下一节会看得明明白白：结构变更要“搬家”，一搬整张表的住址都可能重排，另一只手里的句柄就成了悬空引用。Rust 不允许，Bevy 干脆不给方法。三档总结：

| 句柄 | 读 | 写数据 | 改结构（insert/remove/despawn） | 来源 |
|---|---|---|---|---|
| `EntityRef` | ✓ | | | `world.entity(e)`、查询 |
| `EntityMut` | ✓ | ✓ | | `world.entity_mut([a, b])` 等多户借用 |
| `EntityWorldMut` | ✓ | ✓ | ✓ | `world.entity_mut(e)`、`world.spawn(...)` |

运行对账（`world.spawn` 的返回值正是 `EntityWorldMut`，所以 Listing 11-3 里能直接 `.id()`）：

```console
cargo run -p ch11-deep-ecs --example listing-11-04
```

```text
老蔫儿家：存粮 7 袋，私酿酒：有！
艾达：私酿酒没收。（老蔫儿：哎——）
再访棚屋：Entity despawned: The entity with ID 2v0 is invalid; its index now has generation 1.
Note that interacting with a despawned entity is the most common cause of this error but there are others
台账：老蔫儿 存粮 4 袋（已盖章）
台账：罗兰 存粮 5 袋
```

老蔫儿 7 − 1（税粮）− 2（济贫）= 4，罗兰 3 + 2 = 5，账目两讫。台账的顺序又一次不等于生成顺序——第 3 章的老话题，下一节给出最终解释。

## 普通系统也能拿句柄

`EntityRef` 不是独占系统的专利：它可以直接当查询的 D 槽位用。盘点前夜，预检官在**普通系统**里把全镇翻一遍，顺手再用 `&World`——第 4 章参数表里挂着“第 11 章”的那位——读两个全局数字，最后写一条退出消息收工：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-05.rs:precheck}}
```

<span class="caption">Listing 11-5：Query&lt;EntityRef&gt; + &World + MessageWriter——预检官全都要</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-05
```

```text
thread 'main' (19276) panicked at C:\Users\94887\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bevy_ecs-0.18.1\src\system\system_param.rs:851:13:
error[B0002]: ResMut<bevy_ecs::message::messages::Messages<bevy_app::app::AppExit>> in system listing_11_05::precheck conflicts with a previous Res<bevy_ecs::message::messages::Messages<bevy_app::app::AppExit>> access. Consider removing the duplicate access. See: https://bevy.org/learn/errors/b0002
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Encountered a panic in system `bevy_app::main_schedule::Main::run_main`!
```

第 5 章见过的 B0002，但这次两个当事参数看起来风马牛不相及。读报错：冲突的一边是 `MessageWriter` 背后的 `ResMut<Messages<AppExit>>`（写消息=写资源，第 7 章说过），另一边是个 `Res<Messages<AppExit>>` 的**读**——它从哪来的？从 `&World` 来：**`&World` 声明的是对世界上一切的读，组件、资源、消息无一例外**。它跟同系统里任何一个带写的参数都打架。`Query<EntityRef>` 同理但温和些——它声明“读全部组件”，所以容得下写资源的参数，容不下任何写组件的查询。

修法是老规矩，写活分出去：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-06.rs:precheck}}
```

<span class="caption">Listing 11-6：预检修好了——只读的归预检，写的归收工</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-06
```

```text
预检官挨家挨户翻名册：
  0v0 罗兰：常住，存粮 3 袋，共 3 个组件
  1v0 老蔫儿：常住，存粮 7 袋，共 3 个组件
  2v0 过路货郎：过路，存粮 20 袋，共 2 个组件
合计：全镇 3 个实体；镇库 73 枚银币。
```

这一版的预检官和别的只读系统照常并行。规律一句话：**句柄或参数的权限越宽，能同坐一个系统的邻居越少**——`&World` 读一切，邻居只剩纯只读；`&mut World` 写一切，邻居清零，那就是独占系统。

预检清单里每户还报了“共几个组件”，来自 `house.archetype().component_count()`。`archetype()`——又是这个词。第 3 章说它是“组件组合相同的实体共用的子表”，搬家、行号、遍历顺序的谜底全在里面。下一节进档案室。

# 独占系统：把整个 World 借给你

盘点的活，先试着按老办法立项：数出全镇实体的总数（不限组件组合）、立一块公告牌**并当场复核**、把结果归档成资源**让同一帧的下个系统就能读到**。第一条就写不出申请单——“不限组合”意味着没有哪个 `Query` 能罩住；后两条撞上 `Commands` 的延迟语义。柜台到此为止，得换钥匙。

## 签名里只写一个 `&mut World`

钥匙长这样：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-01.rs:census}}
```

<span class="caption">Listing 11-1（其一）：独占系统——参数是整个世界</span>

参数是 `&mut World` 的函数叫**独占系统**（exclusive system）。“函数就是系统”的规矩走到了极限形态：签名声明的访问集合是“一切的写”。注册方式毫无特殊，`add_systems` 照旧：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-01.rs:main}}
```

<span class="caption">Listing 11-1（其二）：注册照旧——独占系统混在普通系统中间排队</span>

函数体里是三组没见过的操作：`world.entities().count_spawned()` 清点全部实体；`world.spawn(...)` 直接造实体；`world.insert_resource(...)` 直接放资源。临时起意的查询用 `world.query_filtered::<D, F>()` 现场组一个——它和 `Query` 参数的关系下文细说。喇叭排在艾达后面，验证归档的可见时机：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-01.rs:crier}}
```

<span class="caption">Listing 11-1（其三）：同一帧的下个系统，立刻读到独占系统的成果</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-01
```

```text
—— 第 1 帧 ——
  巡逻队：在册住户 3 人，公告牌 0 块。
—— 第 2 帧 ——
  巡逻队：在册住户 3 人，公告牌 0 块。
  艾达：盘点日。全镇静止！（接管 World）
  艾达：全镇实体 3 个，其中在册住户 3 人。
  艾达：公告牌立讫，复核：1 块。（spawn 当场生效）
  喇叭：镇公所归档——在册住户 3 人！
—— 第 3 帧 ——
  巡逻队：在册住户 3 人，公告牌 1 块。
```

对账两条：

- **没有延迟**。`world.spawn` 不进任何队列：第 2 帧立牌，同一个函数里复核就数到了 1 块。对比第 3 章：`Commands` 是“写申请、等同步点”，World 方法是“当场动工”。归档的资源也一样——喇叭和艾达同帧，照样读到了。
- **代价是停摆**。第 6 章说过同步点要“独占 World，所有并行中的系统先收工”——独占系统就是你亲手写的一段停摆：它运行期间，调度器不会让任何其他系统并行。低频、大权限的活（盘点、存档、初始化）用它正合适；每帧热路径上别放。

## 行不通：再带一份普通参数

艾达想顺手带份 `Commands` 排队干点别的：

```rust
{{#include ../../code/ch11-deep-ecs/no-compile/listing-11-02.rs:mixed}}
```

<span class="caption">Listing 11-2：行不通——&mut World 旁边坐不下 Commands</span>

```text
error[E0277]: `fn(&mut World, Commands<'b, 'c>) {take_census}` does not describe a valid system configuration
   --> ch11-deep-ecs\no-compile\listing-11-02.rs:13:36
    |
 13 |     App::new().add_systems(Update, take_census).run();
    |                -----------         ^^^^^^^^^^^ invalid system configuration
    |                |
    |                required by a bound introduced by this call
    |
    = help: the trait `IntoSystem<(), (), _>` is not implemented for fn item `fn(&mut World, Commands<'b, 'c>) {take_census}`
```

编译器直接拒收：这不是一份合法的系统配置。道理想通很简单——普通参数全是“向 World 借某一块”的凭据，而 `&mut World` 已经把整个世界借走了，凭据无处兑现。独占系统的同伴另有一份小名单（`ExclusiveSystemParam`）：`Local<T>` 可以（Listing 11-1 已经用它计帧），还有一位 `&mut SystemState<...>` 是后面 11-4 节的主角。

## 卸下引擎盖：World 是一个普通的值

独占系统拿到的 `&mut World`，和 `App` 内部那个 World 是同一个。但 World 本身不依赖 `App`——它就是个普通的 Rust 值，`new` 出来就能用。艾达晚上回镇公所，用沙盘推演明天的流程：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-03.rs:new}}
```

<span class="caption">Listing 11-3（其一）：不要 App、不要调度，World 自己就能转</span>

资源一族，与系统参数一一对应：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-03.rs:resource}}
```

<span class="caption">Listing 11-3（其二）：资源的直接访问</span>

第 5 章注初始难度时见过一面的 `world.resource`，现在凑齐了全家。柜台参数和 World 方法的对照表：

| 柜台参数 | World 直接访问 |
|---|---|
| `Res<T>` / `ResMut<T>` | `world.resource::<T>()` / `world.resource_mut::<T>()` |
| `Option<Res<T>>` | `world.get_resource::<T>()`（缺货给 `None` 而不是 panic） |
| `Commands::insert_resource` | `world.insert_resource(...)`（当场） |
| `Query<D, F>` | `world.query::<D>()` / `world.query_filtered::<D, F>()` |
| `MessageWriter<M>` | `world.write_message(...)`（第 7 章预告过的那位） |
| `commands.trigger(...)` | `world.trigger(...)` |

查询这一行值得单独跑一遍。`world.query()` 返回的不是 `Query`，而是 **`QueryState`**——`Query` 参数背后的本体（引擎为每个系统的每个查询缓存着一份）。要数据时把 world 借给它：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-03.rs:query}}
```

<span class="caption">Listing 11-3（其三）：query() 给的是 QueryState，iter(&world) 才出数据</span>

注意两次借用的宽窄：`world.query()` 要 `&mut World`（首次见到某个组合时要建缓存），`iter(&world)` 只要共享借用。

借用还会打更狠的架：写总账要 `world.resource_mut`（独占借用），手里攥着它就没法再跑查询——borrow checker 不放行。这类“资源加世界”的组合有个正解叫 `resource_scope`，把资源暂时“摘下来”：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-03.rs:scope}}
```

<span class="caption">Listing 11-3（其四）：resource_scope——资源摘下来用，World 照常自由</span>

第 8 章说 `World::trigger` 是“字面意义的当场触发”，欠的演示现在还上：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-03.rs:trigger}}
```

<span class="caption">Listing 11-3（其五）：trigger 当场执行，observer 跑完才返回</span>

最后拆台。`despawn` 之后，`get_entity` 用 `Result` 报告查无此人：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-03.rs:despawn}}
```

<span class="caption">Listing 11-3（其六）：get_entity——门牌可能已注销</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-03
```

```text
沙盘上第一个小人：罗兰
修桥之后，镇库还剩 70 枚银币
罗兰家存粮 3 袋
老蔫儿家存粮 7 袋
收讫人头税 2 枚，镇库现银 72 枚
敲门——
  （沙盘小人探出头：谁呀？）
——话音未落，门里已经应了。
再访罗兰家：Entity despawned: The entity with ID 0v0 is invalid; its index now has generation 1.
Note that interacting with a despawned entity is the most common cause of this error but there are others
```

敲门那三行的顺序就是 `trigger` 即时性的铁证——`world.trigger` 返回之前，observer 已经应完了门。最后一行的报错也有得读：`0v0` 是“索引 0、世代 0”，despawn 之后索引被回收、世代翻到 1，旧门牌从此作废——第 3 章讲过的世代机制，第一次在报错文本里亲眼见到。

沙盘推演顺利。但 `world.entity(roland)` 拿到的那个“句柄”究竟是什么、能干多少事，值得专开一节——明天上门盘点全指着它。

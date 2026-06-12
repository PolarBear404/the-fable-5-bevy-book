# EntityEvent：指名道姓

锣声是广播，全场都听。但公会楼上的鉴定室做的是另一种生意：委托人递一张单子，写明“鉴定**这把剑**”——事件有目标。用上一节的 `Event` 也能凑合：往结构体里塞个 `Entity` 字段，每个 observer 自己核对“说的是不是我关心的那件”。可一旦想给**某一件**物品安排专属反应，这条路就堵死了——全局 observer 对每一单都响，过滤逻辑写得到处都是。

**EntityEvent** 是带准星的事件。它仍然是 `Event`（上一节的一切照常成立），但触发时引擎知道目标是谁，于是多出一种本事：除了全局 observer，还能运行**只挂在目标实体上**的专属 observer：

```rust
{{#include ../../code/ch08-events-observers/examples/listing-08-03.rs}}
```

<span class="caption">Listing 8-3：EntityEvent——鉴定室里指名道姓</span>

```console
cargo run -p ch08-events-observers --example listing-08-03
```

```text
—— 第 1 帧 ——
鉴定师：铁剑 鉴定完毕，记录在案。
鉴定师：诅咒之剑 鉴定完毕，记录在案。
诅咒之剑：（剑身震颤）谁准你看穿我的底细！
```

- **定义**：`#[derive(EntityEvent)]`，并约定一个名为 `entity` 的字段存放目标。字段想换名字（比如事件里有两个 `Entity`，要指明哪个是目标），给它标 `#[event_target]` 即可。
- **专属 observer**：`spawn(...).observe(系统)` 挂在刚生成的实体上，只在事件瞄准**这个实体**时运行。铁剑被鉴定时它一声不吭。
- **全局 observer 照常收货**：`add_observer` 挂的鉴定师对每一单都响，`identified.entity` 告诉他这单说的是谁——配合第 4 章的 `Query::get` 按图索骥。

输出里专属反应排在全局记录之后，但上一节的警告原样适用：不同 observer 之间的顺序不在保证之列。

## Observer 的真身

`.observe()` 和 `add_observer` 都是语法糖，值得揭开一次：**observer 本身是实体，身上挂着一个 `Observer` 组件**，组件里装着你的系统。`add_observer(系统)` 不过是 `spawn(Observer::new(系统))`；`.observe()` 则是先 `Observer::new` 再调用它的 `watch_entity` 锁定目标后 spawn。一个 `Observer` 可以连续 `watch_entity` 盯住多个实体——一千件装备共用一个 observer 实体，比挂一千个省得多。被盯上的实体则会多出一个 `ObservedBy` 组件记录反向关系。

这套“observer 即实体”的设计带来两个实际好处：observer 可以在运行时随时增删（despawn 那个实体就是注销），以及引擎能自动收尾——专属 observer 盯着的实体**全部**销毁后，observer 实体会被顺手清掉，不会越积越多。

## 单子追进熔炉

指名道姓有一个上一节广播模式不存在的隐患：`commands.trigger` 排进队列时目标还活着，命令应用时它可能已经没了。事件**照样送达**——全局 observer 仍会运行，只是目标查无此人：

```rust
{{#include ../../code/ch08-events-observers/examples/listing-08-04.rs}}
```

<span class="caption">Listing 8-4：鉴定单追进了熔炉——事件送达时，目标可能已经没了</span>

```console
cargo run -p ch08-events-observers --example listing-08-04
```

```text
—— 第 1 帧 ——
学徒：剑已经熔了，但鉴定单还是递了上去……
鉴定师：单子上的 1v0 已经不在了，退单。
```

所以 EntityEvent 的 observer 里查询目标组件，标准姿势是 `get` 加 `let Ok(..) = .. else`，查不到就体面退场——上面几个示例里 `unwrap` 之所以安全，是因为剧本保证了目标活着；真实游戏里没有这种剧本。官方示例（`vendor/bevy/examples/ecs/observers.rs`）处理爆炸连锁时用的是同一套防御。

> **预告**：EntityEvent 还有一手“事件冒泡”——目标实体自己没人处理时，沿父子链向上传给祖先（`#[entity_event(propagate)]`）。它依赖实体之间的层级关系，第 9 章把 `ChildOf` 讲清楚之后才能见真章；第 25 章的鼠标拾取会让你天天和它打交道。

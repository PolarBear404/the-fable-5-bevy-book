# 第一个 Observer

公会大厅挂着一面铜锣。规矩很直接：锣一响，所有人**立刻**放下手里的活——不是“下次轮到我上班时去公告板看看有没有锣响过”。用消息模拟这面锣，你写出来的恰恰是后者。换 Event 和 Observer，只需要两步——**定义**事件、**挂上**观察者，再加一声 `trigger`：

```rust
{{#include ../../code/ch08-events-observers/examples/listing-08-01.rs}}
```

<span class="caption">Listing 8-1：第一个 Observer——敲锣，全公会立刻听见</span>

```console
cargo run -p ch08-events-observers --example listing-08-01
```

```text
—— 第 1 帧 ——
司仪：各位安静——
司仪：（锣槌已经挥出去了）
公会成员：锣响了，集合！
```

逐项对账：

- **定义**：`#[derive(Event)]` 把任意类型变成事件，待遇和 Component、Message 一样——普通结构体即可。
- **挂观察者**：`add_observer` 接收一个函数，要求只有一条：**第一个参数必须是 `On<事件类型>`**。`On` 读作“当……发生时”，`hear_gong(_gong: On<GongStruck>)` 就是“当锣响时，执行这个函数体”。
- **触发**：`commands.trigger(事件值)`。注意这里**没有第三步**——Message 必须 `add_message` 开通道，事件却不用任何注册仪式：挂 observer 本身就是登记，触发时引擎按类型找人。

再看输出的顺序，这是本节真正的考点。司仪的两句话连在一起，锣声响在**之后**——因为 `commands.trigger` 和第 3 章的 `spawn`、`insert` 一样走命令队列，第 6 章的同步点规则原样适用：`strike_gong` 函数体跑完，命令才被应用。但应用的那一刻，事情就和消息分道扬镳了——**所有盯着 `GongStruck` 的 observer 当场、同步、依次跑完**，跑完才轮到队列里的下一条命令。事件从不进入缓冲：没有 `Messages<E>` 资源、没有游标、没有双缓冲清理，自然也没有“读者来晚了”这回事。第 7 章整整一节的寿命规则，在这里没有对应物——事件的寿命就是 observer 运行的那一瞬。

> 想绕过命令队列、连同步点都不等？`World::trigger` 是字面意义的“当场触发”，调用返回前 observer 已经跑完。它需要直接拿到 `World`，第 11 章讲 World API 时再见；在普通系统里，`commands.trigger` 是标准姿势。

## Observer 是如假包换的系统

锣声只是入门。事件可以携带数据，observer 除了第一个参数，剩下的位置想要什么系统参数就声明什么——`Query`、`Res`、`Commands`，第 4、5 章的家当全部可用：

```rust
{{#include ../../code/ch08-events-observers/examples/listing-08-02.rs}}
```

<span class="caption">Listing 8-2：事件带数据，observer 带全套系统参数</span>

```console
cargo run -p ch08-events-observers --example listing-08-02
```

```text
—— 第 1 帧 ——
账房：预扣 80 金币，金库还剩 420。
公告员：新任务「扫荡地窖鼠群」，赏金 80 金币！
账房：预扣 200 金币，金库还剩 220。
公告员：新任务「护送商队过山口」，赏金 200 金币！
```

- **事件带数据**：`On<QuestPosted>` 解引用直达事件本体，`quest.title`、`quest.reward` 随手就拿，不用先 `.read()` 再解构。
- **Observer 就是系统**：账房声明了 `ResMut<Treasury>` 照常拿到金库。一个事件挂任意多个 observer，各自带各自的参数——第 7 章“加功能 = 加读者”的解耦回报，这边一样成立：委托人 `post_quests` 不认识任何一位听众。
- **两单各自完整办结**：第一张任务的两道反应全部跑完，第二张才开始。事件不会交错插队——每次 `trigger` 都把自己的 observer 全部送完才收场。

还有一处对账对不上的地方，恰恰是本节最重要的警告：注册顺序明明是公告员在前、账房在后，输出里却是**账房先开口**。这不是 bug——Bevy 明确不保证同一事件多个 observer 之间的执行顺序，注册顺序、字母顺序都靠不住。两个 observer 若有先后依赖，要么合成一个，要么让前者触发新事件、后者观察那个新事件（这种连锁手法见 Listing 8-7）。把顺序假设写进代码，就是给未来的自己埋雷。

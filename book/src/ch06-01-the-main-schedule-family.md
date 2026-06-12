# Main 调度全家

第 2 章给 Schedule 下过定义：一份"什么时机、按什么顺序跑哪些系统"的清单。当时只露面了两份清单——`Startup` 和 `Update`。实际上 Bevy 准备了一整套，每帧按固定次序逐个执行。本节把全家请出来点名。

## 一帧的全貌

最直接的办法还是打印实验。给八个调度各塞一个打印系统——注册顺序故意打乱：

```rust
{{#include ../../code/ch06-schedules/examples/listing-06-01.rs}}
```

<span class="caption">Listing 6-1：Main 调度全家——注册顺序与执行顺序无关</span>

这里顺手解锁一个新写法：**闭包也能当系统**。它和普通函数遵循同一条规则——每个参数都是合法的系统参数即可（这几个闭包恰好零参数）。后面 `run_if` 还会大量用到这种写法。运行：

```console
cargo run -p ch06-schedules --example listing-06-01
```

```text
—— 第 1 帧 ——
  PreStartup  —— 最早的准备（只跑一次）
  Startup     —— 搭建场景（只跑一次）
  PostStartup —— 开赛前检查（只跑一次）
  First       —— 一帧之始
  PreUpdate   —— 引擎备料
  Update      —— 游戏逻辑
  PostUpdate  —— 引擎善后
  Last        —— 帧末收尾
—— 第 2 帧 ——
  First       —— 一帧之始
  PreUpdate   —— 引擎备料
  Update      —— 游戏逻辑
  PostUpdate  —— 引擎善后
  Last        —— 帧末收尾
```

输出印证三件事：

1. **执行顺序与注册顺序无关**。代码里 `Update` 注册在最前、`PreStartup` 几乎垫底，跑起来各回各位。`add_systems` 只是把系统放进对应调度的清单，清单之间的次序由引擎统一安排。
2. **启动三件套只跑一次。**`PreStartup` → `Startup` → `PostStartup` 只出现在第 1 帧。它们正是第 4 章那句"`Startup` 只在第一次 `update()` 时运行"的完整版本。
3. **`FixedUpdate` 一声不吭**。注册了，却一次都没跑——它需要时钟驱动，而裸 `App` 里没有时钟。本节稍后单独处理它。

把全家列成表。每帧从上往下：

| 调度 | 谁在用 | 典型用途 |
|---|---|---|
| `First` | 引擎 | 帧的起点，刷新时间等最早的内务 |
| `PreUpdate` | 引擎、插件 | 为本帧备料：把操作系统的原始输入整理成 `Update` 可直接读的资源 |
| `RunFixedMainLoop` | 引擎 | 在这里循环驱动 `FixedMain`（零到多次，见下文） |
| `Update` | **你** | 游戏逻辑的默认住址 |
| `SpawnScene` | 引擎 | 场景实例化（第 32 章） |
| `PostUpdate` | 引擎、插件 | 对 `Update` 的成果做善后：传播 `Transform`（第 12 章）、准备渲染数据 |
| `Last` | 引擎 | 帧末收尾 |

启动段同理分三站：`PreStartup`、`Startup`、`PostStartup`，只在第一帧、且在 `First` 之前执行。你的场景搭建放 `Startup`；前后两站和 `First`/`Last` 一样，多数时候留给引擎和插件用。

> 启用 `bevy_state`（默认 features 含它）时，全家还有一位 `StateTransition`，排在 `PreUpdate` 之后，负责游戏状态机的切换——第 10 章的主角，这里先混个脸熟。

这张表回答了一个第 2 章遗留的问题：**引擎自己的系统住在哪**？答案是几乎全在 `Pre`/`Post` 段位。键盘输入由 `bevy_input` 的系统在 `PreUpdate` 写进 `ButtonInput<KeyCode>` 资源，于是你在 `Update` 里读到的输入永远是本帧最新的；`Transform` 的父子传播在 `PostUpdate`，于是你在 `Update` 里挪动实体，引擎保证渲染前已结算到位。`Update` 被刻意留空给你——**你写逻辑，引擎在你之前备料、在你之后善后**，三方靠住址划清界限，互不踩脚。

> 幕后一句：`Main` 自己也是一个调度，它做的事就是按这份清单依次运行各个子调度（清单存在 `MainScheduleOrder` 资源里，可以改，本书用不到）。第 4 章的 `app.update()` "把 Main 调度完整跑一遍"，指的就是这个流程。

## FixedUpdate：按自己的时钟行事

表里的 `RunFixedMainLoop` 站着一位特殊成员。`Update` 每帧跑一次，帧率多少跑多少；但物理、规则结算这类逻辑往往希望**按固定频率**运行——60 帧的机器和 144 帧的机器，世界演化速度应该一致。这就是 `FixedUpdate`：它不跟帧走，跟一台固定步长的时钟走，默认 64 Hz。

工作方式像补课：引擎攒着真实流逝的时间，每帧走到 `RunFixedMainLoop` 时清点一次——攒够一个步长就跑一轮 `FixedUpdate`，攒够两个就连跑两轮，不够就一轮也不跑。观察它需要时钟，所以这次请出 `MinimalPlugins`（第 2 章见过，里面有 `TimePlugin`）。为了让输出可复现，我们把两个旋钮都拧到指定刻度：

```rust
{{#include ../../code/ch06-schedules/examples/listing-06-02.rs}}
```

<span class="caption">Listing 6-2：固定时钟步长 50 毫秒，每帧流逝 30 毫秒——FixedUpdate 时跑时歇</span>

`Time::<Fixed>` 就是那台固定时钟，`from_seconds(0.05)` 把步长从默认的 15.625 毫秒改成 50 毫秒，好让账目凑整。`TimeUpdateStrategy::ManualDuration` 是为测试准备的开关：每帧的"流逝时间"不再看真实世界，固定为 30 毫秒——真实游戏不要动它，这里只为让每次运行的输出一字不差。运行：

```console
cargo run -p ch06-schedules --example listing-06-02
```

```text
—— 第 1 帧（流逝 30 毫秒）——
  Update
—— 第 2 帧（流逝 30 毫秒）——
  Update
—— 第 3 帧（流逝 30 毫秒）——
  FixedUpdate（本步 50 毫秒）
  Update
—— 第 4 帧（流逝 30 毫秒）——
  Update
—— 第 5 帧（流逝 30 毫秒）——
  FixedUpdate（本步 50 毫秒）
  Update
```

对账：第 1 帧时钟刚起步、流逝按 0 计，之后每帧攒 30 毫秒。攒到第 3 帧共 60 毫秒，够一个 50 毫秒步长——`FixedUpdate` 跑一轮，余 10；第 4 帧攒到 40，不够，歇着；第 5 帧 70，再跑一轮。**五帧里它只跑了两次，每次记账恰好 50 毫秒**——`FixedUpdate` 里的 `Res<Time>` 自动切换成固定时钟，`delta()` 永远等于步长，这正是"固定"的含义。输出还白送一个事实：`FixedUpdate` 打印在 `Update` 之前——回看上面的表，`RunFixedMainLoop` 确实排在 `Update` 前面。

`FixedMain` 内部也是对称的五站：`FixedFirst`、`FixedPreUpdate`、`FixedUpdate`、`FixedPostUpdate`、`FixedLast`，分工与主家族一一对应。

选址原则一句话：**逐帧呈现的（相机跟随、UI、音效触发）放 `Update`，按固定节拍演化的（物理、AI 决策、规则结算）放 `FixedUpdate`**。它每帧可能跑零次也可能跑多次，所以放进去的逻辑不能假设"一帧恰好一轮"。固定时间步的完整故事——`Timer`、时间缩放、渲染插值——是第 18 章的主菜。

地图画完了。但同一个调度里面的系统呢？`Update` 里几十个系统，谁先谁后？下一节进入微观世界。

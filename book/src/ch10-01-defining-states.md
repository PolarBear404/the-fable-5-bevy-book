# 定义状态与切换

一台街机只有两种营业状态：待机画面滚字幕，或者一局游戏正打着。先把这件事告诉引擎。

## 状态是一个 enum

```rust
{{#include ../../code/ch10-states/examples/listing-10-01.rs:state}}
```

<span class="caption">Listing 10-1（其一）：用 derive 把一个 enum 变成状态机</span>

`#[derive(States)]` 把普通 enum 升格为状态类型。它对 derive 列表里的同伴有要求：`Clone + PartialEq + Eq + Hash + Debug` 一个不能少——引擎要比较、要哈希、要在日志里打印它。`Default` 则回答“开机时处在哪一格”：标了 `#[default]` 的 `Menu` 就是起点。

和第 5 章的 Resource 一样，状态描述的是**全局唯一**的事实——整个 App 此刻要么在菜单、要么在游戏中，不存在“一半实体在菜单”。实际上它就是用 Resource 实现的，马上你会见到。

## 注册三件套

```rust
{{#include ../../code/ch10-states/examples/listing-10-01.rs:main}}
```

<span class="caption">Listing 10-1（其二）：StatesPlugin + init_state + in_state</span>

三处新面孔，从上往下：

- **`StatesPlugin`**——状态机的发动机。它把一个名叫 `StateTransition` 的调度挂进 Main 调度全家（第 6 章那张表里“排在 `PreUpdate` 之后”的那位），切换状态的实际动作全在里面发生。`DefaultPlugins` 自带它；但本书的纯逻辑示例用的是 `MinimalPlugins`，**没有**它，得手动加一行。它不在 prelude 里，注意文件头上的 `use bevy::state::app::StatesPlugin;`。
- **`init_state::<GameState>()`**——开机上电。它做两件事：把两个资源插进世界——`State<GameState>`（当前值，只读）和 `NextState<GameState>`（切换申请单，可写）；再以 `Default` 为起点，触发一次进入 `Menu` 的“启动转换”（下一节细说）。
- **`in_state(GameState::Menu)`**——第 6 章 `run_if` 预告过的那位。它就是一个普通的运行条件：当前状态等于给定值才放行。待机字幕挂 `Menu`，战斗挂 `Playing`，启停逻辑从此写在调度层，系统函数体里一个 `if` 都不用写。

## 切换：写申请单

剧本系统负责投币：

```rust
{{#include ../../code/ch10-states/examples/listing-10-01.rs:script}}
```

<span class="caption">Listing 10-1（其三）：NextState::set——申请切换</span>

`ResMut<NextState<GameState>>` 就是那张申请单，`next.set(GameState::Playing)` 把它从默认的“无变动”填成“去 Playing”。注意这是**申请**而不是立刻执行——具体什么时候生效，输出会告诉我们。运行：

```console
cargo run -p ch10-states --example listing-10-01
```

```text
—— 第 1 帧 ——
  屏幕：《勇者斗史莱姆》——投币开始
—— 第 2 帧 ——
  屏幕：最高纪录 9999 分，保持者“老蔫儿”
—— 第 3 帧 ——
  罗兰：守了一路商队，也轮到我冒险一回。（投入硬币，叮）
  屏幕：《勇者斗史莱姆》——投币开始
—— 第 4 帧 ——
  屏幕：勇者挥剑！史莱姆 HP 剩 10
—— 第 5 帧 ——
  屏幕：勇者挥剑！史莱姆倒下——通关！
—— 第 6 帧 ——
  老板：打烊喽。（拉闸）
```

前两帧字幕轮播，第 4、5 帧打史莱姆，状态机如约工作。但第 3 帧值得盯一眼：**罗兰投了币，画面却还在滚字幕**——`battle` 没接班，`attract_screen` 也没下班。硬币明明进去了，怎么没反应？

按下不表，先看一个更响亮的失败。

## 忘了 StatesPlugin 会怎样

把发动机那行删掉：

```rust
{{#include ../../code/ch10-states/examples/listing-10-02.rs:main}}
```

<span class="caption">Listing 10-2：行不通——没有 StatesPlugin，init_state 直接 panic</span>

```console
cargo run -p ch10-states --example listing-10-02
```

```text
thread 'main' (26112) panicked at C:\Users\94887\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bevy_state-0.18.1\src\app.rs:96:67:
The `StateTransition` schedule is missing. Did you forget to add StatesPlugin or DefaultPlugins before calling init_state?
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

报错把话说得很全：`init_state` 要把状态机装进 `StateTransition` 调度，而那个调度由 `StatesPlugin` 创建——插件还没加，调度不存在，当场 panic。顺序也有讲究：`add_plugins(StatesPlugin)` 必须写在 `init_state` **之前**。用 `DefaultPlugins` 的程序不会遇到这个问题，但哪天你写无窗口的服务端或测试，撞上这条 panic 就知道是怎么回事了。

## 切换不在本帧生效

回到第 3 帧的疑案。这次让投币系统自己作证——`set` 完立刻回头读一眼 `State` 资源：

```rust
{{#include ../../code/ch10-states/examples/listing-10-03.rs:coin_slot}}
```

<span class="caption">Listing 10-3（其一）：set 之后立刻读，State 还是旧值</span>

屏幕系统每帧报告自己看到的状态。`State<GameState>` 用 `.get()` 取值：

```rust
{{#include ../../code/ch10-states/examples/listing-10-03.rs:screen}}
```

<span class="caption">Listing 10-3（其二）：读 State 资源</span>

```console
cargo run -p ch10-states --example listing-10-03
```

```text
—— 第 1 帧 ——
  屏幕：待机画面（state = Menu）
—— 第 2 帧 ——
  罗兰投币。又凑近看了一眼：画面是 Menu——硬币进去了，怎么没反应？
  屏幕：待机画面（state = Menu）
—— 第 3 帧 ——
  屏幕：战斗画面（state = Playing）
—— 第 4 帧 ——
  屏幕：战斗画面（state = Playing）
```

第 2 帧，`set` 已经调用，可无论是投币系统自己还是排在它后面的屏幕系统，读到的都是 `Menu`；第 3 帧屏幕才切到战斗画面。对账：

- **`NextState::set` 只是填申请单**，`State` 资源纹丝不动。真正搬闸刀的是 `StateTransition` 调度——它检查申请单，有变动才改写 `State`。
- **`StateTransition` 排在 `PreUpdate` 之后、`Update` 之前**（第 6 章的地图）。你在 `Update` 里 `set`，本帧的 `StateTransition` 早就跑完了，申请要等**下一帧帧首**才被受理。所以投币帧全世界看到的都是 `Menu`，没有“前半帧菜单、后半帧战斗”的撕裂帧。
- 这和第 3 章 `Commands` 的延迟是同一族设计：修改先排队，在固定的时机统一落地，换来的是帧内视图的一致。

几条随手要用的细则：

- **想指定起点而不用 `Default`**：`insert_state(GameState::Playing)` 替代 `init_state`，比如做关卡编辑器直接进游戏态调试。
- **一帧内多次 `set`，最后一次赢**——申请单只有一格，后写的覆盖先写的。
- **状态机可以有多台**。`GameState` 管阶段、`AudioState` 管静音，各自 `init_state`、互不相干；本章后半会看到它们之间还能建立从属关系。

“申请下一帧才生效”解释了第 3 帧的惯性。但切换的那个瞬间——字幕收起、锣声敲响的换幕时刻——程序还没有任何代码在管。下一节给换幕时刻装上挂载点。

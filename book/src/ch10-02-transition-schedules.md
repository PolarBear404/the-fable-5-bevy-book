# 换幕时刻：OnEnter 与 OnExit

`run_if(in_state(...))` 管的是“处在某状态的每一帧”。但有些活只属于**换幕的瞬间**：进入菜单的那一刻搭 UI，离开的那一刻拆掉；开局的那一刻摆棋子，结束的那一刻收盘。它们不该每帧跑，也不该由每个系统自己检测“状态是不是刚变过”。

Bevy 给换幕时刻准备了三个专用调度。它们和 `Update` 一样是 Schedule，特别之处在于标签里带着状态值：

- **`OnEnter(state)`**——转换**进入** `state` 时跑一遍；
- **`OnExit(state)`**——转换**离开** `state` 时跑一遍；
- **`OnTransition { exited, entered }`**——恰好从 `exited` 换到 `entered` 这条特定路线时跑一遍，夹在前两位中间执行。

给街机装上全套换幕动作：

```rust
{{#include ../../code/ch10-states/examples/listing-10-04.rs:main}}
```

<span class="caption">Listing 10-4：三个转换调度，外加一个 Startup 对照组</span>

```console
cargo run -p ch10-states --example listing-10-04
```

```text
  [OnEnter(Menu)] 屏幕亮起，待机字幕滚动
  老板：搬个凳子守摊。（Startup）
—— 第 1 帧 ——
  屏幕：《勇者斗史莱姆》——投币开始
—— 第 2 帧 ——
  罗兰投币。（叮）
  屏幕：《勇者斗史莱姆》——投币开始
  [OnExit(Menu)] 字幕收起
  [OnTransition] 读盘画面：LOADING……
  [OnEnter(Playing)] 锣响，勇者登场！
—— 第 3 帧 ——
  屏幕：勇者一剑劈倒史莱姆！
—— 第 4 帧 ——
  罗兰：通关！回待机画面吧。
  屏幕：通关结算画面定格
  [OnExit(Playing)] 结算画面一闪
  [OnEnter(Menu)] 屏幕亮起，待机字幕滚动
—— 第 5 帧 ——
  老板：打烊喽。
  屏幕：《勇者斗史莱姆》——投币开始
```

三处对账：

- **第一行就有故事**：`OnEnter(Menu)` 打印在 `Startup` 的老板**之前**。上一节说 `init_state` 会触发一次进入默认状态的“启动转换”——它的执行时机比 `PreStartup` 还早。所以别在 `OnEnter(初始状态)` 里指望读到 `Startup` 搭建的东西；反过来，`Startup` 倒是能看到 `OnEnter(初始状态)` 的成果。
- **换幕三连的位置**。`OnExit(Menu)`、`OnTransition`、`OnEnter(Playing)` 印在第 2、3 帧的横幅**之间**。它们不属于第 2 帧——投币帧的世界还是 `Menu`（上一节验证过）；它们是**第 3 帧的开头**：`StateTransition` 调度在帧首受理申请、改写 `State`，随即依次运行 `OnExit(旧)` → `OnTransition` → `OnEnter(新)`。横幅是 `Update` 系统打的，转换日志自然挤在两道横幅中间。
- **三个调度都只跑那一遍**。第 3 帧战斗照常、没有任何换幕输出——换幕调度不随帧重复，这正是它和 `run_if` 的分工。

> 转换同时还会广播一条 Message：`StateTransitionEvent<S>`，用 `exited` 与 `entered` 字段记录两端。想对“任何转换”做统一响应（比如打日志）时，用第 7 章的 `MessageReader` 读它就行；引擎自带的调试工具 `bevy::dev_tools::states::log_transitions` 就是这么实现的。日常换幕用 `OnEnter`/`OnExit` 足矣。

## 同值转换：手滑连按会怎样

申请单上填的状态和当前一样，算不算一次转换？街机给出了它的答案——罗兰探身够汽水，手肘撞上了开始键：

```rust
{{#include ../../code/ch10-states/examples/listing-10-05.rs:script}}
```

<span class="caption">Listing 10-5：在 Playing 里再次 set(Playing)，以及 set_if_neq 的防抖</span>

这局游戏有连击数（存在 `Combo` 资源里，`OnEnter(Playing)` 把它清零）。运行：

```console
cargo run -p ch10-states --example listing-10-05
```

```text
—— 第 1 帧 ——
  罗兰投币。（叮）
  [OnEnter(Playing)] 锣响，新的一局！连击清零
—— 第 2 帧 ——
  屏幕：勇者进攻！连击 ×1
—— 第 3 帧 ——
  罗兰探身够汽水，手肘撞上开始键——又一次 set(Playing)！
  屏幕：勇者进攻！连击 ×2
  [OnExit(Playing)] 结算画面一闪
  [OnEnter(Playing)] 锣响，新的一局！连击清零
—— 第 4 帧 ——
  老板边嘟囔边拆开按键，给它加了层防抖垫。
  屏幕：勇者进攻！连击 ×1
—— 第 5 帧 ——
  罗兰的手肘又撞上去——这次是 set_if_neq(Playing)。
  屏幕：勇者进攻！连击 ×2
—— 第 6 帧 ——
  屏幕：勇者进攻！连击 ×3
—— 第 7 帧 ——
  老板：打烊喽。
  屏幕：勇者进攻！连击 ×4
```

- **`set` 不问新旧，一律受理**：第 3 帧的手肘让 `OnExit(Playing)` 和 `OnEnter(Playing)` 原地重跑一轮，连击应声清零——好好的一局就这么没了。这种“从自己到自己”的转换叫**同值转换**（identity transition）。它有时是你要的（比如“重开本关”就是再 `set` 一次当前关卡），更多时候是事故。
- **`set_if_neq` 是带防抖的版本**：目标和当前值相同就当无事发生——第 5 帧之后连击继续涨。两者的区别只在同值这一种情况，普通换状态用谁都一样。
- 留意代码里的 `(*next).set_if_neq(...)`。直接写 `next.set_if_neq(...)` 过不了编译——`ResMut` 自己也有一个 `set_if_neq`（第 5 章变更检测那个），方法解析会先撞上它。先用 `*` 解到 `NextState` 再调用，才是状态机的防抖版本。

搭台拆台的挂载点有了。但 Listing 10-4 的“搭台”只是打印一行字——真要在 `OnEnter` 里生成一批实体，离开时谁来收？下一节把这件事交给引擎。

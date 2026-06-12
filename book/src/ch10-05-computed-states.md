# ComputedStates：推导出来的状态

老板给街机开了**演示模式**：没人玩的时候，机器自己打给路人看，吸引投币。建模很自然——给 `Playing` 加个字段：

```rust
{{#include ../../code/ch10-states/examples/listing-10-09.rs:states}}
```

<span class="caption">Listing 10-9（其一）：带字段的状态值，以及从它推导出的 InAction</span>

状态变体带数据完全合法（derive 的那几个 trait 满足即可），`Playing { demo: true }` 和 `Playing { demo: false }` 是两个不同的状态值。但麻烦跟着来了：战斗画面在演示局和真人局都要演，而 `in_state` 认的是**精确值**——你得写 `in_state(Playing { demo: true }).or(in_state(Playing { demo: false }))`。两个值还能忍，等状态再带上关卡号、难度，每个“不关心细节”的系统都要罗列全部组合，一处加变体处处改。

这就是代码里第二个类型 `InAction` 的存在理由。它实现的不是 `States` 而是 **`ComputedStates`**：声明源状态（`SourceStates = GameState`），给一个 `compute` 函数——源状态每次转换后，引擎调用它算出新值。返回 `Some` 就处在该状态，返回 `None` 则 `State<InAction>` 资源整个不存在。`matches!(source, Playing { .. })` 一行，把“画面上正在过招”从两个具体值里**提炼**了出来——模式通配，之后无论 `Playing` 长出多少字段都不用回头改。

注册和使用：

```rust
{{#include ../../code/ch10-states/examples/listing-10-09.rs:main}}
```

<span class="caption">Listing 10-9（其二）：战斗挂推导态，计分挂精确值，各取所需</span>

战斗挂 `in_state(InAction)`，一个条件覆盖两种局；计分牌只认真人局，继续用精确值 `in_state(Playing { demo: false })`——两套粒度并用，互不妨碍。剧本：机器自己开演示局，罗兰中途投币接管：

```rust
{{#include ../../code/ch10-states/examples/listing-10-09.rs:script}}
```

<span class="caption">Listing 10-9（其三）：演示局 → 真人局 → 回待机</span>

```console
cargo run -p ch10-states --example listing-10-09
```

```text
—— 第 1 帧 ——
  屏幕：《勇者斗史莱姆》——投币开始
—— 第 2 帧 ——
  机器等得无聊，自顾自开了一局演示揽客。
  屏幕：《勇者斗史莱姆》——投币开始
  [OnEnter(InAction)] 机台风扇呼呼转起来
—— 第 3 帧 ——
  屏幕：勇者与史莱姆你来我往
—— 第 4 帧 ——
  罗兰看得手痒：让我来！（叮——切到真人局）
  屏幕：勇者与史莱姆你来我往
—— 第 5 帧 ——
  屏幕：勇者与史莱姆你来我往
  记分牌：+100，共 100 分
—— 第 6 帧 ——
  屏幕：通关！退回待机画面。
  屏幕：勇者与史莱姆你来我往
  记分牌：+100，共 200 分
  [OnExit(InAction)] 风扇停转，机台歇了
—— 第 7 帧 ——
  老板：打烊喽。
  屏幕：《勇者斗史莱姆》——投币开始
```

对账三件事：

- **演示局战斗照演、计分牌沉默**（第 3 帧）；罗兰接管后两行都有（第 5 帧）。
- **第 4 帧最关键：什么都没发生**。`demo: true → demo: false`，源状态实打实换了值（`GameState` 自己的 `OnExit`/`OnEnter` 跑了，本例没注册所以看不见），但 `compute` 两次算出的都是 `Some(InAction)`——风扇没有重启。推导态把“源里的小动静”挡在了门外。
- 真正离开 `Playing { .. }` 一族时（第 6 帧后），`OnExit(InAction)` 才跑。

第 4 帧的安静有个前提——代码里那行 `const ALLOW_SAME_STATE_TRANSITIONS: bool = false;`。这个常量默认是 `true`：源状态**每次**转换（包括 10-2 节那种同值转换）都会让推导态也跟着发一次同值转换、重跑自己的 `OnExit`/`OnEnter`——风扇重启、`OnEnter` 里 spawn 的东西再来一套。把它设成 `false`，含义变成“算出的结果没变，就别折腾”。推导态通常都该这么设；把这行删掉再跑一遍，第 4 帧会多出一对风扇停转/重启，是个值得亲眼看一次的对比。

## 推导态改不得

`InAction` 由 `GameState` 推导而来。要是有人想绕过投币，直接把它拧到“过招中”呢：

```rust
{{#include ../../code/ch10-states/no-compile/listing-10-10.rs:cheat}}
```

<span class="caption">Listing 10-10：行不通——计算状态没有 NextState</span>

```text
error[E0277]: the trait bound `InAction: FreelyMutableState` is not satisfied
   --> ch10-states\no-compile\listing-10-10.rs:31:27
    |
 31 | fn cheat(mut next: ResMut<NextState<InAction>>) {
    |                           ^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `FreelyMutableState` is not implemented for `InAction`
```

`NextState<S>` 的类型参数要求 `FreelyMutableState`（“可自由改写的状态”）——`derive(States)` 和 `derive(SubStates)` 都会附带实现它，唯独 `ComputedStates` 不会。**推导态只有一个事实来源，就是 `compute`**；想改它，去改它的源。数据流单向，状态间永远不会各说各话——和第 9 章 Relationship 的“只写事实源组件”是同一个设计哲学。

最后几条把全家凑齐：

- **源可以不止一个**：`SourceStates` 写成元组 `(GameState, TutorialState)`，任何一个源转换都触发重算；包成 `Option<S>` 则“源不存在也照算”。
- **推导可以级联**：计算状态自己也是合格的源——`SubStates` 的 `#[source]` 同样可以指向一个计算状态。引擎按依赖深度排好计算顺序，循环依赖过不了编译。
- **三类状态在工具面前一律平等**：`in_state`、`OnEnter`/`OnExit`、`DespawnOnExit` 对 `States`、`SubStates`、`ComputedStates` 通用——下一节就把 `DespawnOnExit` 挂到推导态上，解决“demo 切真人不清场”的难题。

零件全齐了。关上工具箱，完整营业一天。

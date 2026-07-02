# 街机厅的一天

全章零件组装：三层状态各司其职——`GameState` 管“谁在玩”（菜单/演示局/真人局），`IsPaused` 子状态管“暂停”（只在游戏中存在），`InAction` 计算状态管“画面上是否正在过招”（推导而来，负责搭台拆台）。一天的营业从待机字幕开始：

```rust
{{#include ../../code/ch10-states/src/main.rs}}
```

<span class="caption">Listing 10-11：完整示例——街机厅的一天（src/main.rs）</span>

```console
cargo run -p ch10-states
```

```text
—— 第 1 帧 ——
  屏幕：《勇者斗史莱姆》——投币开始
—— 第 2 帧 ——
  机器等得无聊，自顾自开了一局演示揽客。
  屏幕：《勇者斗史莱姆》——投币开始
  [OnEnter(InAction)] 风扇转起来，勇者与史莱姆登场
—— 第 3 帧 ——
  屏幕：（演示局）勇者与史莱姆你来我往
—— 第 4 帧 ——
  罗兰看得手痒：让我来！（叮——切到真人局）
  屏幕：（演示局）勇者与史莱姆你来我往
—— 第 5 帧 ——
  屏幕：罗兰操刀，勇者大杀四方
  记分牌：+100，共 100 分
—— 第 6 帧 ——
  老板：汽水好了！——罗兰按下暂停。
  屏幕：罗兰操刀，勇者大杀四方
  记分牌：+100，共 200 分
  [OnEnter(Paused)] “PAUSED”压上画面
—— 第 7 帧 ——
  屏幕：PAUSED（史莱姆保持着挨打的姿势）
—— 第 8 帧 ——
  罗兰一抹嘴：继续！
  屏幕：PAUSED（史莱姆保持着挨打的姿势）
  [OnExit(Paused)] “PAUSED”字样消失
—— 第 9 帧 ——
  屏幕：罗兰操刀，勇者大杀四方
  记分牌：+100，共 300 分
—— 第 10 帧 ——
  屏幕：通关！罗兰心满意足，机器退回待机画面。
  屏幕：罗兰操刀，勇者大杀四方
  记分牌：+100，共 400 分
  [OnExit(InAction)] 风扇停转，机台歇了
—— 第 11 帧 ——
  老板清点：场上还剩 0 个角色。打烊喽。
  屏幕：《勇者斗史莱姆》——投币开始
（run() 返回，街机厅的一天结束了）
```

三处值得回头多看一眼：

- **舞台跟着推导态走，而不是跟着源状态**。角色在 `OnEnter(InAction)` 登场、挂 `DespawnOnExit(InAction)` 退场。于是第 4 帧演示局切真人局——源状态换了值——风扇没重启、角色没重生（`ALLOW_SAME_STATE_TRANSITIONS = false` 把同值推导挡掉了）；直到第 10 帧真正回菜单，清场才发生，第 11 帧老板数到 0。要是把搭台和清场挂在 `GameState` 的两个 `Playing` 值上（各挂各的），罗兰接管的瞬间就会看到角色被清掉再生成一遍。**“哪一层状态变了要折腾，哪一层不用”，正是分层建模的全部意义**。
- **计分牌的条件是两层状态的与**：`in_state(Playing { demo: false }).and_then(in_state(IsPaused::Running))`——真人局且没暂停。第 3 帧（演示）和第 7 帧（暂停）它都沉默，两种沉默来自两个不同的状态机，但写在同一行 `run_if` 里。第 6 章的条件组合在状态机上原样可用。
- **暂停往返没有惊动任何别人**。第 6～8 帧 `IsPaused` 来回切，`GameState` 与 `InAction` 全程不动——没有 `OnExit(InAction)`、没有清场、记分不清零。子状态把“小动作”锁在了自己那一层。

## 这台状态机还会再开机

第 20 章的 Breakout 用的就是本章这套骨架：`Menu / Playing / GameOver` 三态加暂停子状态，菜单 UI 与局内实体各挂各的 `DespawnOnExit`。届时屏幕上真的会有画面，而状态机的部分你已经全部见过。

## 小结

- **状态描述 App 的全局阶段**：`#[derive(States)]` 的 enum，经 `init_state` 注册后以两个资源存在——`State<S>` 读当前值，`NextState<S>` 写切换申请。`StatesPlugin` 是前提（`DefaultPlugins` 自带，`MinimalPlugins` 没有）
- **切换是延迟的**：`set` 只填申请单，下一帧帧首的 `StateTransition` 调度（`PreUpdate` 之后）统一受理——同一帧内全世界看到一致的状态
- **逐帧启停用 `run_if(in_state(s))`**；换幕瞬间用三个转换调度：`OnExit(旧)` → `OnTransition` → `OnEnter(新)`，每次转换各跑一遍；`init_state` 还会在 `PreStartup` 之前触发一次进入默认态的启动转换
- **同值转换**：`set` 同值照样重跑 `OnExit`/`OnEnter`，`set_if_neq` 防抖（写 `(*next).set_if_neq`，绕开 `ResMut` 的同名方法）
- **状态作用域实体**：spawn 时挂 `DespawnOnExit(s)`/`DespawnOnEnter(s)`，转换期间自动 despawn（含子树）；与“`OnEnter` 搭台”构成标准搭配；同值转换照样清场——再 `set` 一次当前状态就是一场干净的重开
- **SubStates 随父而生灭**：`#[source(父 = 模式)]`，匹配则在场（每次出生重置为 `#[default]`），不匹配则连资源一起消失；在场时可自由 `set`
- **ComputedStates 由 `compute` 单向推导**：`None` 即不存在；没有 `NextState`（不实现 `FreelyMutableState`，编译期拦截）；`ALLOW_SAME_STATE_TRANSITIONS = false` 让“结果没变”不折腾；源可以是元组、可以级联
- 转换同时广播 `StateTransitionEvent<S>` 消息，调试工具 `log_transitions` 基于它实现

## 练习

1. **打扫得太勤快**：给 Listing 10-7 的待机画面贴一张“今日免费试玩”海报——`Startup` 里 spawn 一次，挂 `DespawnOnExit(GameState::Menu)`，等真正开局时由引擎撕掉。改一改剧本：投币之前，先让罗兰的手肘在菜单里撞出一次 `set(GameState::Menu)`。预测海报的下场——画面从头到尾都没离开过 `Menu`，它能幸免吗？运行验证；再把这一撞改成 `set_if_neq`，确认海报能撑到开局。
2. **演示局也该歇手**：在 Listing 10-11 里，演示局打到一半罗兰还没来，老板想直接按暂停去吃饭。预测：暂停对演示局有效吗？`IsPaused` 的 `#[source]` 模式匹配的是哪些值？改剧本验证你的预测。
3. **第四种沉默**：Listing 10-11 的记分牌在演示局和暂停时都沉默。再给它加一种沉默——通关后的 `Menu` 里分数应该清零重来吗？分别用“`OnExit(InAction)` 里清零”和“把 `Score` 资源换成挂 `DespawnOnExit(InAction)` 的实体组件”两种方案实现，体会资源与状态作用域实体的取舍。
4. **亲眼看一次默认值的代价**：删掉 Listing 10-9 里的 `ALLOW_SAME_STATE_TRANSITIONS` 那行（回到默认 `true`），预测第 4 帧会多出哪两行输出，运行对答案。

下一章收起剧场，掀开引擎盖：`Query` 和 `Commands` 之下还有一层更直接的世界访问——`World`、`EntityRef`/`EntityMut`、独占系统。读懂它们，官方源码和高级示例就不再是天书。

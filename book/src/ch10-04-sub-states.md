# SubStates：挂在别的状态名下

给街机加暂停键。第一反应是扩充 enum——`Menu`、`Playing`、`Paused` 三个变体平起平坐。写两行就会发现不对劲：暂停只是游戏中的一个小动作，可在三平级的世界里，`Playing → Paused` 和 `Playing → Menu` 是同一种事——都是“离开 Playing”。上一节挂在角色身上的 `DespawnOnExit(GameState::Playing)` 不会区分这两者：**罗兰按一下暂停，勇者和史莱姆当场被清掉了**。你当然可以换成 bool 资源 `is_paused` 来绕开，但那又回到了本章开篇的老路：没有换幕挂载点，回菜单时还得记着手工把它复位。

“暂停”真正的形状是：**只在游戏中才存在的一个小状态机**。Bevy 把这种从属关系做成了 `SubStates`：

```rust
{{#include ../../code/ch10-states/examples/listing-10-08.rs:states}}
```

<span class="caption">Listing 10-8（其一）：用 #[source] 声明从属——IsPaused 只在 Playing 期间存在</span>

和 `States` 比只有两处不同：derive 换成 `SubStates`，外加一行 `#[source(GameState = GameState::Playing)]`——**源状态**等于 `Playing` 时我存在，否则我（连同 `State<IsPaused>` 资源一起）消失。`#[default]` 现在多了一层含义：每次“出生”都从 `Running` 起步。

注册用 `add_sub_state`，使用上和普通状态完全一致——`in_state`、`OnEnter`/`OnExit`、`NextState` 全套照用：

```rust
{{#include ../../code/ch10-states/examples/listing-10-08.rs:main}}
```

<span class="caption">Listing 10-8（其二）：战斗挂的条件从 in_state(Playing) 换成了 in_state(IsPaused::Running)</span>

注意战斗系统的条件——不再是“游戏中”，而是“游戏中**且**没暂停”，一个条件同时表达了两层：`IsPaused::Running` 成立的前提是 `IsPaused` 存在，而它存在就意味着 `GameState::Playing`。剧本让罗兰把暂停键用了个遍：

```rust
{{#include ../../code/ch10-states/examples/listing-10-08.rs:script}}
```

<span class="caption">Listing 10-8（其三）：暂停取汽水，又带着暂停直接退到菜单</span>

```console
cargo run -p ch10-states --example listing-10-08
```

```text
—— 第 1 帧 ——
  屏幕：《勇者斗史莱姆》——投币开始
—— 第 2 帧 ——
  罗兰投币。（叮）
  屏幕：《勇者斗史莱姆》——投币开始
  [OnEnter(IsPaused::Running)] 机内时钟走字
—— 第 3 帧 ——
  屏幕：勇者挥剑，史莱姆向后弹开
—— 第 4 帧 ——
  老板：罗兰，你的汽水！——罗兰按下暂停去拿。
  屏幕：史莱姆鼓起来，撞向勇者
  [OnEnter(IsPaused::Paused)] “PAUSED”压上画面
—— 第 5 帧 ——
  屏幕：PAUSED（史莱姆保持着挨打的姿势）
—— 第 6 帧 ——
  小芙：门口有杂耍！罗兰顾不上恢复，直接退到待机画面。
  屏幕：PAUSED（史莱姆保持着挨打的姿势）
  [OnExit(IsPaused::Paused)] “PAUSED”字样消失
—— 第 7 帧 ——
  （此刻 State<IsPaused> 资源：已经不存在）
  屏幕：《勇者斗史莱姆》——投币开始
—— 第 8 帧 ——
  罗兰回来了，再投一币。（叮）
  屏幕：《勇者斗史莱姆》——投币开始
  [OnEnter(IsPaused::Running)] 机内时钟走字
—— 第 9 帧 ——
  屏幕：勇者挥剑，史莱姆向后弹开
—— 第 10 帧 ——
  老板：打烊喽。
  屏幕：史莱姆鼓起来，撞向勇者
```

这趟剧本验证了子状态的三条契约：

- **随父而生**：投币进 `Playing` 的同一次转换里，`IsPaused` 出现并走了自己的 `OnEnter(Running)`（第 2、3 帧之间）。进入的顺序是先父后子——父状态的 `OnEnter` 先跑，子状态的随后。
- **随父而亡，且死得体面**：第 6 帧带着暂停直接退菜单，引擎自动跑了 `OnExit(IsPaused::Paused)`——“PAUSED”字样是引擎收走的，不是我们写代码收的。退出的顺序反过来，先子后父：子状态先退场，父状态的 `OnExit` 殿后。第 7 帧确认 `State<IsPaused>` 资源已经不存在。
- **每次出生都是新的**：第 8 帧再投币，`IsPaused` 重置回 `#[default]` 的 `Running`——上一局的暂停不会阴魂不散地跟进新一局。对比一下：如果用 bool 资源实现暂停，这里就是最容易忘的那行“重置代码”。

两条边界补齐：

- 子状态在场时可以自由 `set`（暂停/恢复就是普通的 `NextState<IsPaused>` 写入）；**不在场时 `set` 无效**——`Menu` 里按暂停键，引擎只当没看见。
- 父状态在源值**之内**变动时（下一节会出现 `Playing { demo: true } → Playing { demo: false }` 这种换法），子状态保持现值不重置——`#[source]` 的右边其实是个模式，匹配上就算“仍在场”。

子状态解决了“从属”。还有一种相反的需求：不是在状态下面挂新状态，而是从已有状态里**提炼视图**。最后一种状态类型出场。

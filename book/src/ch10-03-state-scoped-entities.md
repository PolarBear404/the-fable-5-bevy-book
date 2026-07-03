# 状态作用域实体：自动清场

`OnEnter(Playing)` 里生成的勇者和史莱姆，是这一局的人。一局结束回到菜单，他们就该退场。先看看没人管退场会怎样——开局生成两个角色，别的什么都不做：

```rust
{{#include ../../code/ch10-states/examples/listing-10-06.rs:spawn}}
```

<span class="caption">Listing 10-6（其一）：只管生，不管收</span>

每帧清点画面上的角色：

```rust
{{#include ../../code/ch10-states/examples/listing-10-06.rs:report}}
```

<span class="caption">Listing 10-6（其二）：清点系统</span>

剧本是一场败仗：投币、Game Over 回菜单、不服气再投一币。运行：

```console
cargo run -p ch10-states --example listing-10-06
```

```text
—— 第 1 帧 ——
  罗兰投币。（叮）
  画面（Menu）上空无一人
  [OnEnter(Playing)] 勇者与史莱姆登场
—— 第 2 帧 ——
  画面（Playing）上站着 2 个角色：勇者、史莱姆
—— 第 3 帧 ——
  屏幕：史莱姆扑倒勇者——GAME OVER，退回待机画面。
  画面（Playing）上站着 2 个角色：勇者、史莱姆
—— 第 4 帧 ——
  画面（Menu）上站着 2 个角色：勇者、史莱姆
—— 第 5 帧 ——
  罗兰：不服，再来！（叮）
  画面（Menu）上站着 2 个角色：勇者、史莱姆
  [OnEnter(Playing)] 勇者与史莱姆登场
—— 第 6 帧 ——
  画面（Playing）上站着 4 个角色：勇者、史莱姆、勇者、史莱姆
—— 第 7 帧 ——
  老板：打烊喽。
  画面（Playing）上站着 4 个角色：勇者、史莱姆、勇者、史莱姆
```

两处病灶，一处比一处重：第 4 帧，**待机画面上站着上一局没散场的勇者和史莱姆**；第 6 帧，新一局的 `OnEnter` 又生成一套，**两个勇者对两只史莱姆**，场面彻底失控。这就是开篇说的“幽灵实体”：生成有挂载点，退场全靠自觉。

按第 8 章的思路，你可能想到在 `OnExit(Playing)` 里写个清场系统——能行，但每写一类“局内实体”都得记着把它纳入清单。Bevy 的做法更顺手：**让实体自己声明归属**。

## DespawnOnExit：把退场写在出生证上

修复只需要在生成时多挂一个组件：

```rust
{{#include ../../code/ch10-states/examples/listing-10-07.rs:spawn}}
```

<span class="caption">Listing 10-7：挂上 DespawnOnExit，离开 Playing 时引擎自动清场</span>

`DespawnOnExit(state)` 是个普通组件，意思是：**世界离开 `state` 的那一刻，把我连同子树一起 despawn**。同样的剧本再跑一遍，结尾添了一场戏——上一节撞过开始键的那只手肘，这回落在挂好了 `DespawnOnExit` 的场子上：

```console
cargo run -p ch10-states --example listing-10-07
```

```text
—— 第 1 帧 ——
  罗兰投币。（叮）
  画面（Menu）上空无一人
  [OnEnter(Playing)] 勇者与史莱姆登场
—— 第 2 帧 ——
  画面（Playing）上站着 2 个角色：勇者、史莱姆
—— 第 3 帧 ——
  屏幕：史莱姆扑倒勇者——GAME OVER，退回待机画面。
  画面（Playing）上站着 2 个角色：勇者、史莱姆
—— 第 4 帧 ——
  画面（Menu）上空无一人
—— 第 5 帧 ——
  罗兰：不服，再来！（叮）
  画面（Menu）上空无一人
  [OnEnter(Playing)] 勇者与史莱姆登场
—— 第 6 帧 ——
  画面（Playing）上站着 2 个角色：勇者、史莱姆
—— 第 7 帧 ——
  罗兰探身够汽水，手肘又撞上了开始键——又一次 set(Playing)！
  画面（Playing）上站着 2 个角色：勇者、史莱姆
  [OnEnter(Playing)] 勇者与史莱姆登场
—— 第 8 帧 ——
  画面（Playing）上站着 2 个角色：勇者、史莱姆
—— 第 9 帧 ——
  老板：打烊喽。
  画面（Playing）上站着 2 个角色：勇者、史莱姆
```

第 4 帧空无一人，第 6 帧恰好一套——两处病灶都好了。第 7 帧的手肘另有看头：又一次同值 `set(Playing)`，`OnEnter` 照例重跑、又生成了一套角色，可第 8 帧清点仍是 2 个，不是 Listing 10-6 那种越攒越多。差就差在**同值转换也是一次不折不扣的转换，清场照做**：旧的一套在转换里先被收走，`OnEnter` 再摆上新的一套——一肘子换来一场干净的重开。上一节说“重开本关就是再 `set` 一次当前关卡”，兑现的正是这个效果；反过来，不想被手滑重开，`set_if_neq` 拦下的同值转换连清场带搭台一起拦掉。

几条边界，把这个组件用稳：

- **清场发生在转换期间**，与 `OnExit(Playing)` 同一阶段、且保证在 `OnEnter(新状态)` 运行之前落地——新状态的搭台系统看到的一定是清过的场子。
- **despawn 是第 9 章的那个 despawn**：带 `Children` 等 `linked_spawn` 关系的，整棵子树一起走。给“局内场景”的根实体挂一个 `DespawnOnExit`，整棵树就托管了。
- 还有个反向的兄弟 **`DespawnOnEnter(state)`**：进入某状态时清。常见用法是挂在“游戏结束”画面的遗留物上——进入 `Menu` 就扫掉。

**“OnEnter 搭台 + DespawnOnExit 挂牌”就是 Bevy 管理阶段性实体的标准搭配**：生成处声明归属，退场全自动，加多少种局内实体都不用维护清单。第 20 章的 Breakout 会原样复用这套搭配。

到这里，单层状态机的全套工具齐了。但真实游戏的状态往往不止一层——“暂停”就不是和“菜单/游戏中”平级的阶段，它只在游戏中才有意义。下一节给状态分层。

# 资源的变更检测

第 4 章用 `Changed<Health>` 只挑“动过的”行。资源也有同款能力，但形式不同：资源不在实体表里，没有行可筛，所以变更检测不走查询过滤器，而是 `Res`/`ResMut` 自带的两个方法——

- **`is_changed()`**：自本系统上次运行以来，这份资源被写过（或刚插入）吗？
- **`is_added()`**：自本系统上次运行以来，这份资源是新插入的吗？

判定口径与组件版完全一致：**写访问即变更，不比较值**。`ResMut` 一被可变解引用就记账，哪怕写进去的是原值。对策也是第 4 章那位老朋友：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-07.rs:shoot}}
```

<span class="caption">Listing 5-7（节选）：set_if_neq——值没变就不留变更记录</span>

`set_if_neq` 先用 `PartialEq` 比较新旧值（所以 `Score` 这次 derive 了 `PartialEq`），不同才写入并记账，相同则什么都不发生。记分牌据此偷懒：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-07.rs:scoreboard}}
```

<span class="caption">Listing 5-7（节选）：is_changed 只在分数真的变了时刷新</span>

```console
cargo run -p ch05-resources --example listing-05-07
```

```text
第 1 枪：命中
记分牌通电，开始计分
记分牌刷新 → 10 分
第 2 枪：脱靶
第 3 枪：命中
记分牌刷新 → 20 分
```

三处看点：

1. **首帧一切皆新**，资源版同样成立：`Score` 是开赛前 `insert_resource` 进来的，第一帧 `is_added()` 和 `is_changed()` 双双为真——“通电”只此一声；
2. 第 2 枪脱靶，`set_if_neq(Score(10))` 发现 10 == 10，不记账——**记分牌整帧沉默**；
3. 若把 `shoot` 最后两行换成朴素的 `score.0 += hit`，第 2 枪也会触发“记分牌刷新 → 10 分”——加零也是写，写了就算变。真实游戏里记分牌刷新可能牵扯 UI 重绘，这笔账不省白不省。

> 变更检测还能再省一步：第 6 章的 `run_if(resource_changed::<Score>)` 可以让 `scoreboard` 在分数没变的帧**根本不运行**，连 `if` 都不用进。

## 拼起来：打靶场四枪

本章全部内容合成一个程序：靶子是实体（每环一个基础分），难度、规则、计分板、双倍卡是资源——每实体数据与全局数据同台分工：

```rust
{{#include ../../code/ch05-resources/src/main.rs}}
```

<span class="caption">Listing 5-8：完整示例——打靶场四枪（src/main.rs）</span>

```console
cargo run -p ch05-resources
```

```text
打靶场开张：职业场，全场 2 倍记分
第 1 枪：命中 外环，+4 分
记分牌 → 4 分
第 2 枪：命中 红心，+20 分
摊主：红心都让你打中了，这张双倍卡送你！
记分牌 → 24 分
第 3 枪：命中 内环，+20 分（双倍卡生效）
摊主：双倍卡到期，收回了。
记分牌 → 44 分
第 4 枪：脱靶
```

对着输出清点本章的工具：

- **构建期的初始化顺序**：`Difficulty` 用 `insert_resource` 先就位，`ScoreRules` 的 `FromWorld` 紧随其后按难度算出 2 倍率，`Score` 由 `init_resource` 按 `Default` 给 0 分开局——三行的先后不是巧合，是依赖；
- `setup_range` 在 `Startup` 里既 `spawn` 靶子实体又读 `Res<ScoreRules>`——组件归实体，规则归全局，一个系统两头取用；
- `shoot` 的得分公式同时取三处数据：靶环基础分（组件 `Points`）、全场倍率（资源 `ScoreRules`）、双倍卡行情（**`Option<Res<DoubleCard>>`**）；
- 摊主第 2 枪后发卡、第 3 枪后收卡，命令帧末落地，所以双倍只罩住第 3 枪——**运行期插拔**资源的现场；
- 第 4 枪脱靶，`set_if_neq` 加零不记账，**`is_changed`** 让记分牌全场静默——四枪只刷新三次。

## 小结

- **Resource = World 里按类型存放、全局唯一的数据**；`#[derive(Resource)]` 即可，要求 `Send + Sync + 'static`。每实体一份的数据用 Component，全场一份的用 Resource，只有自己用的记忆用 `Local`
- **`Res<T>` 读、`ResMut<T>` 写**；访问声明照常驱动并行调度。同一系统对同一资源一读一写 → **B0002** panic，删掉冗余的 `Res` 即可
- **缺失即 panic**：`Res` 把“资源不存在”当 bug；时有时无是设计时，用 `Option<Res<T>>` 分支处理，或 `If<Res<T>>` 跳过系统
- **三条注册路径**：`App::insert_resource`/`init_resource` 构建期立即生效；Plugin 的 `build` 里同名调用随 `add_plugins` 执行；`Commands` 版本运行期排队、同步点落地。增删资源属于结构修改，“改值直接写、改结构走 Commands”对资源同样成立
- **`insert` 覆盖，`init` 让位**；`init_resource` 的初始值来自 `FromWorld`（实现 `Default` 的类型自动获得），`from_world` 能读整个 World——**初始化顺序就是书写顺序**，谁依赖谁，谁写在后面
- **变更检测是方法不是过滤器**：`is_changed()`/`is_added()`；写访问即变更、首帧一切皆新的口径与组件一致；值没变就别记账，用 `set_if_neq`（要求 `PartialEq`）

## 练习

1. **共享数据**：给 Listing 5-1 加一个弹药资源 `Bullets(u32)`，初始 3 发：`shoot` 每枪 -1，没子弹时只打印“没子弹了”不再加分；`announce` 同时播报剩余弹药。跑 5 帧，确认后两帧分数不再涨。体会同一份资源被两个系统一写一读的分工。
2. **初始化顺序**：不看书，先预测——把 Listing 5-6 职业场那段的 `insert_resource` 与 `init_resource` 两行对调会发生什么？什么时机发生？然后运行验证。再试第三种写法：在 `init_resource::<ScoreRules>()` **之前**手动 `insert_resource(ScoreRules { bullseye: 99 })`，解释输出为什么是 99。
3. **变更检测**：把 Listing 5-8 `shoot` 末尾的 `set_if_neq` 换回 `score.0 += gained`，先预测第 4 枪后记分牌的行为，运行验证；再想想 `scoreboard` 里若加一段 `is_added` 的欢迎语，它会在第几枪出现，为什么只出现一次。

下一章直面那个被我们反复绕开的问题：系统到底**什么时候**跑？`Startup` 和 `Update` 之外还有哪些调度、`.chain()` 之外还有什么排序手段、同步点究竟插在哪——Bevy 程序行为的根源，尽在 Schedule。

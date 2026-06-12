# 排序：chain、before 与 after

进入微观世界之前，先把一条原则说透：**同一调度内，系统默认没有顺序。**

这不是设计缺陷，是并行的代价与红利。第 4 章说过，调度器按签名计算每个系统的访问集合，互不冲突的系统会被扔到多个线程同时跑。“同时”就意味着谈不上先后；强行给所有系统排一条总队，等于亲手放弃多核。所以 Bevy 的立场是：**顺序是稀缺品，谁需要谁声明。**

大多数系统确实不需要——羊吃草和商店补货互不相干，谁先谁后无所谓。真正危险的是这种组合：**两个系统访问同一份数据、其中至少一方在写、又没有声明顺序**。结果不是崩溃，而是更隐蔽的东西——结果取决于调度器本次的心情。

## .chain()：流水线

声明顺序的第一件工具是老朋友。把矿场的三道工序串起来：

```rust
{{#include ../../code/ch06-schedules/examples/listing-06-03.rs:main}}
```

各道工序都对同一份库存动手：

```rust
{{#include ../../code/ch06-schedules/examples/listing-06-03.rs:systems}}
```

<span class="caption">Listing 6-3：.chain()——流水线按书写顺序执行</span>

```console
cargo run -p ch06-schedules --example listing-06-03
```

```text
矿工：+3 矿石（库存 3）
冶炼炉：出锭 3 根（库存 3）
铸币机：+3 金币（金库 3）
矿工：+3 矿石（库存 3）
冶炼炉：出锭 3 根（库存 3）
铸币机：+3 金币（金库 6）
```

每帧挖 3、炼 3、铸 3，分毫不差。`.chain()` 在元组的相邻元素之间逐对建立“前者先跑完”的约束——注意它管的是**这一个元组内部**，不是整个调度。

试着删掉 `.chain()` 重跑几次：三个系统都写 `Stockpile`，调度器只保证它们不同时跑，不保证次序。也许铸币机先开机，铸了 0 枚金币；也许冶炼炉排最后，矿石攒到下一帧才出锭。这种“账对不上但不报错”的漂移，正是本节末尾要请出检测工具的原因。

## before / after：点名排序

`.chain()` 要求系统写在同一个元组里。但顺序需求常常跨越注册点——典型的是和**别人的**系统排顺序：那可能是另一个文件里的另一个 Plugin，轮不到你把它塞进自己的元组。这时用 `.before()` / `.after()` 点名：

```rust
{{#include ../../code/ch06-schedules/examples/listing-06-04.rs:main}}
```

<span class="caption">Listing 6-4：before/after——把自己嵌进别人的流水线</span>

输出与 Listing 6-3 的一帧完全相同。两个细节值得停一下：

1. **约束跨越了注册点。**`dig` 和 `mint` 注册在前一行，冶炼炉后到，照样把自己嵌进中间。约束描述的是调度图上的边，不在乎代码写在哪。
2. **顺序有传递性**。我们没写“`dig` 先于 `mint`”，但 `dig` → `smelt` → `mint` 两条边连起来，三者的总顺序就定了——输出里矿工永远第一个发言。

`before`/`after` 有两个不报错的坑，提前立牌：

- **点名的目标必须真的在同一调度里。**`smelt.after(dig)` 并不会替你注册 `dig`；如果 `dig` 压根没注册，这条约束就悬空作废，没有警告。
- **跨调度的排序会被静默忽略**。给 `Update` 的系统声明 `.before(某个 PostUpdate 系统)` 不会生效——不同调度的先后由上一节那张表决定，轮不到系统级约束插手。

选型口诀：**一串自己人，`.chain()`；和别人对齐，`before`/`after`。**

## 让调度器自己交代：歧义检测

“顺序不定 + 数据冲突”的组合有个正式名字——**执行顺序歧义**（ambiguity）。它不会报错，全靠人眼盯防未免太业余；调度器其实自带检举功能，默认关着，打开看看：

```rust
{{#include ../../code/ch06-schedules/examples/listing-06-05.rs:main}}
```

<span class="caption">Listing 6-5：打开歧义检测，让调度器点名可疑系统对</span>

`edit_schedule` 给指定调度调整构建设置，`ambiguity_detection: LogLevel::Warn` 让它在构建时把所有歧义对用警告打出来。警告走日志通道，所以裸 `App` 要手动添上 `LogPlugin`（用 `DefaultPlugins` 时自带）。运行：

```console
cargo run -p ch06-schedules --example listing-06-05
```

```text
审计员记下库存：0
2026-06-12T00:24:26Z  WARN bevy_ecs::schedule::schedule: Update schedule built
successfully, however: 1 pairs of systems with conflicting data access have
indeterminate execution order. Consider adding `before`, `after`, or
`ambiguous_with` relationships between these:
 -- audit and dig
    conflict on: ["listing_06_05::Stockpile"]
```

调度器点名了 `audit` 和 `dig`，连冲突的数据是哪份都列了出来。注意程序输出：这次审计员数到 0——他抢在矿工前面跑了。下次、换台机器、换个 Bevy 版本，都可能变成 3。没有任何承诺，这正是警告想说的。

两种处理方式：加 `before`/`after` 把顺序定死；或者确认这对冲突无害（比如两个系统只是各自累加统计量，谁先谁后结果一样），用 `.ambiguous_with(另一方)` 显式按下警告——留下一句注释说明为什么无害，这是给未来的自己写的。

> 歧义检测是按调度配置的开关，开发期给 `Update` 开着、发布前关掉是常见做法。引擎内部系统之间偶尔也有已知无害的歧义，全局打开时见到不必惊慌。

排序工具还差最后一块：`before`/`after` 点名的都是单个系统，几十个系统的工程里，难道要两两点名？下一节把系统编成班组。

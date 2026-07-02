# 变更检测的发条

第 4 章学 `Changed`/`Added` 时记下了两条经验法则：“窗口是本系统上一次运行到这一次”“写访问本身就是变更”。法则好用，但它们为什么成立、边界在哪，当时按下没表。盘点正好用得上——艾达只想登记“今年动过的账”——把发条拆开看。

## 秒针、印章、窗口

机件一共三样：

- **秒针**。World 里有一只全局计数器 `Tick`，**每个系统每运行完一次，它加一**——不是每帧加一。它是变更检测的时间单位。
- **印章**。每个组件实例（资源同理）随身带两枚 tick：`added`（何时挂上）和 `changed`（最近何时被写）。通过 `&mut` 解引用组件的那一刻，引擎把当前秒针值盖进 `changed`——不比较新旧值，碰了就盖。
- **窗口**。每个系统记着自己上次运行时的秒针值 `last_run`。`Changed<T>` 过滤器做的事不过是一道算术题：`changed` 印是否落在 `(last_run, this_run]` 窗口里。

三样机件都能在运行时直接看。查询的 D 槽位写 `Ref<T>`——读访问与 `&T` 同价，但像第 5 章 `Res` 一样带上了变更检测的方法；系统参数 `SystemChangeTick` 报出本系统的窗口两端：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-11.rs:audit}}
```

<span class="caption">Listing 11-11（其一）：账房——Ref 看印章，SystemChangeTick 看窗口</span>

对面安排一个剧本，五帧演四种写法：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-11.rs:script}}
```

<span class="caption">Listing 11-11（其二）：四种写法，账房只听得见两种</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-11
```

```text
—— 第 1 帧 ——
  账房：镇仓的账动了！现存 50 袋（added=true，last_changed=2，窗口 (1036800006, 8]）
—— 第 2 帧 ——
  小工掸灰，把账目原样抄了一遍（解引用了 &mut，值没变）。
  账房：镇仓的账动了！现存 50 袋（added=false，last_changed=31，窗口 (8, 32]）
—— 第 3 帧 ——
  小工长记性了：set_if_neq——值不变就不惊动账房。
  账房：无事。（账面 50 袋）
—— 第 4 帧 ——
  掌柜的悄悄补了 3 袋陈账（bypass_change_detection）。
  账房：无事。（账面 53 袋）
—— 第 5 帧 ——
  镇仓入库 10 袋，正大光明记一笔。
  账房：镇仓的账动了！现存 63 袋（added=false，last_changed=108，窗口 (84, 109]）
```

五帧逐条对账：

1. **第 1 帧**：`added=true`，组件出生时两枚印一起盖（所以 `Added` 算 `Changed` 的子集）。看窗口左端那个天文数字——账房从未运行过，没有真实的 `last_run`，引擎把窗口左端拉到允许的最远处。“首帧一切皆新”这条经验法则的机械成因，就是这个被拉满的窗口。
2. **第 2 帧**：误报的解剖。小工只是原样抄写，但 `&mut` 解引用即盖章：`last_changed=31` 落在账房的窗口 `(8, 32]` 里——算术成立，账房就喊。顺便注意秒针的步幅：一帧过去，窗口从 8 走到 32，因为 `MinimalPlugins` 这一帧里跑了二十来个系统，每个都让秒针加一。
3. **第 3 帧**：`set_if_neq` 先比较再写，值相同就**不解引用**，印章保持旧值、落在窗口外——这就是它防误报的全部原理（第 4、5 章用过的工具，机制至此透明）。
4. **第 4 帧**：`bypass_change_detection()` 给你一个**绕过盖章的裸引用**——账面明明改成了 53，账房毫不知情。看着输出体会一下危险：账实不符，且永远不会有人发现。它的正当用途很窄：改的是纯内部缓存、确实不想惊动任何下游时（网络回滚、插值缓存这类场景）才碰它。
5. **第 5 帧**：正大光明的写法，`108 ∈ (84, 109]`，理所应当被听见。

## 谁干的：changed_by

印章记了“何时”，不记“何人”。库房黄油连续两天对不上数，后厨和酒馆互相指认——这种时候有个专门的办案工具。先在 Cargo.toml 里开 `track_location`：

```toml
{{#include ../../code/ch11-deep-ecs/Cargo.toml:deps}}
```

<span class="caption">Listing 11-12（其一）：track_location——给每枚印章附上案发地点</span>

开了它，`Ref`/`Res` 等就多出一个能用的 `changed_by()`，报出**最近一次写发生的源码位置**。两名嫌疑人，一名侦探：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-12.rs:suspects}}
```

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-12.rs:detective}}
```

<span class="caption">Listing 11-12（其二）：changed_by()——印章上的指纹</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-12
```

```text
—— 第 1 帧 ——
  侦探：库房的黄油余 12 桶——经手处：ch11-deep-ecs\examples\listing-11-12.rs:18:22
—— 第 2 帧 ——
  侦探：库房的黄油余 11 桶——经手处：ch11-deep-ecs\examples\listing-11-12.rs:34:9
—— 第 3 帧 ——
  侦探：库房的黄油余 8 桶——经手处：ch11-deep-ecs\examples\listing-11-12.rs:46:9
```

三帧三个地址，回头对照源码：第 18 行是 `Startup` 里的 spawn（出生即盖章，地点都给你记着），第 34 行是后厨的 `-= 1`，第 46 行是酒馆的 `-= 3`。多个系统都在写同一个组件、不知道哪只手干的——这是实际项目里相当高频的疑难杂症，`changed_by` 一行定位。代价是每次写都要记一份位置信息，所以它做成了默认关闭的 feature：调试时开，发布构建记得关。

发条拆完，两条旧法则升格为机制：窗口是秒针区间的算术，误报是“碰即盖章”的代价。但世界上还有一种“看不见”与印章无关——实体明明在，所有查询却都装作没看见。下一节，铁匠铺要过冬了。

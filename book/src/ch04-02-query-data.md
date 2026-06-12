# Query：精确取数据

`Query` 的类型有两个槽位：`Query<D, F>`。**D（data）说“取什么”，F（filter）说“筛哪些行”**，F 可以省略。第 3 章用过的形态都能对号入座：`Query<&Health>` 只填了 D；`Query<(Entity, &Name, &Health)>` 的 D 是元组；`Query<&mut Health, With<Monster>>` 两个槽位都有。本节穷尽 D 槽位的常用写法，过滤器留给下一节。

## 必有、可选、在场与否

到目前为止，D 里写 `&T` 意味着“该实体必须有 `T` 这一列，借我读”——缺这一列的行直接不匹配。但牧场上的住户参差不齐：羊有饥饿值，牧羊犬没有；有的戴铃铛，有的没戴。想把它们放进同一张花名册，就需要两种更宽容的取法：

```rust
{{#include ../../code/ch04-systems-queries/examples/listing-04-02.rs:spawn}}
```

```rust
{{#include ../../code/ch04-systems-queries/examples/listing-04-02.rs:roster}}
```

<span class="caption">Listing 4-2（节选）：Option 取“可能没有的列”，Has 只问在不在场</span>

- **`Option<&T>`**：实体有 `T` 就给 `Some(&T)`，没有也照样匹配，给 `None`。带上它不会缩小查询范围。
- **`Has<T>`**：只回答“这一列在不在场”，给一个 `bool`，完全不读数据。

```console
cargo run -p ch04-systems-queries --example listing-04-02
```

```text
=== 农场花名册 ===
2v0  阿黄  饥饿 —  戴铃铛
3v0  灰背  饥饿 —  无铃铛
1v0  小黑  饥饿 9  无铃铛
0v0  小白  饥饿 6  戴铃铛
```

四个实体全部上榜——`&Name` 是唯一的硬性条件，`Option` 和 `Has` 都不淘汰任何行。顺序又一次不按生成顺序来：四个实体的组件组合各不相同，分属四张子表（第 3 章讲过的 Archetype），不必惊讶。

`Option<&T>` 和 `Has<T>` 的分工很简单：要读值用前者，只看有无用后者。`Has` 还有个隐藏的好处——它不访问数据，所以永远不会和别的系统抢 `T` 这一列，这在后面讲冲突时会显出价值。

顺带补全元组的规则：D 槽位的元组最多 15 个成员，嵌套元组可以突破上限——和 Bundle 一个味道。

## 按 Entity 直取某一行

`for` 循环是“全体过一遍”，但很多逻辑长的是另一个样子：**手里攥着某个实体的 ID，只想要它那一行**。第 3 章说过 `Entity` 可以存进组件，作为指向另一个实体的引用——现在兑现它。牧羊犬阿黄心里有一只最爱的羊：

```rust
{{#include ../../code/ch04-systems-queries/examples/listing-04-03.rs:spawn}}
```

`spawn(...).id()` 当场拿到卷卷的 `Entity`（第 3 章讲过：ID 是预留的，不用等实体真正出生），转手存进阿黄的 `Favorite` 组件。之后任何系统都能顺着这个 ID 找上门：

```rust
{{#include ../../code/ch04-systems-queries/examples/listing-04-03.rs:get_mut}}
```

<span class="caption">Listing 4-3（节选）：get_mut 按 ID 精确取行（Single 参数稍后解释）</span>

**`get(entity)`** 和 **`get_mut(entity)`** 是 Query 的随机访问接口：不遍历，直接要某一行的数据，返回 `Result`。拿到 `Err` 有两种可能——实体已经不存在，或者实体还在但不匹配这条查询（缺列，或被过滤器拦下）。示例里两个分支都写了，哪边会执行，运行输出见分晓。

另一个常用搭配值得记住：**可变查询的 `iter()` 给出的是只读视图**。先用 `iter()` 配合迭代器适配器找出目标（比如生命值最低的那只羊），再用 `get_mut` 对它精确动手——“先侦察、后开刀”，本章最终示例的狼就这么捕猎。

## 恰好一个：single() 与 Single

还有一类查询，目标天生只有一个：唯一的玩家、唯一的相机、唯一的牧羊犬。为它写 `for` 循环很别扭，Bevy 给了两种正经写法。

第一种是方法 **`single()`**（可变版 `single_mut()`）：恰好一个匹配实体时返回 `Ok`，零个或多个都返回 `Err`，怎么处置由你：

```rust
{{#include ../../code/ch04-systems-queries/examples/listing-04-03.rs:single_method}}
```

第二种是系统参数 **`Single<D, F>`**——把“恰好一个”提升为整个系统的前置条件：

```rust
{{#include ../../code/ch04-systems-queries/examples/listing-04-03.rs:single_param}}
```

<span class="caption">Listing 4-3（节选）：single() 自己处理失败，Single 让系统整个跳过</span>

`Single` 在系统运行前校验：匹配的实体恰好一个，解引用直达数据（`*wolf` 就是 `&Name`，不用再迭代）；零个或多个，**这个系统本帧直接被跳过，静默无声**——不报错、不打日志。这正是两者的分野：

- 拿不到该当异常处理、函数体还有别的事要干——用 `single()`，自己接 `Result`；
- “没有就歇着，有了再干活”本身就是业务语义——用 `Single`，签名替你把守。

注意 `Query` 参数自己从不“失败”：匹配零行的查询完全合法，只是迭代零次。`Single` 是把“必须恰好一个”这个额外要求显式写进了签名。还有个近亲 **`Populated<D, F>`**：要求“至少一个”，不满足同样跳过系统，解引用后当普通 Query 用。

## 一夜变天

Listing 4-3 把这三样工具放进了一个两幕剧：第一帧岁月静好；入夜后牧场主把卷卷卖了，围栏外又摸来一只狼：

```rust
{{#include ../../code/ch04-systems-queries/examples/listing-04-03.rs:nightfall}}
```

<span class="caption">Listing 4-3（节选）：用 Commands 制造变故，Local&lt;bool&gt; 保证只发生一次</span>

```console
cargo run -p ch04-systems-queries --example listing-04-03
```

```text
阿黄 给 卷卷 留了口粮（饥饿降到 3）
独狼 灰背 在围栏外游荡，盯紧它
狼口普查：只有 灰背 一只
〔夜里：卷卷被卖掉了，又一只狼摸了过来〕
阿黄 到处找不到心爱的羊……
狼口普查：不是一只（2 只），全员戒备！
```

前四行是第一帧，后两行是第二帧。对照三个系统看第二帧：

1. **`get_mut` 走了 `Err` 分支**——`Favorite` 里的 ID 还在，但那一行没了。存起来的 `Entity` 不保证永远有效，每次使用都要面对 `Result`，这是 ECS 引用的常态；
2. **`watch_lone_wolf` 那行消失了**——狼变成两只，`Single` 校验失败，系统被静默跳过。好用，但调试时要记得这种“无声缺席”；
3. `wolf_census` 自己消化了 `Err`，照常发声。

取数据的功夫齐了。但目前为止我们圈定实体的手段还只有 `With`——下一节把过滤器配齐。

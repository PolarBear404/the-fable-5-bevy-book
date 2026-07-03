# 借用冲突与 ParamSet

本章开头说过，调度器靠访问声明保证系统**之间**互不踩脚。现在回答最后的问题：一个系统**自己的**两个参数撞上了怎么办。

场景是真实的：羊圈里要给伤员加餐，也要给幼崽加餐——两条规则，两个查询，都要写 `Hunger`：

```rust
{{#include ../../code/ch04-systems-queries/examples/listing-04-06.rs:conflict}}
```

<span class="caption">Listing 4-6（节选）：两个 &mut Hunger 查询——编译通过，运行 panic</span>

Rust 的借用检查器对此无能为力：在它眼里这只是两个不同类型的参数值，看不出它们背后是 World 里的同一列。但危险是实打实的——羊圈里有只**受伤的幼崽**小不点，两个查询都匹配它，真让两个 `&mut` 同时指向它的 `Hunger` 就违反了别名规则。Bevy 在系统初始化时替借用检查器补上这一刀：

```console
cargo run -p ch04-systems-queries --example listing-04-06
```

```text
error[B0001]: Query<'_, '_, (Name, &mut Hunger), With<Young>> in system
listing_04_06::extra_rations accesses component(s) Hunger in a way that
conflicts with a previous system parameter. Consider using `Without<T>` to
create disjoint Queries or merging conflicting Queries into a `ParamSet`.
See: https://bevy.org/learn/errors/b0001
```

程序在第一帧直接 panic，错误编号 **B0001**——Bevy 给常见运行期错误编了号，官网能按号查到详解。注意引擎判定冲突的口径：`With<Wounded>` 和 `With<Young>` **不能证明**两个查询不相交（一个实体完全可以两个标记都有），所以按“都可能访问 `Hunger`”从严处理。就算羊圈里恰好没有受伤的幼崽，这个系统也照样 panic——判的是声明，不是当前数据。

> 报错里能看到类型名，是因为本章的 crate 打开了 bevy 的 `debug` feature；不开的话所有名字都显示为占位符，排错体验天差地别：
>
> ```toml
> {{#include ../../code/ch04-systems-queries/Cargo.toml:deps}}
> ```

## 化解冲突的三板斧

报错信息自己就给了两条出路，加上一条更朴素的，按优先级排：

1. **拆成两个系统**。一个系统管伤员加餐，一个管幼崽加餐，各自只有一个查询，冲突无从谈起。多数“一个系统里塞了两个 `&mut` 查询”的场面，根源是这个系统揽了两件事——拆开往往是更好的设计，这是首选。
2. **用 `Without` 证明不相交**。如果两个查询在业务上**确实**互斥，把互斥写进过滤器：`(With<Wounded>, Without<Young>)` 配 `With<Young>`，引擎一看便知两边不可能撞车，直接放行。但在本例行不通——受伤的幼崽真实存在，加上 `Without` 会改变语义：小不点会少吃一餐。
3. **`ParamSet`**。访问真的需要重叠、又不想拆系统时的正解。

`ParamSet` 把一组互相冲突的参数打包，约束你**同一时刻只能用其中一个**：

```rust
{{#include ../../code/ch04-systems-queries/examples/listing-04-07.rs:paramset}}
```

<span class="caption">Listing 4-7（节选）：ParamSet 让两个冲突查询分时上岗</span>

成员按位置取：`p0()`、`p1()`……最多 8 个。每个 `pN()` 都可变借用整个 `ParamSet`，所以第一个查询用完之前，借用检查器不许你碰第二个——冲突从“运行期 panic”变回了 Rust 自己能管的编译期规则。引擎也认这笔账：声明在 `ParamSet` 里的访问不再触发 B0001。运行：

```console
cargo run -p ch04-systems-queries --example listing-04-07
```

```text
小不点（伤员）加餐
老灰（伤员）加餐
小不点（幼崽）加餐
=== 晚间清点 ===
小不点  饥饿 6
老灰  饥饿 4
卷卷  饥饿 3
```

小不点吃到了两餐（8 → 6）——重叠的实体被两条规则各处理一次，语义分毫不差。这正是 `Without` 给不了的结果。

## 拼起来：牧场三日

本章全部内容合成一个程序。羊群吃草，狼挑最弱的下口，牧羊犬护场，伤员静养，羊羔出生——三天的剧本全由查询驱动：

```rust
{{#include ../../code/ch04-systems-queries/src/main.rs}}
```

<span class="caption">Listing 4-8：完整示例——牧场三日（src/main.rs）</span>

```console
cargo run -p ch04-systems-queries
```

```text
—— 第 1 天 ——
灰背 咬伤了 卷卷！（生命 15）
阿黄 发现 卷卷 受了伤！
阿黄 冲出去，把 灰背 赶出了牧场
名册新增：小白
名册新增：小黑
名册新增：卷卷
· 夜幕点名
  阿黄  饥饿 —  生命 70
  小白  饥饿 5  生命 60
  小黑  饥饿 5  生命 55
  卷卷  饥饿 3  生命 15（伤）
—— 第 2 天 ——
卷卷 还在羊圈静养（生命 30）
· 夜幕点名
  阿黄  饥饿 —  生命 70
  小白  饥饿 4  生命 60
  小黑  饥饿 4  生命 55
  卷卷  饥饿 2  生命 30（伤）
—— 第 3 天 ——
卷卷 伤愈归队（生命 45）
名册新增：羊羔
· 夜幕点名
  阿黄  饥饿 —  生命 70
  小白  饥饿 3  生命 60
  小黑  饥饿 3  生命 55
  卷卷  饥饿 1  生命 45
  羊羔  饥饿 6  生命 50
```

对着输出清点本章的工具，每一件都在岗位上：

- `sunrise` 和 `lambing` 各揣一份 **`Local`**——天数计数器和“羊羔只生一次”的开关，互不相干；
- `wolf_attack` 是“**先侦察后开刀**”的标准现场：`iter()` 只读扫描配 `min_by_key` 选出最弱的羊，再 `get_mut` 精确咬下去；它的查询用 **`Without<Wounded>`** 放过伤员，狼被赶走后查询为空，循环零次，系统安静待命——空查询不需要任何判空代码;
- `sheepdog_guard` 用 **`Single`** 锁定唯一的牧羊犬，用 **`Added<Wounded>`** 只对新伤员出动——第 3 章的 `Commands` 同步点语义在这里接力：狼咬人排进队列，下一个系统就能看见新标记；
- 第 1 天的三条“名册新增”是**首帧全新**效应的现场重演：`register` 第一次运行，启动时生成的三只羊对它全是新面孔；第 3 天那条才是真正的新生儿；
- `nightfall` 一条查询点名全场：**`Or`** 圈定羊、狼、牧羊犬，**`Option<&Hunger>`** 容下没有饥饿值的阿黄，**`Has<Wounded>`** 给卷卷标上“（伤）”；
- `Sheep` 上的 `#[require(Hunger, Health)]` 让羊羔一行 `spawn((Name, Sheep))` 就五脏俱全——第 3 章的功课。

还有一件值得一提的事：**这个程序没有一处用到 `ParamSet`**。八个系统、十几条查询，靠拆分职责和过滤器就让所有访问各行其道——这是常态。`ParamSet` 是化解冲突的最后一招，不是日常工具。

## 小结

- **System = 参数全是系统参数的普通函数**；签名即访问声明，调度器据此并行。参数顺序任意，最多 16 个
- **`Local<T>`**：系统私有、跨帧存活的状态；初始值来自 `Default`；不能共享——共享是 Resource 的活
- **`app.update()` = 跑一帧**；`Startup` 只在第一帧运行。手动驱动是纯逻辑实验的利器
- **D 槽位**：`&T` 必有、`Option<&T>` 可选、`Has<T>` 只问在场、`Entity` 要行号；`get`/`get_mut` 按 ID 直取（返回 `Result`）；`single()` 自己处理“恰好一个”，**`Single`** 参数失败时静默跳过系统
- **F 槽位**：元组是且、`Or` 是或、`Without` 是非，任意嵌套；只筛行不读数据，能用 `With` 就别在 D 里写 `&T`
- **变更检测**：`Added` 看新挂上，`Changed` 看挂上或写过；窗口是本系统两次运行之间；首帧一切皆新；**写访问即变更，不比较值**（要比较用 `set_if_neq`）
- **两个参数写同一列 → B0001 panic**；化解按序：拆系统 → `Without` 证明不相交 → `ParamSet` 分时复用

## 练习

1. **数据形态**：给夜幕点名加一列：用 `Has<Sheepdog>` 在阿黄那行末尾标出“〔犬〕”。再给灰背补一个 `Hunger(9)`，看看它在第 1 天点名里的样子（提示：它当晚还在不在场？）。
2. **变更检测**：给 Listing 4-8 加一个 `health_watch` 系统（排在 `nightfall` 之前），用 `Changed<Health>` 报告当天生命值有变动的动物。先预测三天各报告谁，再运行验证——第 1 天和第 3 天的名单都值得想清楚。
3. **化解冲突**：把 Listing 4-7 的 `extra_rations` 拆成 `feed_wounded` 和 `feed_young` 两个系统（用 `.chain()` 保持顺序），确认输出与 `ParamSet` 版完全一致。体会一下为什么“拆系统”排在三板斧的第一位。

下一章解决本章 `Local` 留下的缺口：多个系统要共享的全局数据放哪里？比分、难度设置、随机数种子——它们不挂在任何游戏对象名下，World 里为它们留了另一种住所：**Resource**。第 2 章那个 `Res<Time>` 也将正式归队。

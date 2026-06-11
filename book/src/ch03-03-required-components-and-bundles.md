# Required Components 与 Bundle

第 2 章 `spawn(Camera2d)` 时说过：表里的列比你写的多——`Camera`、投影方式等一整套组件被自动补了上来。机制的名字当时就报过：**required components**（必需组件）。现在拆开看它怎么运作。

## 声明"有我必有它"

required components 让一个组件类型在定义处声明：凡是挂上我的实体，必须同时有某几个组件；spawn 时缺了哪个，引擎就地补上。声明用 `#[require(...)]` 属性：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-05.rs:definitions}}
```

<span class="caption">Listing 3-5（节选）：用 #[require] 声明必需组件</span>

这十几行声明了两条规则：

- `Monster` 要求 `Health`。只写类型名，缺失时用该类型的 `Default` 构造——所以我们给 `Health` 实现了 `Default`，默认 30 点血。
- `Golem` 要求 `Monster` 和 `Health(120)`。后者是**内联构造器**：石巨人的默认血量不是 30，而是它自己声明的 120。除了 `类型名(值)`，还有更自由的 `类型名 = 任意表达式` 形式，以及枚举变体形式（`#[require(D::One)]`）。

而且要求是**递归**的：`Golem` 要求 `Monster`，`Monster` 又要求 `Health`——挂上 `Golem` 的实体最终三者齐备。三种情形各 spawn 一个验证：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-05.rs:spawn}}
```

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-05.rs:roster}}
```

<span class="caption">Listing 3-5（节选）：三种 spawn 与按 Monster 筛选的清单</span>

```console
cargo run -p ch03-entities-components --example listing-03-05
```

```text
=== 怪物清单 ===
石巨人  HP 120
史莱姆  HP 30
史莱姆王  HP 99
```

三行输出对应三条规则（顺序又一次不按生成顺序来，意料之中）：

1. **缺失补齐**：史莱姆只 spawn 了 `Monster`，`Health` 自动补上，值是 `Default` 的 30；
2. **手动优先**：史莱姆王手动给了 `Health(99)`，required 构造器让位——你写的永远赢；
3. **递归生效**：石巨人只 spawn 了 `Golem`，却出现在 `With<Monster>` 的清单里，说明 `Monster` 被递归补上了，血量用的是 `Golem` 声明的 120，而不是 `Monster` 链上的默认 30——直接声明比间接声明优先级高。

现在回头看第 2 章就一目了然了。翻开 Bevy 源码：`Camera2d` 的定义上挂着 `#[require(Camera, Projection::Orthographic(...), Frustum = ...)]`——三种构造语法全用上了；`Sprite` 则要求 `Transform`、`Visibility` 等。所谓"自动冒出来的列"，全是各类型自己声明的标配。

这个机制的价值在于把"这类东西必须有什么"写在类型定义处，而不是散落在每个 spawn 调用里：调用方少写样板，更重要的是**忘不了**——你不可能 spawn 出一个没有 `Health` 的 `Monster`，就像不可能 spawn 出一个没法渲染的 `Sprite`。它给了组合式的 ECS 一点"继承"的便利，却没有继承树的僵硬：required 关系是按组件声明的图，不是单根的树。

## Bundle：打包一组组件

从第 2 章用到现在的元组，该给个正式名分了。**Bundle** 是"一组静态确定的组件"的抽象：`spawn` 和 `insert` 接受的、`remove` 摘除的，都是 Bundle。单个组件是 Bundle；至多 15 个成员的元组是 Bundle；元组套元组也是。

元组之外，Bundle 还有具名形式——派生宏 `#[derive(Bundle)]`：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-06.rs:bundle}}
```

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-06.rs:spawn}}
```

<span class="caption">Listing 3-6（节选）：derive(Bundle) 的定义与使用</span>

字段必须全是组件（或嵌套的 Bundle），spawn 时按字段填值。它适合"同一形状反复生成"的场合：出怪表、子弹工厂——把形状定义一次，调用处只填数据。

但要听一句源码文档里的告诫：**Bundle 只是组件的集合，不是行为单元**。它在 spawn 那一刻就解散成一个个组件，World 里不留任何"这行来自 MonsterBundle"的痕迹——所以也不存在"查询某个 Bundle"的语法，System 永远按组件组合筛选。不要把 Bundle 当成类或抽象边界来设计。

Bundle 和 required components 怎么分工？看表达的内容：

- **类型天生需要什么**——用 required components。"怪物必有血条"是 `Monster` 这个类型的内在事实，写在类型上，谁 spawn 都生效。
- **这次调用想打包什么**——用元组或 `derive(Bundle)`。"这一波出怪长这样"是调用点的便利。

Bevy 自己的取舍很能说明问题：0.15 之前引擎 API 充斥着 `SpriteBundle`、`Camera2dBundle` 这类打包类型，required components 落地后已全部移除——如今 `spawn(Camera2d)` 一个组件就够，标配由类型自己声明。

## 拼起来：地下城的一个回合

本章全部内容合成一个程序：

```rust
{{#include ../../code/ch03-entities-components/src/main.rs}}
```

<span class="caption">Listing 3-7：完整示例——组件定义、required components、批量生成与一个战斗回合（src/main.rs）</span>

```console
cargo run -p ch03-entities-components
```

```text
=== 开场清单 ===
3v0  石巨人  HP 120
1v0  史莱姆  HP 30
2v0  史莱姆王  HP 99
4v0  蝙蝠 1 号  HP 10
5v0  蝙蝠 2 号  HP 10
6v0  蝙蝠 3 号  HP 10
0v0  罗兰  HP 100
石巨人 负伤，剩 70 点生命
史莱姆 倒下了
史莱姆王 负伤，剩 49 点生命
蝙蝠 1 号 倒下了
蝙蝠 2 号 倒下了
蝙蝠 3 号 倒下了
牧师治疗了 史莱姆王
牧师治疗了 石巨人
=== 战后清点 ===
3v0  石巨人  HP 70
2v0  史莱姆王  HP 49
0v0  罗兰  HP 100
```

逐段读下来应该没有任何陌生面孔：组件定义带着 `#[require]`，地下城用元组和 `spawn_batch` 搭建，回合里值修改直接写、结构修改走 `Commands`，四个系统用 `.chain()` 串成固定顺序。`With<Monster>` 让罗兰免于被自己横扫——标记组件在干筛选的本职。

## 小结

- **定义组件**：`#[derive(Component)]`，任何自有类型皆可；空结构体是标记组件，"有没有"本身就是信息
- **Entity = 行号 + 世代号**：轻量、可复制的 ID；行号会复用，世代号防止旧 ID 错认新实体；查询遍历顺序不可依赖
- **Commands 是指令队列**：结构修改（spawn/despawn/insert/remove）排队到同步点统一应用；组件值修改经 `&mut` 查询当场生效；调度结束必清队
- **required components**：`#[require(...)]` 把"类型必备的组件"声明在定义处——缺失补齐、手动优先、递归生效
- **Bundle** 是组件包：元组或 `derive(Bundle)`；只是集合、不是行为单元，World 里不留打包痕迹

## 练习

1. **新列**：给罗兰加一个 `Mana(i32)` 组件（法力值），并让开场清单多打印一列法力。
2. **标记**：定义 `Flying` 标记组件挂到三只蝙蝠身上，再写一个只清点飞行单位的系统加进链里（提示：照着 `print_roster` 改，过滤器换成 `With<Flying>`）。
3. **require 链**：定义 `Boss` 组件，要求 `Golem` 且血量 500；只 spawn 一个 `Boss`，验证清单里它的 `Monster`、`Health` 是否齐备、血量是多少；再 spawn 一个手动带 `Health(1)` 的 `Boss`，确认手动值胜出。

下一章给 Query 正名。本章它一直在打杂——元组取多列、`With` 筛行、`&mut` 改值、连 `Entity` 都能要——但这只是它能力的零头：过滤器全家、变更检测、单实体快捷方式、借用冲突的化解，全在第 4 章。

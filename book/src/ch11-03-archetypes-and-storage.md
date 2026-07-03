# 档案室：Archetype 与两种存储

本书欠了两笔旧账。第 1 章：“同类组件在内存中连续存储……细节在第 11 章展开”；第 3 章：“World 把组件组合完全相同的实体放在同一张子表里……存储细节在第 11 章”。现在跟艾达进档案室，一次结清。

## 一种组合，一册档案

那个“子表”的正式名字是 **Archetype**（原型）：组件组合完全相同的实体归同一册。册数有多少，World 直接报：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-07.rs:archetypes}}
```

<span class="caption">Listing 11-7（其一）：archetypes().len()——眼看着册数涨</span>

```text
空世界的档案册数：2
住进两户（组件组合相同）：3 册
再住进一户（多一个组件）：4 册
```

“空”世界就自带两册：`ArchetypeId(0)` 是空组合专用的一册，另一册的住户正是 11-1 节沙盘里占走 0 号门牌的那位——`World::new()` 自带的资源实体（真身 11-7 节揭晓）。此后照常：罗兰和老蔫儿组合相同、合用一册，多带一个 `Shop` 的杂货铺老板另开一册。**查询的匹配单位就是册**：`Query<&Name, With<Resident>>` 先筛出“含 `Name` 和 `Resident` 的册”，再顺着册扫——这就是第 3 章“遍历按子表逐张扫、册间次序是引擎内务”的全部背景，也是“永远不要依赖遍历顺序”的根源。

每册档案的数据默认放在一张 **Table**（表）里，**列式**存储：同一种组件排成连续一列，一行对应一个实体。查询扫表就是顺着列走，内存连续、缓存命中率高——第 1 章承诺的“高性能的主要来源”，机制就这么朴素。

## 搬家

列式存储有个直接推论：**实体的组件组合一变，它就不再属于原来那册**——整行数据得搬进新册的表里。亲眼看。`EntityLocation` 记录每个实体“在哪册、哪张表”：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-07.rs:move}}
```

<span class="caption">Listing 11-7（其二）：insert 一次，搬一次家</span>

```text
盖章（Table 组件）：archetype ArchetypeId(2) → ArchetypeId(4)，table TableId(2) → TableId(4)
贴记号（SparseSet 组件）：archetype ArchetypeId(4) → ArchetypeId(5)，table TableId(4) → TableId(4)
```

盖章（insert 一个普通组件）让罗兰从 `ArchetypeId(2)` 搬进了新开的 `ArchetypeId(4)`，table 跟着换——这就是“搬家”：旧表里他那行被抽走（表尾一行顶上来补位，行号重排），新表添一行。现在上一节的悬案可以结了：**为什么多户同借时只给降档的 `EntityMut`**——任何一户 insert 都可能引发整表行号重排，另一只手里的句柄立刻作废。也顺手解释了 `world.query()` 为什么要 `&mut World`：首次见到新组合要登记册目。

第二行输出更有意思：贴记号换了 archetype（身份毕竟变了），**table 却纹丝不动**。因为 `Flagged` 声明了另一种存储：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-07.rs:sparse}}
```

<span class="caption">Listing 11-7（其三）：声明 SparseSet 存储——不进表的组件</span>

**SparseSet**（稀疏集）存储的组件不进 Table，单独放在旁边一个“实体 → 值”的柜子里。插拔它不搬数据行，代价是查询扫到它时要按实体逐个去柜子里查，没有连续内存可言。取舍一句话：

| | Table（默认） | SparseSet |
|---|---|---|
| 遍历 | 快（列式连续） | 慢（逐实体查柜） |
| insert / remove | 慢（搬家） | 快（不搬家） |
| 适合 | 绝大多数组件 | 频繁穿脱的临时标记 |

拿不准就用默认值；等性能分析真指向“某个标记每帧插拔几千次”，再把那一个组件改成 SparseSet。

## 翻档案：inspect_entity

检查器的核心一招藏在档案室：不知道实体身上有什么，问 World 要它的档案页——

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-07.rs:inspect}}
```

<span class="caption">Listing 11-7（其四）：inspect_entity——逐项报出组件名</span>

```text
罗兰户的档案页：[Name] [Resident] [Registered] [Flagged]
```

`inspect_entity` 给出一串 `ComponentInfo`，`name()` 报类型名（能看到人话全靠 Cargo.toml 里开的 `debug` feature，否则只剩匿名编号），`.shortname()` 去掉模块前缀。`EntityRef::contains` 是“点名问有没有”，`inspect_entity` 是“有什么全报上来”——终章的存档就靠它。

档案室到此参观完毕。回到地面：独占系统好用，但写起来满手 `world.query_filtered::<...>()`，柜台时代的舒服姿势全没了。下一节把姿势找回来。

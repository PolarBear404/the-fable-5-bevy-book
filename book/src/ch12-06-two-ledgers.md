# 两本账：GlobalTransform 与传播

上一节末尾的问题——渲染器画月亮用的是哪本账——答案是每个带 `Transform` 的实体身上都有的另一个组件：**`GlobalTransform`**（全局变换——实体在世界坐标里的最终位置）。你从没亲手插过它，是 required components 在代劳：`Transform` 的定义上挂着 `#[require(GlobalTransform)]`。

于是每个实体都记着两本账：

| | `Transform` | `GlobalTransform` |
|---|---|---|
| 记的是 | 相对父亲的**局部**坐标（没爹则等于世界坐标） | 折算到底的**世界**坐标 |
| 谁来写 | **你**——所有摆放、运动代码都改它 | **引擎**——每帧沿父子树自动折算 |
| 谁来读 | 你的逻辑代码 | 渲染器；以及你需要“世界里到底在哪”时 |

折算的算法第 9 章已经口头预告过：从树根往下，把每一级的局部变换**逐级相乘**。月亮的世界位置 = 地球盘的变换 × 地球的变换 × 月亮盘的变换 × 月亮自己的 `(55, 0, 0)`。这套折算叫 **Transform 传播**（propagation）。

`GlobalTransform` 没有公开字段，也不该手改——你写进去的值会在下一轮传播里被冲掉。它内部存的不是“平移、旋转、缩放”三件套，而是一个仿射矩阵（`Affine3A`），读取要走方法：`translation()`、`rotation()`、`scale()`，或者一次性拆成三件套的 `compute_transform()`。

## 传播的时刻表

引擎“每帧自动折算”——具体是每帧的**什么时候**？第 6 章的 Main 调度表里埋过一行：`PostUpdate` 阶段，“传播 `Transform`（第 12 章）”。现在兑现。传播系统全部挂在 `PostUpdate` 的 **`TransformSystems::Propagate`** 集合里（`SystemSet` 的实战样本：引擎公开集合名，供你的系统 `before`/`after`）。另有一遍跑在 `PostStartup`，专为 `Startup` 里刚生成的实体把第一帧的账目铺正。

时刻表决定了一个所有 Bevy 新手都会撞上的现象。做个实验，无窗口、两帧、看得清清楚楚：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-10.rs:main}}
```

<span class="caption">Listing 12-10（其一）：无窗口实验台——MinimalPlugins 不带传播，手动补 TransformPlugin（examples/listing-12-10.rs）</span>

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-10.rs:push}}
```

<span class="caption">Listing 12-10（其二）：装配工在 Update 里改 Transform</span>

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-10.rs:observe}}
```

<span class="caption">Listing 12-10（其三）：紧跟着的观测站同帧读两本账</span>

```console
cargo run -p ch12-transforms --example listing-12-10
```

```text
—— 第 1 帧 ——
  装配工：探针推到 x = 100，籍册改讫。
  观测站：籍册 x = 100，实测 x = 0
—— 第 2 帧 ——
  观测站：籍册 x = 100，实测 x = 100
```

第 1 帧：`Transform` 已经是 100，`GlobalTransform` 还是 0。两个系统都在 `Update`，而传播在 `PostUpdate`——观测站读账的时候，新籍册还没送去折算。要到本帧晚些时候传播跑完，实测才追上；第 2 帧再读，两本账一致。

把规则记成两条：

- **画面不受影响**。`Update` 改的 `Transform`，同一帧的 `PostUpdate` 就折算完，渲染照新账画——你的运动代码不欠画面什么；
- **同帧读 `GlobalTransform` 的系统，读到的是上一帧的世界坐标**。瞄准、吸附、距离判定这类“读别人世界位置”的逻辑，拿到的数据天然旧一帧。

旧一帧要紧吗？多数场合不要紧——16 毫秒前的位置，肉眼分不出。真遇到分得出的（贴脸跟随抖动、高速物体瞄不准），三条对策按代价排序：直接读 `Transform`（仅当目标没有父级，局部即世界）；把读账的系统排到传播之后（`.after(TransformSystems::Propagate)` 挂进 `PostUpdate`）；或者用系统参数 **`TransformHelper`** 当场沿树现算一份最新值（每次都是一笔遍历开销，别放进万人循环）。

## 断链事故：B0004

两本账都靠树维系，树就有断链的事故等着新手。上一节转盘上那句“`Transform` 和 `Visibility` 一个都不能少”，现在拆掉看看会怎样——两块偷工减料的转盘：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-11.rs:setup}}
```

<span class="caption">Listing 12-11：转盘 A 缺 Transform，转盘 B 缺 Visibility（examples/listing-12-11.rs）</span>

```console
cargo run -p ch12-transforms --example listing-12-11
```

窗口照常打开，控制台先递上两条警告（`Name` 组件让警告直接点名，调试父子关系时强烈建议给关键实体起名）：

```text
WARN bevy_ecs::hierarchy: warning[B0004]: The 行星甲 entity with the GlobalTransform component has a parent (the 转盘A entity) without GlobalTransform.
This will cause inconsistent behaviors! See: https://bevy.org/learn/errors/b0004
WARN bevy_ecs::hierarchy: warning[B0004]: The 行星乙 entity with the InheritedVisibility component has a parent (the 转盘B entity) without InheritedVisibility.
This will cause inconsistent behaviors! See: https://bevy.org/learn/errors/b0004
```

**B0004** 的意思：孩子身上有某个“沿树继承”的组件，父亲却没有——继承链在父亲那儿断了。画面上的症状一轻一重，值得分开验尸：

- **行星甲（父亲缺 `Transform`）：瘫在窗口正中央**，写好的 `(150, 0)` 全然无效。传播是从“有 `Transform` 的树根”往下走的，转盘 A 不在任何一棵传播树上，行星甲的 `GlobalTransform` 永远停在出厂默认值——原点。位置、旋转、缩放，整套报废；
- **行星乙（父亲缺 `Visibility`）：照常显示，位置也对**——眼下看不出任何毛病。毛病是潜伏的：可见性继承链断了，将来你想“隐藏整条轨道”（把父亲设成 `Visibility::Hidden`，第 15 章的内容）时，行星乙不会跟着隐身。正应了警告里那句 inconsistent behaviors——不是立刻坏，是说不准什么时候坏。

修法不值一行字：缺啥补啥。空实体要当爹，`Transform` 加 `Visibility` 二件套带齐，就是上一节转盘的标准配置。

两本账、一张时刻表、一种断链事故——`Transform` 的机制部分到此完整。下一节换换脑子，看看 `bevy_math` 里那些纯几何的小工具。

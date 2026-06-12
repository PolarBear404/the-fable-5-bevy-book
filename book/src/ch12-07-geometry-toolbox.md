# 观测站的几何课

机械部分完工，轮到观测员小满提需求了。她的工作清单上有两桩差事：取景框就那么大，得随时知道**哪颗天体在镜头里**；外圈计划撒一圈小行星带，跑出带子的石头要**拉回原位**。两桩都不是 `Transform` 的活——是纯几何题。`bevy_math` 为此备了一抽屉零件。

## Rect：一块平面范围

**`Rect`**（轴对齐矩形——两条边永远水平、两条边永远竖直）是“范围”的标准表达：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-12.rs:rect}}
```

<span class="caption">Listing 12-12（其一）：取景框——from_center_size 与 contains（examples/listing-12-12.rs）</span>

构造可以从中心加尺寸（`from_center_size`），也可以从两个对角（`from_corners`）；查询用 `contains(point)`。同族方法都顾名思义：`union`（两框的并）、`intersect`（交集）、`inflate`（四边外扩）——做镜头范围、攻击判定框、UI 命中区，这一个类型通吃。

## 几何原语：会答题的形状

圆、环、多边形这类标准形状，`bevy_math::primitives` 模块里各有一个类型，统称**几何原语**（primitives）。它们是纯数学对象——只描述形状、回答几何问题，跟渲染没有半点瓜葛：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-12.rs:circle}}
```

<span class="caption">Listing 12-12（其二）：Circle 报直径、周长、面积</span>

周长和面积来自 `Measured2d` 这个 trait（2D 度量——所有平面原语都实现它，3D 原语对应 `Measured3d`），用时记得 `use`。

小行星带是个**环**——内外两个半径之间的地带，原语里叫 **`Annulus`**。它身上正好有小满要的“拉回原位”：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-12.rs:annulus}}
```

<span class="caption">Listing 12-12（其三）：Annulus 与 closest_point——越界的石头各自归位</span>

```console
cargo run -p ch12-transforms --example listing-12-12
```

```text
取景框中心 [0, 0]，半尺寸 [280, 160]
地球 [240, 0] 在镜头里吗？true
彗星 [-120, 320] 在镜头里吗？false
地球轨道：直径 480，周长约 1508，圈住面积约 180956
小行星带厚度 80
外逃的小行星 [450, 0] 押回 [380, 0]
内坠的小行星 [120, 160] 拉回 [180, 240]
```

`closest_point(p)` 给出形状上离 `p` 最近的点：点在形状里，原样奉还；飘出外缘，押回外圈；坠进内洞，提到内圈。“把越界的东西夹回合法范围”这件事，自己写要分三种情况讨论，原语一个方法收口。

用原语有一条纪律：**它们都蹲在自己的原点**。`Circle::new(240.0)` 是“以原点为圆心、半径 240”，`Annulus` 同理——形状本身不记位置。拿世界坐标的点去提问前，先把点换算进形状的局部坐标（通常就是减掉形状所在的中心位置）；本节的演算里太阳恰好在原点，这一步才得以省略。

抽屉里远不止这三件：`Rectangle`、`Triangle2d`、`Capsule2d`、`RegularPolygon`（正多边形）、`Segment2d`（线段，能报方向和法线）……3D 侧还有 `Sphere`、`Cuboid`、`Cylinder` 一整套。本章先记住它们“纯数学、会答题”的身份就够——到第 15 章和第 21 章，正是这些类型被喂给 Mesh 系统，变成屏幕上看得见的圆和方。一个类型、两副面孔，往后你会常跟它们打照面。

零件备齐，开馆。

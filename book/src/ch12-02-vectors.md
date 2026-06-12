# Vec：既是位置，也是箭头

动工之前，老盖得在图纸上把数算清：彗星离太阳多远？朝哪个方向飞？这些演算用的数学类型就是 `Transform::translation` 的类型本身——**`Vec3`**（三维向量，x、y、z 三个 `f32`），以及它的平面兄弟 **`Vec2`**。它们是普通的 `Copy` 值，不是组件、不需要 `App`，在 `main` 里就能把脾气摸清：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-03.rs:positions}}
```

<span class="caption">Listing 12-3（其一）：三个天体的位置，各用一个 Vec2（examples/listing-12-03.rs）</span>

同一个 `Vec2` 有两种解读。写成 `comet` 时它是**位置**——图纸上的一个点；而两个位置相减，得到的是**箭头**——从一点指向另一点的位移，有方向、有长度：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-03.rs:length_distance}}
```

<span class="caption">Listing 12-3（其二）：位置相减得箭头，箭头的长度是距离</span>

`length()` 给出箭头的长度，`distance()` 是“相减再取长度”的快捷写法。游戏里的“索敌半径”“拾取范围”，本质都是这一行。

> 性能提示：比较距离用 `length_squared()`（长度的平方）更省——开平方是这几个操作里唯一贵的一步，而“谁更近”用平方比较结果一样。

## 归一化：把箭头压成纯方向

箭头携带两份信息：方向和长度。很多场合只要方向——“朝那边走，速度我自己定”。`normalize()` 把箭头压成长度为 1 的**单位向量**，只留方向：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-03.rs:normalize}}
```

<span class="caption">Listing 12-3（其三）：normalize 只留方向</span>

拿到单位方向后乘上标量就是“沿这个方向走多远”：`bearing * 120.0` 是沿方向 120 单位的位移。第 2 章方块的运动公式、日后所有“朝目标移动”的代码，骨架都是 `位置 += 方向 * 速度 * dt`。

小心一个坑：**零向量没有方向**，`Vec2::ZERO.normalize()` 给出的是 NaN（非数），带着它继续算会污染一切。拿不准输入是否为零时用 `normalize_or_zero()`，或者用下一节的 `Dir2` 类型把检查交给类型系统。

## 点积：方向之间的问答

两个单位向量的**点积**（`dot`）是一个标量，直接回答“它们有多同向”：同向得 1，垂直得 0，反向得 −1。观测站靠它判断彗星的来意：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-03.rs:dot}}
```

<span class="caption">Listing 12-3（其四）：点积判断“冲着来还是横着过”</span>

“敌人在我身前还是身后”“子弹算不算迎面命中”，都是一次点积的事。

## lerp 与 2D/3D 互转

最后两件随手用的小工具——按比例取中间点的 `lerp`（线性插值），和 `Vec2` 与 `Vec3` 的互转：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-03.rs:lerp_extend}}
```

<span class="caption">Listing 12-3（其五）：lerp 取中点；extend 补 z、truncate 截 z</span>

`extend(z)` 给平面坐标补上第三维，`truncate()` 把它截回去。这对组合在 2D 游戏里出场率极高：游戏逻辑在平面上用 `Vec2` 思考，而 `translation` 是 `Vec3`——上一节刚讲过，那个 z 是图层号，不该被平面运算搅动。“`xy` 归逻辑，z 归图层”的纪律，靠它们维持。

```console
cargo run -p ch12-transforms --example listing-12-03
```

```text
老盖量彗星：离太阳 200，离地球 340
彗星的方位（单位向量）：[-0.6, 0.8]，长度 1.00
第一晚：速度·朝日方向 = 1（全速冲着太阳来！）
第二晚：速度·朝日方向 = 0（虚惊一场，正横着掠过）
日地连线的中点：[90, 0]
彗星入册：[-120, 160, 5]，截回平面：[-120, 160]
```

图纸演算完毕。`Vec` 家族还有不少成员——整数版的 `IVec2`/`UVec2`、四维的 `Vec4`、双精度的 `DVec3`——API 形状全一样，用到再查。下一节正式开机：用 `Transform` 把太阳装上转轴。

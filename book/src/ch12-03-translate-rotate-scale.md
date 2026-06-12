# Transform：摆放、旋转与缩放

正式开机。翻开 `Transform` 的定义，它就是三个公开字段：

| 字段 | 类型 | 回答的问题 | 默认值 |
|---|---|---|---|
| `translation` | `Vec3` | 在哪儿 | `Vec3::ZERO`（原点） |
| `rotation` | `Quat` | 朝向哪儿 | `Quat::IDENTITY`（不旋转） |
| `scale` | `Vec3` | 多大个儿 | `Vec3::ONE`（原始尺寸） |

三个默认值合在一起叫 `Transform::IDENTITY`——“原样放在原点”。第一桩订单是太阳：摆在正中央、带一点出厂倾角、整体放大两成，然后让它转起来。

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-04.rs:setup}}
```

<span class="caption">Listing 12-4（其一）：构造器链——位置、旋转、缩放逐项补齐（examples/listing-12-04.rs）</span>

这串链式写法是 Bevy 代码里的高频句式：`from_xyz`/`from_translation`/`from_rotation`/`from_scale` 任选一个起头，`with_rotation`/`with_scale`/`with_translation` 接着补。字段是公开的，不喜欢链式也可以直接用结构体字面量加 `..default()`。

`Quat::from_rotation_z(0.3)` 是本章第一次正面接触 **`Quat`**（四元数——表示 3D 旋转的数学对象）。它的内部构造留给下一节，眼下只需要一条用法：**2D 的旋转全是“绕 z 轴转”**——想象一根垂直于屏幕的轴从方块中心穿过，`from_rotation_z(角度)` 就是绕这根轴拧。角度的单位是**弧度**：`PI` 是半圈，`TAU`（2π）是整圈，`0.3` 约 17°。方向遵循右手定则：**正角逆时针**。

## 两种改法：累加与盖写

太阳要自转，还要像呼吸一样脉动。两个系统用了两种不同的修改手法：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-04.rs:spin}}
```

<span class="caption">Listing 12-4（其二）：rotate_z——在现有姿态上累加</span>

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-04.rs:pulse}}
```

<span class="caption">Listing 12-4（其三）：scale 赋值——按公式直接盖写</span>

```console
cargo run -p ch12-transforms --example listing-12-04
```

金色方块缓缓旋转，同时大小随正弦起伏。对照两个系统：

- `rotate_z(增量)` 是**累加式**——“在现在的基础上再转这么多”。每帧的增量乘了 `time.delta_secs()`（上一帧到这一帧的秒数），所以无论机器跑 60 帧还是 144 帧，太阳每秒都转 0.6 弧度——这是“速度 × 时间”的老朋友，时间系统的全貌在第 18 章；
- `scale = 公式(t)` 是**盖写式**——每帧根据“启动以来的总秒数”算出该有的值，直接覆盖。注意它顺手覆盖掉了出生时 `with_scale(1.2)` 的设定：盖写不在乎先前是什么。

两种手法没有优劣，只有适配：持续累积的运动（自转、巡航）用累加，由时间函数完全决定的状态（脉动、摆动、补间动画）用盖写。混用时小心 Listing 12-4 这种“出生值被悄悄覆盖”的情况。

> `scale` 是 `Vec3`，三个轴可以不等比：`Vec3::new(2.0, 1.0, 1.0)` 把方块拉成横放的砖。负数缩放在数学上是镜像翻转，2D 里偶尔用来“让角色面朝左”，但它会连带翻转旋转方向的视觉效果，初学阶段建议别碰。

## rotate_around：绕着别人转

太阳能自转了，行星呢？公转不是绕自己转，是绕**太阳**转。`Transform` 上恰好有一个为此准备的方法：

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-05.rs:setup}}
```

<span class="caption">Listing 12-5（其一）：水星与地球，各带一个公转角速度（examples/listing-12-05.rs）</span>

```rust
{{#include ../../code/ch12-transforms/examples/listing-12-05.rs:orbit}}
```

<span class="caption">Listing 12-5（其二）：rotate_around——绕指定点转过给定旋转</span>

```console
cargo run -p ch12-transforms --example listing-12-05
```

两颗行星各按各的速度绕太阳画圆，里圈的水星明显比外圈的地球勤快。`rotate_around(point, step)` 每帧做的事：把 `translation` 绕 `point` 转过 `step` 那么多角度。

盯着看几秒还能发现一个副产品：**方块在公转的同时也在缓缓自转**，始终拿同一条棱对着太阳——像被潮汐锁定的月球。这不是巧合：翻开源码，`rotate_around` = `translate_around`（位置绕点转）+ `rotate`（自身姿态同步转）。只想要位置画圆、姿态保持端正？换 `translate_around` 就是。

太阳系仪初具雏形：太阳自转，双星绕日。但订单上还有一行没动——“月亮绕着地球转”。月亮的圆心不是固定的原点，而是一颗正在公转的行星……这个难题先记下，第 5 节再收拾它。下一节先补齐旋转的另一半本领：怎么让一个东西**朝向**该朝的方向。

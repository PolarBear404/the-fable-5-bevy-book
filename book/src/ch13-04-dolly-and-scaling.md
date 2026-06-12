# 推拉与适配：scale 与 ScalingMode

分镜表上写着三种景别：远景交代全场，中景跟住调度，特写怼脸拍情绪。摄影上这叫推拉镜头——可在正交投影的世界里，把相机沿 z 轴“拉远”毫无用处：正交的本性就是**远近一样大**，z 挪到天边，画面纹丝不动。正交世界的变焦环长在投影上。

## 一个常见的坑

变焦要动投影参数，那就得亲手构造一份 `OrthographicProjection`。凭 Rust 直觉，改一个字段、其余走默认，标准写法应该是结构体更新语法配 `..default()`：

```rust
{{#include ../../code/ch13-cameras/no-compile/listing-13-06.rs:no_default}}
```

<span class="caption">Listing 13-6：行不通——`OrthographicProjection` 拒绝 `..default()`（no-compile/listing-13-06.rs）</span>

编译器一口回绝：

```text
error[E0277]: the trait bound `bevy::prelude::OrthographicProjection: std::default::Default` is not satisfied
  --> ch13-cameras\no-compile\listing-13-06.rs:11:15
   |
11 |             ..default() // 错误：OrthographicProjection 没实现 Default
   |               ^^^^^^^^^ the trait `std::default::Default` is not implemented for `bevy::prelude::OrthographicProjection`
```

这不是疏忽，是 Bevy 故意不给 `OrthographicProjection` 实现 `Default`。原因藏在上一节的 `near`/`far` 里：2D 惯用 `near = -1000`（相机身后也算数，z 只是图层号），3D 惯用 `near = 0`（背后的东西不该入镜）——没有一份默认值能同时伺候两边。所以构造函数一分为二，逼你表明身份：

- `OrthographicProjection::default_2d()`——`near = -1000`，给 2D 用；
- `OrthographicProjection::default_3d()`——`near = 0`，给 3D 用。

把 `..default()` 换成 `..OrthographicProjection::default_2d()`，就是正确的打开方式。网上不少老版本教程还在写 `..default()`，遇到这条报错你现在知道病根了。

## 三档分镜

正式的变焦环是 `scale` 字段：取景框整体乘上这个倍数。`scale = 1.5`，取景框放大 1.5 倍，更多世界挤进同一幅画面，物体显小——这是远景；`scale = 0.6`，取景框缩小，物体放大——这是特写。给片场排一张三档分镜表，三秒一切：

```rust
{{#include ../../code/ch13-cameras/examples/listing-13-07.rs:camera}}
```

<span class="caption">Listing 13-7（节选一）：自定义投影的正确姿势（examples/listing-13-07.rs）</span>

```rust
{{#include ../../code/ch13-cameras/examples/listing-13-07.rs:cut}}
```

<span class="caption">Listing 13-7（节选二）：三秒切一镜，改 scale 就是推拉（examples/listing-13-07.rs）</span>

切镜系统的写法都是旧识：`Local` 计时（第 4 章）、`Single<&mut Projection>` 拿独苗（第 4 章）、改 enum 字段前先 `if let` 验流派。投影一被改写，引擎在帧末自动重算取景框——`area` 应声伸缩，上一节的报点系统若还在场，能看到范围跟着分镜变。

```console
cargo run -p ch13-cameras --example listing-13-07
```

```text
老雷：按分镜表走——远景开场，三秒一切。
老雷：切中景！（scale = 1）
老雷：切特写！（scale = 0.6）
```

画面三秒一跳：远景里全场尽收，灯笼柱排成糖葫芦；特写里阿燕占满半屏，红衣的像素粒粒可数。注意这是“硬切”——`scale` 瞬间改值。想要“缓推缓拉”的电影感？上一节刚学的 `smooth_nudge` 对 `f32` 同样适用，练习里见。

## 窗口一拉，画面听谁的

Listing 13-7 的相机还埋了第二处新东西：`scaling_mode` 字段换成了 `ScalingMode::FixedVertical { viewport_height: 600.0 }`。它管的是另一件事：**窗口尺寸变了，取景框怎么应对**。

默认值 `WindowSize` 就是前几节实测过的一比一规则：窗口多少像素，取景框就多少世界单位。这条规则对工具类应用很自然，对游戏却常常是错的——玩家把窗口拖大一倍，看到的世界就多一倍，等于开了“大窗口física外挂”。`ScalingMode` 的六个变体就是六种应对方针：

| `ScalingMode` | 方针 |
|---|---|
| `WindowSize` | 一比一：窗口多大，世界看多大（默认） |
| `Fixed { width, height }` | 死守一块固定大小的世界，窗口变形就跟着拉伸变形 |
| `FixedVertical { viewport_height }` | 纵向视野锁死，横向随窗口比例配平 |
| `FixedHorizontal { viewport_width }` | 横向视野锁死，纵向随窗口比例配平 |
| `AutoMin { min_width, min_height }` | 保比例缩放，保证至少看见这么大一块 |
| `AutoMax { max_width, max_height }` | 保比例缩放，保证最多看见这么大一块 |

`FixedVertical { viewport_height: 600.0 }` 翻译成人话：不管窗口怎么拉，画面里**上下方向永远是 600 个世界单位**，左右按窗口宽高比补齐。跑起 Listing 13-7 亲手拖一拖窗口边框，对比前几个示例（默认 `WindowSize`）的手感：

- `WindowSize`：窗口拉高，看见更多世界，灯笼柱大小不变；
- `FixedVertical`：窗口拉高，世界跟着放大，纵向能看见的范围分毫不让。

游戏里最常用的正是 `FixedVertical`/`FixedHorizontal` 与两个 `Auto` 流派——视野是设计师定的，不是玩家的显示器定的。`scale` 与 `scaling_mode` 可以叠用：先按模式算出基础取景框，再乘 `scale` 推拉，分工互不打架。

镜头会跟、会推拉、会应对窗口了——单机位的本事齐了。可老雷的片子是双主角：阿燕在东，踏雪在西，一台机器盯不过来。下一节，导播台上墙。

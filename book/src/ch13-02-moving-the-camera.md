# 机位即实体：移动与跟拍

片场搭好了，老雷喊出第一个调度：阿燕满场走一个大“8”字，1 号机全程跟住他。

走位是第 12 章的手艺——每帧按时间公式盖写 `translation`。跟拍是新课题，但想一想相机的身份，答案自己浮出来：相机是实体，实体有 `Transform`，那么**移动镜头就是改相机的 `translation`**，跟移动阿燕没有任何区别。

第一版跟拍最直白：阿燕在哪，镜头中心就在哪。

```rust
{{#include ../../code/ch13-cameras/examples/listing-13-03.rs:walk}}

{{#include ../../code/ch13-cameras/examples/listing-13-03.rs:follow}}
```

<span class="caption">Listing 13-3：跟拍第一版——逐帧把相机坐标钉在阿燕身上（examples/listing-13-03.rs）</span>

两个系统在 `Update` 里串成 `(walk_hero, follow_hero).chain()`——第 6 章讲过的执行顺序在这里是刚需：镜头必须读到**本帧**走完位的新坐标，否则永远慢一帧。`follow_hero` 的查询过滤器也值得一看：`With<Camera2d>` 配 `Without<Hero>`，跟官方示例一个写法——两个参数一个可变借相机的 `Transform`、一个只读阿燕的 `Transform`，`Without` 向调度器证明两边不会摸到同一个实体（第 4 章的借用功课）。

```console
cargo run -p ch13-cameras --example listing-13-03
```

```text
老雷：阿燕走起来，1 号机跟住他！
```

跑起来你会看见一个有趣的景象：阿燕一动不动地钉在窗口正中央，反倒是灯笼柱、地毯、整个片场在“倒着”滑动。这不是错觉，是跟拍的本质——**镜头与主角零相对运动，于是世界反向流动**。盯住任何一根灯笼柱看几秒，它划过画面的轨迹正是阿燕走位的镜像。

还有个小细节：`follow_hero` 只抄 x 和 y，z 留在原地。2D 相机的 z 不参与构图（下一节讲投影时你会看到为什么），但乱改它可能把实体甩出可见范围——第 12 章说过默认相机只见 z 轴 ±1000，那个范围是**以相机自己的 z 为基准**量的。让相机的 z 待在出厂位置，是省心的习惯。

## 镜头要像人扛的

老雷看了两秒回放就摇头：太硬了。真人摄影师跟拍是有惯性的——主角突然变向，镜头总要慢半拍才追上来，画面才有“呼吸”。逐帧硬抄坐标，等于把摄影机焊在演员头顶。

数学上，“慢半拍地追”就是插值：镜头每帧不是跳到目标点，而是向目标点**靠近一段**。第 12 章用过 `lerp` 取两点中间；但跟拍是每帧连续进行的，直接 `lerp(target, 0.1)` 的追赶速度会随帧率波动——帧率翻倍，每秒“靠近 10%”就执行了双倍次数。Bevy 为这个场景备了专门的工具：`smooth_nudge`，帧率无关的平滑逼近，来自 `StableInterpolate` 这个 trait（与 `Vec3` 一样从 `bevy_math` 进了 prelude）。

```rust
{{#include ../../code/ch13-cameras/examples/listing-13-04.rs:follow}}
```

<span class="caption">Listing 13-4：跟拍第二版——smooth_nudge 给镜头装上“人手”（examples/listing-13-04.rs）</span>

`smooth_nudge(&target, decay_rate, delta)` 三个参数：目标点、衰减速率、本帧时长。衰减速率越大追得越紧——`2.0` 是松弛的跟拍手感，`8.0` 就接近硬跟随了。把 `delta_secs()` 喂进去之后，无论 30 帧还是 144 帧，镜头的追赶节奏完全一致。`target` 用 `hero.translation.with_z(lens.translation.z)` 拼出来：x、y 追阿燕，z 永远是相机自己的——上一段的习惯，用一个方法就守住了。

```console
cargo run -p ch13-cameras --example listing-13-04
```

```text
老雷：镜头柔一点——像人扛的，不是钉死的。
```

这一版里阿燕不再钉死在中心：他疾走时会“甩开”镜头小半个身位，停步转向时镜头又柔柔地荡回来对准他。两版示例只差 `follow_hero` 的三行实现，运镜的气质天差地别——值得来回切换跑几次，体会 `decay_rate` 改成 `0.5` 和 `10.0` 的手感。

机位会跟人了，但小满那笔旧账还没销：镜头此刻**到底拍到了哪一块世界**？答案在相机的另一位随从身上——下一节拆 `Projection`。

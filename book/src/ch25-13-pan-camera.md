# 2D 的现成脚架：PanCamera

2D 那边配的是另一台：**PanCamera**——平移、缩放、旋转，地图编辑器和策略视角的标配三件套。开门的 feature 是 `pan_camera`（上一节的 `Cargo.toml` 里已一并开好），组件挂上 `Camera2d` 就归它管：

```rust
{{#include ../../code/ch25-picking/examples/listing-25-14.rs:camera}}
```

<span class="caption">Listing 25-14（其一）：PanCamera 挂上 Camera2d（examples/listing-25-14.rs）</span>

它不像 FreeCamera 那样配置/状态分家——一件组件全包，键位字段全是 `Option<KeyCode>`（哪路不要就设 `None`，单独摘掉）。出厂账单：WASD 平移、Q/E 逆顺时针旋转、`+`/`-` 或滚轮缩放、**鼠标左键拖动平移**。这里只把 `pan_speed` 从 500 提到 700（单位是世界单位/秒——2D 里通常就是像素/秒，夜市长街三百多像素一间铺面，700 巡起来不拖沓）。

场子是五盏灯笼一字排开的长街，阿燕站正中。空格报账（位置读 `Transform`、缩放读 `PanCamera` 自己的 `zoom_factor`），四路输入各试一遍：

```console
cargo run -p ch25-picking --example listing-25-14
```

```text
老雷：夜市长街——WASD 移镜头，QE 转，+/- 或滚轮推拉，左键一拖就走。
小棠：空格报机位。
场记：机位 (0, 0)，转角 0 度，变焦 1.00。
场记：机位 (701, 0)，转角 0 度，变焦 1.00。
场记：机位 (701, 0)，转角 0 度，变焦 0.70。
场记：机位 (1014, 0)，转角 0 度，变焦 0.70。
场记：机位 (1014, 0)，转角 91 度，变焦 0.70。
```

```rust
{{#include ../../code/ch25-picking/examples/listing-25-14.rs:report}}
```

<span class="caption">Listing 25-14（其二）：空格报机位、转角、变焦</span>

逐行对账：

- **D 按住一秒**：x 走了 701 ≈ `pan_speed` × 1 秒，平移就是老实的线性速度；
- **滚轮向上三格**：`zoom_factor` 从 1.00 到 0.70——每格改 `zoom_speed`（出厂 0.1），**线性**加减（对比 FreeCamera 滚轮的指数调速——2D 缩放范围窄，出厂给了 `min_zoom: 0.1`/`max_zoom: 5.0` 的夹子，线性够用）。数值越**小**画面越**大**：`zoom_factor` 改的是相机的 `Transform.scale`，第 13 章正交相机的账——缩相机等于放大世界；
- **左键往左拖**：镜头 x 反而**变大**（701 → 1014）——拖的是「街景」，街往左走、镜头往右挪，地图应用的标准手感。300 出头的位移还对着变焦打了七折（0.35 屏宽 × 1280 像素 × 0.7 ≈ 313）：拖动换算连着缩放一起算，画面上正好是「指哪儿街跟到哪儿」；
- **Q 按住半秒**：转角 91 度 ≈ `rotation_speed`（出厂 π 弧度/秒）× 0.5 秒。

![夜市长街的两联截图：左幅开场机位，五盏灯笼横排、阿燕居中；右幅镜头右移拉近后，街景左移放大，只剩三盏灯笼在画面里](images/ch25/fig-25-13-pan-street.png)

<span class="caption">Figure 25-13：PanCamera 巡街——平移线性、缩放带夹子、拖动反向跟手</span>

最后揭一层有意思的底：**左键拖动平移，走的就是本章的拾取事件**。翻 `pan_camera.rs` 的源码——窗口实体上的 `DragStart` 一响，插件当场 `spawn` 一个盯着 `Pointer<Drag>` 的 `Observer` 挂到窗口上（第 8 章手动挂观察者的那套），`DragEnd` 一响再把它拆掉——观察者随拖拽会话生灭，平移增量从 `drag.delta` 里来，和你在 25.7 节用的完全是同一套 API。官方自家的控制器把「指针事件当相机输入」用成了正式手法——下一节我们照方抓药，给收场戏配一台完全由指针事件驱动的转台相机。

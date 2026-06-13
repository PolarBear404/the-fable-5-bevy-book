# Bloom：让亮的东西溢出来

`Bloom` 是最常见的后处理之一：它从画面里提取亮的部分，模糊后再叠回主画面。它不会让灯真的照亮世界，也不会替代 PBR 光照；它只是让“亮到刺眼”的地方在屏幕上有溢出的感觉。

Bevy 0.18.1 的 `Bloom` 组件在 `bevy::post_process::bloom` 下，预置了 `Bloom::NATURAL`、`Bloom::ANAMORPHIC`、`Bloom::OLD_SCHOOL`、`Bloom::SCREEN_BLUR`。本章的 `BloomPreset` 只是给这些常量做一个可循环的菜单：

```rust
{{#include ../../code/ch26-post-processing-aa/src/main.rs:apply_settings}}
```

<span class="caption">Listing 26-5（节选一）：根据 `QualitySettings` 同步 HDR、Tonemapping、Bloom、景深和运动模糊</span>

Listing 26-5 里有一个教学用的规矩：如果 `settings.bloom` 为真，就插入当前 preset 对应的 `Bloom`；如果关掉，就移除 `Bloom`。实际项目里你也可以不移除组件，而是修改字段、降低强度或用自己的菜单状态驱动。这里选择插入/移除，是为了让“相机上有什么组件，就启用什么后处理”这件事更直观。

按 `3` 开关 Bloom，按 `B` 切换 preset。观察发光球和发光条：

- `NATURAL` 比较克制，适合默认画质；
- `ANAMORPHIC` 有更明显的横向拖光，适合科幻 UI、强镜头光；
- `OLD_SCHOOL` 更接近早期游戏里夸张的泛光；
- `SCREEN_BLUR` 像把整张亮画面揉开，适合特殊镜头，不适合长期默认开启。

Bloom 最容易被误用。自发光材质的 `emissive` 太高、Bloom 太强、Tonemapping 又太硬时，画面会失去层次。一个实用顺序是：先定 HDR 和 Tonemapping，再把自发光强度调到合理，最后只给 Bloom 留一点“空气感”。

本章没有网页 demo 的主要原因也在这里：Bevy 0.18.1 的 Bloom 源码明确写着它当前不兼容 WebGL2。以后如果本书统一迁到 WebGPU 路线，再为这类章节补交互网页 demo 会更稳。

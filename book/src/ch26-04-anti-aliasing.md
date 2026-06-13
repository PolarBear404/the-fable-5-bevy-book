# 抗锯齿：MSAA、FXAA、SMAA 与 TAA

抗锯齿解决的是边缘和细线条在像素网格上闪、抖、锯齿的问题。Bevy 0.18.1 里常见选择有四类：

- `Msaa`：多重采样，发生在几何渲染阶段，适合几何边缘；代价随 sample 数增加；
- `Fxaa`：屏幕后处理，便宜、快速，但可能让画面略糊；
- `Smaa`：屏幕后处理，通常比 FXAA 更保边缘，成本也更高；
- `TemporalAntiAliasing`：利用历史帧和抖动采样，能处理闪烁和细节 aliasing，但依赖运动向量、历史缓冲，并且要求 `Msaa::Off`。

本章示例一次只启用一种抗锯齿。按空格在 `Off -> MSAA 4x -> FXAA -> SMAA -> TAA` 之间循环：

```rust
{{#include ../../code/ch26-post-processing-aa/src/main.rs:aa_switch}}
```

<span class="caption">Listing 26-5（节选二）：FXAA、SMAA、TAA 都会把 `Msaa` 关掉；MSAA 则不再挂屏幕后处理 AA 组件</span>

这段代码有两个重要判断。

第一，`Msaa` 和 TAA 不能一起开。Bevy 0.18.1 的 `TemporalAntiAliasing` 文档和运行时检查都要求相机使用 `Msaa::Off`。本章的切换逻辑在进入 TAA 时显式设成 `Msaa::Off`。

第二，FXAA、SMAA 和 TAA 都是相机组件，不是全局开关。不同相机可以用不同抗锯齿策略。比如主 3D 相机用 TAA，小地图相机或像素风 2D 相机可能完全关 AA。

`ContrastAdaptiveSharpening` 不是抗锯齿本体，而是一个常见的收尾锐化 pass。TAA、FXAA、SMAA 都可能让画面略软，适量 CAS 可以把细节拉回来。本章把它放在 `6` 键：

```rust
{{#include ../../code/ch26-post-processing-aa/src/main.rs:finishing_passes}}
```

<span class="caption">Listing 26-5（节选三）：CAS 锐化和色差都是相机后处理收尾组件</span>

默认建议可以很简单：

- 3D PBR 项目先试 `TemporalAntiAliasing + 轻微 CAS`；
- 低端设备或画面非常快的项目试 `Fxaa` 或 `Smaa`；
- 几何边缘很明显、透明和后处理较少的场景可以试 `Msaa::Sample4`；
- 像素风、UI 截图、调试视图要认真考虑关掉 AA。

抗锯齿没有单一答案。你要看的不是静态截图里哪条边最干净，而是运动中细线、远处高频纹理、相机转动和 UI 叠加是否稳定。

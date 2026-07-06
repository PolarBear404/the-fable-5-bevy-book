# 画质：后处理与抗锯齿

夜戏排熟了，老雷打算印一批宣传画贴满府城。头一张样片是老鲁用窗口截图凑的，贴出去当晚就被看客指点：灯笼是亮，可一点不晃眼，像三颗糖纸剪的月亮；走马灯转着转着，每一帧都钉得死死的，反倒不像在转；栏杆的边缘一格一格，凑近看全是楼梯。老雷托牙行请来盛记照相馆的**盛师傅**——此人进园子不看戏，先看光，第二句话就把病根说破了：“景是好景，**拍**得不讲究。”

前几章我们一直在往场景里*添东西*：网格、材质、灯、影子。这一章一样东西都不添——只调**怎么把已经算好的画面送到屏幕上**。这一段路统称**后处理（post-processing）**：主渲染 pass 把场景画进一张中间纹理之后、最终呈现之前，一串对整张图动手的加工工序。Bevy 把这些工序做成了你熟悉的样子——**相机实体上的组件**，挂上生效、拆下失效、改字段就是拧旋钮。

跟着盛师傅的活儿排期，本章逐间暗房走：

- **底片与冲印**——`Hdr` 换上高动态范围的中间画布，`Tonemapping` 的九种冲印配方各是什么脾气；
- **辉光**——`Bloom` 让灯笼真的晃眼：强度是全场的散射量，想让哪盏更亮该去调它的 emissive；四个预设一档一种美学；顺带亲眼看一次拔掉底片后辉光**一声不吭**的死法；
- **景深**——`DepthOfField` 的对焦台：焦距、光圈、Bokeh 光斑，还有一个“广角镜下景深隐形”的实测教训；
- **运动模糊**——`MotionBlur` 的快门角，走马灯的拖影从分身鬼影到连贯一片；
- **镜头三件套**——暗角 `Vignette`、畸变 `LensDistortion`、色差 `ChromaticAberration`：老镜头的三种“毛病”，全是可调的味精；
- **自动测光**——`AutoExposure` 盯着直方图自己拨第 22 章那只曝光表，亮暗过渡的爬坡速度也归你管；
- **锯齿与磨边**——先搞清楚阶梯从哪来，再把四家磨边师傅逐个请上台：`Msaa`、`Fxaa`、`Smaa`、TAA，外加锐化找补的 `ContrastAdaptiveSharpening`；TAA 那条“必须关 MSAA”的硬规矩要亲眼看它刷屏；
- **一瞥未来**——实验性光追 `bevy_solari` 的定位与门槛，只看地图不上车；
- **《定妆照》**——全章合龙：一块画质开关面板，逐项拨给老雷看。

这些组件分住四个 crate：`Hdr` 与 `Exposure` 在 `bevy_camera`，`Tonemapping`（连同 `DebandDither`）在 `bevy_core_pipeline`，辉光、景深、运动模糊、三件套、自动曝光在 `bevy_post_process`，四家磨边在 `bevy_anti_alias`。听着分散，用起来不用记门牌：`Msaa` 在 `bevy::prelude` 里，其余从 `bevy::camera`、`bevy::core_pipeline`、`bevy::post_process`、`bevy::anti_alias` 显式引入，正文用到时逐个点名。这几个 crate 全在默认 feature 集合里——连 tonemapping 和 SMAA 的两套查色表（`tonemapping_luts`、`smaa_luts`）都是——本章一扇 feature 的门都不用开：

```toml
{{#include ../../code/ch26-quality/Cargo.toml}}
```

唯一的资产是总成《定妆照》屏上状态牌用的中文字体，复用第 16 章的子集，一键就位：

```console
py -3.11 scripts/make_ch26_assets.py
```

开工。

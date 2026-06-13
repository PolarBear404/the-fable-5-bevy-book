# 进度总览

> 每完成一个步骤就更新此文件。状态：⬜ 未开始 / 🟡 进行中 / ✅ 完成

## 项目阶段

| 阶段 | 状态 | 说明 |
|---|---|---|
| 脚手架 | ✅ 2026-06-12 | 目录结构、mdBook、Cargo workspace、规范、斜杠命令 |
| vendor/bevy 源码就位 | ✅ 2026-06-12 | v0.18.1 @ f667c28，55 crate / 375 示例 |
| Bevy 0.18.1 编译验证 | ✅ 2026-06-12 | `cargo check -p smoke` 通过 |
| 全书大纲 OUTLINE.md | ✅ 2026-06-12 | 38 章 + 6 附录，用户已审定 |
| 章节写作 | 🟡 | 进度见下表 |

## 章节状态

| 章 | 标题 | 状态 | 产物（crate / 正文文件，动工时填写） |
|---|---|---|---|
| 1 | 认识 Bevy | ✅ | book/src/ch01-00～04（本章无代码 crate；引用 code/Cargo.toml 的 dev_profile 片段） |
| 2 | 第一个 Bevy App | ✅ | code/ch02-first-app（src/main.rs + examples/listing-02-01～06）；book/src/ch02-00～03 |
| 3 | Entity 与 Component | ✅ | code/ch03-entities-components（src/main.rs + examples/listing-03-01～06）；book/src/ch03-00～03 |
| 4 | System 与 Query | ✅ | code/ch04-systems-queries（src/main.rs + examples/listing-04-01～07，开 bevy `debug` feature）；book/src/ch04-00～04 |
| 5 | Resource——全局唯一数据 | ✅ | code/ch05-resources（src/main.rs + examples/listing-05-01～07，开 bevy `debug` feature）；book/src/ch05-00～04 |
| 6 | Schedule 与执行顺序 | ✅ | code/ch06-schedules（src/main.rs + examples/listing-06-01～08，开 bevy `debug` feature）；book/src/ch06-00～05 |
| 7 | Message——缓冲消息 | ✅ | code/ch07-messages（src/main.rs + examples/listing-07-01～08，开 bevy `debug` feature）；book/src/ch07-00～05 |
| 8 | Event 与 Observer | ✅ | code/ch08-events-observers（src/main.rs + examples/listing-08-01～09，开 bevy `debug` feature）；book/src/ch08-00～05 |
| 9 | 实体关系与层级 | ✅ | code/ch09-relationships（src/main.rs + examples/listing-09-01～09，其中 09-05 在 no-compile/ 下为编译失败示例，开 bevy `debug` feature）；book/src/ch09-00～05 |
| 10 | State——游戏状态机 | ✅ | code/ch10-states（src/main.rs + examples/listing-10-01～09，其中 10-10 在 no-compile/ 下为编译失败示例，开 bevy `debug` feature）；book/src/ch10-00～06 |
| 11 | 深入 ECS | ✅ | code/ch11-deep-ecs（src/main.rs + examples/listing-11-01～13，其中 11-02 在 no-compile/ 下为编译失败示例、11-05 为运行 panic 示例；main.rs 即 Listing 11-14；开 bevy `debug`+`track_location` feature）；book/src/ch11-00～07 |
| 12 | Transform 与坐标系统 | ✅ | code/ch12-transforms（src/main.rs + examples/listing-12-01～12，main.rs 即 Listing 12-13，开 bevy `debug` feature）；book/src/ch12-00～08；插图 images/ch12（Figure 12-1～12，含 1 张动图，scripts/make_ch12_figures.py 一键重建） |
| 13 | Camera 与视口 | ✅ | code/ch13-cameras（src/main.rs + examples/listing-13-01～11，其中 13-06 在 no-compile/ 下为编译失败示例；main.rs 即 Listing 13-12）；book/src/ch13-00～08；插图 images/ch13（Figure 13-1～9，含 1 张动图，scripts/make_ch13_figures.py 一键重建） |
| 14 | Asset 系统 | ✅ | code/ch14-assets（src/main.rs + examples/listing-14-01～10，main.rs 即 Listing 14-11；开 bevy `file_watcher` feature，依赖 thiserror）；book/src/ch14-00～08；插图 images/ch14（Figure 14-1～7，scripts/make_ch14_figures.py 一键重建；美术资产由 scripts/make_ch14_assets.py 生成） |
| 15 | 2D 渲染：Sprite 与图集 | ✅ | code/ch15-sprites（src/main.rs + examples/listing-15-01～11，main.rs 即 Listing 15-12）；book/src/ch15-00～07；插图 images/ch15（Figure 15-1～14，含 3 张 SVG、1 张动图，scripts/make_ch15_figures.py 一键重建；美术资产由 scripts/make_ch15_assets.py 生成） |
| 16 | 文本与字体 | ✅ | code/ch16-text（src/main.rs + examples/listing-16-01～10，main.rs 即 Listing 16-11；中文字体资产为 Noto Sans SC 的 GB2312 子集，scripts/make_ch16_assets.py 下载/子集化/按 OFL 改名一键重建）；book/src/ch16-00～08；插图 images/ch16（Figure 16-1～12，含 1 张 SVG、2 张动图，scripts/make_ch16_figures.py 一键重建） |
| 17 | 输入处理 | ✅ | code/ch17-input（src/main.rs + examples/listing-17-01～07，其中 17-02 在 no-compile/ 下为编译失败示例；main.rs 即 Listing 17-9；assets 全部复用 ch15/ch16，scripts/make_ch17_assets.py 一键就位）；book/src/ch17-00～07；插图 images/ch17（Figure 17-1～8，含 4 张 SVG、1 张动图，scripts/make_ch17_figures.py 用 SendInput 发真实键鼠一键重建） |
| 18 | 时间、定时器与 FixedUpdate | ✅ | code/ch18-time（src/main.rs + examples/listing-18-01～09，其中 18-02 在 no-compile/ 下为编译失败示例；main.rs 即 Listing 18-10；assets 全部复用 ch15/ch16，scripts/make_ch18_assets.py 一键就位）；book/src/ch18-00～06；插图 images/ch18（Figure 18-1～10，含 6 张 SVG、1 张动图，scripts/make_ch18_figures.py 用 SendInput 发真实按键一键重建） |
| 19 | 音频 | ✅ | code/ch19-audio（src/main.rs + examples/listing-19-01～08，main.rs 即 Listing 19-9；开 bevy `wav` feature，报错环节为运行期 UnrecognizedFormat panic（19.1，真实复现）；音频资产全部由 scripts/make_ch19_assets.py 用 Python 标准库合成 WAV，字体贴图复用前章）；book/src/ch19-00～06；插图 images/ch19（Figure 19-1～7，含 3 张 SVG，scripts/make_ch19_figures.py 用 SendInput 发真实按键一键重建，波形图直接读合成 WAV 采样绘制） |
| 20 | 项目实战 I：完整的 2D 游戏 | ✅ | code/ch20-breakout（src/ 为 main.rs + game/menu/score/audio 四插件的最终版；examples/listing-20-01～07 为分步阶段版，no-compile/listing-20-08 为编译失败示例；开 bevy `wav` feature，报错环节为 E0603 与“忘注册插件→缺资源 panic”双坑，均真实复现；音效由 scripts/make_ch20_assets.py 合成，BGM/堂鼓复用 ch19、字体复用 ch16）；book/src/ch20-00～08；插图 images/ch20（Figure 20-1～11，含 4 张 SVG、1 张动图，scripts/make_ch20_figures.py 一键重建——内置跟球 bot 发真实按键打出全部局面） |
| 21 | 3D 入门：Mesh 与 Material | ✅ | code/ch21-meshes（src/main.rs 即 Listing 21-10 + examples/listing-21-01～09，其中 21-04 在 no-compile/ 下为编译失败示例（E0308 忘 meshes.add）；另一坑为运行期静默缺陷：手搓 Mesh 忘写法线→旗面对灯失聪，零警告；班旗贴图由 scripts/make_ch21_assets.py 用 PIL 合成）；book/src/ch21-00～06；插图 images/ch21（Figure 21-1～12，含 3 张 SVG，scripts/make_ch21_figures.py 一键重建） |
| 22 | 光照与阴影 | ✅ | code/ch22-lighting（src/main.rs 即 Listing 22-11 昼夜光照切换台 + examples/listing-22-01～10，其中 22-08 在 no-compile/ 下为编译失败示例（E0277：AmbientLight 是组件、GlobalAmbientLight 才是资源）；env map 用 scripts/make_ch22_assets.py 合成的竖摞六面 cubemap PNG + GeneratedEnvironmentMapLight，装配后再挂以避开「源图非正方形」运行期 panic（先跑后写真实复现））；book/src/ch22-00～08；插图 images/ch22（Figure 22-1～15，含 6 张手绘 SVG：三种灯几何、雾衰减曲线，以及阴影贴图原理/shadow acne 与 bias/级联/立方体贴图与环境光照四张原理图；9 张截图由 scripts/make_ch22_figures.py 一键重建——昼夜四档用 ALT 解前台锁后发真实空格切档截帧） |
| 23 | glTF 与 3D 资产 | ✅ | code/ch23-gltf（src/main.rs 即 Listing 23-6 角儿登场 + examples/listing-23-01～05，其中 23-03 在 no-compile/ 下为编译失败示例（E0308：`Handle<Gltf>` 塞进要 `Handle<Scene>` 的 SceneRoot）；木偶 `puppet.gltf` 由 scripts/make_ch23_assets.py 用纯 Python 标准库手写——7 个命名节点 + 命名动画 Swing（节点变换、无蒙皮），不开任何新特性，glTF 加载与动画都在默认 3d 特性里）；book/src/ch23-00～07；插图 images/ch23（Figure 23-1～7，含 3 张手绘 SVG：glTF 集装箱/标签、节点树→实体树、Blender 工作流；4 张截图（含 1 张行进三连帧）由 scripts/make_ch23_figures.py 一键重建）；**WASM 网页 demo（两个）**：① ch23-07 用 src/main.rs（`web_window` + `orbit_camera` 拖动转视角两锚点）；② ch23-05 用 examples/listing-23-05.rs，在正文 ANCHOR 截取区之外加 `WindowPlugin`(canvas `#bevy-ch23-anim`) 与 `toggle_graph_on_click`——点画面 remove/insert `AnimationGraphHandle`，亲手重现「漏接图、动作不动还不报错」的哑巴坑（正文印出的代码一字未动；占位图复用行进三连帧 Figure 23-5）。`scripts/build_ch23_wasm.py` 推广为多 demo（DEMOS 列表，bin + example 两种目标），`wasm-release` profile（opt-level=z+lto+strip，各 ~20 MB）一键编出 `ch23_gltf`/`ch23_anim` 两套产物入 `book/src/demos/ch23/`（共用 puppet.gltf），经懒加载模板 `book/theme/demo.{css,js}` 内嵌（占位图点击注入 iframe，截图兜底）；产物 gitignore，两 demo 均已在浏览器真 GPU 实跑验证（ch23-07 渲染/动画/挂旗；ch23-05 踏步→点击冻结→再点复活） |
| 24 | PBR 材质深入 | ✅ | code/ch24-pbr-materials（src/main.rs 即 Listing 24-8 材质球画廊 + examples/listing-24-01～07，其中 24-02 在 no-compile/ 下为编译失败示例（E0308：emissive 要 `LinearRgba` 却塞了 `Color`）；不开任何新特性——clearcoat/emissive/normal/alpha/double_sided/depth_bias 等标量字段都在默认特性里）；book/src/ch24-00～07；三件贴图（studs-normal 法线图、lattice 镂空图、skybox 影棚环境）由 scripts/make_ch24_assets.py 纯 PIL 合成；插图 images/ch24（Figure 24-1～11，含 4 张手绘 SVG：材质控制台/透明算法/清漆分层/绕序与剔除，6 张截图 + 1 张双面自转动图由 scripts/make_ch24_figures.py 一键重建）；**WASM 网页 demo**：src/main.rs（`web_window` canvas `#bevy-ch24` + 自转转台 + 空格/点击 `toggle_clearcoat`），scripts/build_ch24_wasm.py 编出 ch24_gallery（~20.7 MB）入 book/src/demos/ch24，懒加载模板内嵌，已在浏览器真 GPU（WebGL2）实跑验证（渲染/自转/点击拨清漆） |
| 25 | Picking 与相机控制 | ⬜ | |
| 26 | 画质：后处理与抗锯齿 | ⬜ | |
| 27 | Gizmos、诊断与开发工具 | ⬜ | |
| 28 | UI 基础：Node 与布局 | ⬜ | |
| 29 | UI 交互与控件（含项目实战 II） | ⬜ | |
| 30 | 动画系统 | ⬜ | |
| 31 | Reflect——运行时反射 | ⬜ | |
| 32 | Scene——场景序列化与数据驱动 | ⬜ | |
| 33 | 日志、错误处理与远程调试 | ⬜ | |
| 34 | 异步与并行 | ⬜ | |
| 35 | 窗口与平台细节 | ⬜ | |
| 36 | 自定义 Material 与 Shader | ⬜ | |
| 37 | 渲染架构导览 | ⬜ | |
| 38 | 发布你的游戏 | ⬜ | |
| 附A | 编译加速与安装疑难 | ⬜ | |
| 附B | Cargo features 完整清单 | ⬜ | |
| 附C | ECS 速查表 | ⬜ | |
| 附D | 生态系统地图（是否纳入待用户定） | ⬜ | |
| 附E | 版本迁移方法论 | ⬜ | |
| 附F | 中英术语对照表 | ⬜ | |

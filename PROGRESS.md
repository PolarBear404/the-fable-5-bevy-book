# 进度总览

> 每完成一个步骤就更新此文件。状态：⬜ 未开始 / 🟡 进行中 / ✅ 完成

## 项目阶段

| 阶段 | 状态 | 说明 |
|---|---|---|
| 脚手架 | ✅ 2026-06-12 | 目录结构、mdBook、Cargo workspace、规范、斜杠命令 |
| vendor/bevy 源码就位 | ✅ 2026-06-12（2026-07-02 换 0.19.0） | v0.19.0 @ c6f634c，59 crate |
| Bevy 0.18.1 编译验证 | ✅ 2026-06-12 | `cargo check -p smoke` 通过 |
| 全书大纲 OUTLINE.md | ✅ 2026-06-12 | 38 章 + 6 附录，用户已审定 |
| 章节写作 | 🟡 | 进度见下表 |
| Bevy 0.19 大纲评估与 OUTLINE 修订 | ✅ 2026-07-02 | migration/0.19-outline-assessment.md，P-1～P-13 全部批准；38 章 + 6 附录结构不变，ch1–21 编号全保 |
| 0.19 全书迁移执行（S1–S3） | ✅ 2026-07-03 | 计划、逐章工单与实测数据存档于 migration/0.19-migration-plan.md。S1＋检查点 C1/C2（2026-07-02）→ S2 逐章 ch1–21（2026-07-03）→ S3 收尾（2026-07-03）：cargo check 全绿零警告、mdbook build 绿；无痕检查全书 grep 逐条判读——introduction.md 残留「0.18.1」口径漏网已订正，ch04/ch08/ch12 三处零星复查点落实，引擎史实叙述（SpriteBundle/KeyCode::A/Event 曾用名）判读为读者服务、统一保留；make_ch14_figures.py 裁剪改按截图宽自动定标（125% 下与旧产物逐字节回归）；vendor/bevy-0.18 已删、CLAUDE.md 过渡期条目移除。是否并回 main 由用户决定 |

## 章节状态

| 章 | 标题 | 状态 | 产物（crate / 正文文件，动工时填写） |
|---|---|---|---|
| 1 | 认识 Bevy | ✅ | book/src/ch01-00～04（本章无代码 crate；引用 code/Cargo.toml 的 dev_profile 片段）；已迁 0.19 |
| 2 | 第一个 Bevy App | ✅ | code/ch02-first-app（src/main.rs + examples/listing-02-01～06）；book/src/ch02-00～03；已迁 0.19 |
| 3 | Entity 与 Component | ✅ | code/ch03-entities-components（src/main.rs + examples/listing-03-01～06）；book/src/ch03-00～03；已迁 0.19 |
| 4 | System 与 Query | ✅ | code/ch04-systems-queries（src/main.rs + examples/listing-04-01～07，开 bevy `debug` feature）；book/src/ch04-00～04；已迁 0.19 |
| 5 | Resource——全局唯一数据 | ✅ | code/ch05-resources（src/main.rs + examples/listing-05-01～08，开 bevy `debug` feature）；book/src/ch05-00～05；已迁 0.19（新增“资源的本质”一节） |
| 6 | Schedule 与执行顺序 | ✅ | code/ch06-schedules（src/main.rs + examples/listing-06-01～08，开 bevy `debug` feature）；book/src/ch06-00～05；已迁 0.19 |
| 7 | Message——缓冲消息 | ✅ | code/ch07-messages（src/main.rs + examples/listing-07-01～08，开 bevy `debug` feature）；book/src/ch07-00～05；已迁 0.19 |
| 8 | Event 与 Observer | ✅ | code/ch08-events-observers（src/main.rs + examples/listing-08-01～10，开 bevy `debug` feature）；book/src/ch08-00～05；已迁 0.19（新增“打烊之后：给 observer 挂 run_if”一节，原 8-3～8-9 顺延为 8-4～8-10） |
| 9 | 实体关系与层级 | ✅ | code/ch09-relationships（src/main.rs + examples/listing-09-01～10，其中 09-05 在 no-compile/ 下为编译失败示例；main.rs 即 Listing 9-11；开 bevy `debug` feature）；book/src/ch09-00～05；已迁 0.19（新增“allow_self_referential：允许自指”一节，配 listing-09-10） |
| 10 | State——游戏状态机 | ✅ | code/ch10-states（src/main.rs + examples/listing-10-01～09，其中 10-10 在 no-compile/ 下为编译失败示例，开 bevy `debug` feature）；book/src/ch10-00～06；已迁 0.19（同值转换清场行为实测入正文，listing-10-07 加手肘一幕） |
| 11 | 深入 ECS | ✅ | code/ch11-deep-ecs（src/main.rs + examples/listing-11-01～16，其中 11-02 在 no-compile/ 下为编译失败示例、11-05 为运行 panic 示例；main.rs 即 Listing 11-17；开 bevy `debug`+`track_location` feature）；book/src/ch11-00～09；已迁 0.19（新增“资源实体与混合查询”“连续访问”两节，配 listing-11-14～16，原盘点日顺延为 ch11-09） |
| 12 | Transform 与坐标系统 | ✅ | code/ch12-transforms（src/main.rs + examples/listing-12-01～12，main.rs 即 Listing 12-13，开 bevy `debug` feature）；book/src/ch12-00～08；插图 images/ch12（Figure 12-1～12，含 1 张动图，scripts/make_ch12_figures.py 一键重建）；已迁 0.19（代码零改；B0004 警告块与 GlobalTransform 手改叙述按实测更新；7 张运行图重建逐图核对） |
| 13 | Camera 与视口 | ✅ | code/ch13-cameras（src/main.rs + examples/listing-13-01～11，其中 13-06 在 no-compile/ 下为编译失败示例；main.rs 即 Listing 13-12）；book/src/ch13-00～08；插图 images/ch13（Figure 13-1～9，含 1 张动图，scripts/make_ch13_figures.py 一键重建）；已迁 0.19（代码零改；order 冲突警告改单次告警叙述、窗口实体号按实跑更新；7 张运行图重建逐图核对） |
| 14 | Asset 系统 | ✅ | code/ch14-assets（src/main.rs + examples/listing-14-01～11，main.rs 即 Listing 14-12；开 bevy `file_watcher` feature，依赖 thiserror）；book/src/ch14-00～09；插图 images/ch14（Figure 14-1～7，scripts/make_ch14_figures.py 一键重建；美术资产由 scripts/make_ch14_assets.py 生成）；已迁 0.19（新增“白纸黑字：把资产存回磁盘”一节（ch14-07，AssetSaver/save_using_saver，配 listing-14-09，原 09/10 顺延为 10/11、细则与开机大吉顺延为 ch14-08/09）；load_builder 正文跟进；热重载三实验回归；5 张运行图重建逐图核对） |
| 15 | 2D 渲染：Sprite 与图集 | ✅ | code/ch15-sprites（src/main.rs + examples/listing-15-01～11，main.rs 即 Listing 15-12）；book/src/ch15-00～07；插图 images/ch15（Figure 15-1～14，含 3 张 SVG、1 张动图，scripts/make_ch15_figures.py 一键重建；美术资产由 scripts/make_ch15_assets.py 生成）；已迁 0.19（代码零改；load_builder 口径跟进、border 越界 ERROR 块按实跑双条更新；11 张运行图重建逐图核对，其中 8 张与旧版逐字节一致） |
| 16 | 文本与字体 | ✅ | 已迁 0.19。code/ch16-text（src/main.rs + examples/listing-16-01～14，main.rs 即 Listing 16-15；listing-16-05 走 crate 转发 feature `system_font_discovery` + required-features 门控；中文字体资产为 Noto Sans SC 的 GB2312 子集，另有可变字体 MonaSans-VariableFont.ttf，scripts/make_ch16_assets.py 一键重建）；book/src/ch16-00～12（新增 16-03 字模三种叫法／16-04 向系统借字模／16-05 可变字体／16-07 会自己变的字号）；插图 images/ch16（Figure 16-1～16，含 1 张 SVG、2 张动图，scripts/make_ch16_figures.py 一键重建） |
| 17 | 输入处理 | ✅ | code/ch17-input（src/main.rs + examples/listing-17-01～07，其中 17-02 在 no-compile/ 下为编译失败示例；main.rs 即 Listing 17-9；assets 全部复用 ch15/ch16，scripts/make_ch17_assets.py 一键就位）；book/src/ch17-00～07；插图 images/ch17（Figure 17-1～8，含 4 张 SVG、1 张动图，scripts/make_ch17_figures.py 用 SendInput 发真实键鼠一键重建）；已迁 0.19（代码零改、输出块零改——键鼠全示例实跑逐字一致，含系统重复探针与 448/−512 坐标复现；gamepad/touch 无硬件，vendor 源码 0.18↔0.19 diff 实证零变化＋编译实跑容错路径；4 张位图重建，fig-17-05 与旧图逐像素一致） |
| 18 | 时间、定时器与 FixedUpdate | ✅ | code/ch18-time（src/main.rs + examples/listing-18-01～10，其中 18-02 在 no-compile/ 下为编译失败示例；main.rs 即 Listing 18-11；assets 全部复用 ch15/ch16，scripts/make_ch18_assets.py 一键就位）；book/src/ch18-00～07；插图 images/ch18（Figure 18-1～11，含 7 张 SVG、1 张动图，scripts/make_ch18_figures.py 用 SendInput 发真实按键一键重建）；已迁 0.19（新增“过一会儿再办：延迟命令”一节，配 listing-18-06、Figure 18-5，原 18-4～18-6 节与 Listing/Figure 顺延；驿站听戏台钟实测入正文） |
| 19 | 音频 | ✅ | code/ch19-audio（src/main.rs + examples/listing-19-01～08，main.rs 即 Listing 19-9；开 bevy `wav` feature，报错环节为运行期 UnrecognizedFormat panic（19.1，真实复现）；音频资产全部由 scripts/make_ch19_assets.py 用 Python 标准库合成 WAV，字体贴图复用前章）；book/src/ch19-00～06；插图 images/ch19（Figure 19-1～7，含 3 张 SVG，scripts/make_ch19_figures.py 用 SendInput 发真实按键一键重建，波形图直接读合成 WAV 采样绘制）；已迁 0.19 |
| 20 | 项目实战 I：完整的 2D 游戏 | ✅ | code/ch20-breakout（src/ 为 main.rs + game/menu/score/audio 四插件的最终版；examples/listing-20-01～07 为分步阶段版，no-compile/listing-20-08 为编译失败示例；开 bevy `wav` feature，报错环节为 E0603 与“忘注册插件→缺资源 panic”双坑，均真实复现；音效由 scripts/make_ch20_assets.py 合成，BGM/堂鼓复用 ch19、字体复用 ch16）；book/src/ch20-00～08；插图 images/ch20（Figure 20-1～11，含 4 张 SVG、1 张动图，scripts/make_ch20_figures.py 一键重建——内置跟球 bot 发真实按键打出全部局面）；已迁 0.19（代码近零改；全流程回归双胜负路径实测；panic 块按实跑更新；官方示例路径改 showcase/；bot 几何改为按窗口宽度自动定标；7 张位图重建逐图核对） |
| 21 | 3D 入门：Mesh 与 Material | ✅ | code/ch21-meshes（src/main.rs 即 Listing 21-10 + examples/listing-21-01～09，其中 21-04 在 no-compile/ 下为编译失败示例（E0308 忘 meshes.add）；另一坑为运行期静默缺陷：手搓 Mesh 忘写法线→旗面对灯失聪，零警告；班旗贴图由 scripts/make_ch21_assets.py 用 PIL 合成）；book/src/ch21-00～06；插图 images/ch21（Figure 21-1～12，含 3 张 SVG，scripts/make_ch21_figures.py 一键重建）；已迁 0.19（代码零改；main＋8 examples 全实跑输出逐字一致，法线静默缺陷、背面剔除、空格换漆闭环全部复现；E0308 报错块按实跑更新（bevy_mesh-0.19.0 行号 102）；灯光/图元/材质点名 API 对照 vendor 全未变；9 张位图重建逐图核对） |
| 22 | 光照与阴影 | ⬜ | |
| 23 | glTF 与 3D 资产 | ⬜ | |
| 24 | PBR 材质深入 | ⬜ | |
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

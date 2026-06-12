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
| 12 | Transform 与坐标系统 | ✅ | code/ch12-transforms（src/main.rs + examples/listing-12-01～12，main.rs 即 Listing 12-13，开 bevy `debug` feature）；book/src/ch12-00～08 |
| 13 | Camera 与视口 | ⬜ | |
| 14 | Asset 系统 | ⬜ | |
| 15 | 2D 渲染：Sprite 与图集 | ⬜ | |
| 16 | 文本与字体 | ⬜ | |
| 17 | 输入处理 | ⬜ | |
| 18 | 时间、定时器与 FixedUpdate | ⬜ | |
| 19 | 音频 | ⬜ | |
| 20 | 项目实战 I：完整的 2D 游戏 | ⬜ | |
| 21 | 3D 入门：Mesh 与 Material | ⬜ | |
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

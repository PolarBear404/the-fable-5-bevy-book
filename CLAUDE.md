# The Bevy Book — 项目规范

系统性的 Bevy 引擎中文教程，体例对标 The Rust Book：mdBook 正文 + 全部经编译验证的配套代码。
所有会话一律用中文交流和写作。

## 读者与定位

- 读者已掌握 Rust 基础（所有权、trait、泛型、闭包、模块系统）——本书不解释 Rust 语言本身
- 读者零游戏引擎/游戏开发经验——所有引擎概念（ECS、渲染、资产、变换、游戏循环…）从零讲起
- 覆盖目标：Bevy 官方功能全貌，深度以"读完能独立做出中小型游戏、能看懂官方文档与示例"为准

## 版本锁定（铁律）

- 全书基于 **Bevy 0.18.1**（crates.io 精确锁定 `=0.18.1`），Rust edition 2024
- 升级 Bevy 版本是只有用户才能发起的重大决定；任何会话都禁止顺手升级或放宽版本号
- 0.19 已在 RC 阶段：正式发布后是否迁移由用户决定，届时做一次性的全书迁移 pass

## API 事实来源（铁律）

1. 写任何 Bevy API 之前，先在 `vendor/bevy/`（v0.18.1 源码）里确认它的存在与签名；
   官方示例在 `vendor/bevy/examples/`，是每章选材的第一参考
2. 禁止凭记忆写 Bevy API——模型记忆很可能对应旧版本；拿不准就 grep 源码
3. `vendor/bevy/` 只读、不入 git。缺失时恢复：
   `git clone --depth 1 --branch v0.18.1 https://github.com/bevyengine/bevy.git vendor/bevy`

## 目录结构

| 路径 | 作用 |
|---|---|
| `OUTLINE.md` | 全书大纲；用户审定后是章节范围的唯一依据 |
| `PROGRESS.md` | 进度状态；每完成一步就更新 |
| `book/` | mdBook 工程，正文在 `book/src/` |
| `code/` | Cargo workspace；每章一个 crate，命名如 `ch03-ecs-basics` |
| `vendor/bevy/` | 锁定版本的 Bevy 源码（只读参考，不入 git） |

## 代码规范

- 正文中的每段 Rust/TOML 代码必须来自 `code/` 下真实文件，用 mdBook 的 `{{#include}}` 引入；
  片段用 `// ANCHOR: name` / `// ANCHOR_END: name` 截取。禁止在 .md 里手写 Rust 代码块
  （shell 命令、程序输出示例除外）
- 新章 crate 要加入 `code/Cargo.toml` 的 `members`；依赖写 `bevy = { workspace = true }`
- 每章完成的标准：`cargo check -p <crate>` 全绿；可运行示例至少 `cargo run` 启动核实一次，
  确认实际行为与正文描述一致
- 一章内的多个阶段版本放该 crate 的 `examples/`（`listing-03-01.rs`…），与正文 Listing 编号对应
- 示例代码追求教学清晰：少抽象、不炫技、变量名表意，与正文逐段对应

## 行文规范

- Bevy 专有术语保留英文（Entity、Component、System、Resource、Query、Plugin、Schedule、Asset…），
  每个术语首次出现时用一句中文解释；一般概念用中文
- 文件名一律英文 kebab-case（如 `ch03-01-entities-and-components.md`），章节标题用中文
- 每章结构：本章学什么（2-3 句）→ 代码驱动的正文 → 小结 → 练习（可选）
- 语气直接、准确，像 The Rust Book；不堆比喻、不写空话；对运行行为的描述必须先跑过代码再下结论
- **风格基准：第 1 章**（book/src/ch01-*，用户 2026-06-12 审定）。动笔写新章之前先重读第 1 章找语感；
  语气、小节长度、术语密度有争议时一律以它为准

## 插图规范（2026-06-12 起，ch13 首用）

- 图存 `book/src/images/chXX/`，命名 `fig-XX-NN-slug.{png,webp,svg}`；正文编号 `Figure XX-N` 按章内
  出现顺序，与 Listing 编号互相独立；每图必有中文 alt 文本 + `<span class="caption">Figure XX-N：…</span>`
- 运行截图/动图必须脚本化可复现：底层工具 `scripts/capture.py`（PrintWindow 截窗口客户区、物理像素、
  以窗口出现时刻为时间零点，不怕遮挡与 DPI 缩放），每章配一个 `scripts/make_chXX_figures.py` 一键重建。
  禁止手工截图——版本迁移时全部插图要能一条命令重新生成
- 图是佐证与直观化，不是信息的唯一载体：删掉所有插图后正文仍须自足（兼顾打印/无障碍）
- 动图克制使用，仅当"运动本身是教学点"（如跟拍手感对比）；WebP 动图，单图 ≤ 2 MB
- 概念示意图手绘 SVG：浅色卡片底（#f7f5f0 圆角矩形打底），明暗主题下均可读
- 截图与正文输出一样受"先跑后写"铁律约束：画面与正文描述不符时，修代码或修正文，不准 P 图

## 交互演示规范（2026-06-13 起，ch23 首用）

- **何时加（默认不加，从严）**：仅当"交互/实时运行本身是教学点"——3D 要转着看、参数要即时拨、
  动画手感，静态截图损失大的章（光照、材质、相机、动画、物理…）。纯逻辑章（ECS、Schedule、
  消息…）一律不加，静态代码 + 输出已足；判断尺度同「动图克制使用」
- **同源代码**：demo 必须是本章真实可编译的示例（`main.rs` 或某 listing），靠
  `WindowPlugin { canvas, fit_canvas_to_parent }`（这两个字段在非 web 平台无效）做到桌面/网页跑同
  一份，受"正文代码＝编译过的代码"铁律约束
- **构建脚本化**：每个 demo 配一个 `scripts/build_chXX_wasm.py`（对标 `make_chXX`），一键
  `cargo build --profile wasm-release --target wasm32-unknown-unknown` + `wasm-bindgen --target web`
  输出到 `book/src/demos/chXX/`；`wasm-release` profile（opt-level="z" + lto + strip）压体积；
  `wasm-bindgen-cli` 版本须对齐 `code/Cargo.lock`。产物（*.js/*.wasm/assets）gitignore、禁止手工
  构建——版本迁移时要能一条命令重生
- **嵌入即懒加载**：复用通用模板 `book/theme/demo.{css,js}`（`book.toml` 接 additional-css/js），
  正文只写 `<figure class="bevy-demo" data-src="demos/chXX/index.html">` + 占位图 + figcaption；点击
  占位图才注入 iframe（避免一进页面就下几十 MB）。占位图用该章真实截图——无 JS/打印/删掉 demo 时，
  正文凭占位图 + 图注仍须自足（同插图规范）
- **web 输入坑**：网页下的鼠标拖动交互禁用 `AccumulatedMouseMotion`（winit 的 raw 鼠标位移在浏览器
  里要 pointer lock 才有数据，iframe demo 中恒为 0、桌面却正常，极易漏判）；改用
  `Window::cursor_position()` + `Local` 存上一帧算 delta（即第 17 章的 cursor_position 手法）
- **先跑后写**：demo 同受铁律约束——发布前必须在浏览器实跑，确认渲染/动画/交互与正文描述一致
  （可用 Claude Preview 起本地 server + 真 GPU 验证；合成 pointer 事件能驱动 cursor_position 路径）

## 工作流

- 写章节：`/write-chapter <编号>`（流程内置于命令）；交叉审阅：`/review-part <范围>`
- 每章完成后：更新 `PROGRESS.md` 与 `book/src/SUMMARY.md`，git commit（信息格式 `ch03: ECS 入门`）
- 本地预览：`mdbook serve book`；构建检查：`mdbook build book`
- 环境：Windows 11 + PowerShell；项目路径含空格，命令里路径要加引号

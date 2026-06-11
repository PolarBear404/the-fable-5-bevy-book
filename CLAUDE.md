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

## 工作流

- 写章节：`/write-chapter <编号>`（流程内置于命令）；交叉审阅：`/review-part <范围>`
- 每章完成后：更新 `PROGRESS.md` 与 `book/src/SUMMARY.md`，git commit（信息格式 `ch03: ECS 入门`）
- 本地预览：`mdbook serve book`；构建检查：`mdbook build book`
- 环境：Windows 11 + PowerShell；项目路径含空格，命令里路径要加引号

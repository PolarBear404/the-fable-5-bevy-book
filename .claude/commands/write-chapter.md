---
description: 按项目规范完整地写一章（代码先行 → 编译验证 → 正文 include → 构建 → 收尾）
argument-hint: <章节编号，如 3 或 3.2>
---

写第 $ARGUMENTS 章。严格按以下顺序执行，前一步没完成不进入下一步：

1. **定范围**：读 OUTLINE.md 中本章条目与 PROGRESS.md；浏览前一章正文的结尾和后一章的大纲，
   确保衔接——不依赖尚未讲过的概念，不重复已讲过的内容。如发现本章合理范围与大纲有出入，
   先停下来向用户说明，不要擅自改大纲。
2. **代码先行**：在 code/ 下创建或更新本章 crate（命名如 ch03-ecs-basics，并加入 workspace 的
   members）。写出本章全部示例代码。每个 Bevy API 用法必须先在 vendor/bevy/ 中确认存在与签名
   （铁律，见 CLAUDE.md）。逐个通过 `cargo check -p <crate>`；可运行示例至少 `cargo run` 启动
   核实一次，确认实际行为与将要写的正文一致。
3. **写正文**：在 book/src/ 写本章 .md（文件名英文 kebab-case）。所有 Rust/TOML 代码块一律
   `{{#include}}` 自 code/ 的文件（用 ANCHOR 片段截取），禁止手写。遵守 CLAUDE.md 行文规范。
4. **构建验证**：`mdbook build book` 通过、无 include 解析失败；检查本章内部链接和 Listing
   编号的连续性。
5. **收尾**：更新 book/src/SUMMARY.md 和 PROGRESS.md 的本章状态；git commit，信息格式
   `ch03: 章题`。

完成后向用户汇报：本章覆盖了大纲中的哪些点、有意留到后续章节的点、与前后章的衔接处。

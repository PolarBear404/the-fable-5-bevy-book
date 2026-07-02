# 安装与项目搭建

## 前置条件

**Rust 工具链 ≥ 1.95.0**。这是 Bevy 0.19.0 声明的最低 Rust 版本，建议直接用 rustup 更新到最新 stable：

```console
rustup update
rustc --version   # 确认 ≥ 1.95.0
```

各操作系统还需要少量系统依赖：

- **Windows**：MSVC 工具链（Visual Studio Build Tools）。如果你装 Rust 时按默认流程走过一遍，它已经在了。
- **macOS**：`xcode-select --install` 安装命令行工具。
- **Linux**：需要 ALSA、udev 等开发库，各发行版的安装命令见官方文档 [linux_dependencies.md](https://github.com/bevyengine/bevy/blob/v0.19.0/docs/linux_dependencies.md)。

## 创建项目

```console
cargo new my_game
cd my_game
cargo add bevy@0.19.0
```

`cargo add bevy@0.19.0` 会在 `Cargo.toml` 里写入与本书一致的版本。然后验证一切就绪：

```console
cargo check
```

**第一次会很慢，这是正常的。** Cargo 要下载并编译几百个依赖 crate——视机器和网络，从几分钟到十几分钟不等。这个成本只付一次：依赖编译结果会被缓存，之后日常开发的增量编译以秒计。趁编译跑着，正好把下面的编译加速配置加上。

## 编译加速（强烈建议）

在你项目的 `Cargo.toml` 末尾加上这段配置（本书配套仓库也是这么配的）：

```toml
{{#include ../../code/Cargo.toml:dev_profile}}
```

它的含义是：开发模式下，**你自己的代码**用低优化（编译快），**所有依赖**用高优化（只编译一次，值得）。游戏的运行性能大头在引擎代码里，所以这样配置后，开发期的游戏跑起来依然流畅，而你每次改代码的重编译却很快。

另一个常用手段是 `dynamic_linking`——把 Bevy 链接成动态库，跳过大部分链接时间：

```console
cargo run --features bevy/dynamic_linking
```

注意它**只用于开发期**：发布构建不要带这个 feature。更多加速手段（更换 linker、编译缓存等）见附录 A。

## 验证环境

空项目加上 Bevy 依赖、`cargo check` 跑出绿色的 `Finished`，环境就绪。下一节交代本书的使用方法，然后第 2 章正式写代码。

# The Fable 5 Bevy Book

这是一个使用 [mdBook](https://rust-lang.github.io/mdBook/) 编写的 Bevy 中文教程。正文工程位于 `book/`，章节 Markdown 文件位于 `book/src/`。

## 使用 mdBook 打开教程

先确认本机已经安装 `mdbook`：

```powershell
mdbook --version
```

如果命令不存在，可以通过 Cargo 安装：

```powershell
cargo install mdbook
```

在仓库根目录运行下面的命令启动本地预览服务器，并自动在浏览器中打开教程：

```powershell
mdbook serve book --open
```

也可以先进入 mdBook 工程目录再启动：

```powershell
cd book
mdbook serve --open
```

默认情况下，mdBook 会监听 `http://localhost:3000/`。保存正文文件后，浏览器页面会自动刷新。

## 构建静态版本

如果只想生成 HTML 文件而不启动预览服务器，可以运行：

```powershell
mdbook build book
```

生成结果位于 `book/book/`，入口文件是 `book/book/index.html`。

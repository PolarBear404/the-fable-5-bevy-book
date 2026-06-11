# Plugin：App 能力的来源

**Plugin**（插件）是把"对 App 的一组配置"打包起来的单元：注册几个 System、放入一些数据、替换 runner……任何你能对 App 做的事，都能装进一个 Plugin。Bevy 引擎自身就是这么组织的——时间是插件，日志是插件，窗口、渲染、音频全是插件。

## MinimalPlugins：先把循环装上

```rust
{{#include ../../code/ch02-first-app/examples/listing-02-03.rs}}
```

<span class="caption">Listing 2-3：用 MinimalPlugins 获得主循环，每秒跑一轮</span>

`add_plugins` 负责装插件。`MinimalPlugins` 不是单个插件，而是一个 **PluginGroup**（插件组）：官方打包好的最小运行环境，只有四个成员——线程池（`TaskPoolPlugin`）、帧计数（`FrameCountPlugin`）、时间（`TimePlugin`），以及关键的 `ScheduleRunnerPlugin`：**它把默认那个"跑一遍就走"的 runner 换成真正的循环**。

这个循环默认不停顿，以 CPU 允许的最快速度空转。所以这里用插件组的 `.set()` 方法**定制组内某个成员**：把组里的 `ScheduleRunnerPlugin` 替换成"每轮之间至少等一秒"的配置。运行：

```text
Hello, Bevy!
Hello, Bevy!
Hello, Bevy!
（每秒一行，按 Ctrl+C 终止）
```

同一个 `hello` 一行没改，从"跑一次"变成了"每秒一次"——改变的只是插件。`MinimalPlugins` 适合不需要窗口的场合：专用服务器、命令行工具、自动化测试。

## DefaultPlugins：完整的引擎

把插件组换掉，其余原样：

```rust
{{#include ../../code/ch02-first-app/examples/listing-02-04.rs}}
```

<span class="caption">Listing 2-4：换上 DefaultPlugins——窗口出现了</span>

```console
cargo run -p ch02-first-app --example listing-02-04
```

这次**一个窗口打开了**（标题是可执行文件名——`Window` 的默认标题策略；首次运行前会停顿几秒，初始化显卡）。控制台里出现两类输出：先是几行 INFO 日志——`LogPlugin` 在记录系统信息和显卡型号——然后是刷屏的问候：

```text
2026-06-11T17:00:21Z  INFO bevy_diagnostic::…: SystemInfo { os: "Windows 11 Home China", cpu: "13th Gen Intel(R) Core(TM) i7-13700H", … }
2026-06-11T17:00:28Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 4070 Laptop GPU", backend: Vulkan, … }
Hello, Bevy!
Hello, Bevy!
（每帧一行，停不下来）
```

`Update` 现在真的每帧跑一次——在写作本书的机器上，窗口开着的十来秒里刷了一万两千多行。眼下帧率之所以毫无节制，是因为这个 App 连相机都没有、无画可渲染，循环不受显示器刷新节奏的约束；画面与帧率的节奏控制在第 13、18 章展开。

**点击窗口的关闭按钮，程序退出。**这个行为来自 `WindowPlugin` 的默认退出条件："所有窗口关闭即退出"。

驱动这个循环的是 `WinitPlugin`——Bevy 与操作系统窗口体系的桥梁，基于 Rust 生态的跨平台窗口库 [winit](https://github.com/rust-windowing/winit)。它把 runner 替换成**操作系统的事件循环**：移动窗口、缩放、键鼠输入这些事件由系统推送给程序，程序每帧执行一轮调度并渲染。从此主循环不再是你代码里的某个 `loop`，而是操作系统、winit 与 Bevy 协作的产物——这也是 `run()` 之后的代码不该指望被执行的原因：在一些平台上，事件循环一结束进程就直接终止了。

`DefaultPlugins` 按默认 feature 算有三十多个成员，本书后面的章节会逐个见到它们，这里只需要一个印象：

- **基础设施**：日志、时间、诊断、线程池
- **窗口与输入**：`WindowPlugin`、`WinitPlugin`、`InputPlugin`、无障碍支持
- **资产与场景**：`AssetPlugin`、`ScenePlugin`
- **渲染一族**：渲染器、相机、灯光、Sprite、文本、UI、PBR、glTF、Gizmos
- **其他**：音频、手柄、动画、状态机、拾取

整组随 Cargo feature 裁剪（附录 B 有完整清单）；组内成员可以 `.set()` 定制，也可以 `.disable::<T>()` 禁用。何时用哪个组：做游戏用 `DefaultPlugins`，无头程序用 `MinimalPlugins`。

## 定制窗口

窗口想要自己的标题和尺寸？窗口的一切参数都是数据，装在 `WindowPlugin` 里：

```rust
{{#include ../../code/ch02-first-app/examples/listing-02-05.rs}}
```

<span class="caption">Listing 2-5：通过 .set() 配置 WindowPlugin</span>

`Window` 是个普通结构体：`title` 是标题，`resolution` 是分辨率（单位为物理像素，`(800, 600).into()` 把元组转换过去）。它的字段很多，`..default()` 让其余一切保持默认——这个写法在 Bevy 代码里无处不在，之后不再解释。运行后，窗口标题变成"我的第一个 Bevy 窗口"，尺寸 800×600。

一个此刻可以不在意、但值得记一笔的事实：`Window` 本身是一个 Component，这个窗口就是 World 里的一个实体。在 Bevy 里连窗口都长在那张"表"上——第 35 章会用查询来操控多窗口。

## 写一个自己的 Plugin

Plugin 不是引擎的特权。它是一个 trait，只有一个必须实现的方法：

```rust
{{#include ../../code/ch02-first-app/examples/listing-02-06.rs}}
```

<span class="caption">Listing 2-6：第一个自己的 Plugin</span>

`build` 的参数正是 `&mut App`——你在 `main` 里对 App 做的任何事，都可以原样搬进来。这里登记了两个 System，顺便引出 `Update` 的同伴 **`Startup`**：只在启动时执行一次的 Schedule，初始化逻辑的安身之处。运行：

```text
[Startup] HelloPlugin is on!
[Update] hello again
[Update] hello again
（[Startup] 恰好一行，[Update] 每秒一行）
```

本章开头说 App 是 Plugin 的容器，现在可以补全后半句：**写 Bevy 游戏，就是写一组 Plugin。**角色控制一个插件、计分一个插件、UI 一个插件——第 20 章的打砖块项目就按这个方式组织。社区生态同样如此：物理引擎、地图工具……第三方功能几乎都以 Plugin 形式分发，`add_plugins` 一行接入。顺带一提，`add_plugins` 也接受元组——`add_plugins((MinimalPlugins, HelloPlugin))` 一次装多个。

基础设施齐了。下一节，让屏幕上出现点东西——并让它动起来。

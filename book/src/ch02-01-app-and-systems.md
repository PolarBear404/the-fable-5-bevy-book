# App 与 System

万事从一个空程序开始。本节只前进三小步——空的 App、第一个 System、一个出人意料的运行结果——但每一步都会暴露 Bevy 程序的一块地基。

## 最小的 App

```rust
{{#include ../../code/ch02-first-app/examples/listing-02-01.rs}}
```

<span class="caption">Listing 2-1：能编译运行的最小 Bevy 程序</span>

`bevy::prelude` 汇集了日常所需的几乎所有类型，文件开头 `use bevy::prelude::*;` 是 Bevy 项目的标准动作，本书所有示例都这么写。

运行它：

```console
cargo run -p ch02-first-app --example listing-02-01
```

程序一闪而过，什么都没发生，退出码 0。但这两行代码已经引出了本书最常打交道的类型——**App**：整个 Bevy 程序的容器与装配台。`App::new()` 造出它，随后的链式调用向它登记数据和逻辑，最后 `run()` 把控制权整个交给它。

`App::new()` 给你的也不是全然的空：里面有一个空的 **World**——第 1 章那张"表"的正式名字，所有 Entity 和 Component 都将存放在这里——还有一套调度骨架，规定了一轮更新中各阶段的先后顺序。没有的是：循环、窗口、渲染……稍后你会看到它们从哪来。

## 注册第一个 System

第 1 章说过，System 是"对表的查询 + 处理"。落到代码上，**System 就是一个普通的 Rust 函数**：

```rust
{{#include ../../code/ch02-first-app/examples/listing-02-02.rs}}
```

<span class="caption">Listing 2-2：注册第一个 System</span>

`hello` 没有参数——一个什么都不查询的 System，完全合法。`add_systems(Update, hello)` 把它登记进名为 `Update` 的 **Schedule**（调度——一份"什么时机、按什么顺序跑哪些系统"的清单，第 6 章专门讲）。`Update` 是其中最常用的一个：每轮更新跑一次。注意传的是函数名 `hello` 而不是调用结果 `hello()`——你把函数本身交给 App，何时调用它是引擎的事。

运行，输出：

```text
Hello, Bevy!
```

打印了——但只打印一次，程序就退出了。说好的"每轮更新跑一次"呢？

## 为什么只跑了一次

一个游戏的一生是一个循环：读取输入 → 更新游戏世界 → 渲染画面，每秒重复几十次，每一轮叫一**帧**（frame）。

而 Listing 2-2 里没有任何循环。`run()` 所做的，是把 App 移交给 **runner**（运行器）——一段决定"App 怎么跑"的函数。默认的 runner 极其朴素：把所有调度执行**一遍**，然后返回。于是 `Update` 忠实地执行了"每轮一次"——总共一轮。

这是有意的设计：**主循环不是 App 的内置行为，而是可替换的部件。**无窗口的服务器、被测试框架驱动的测试、由操作系统事件驱动的桌面窗口，需要的循环各不相同。Bevy 把"提供循环"连同窗口、渲染这些能力，全部做成了可插拔的单元——Plugin。下一节就把它们装上。

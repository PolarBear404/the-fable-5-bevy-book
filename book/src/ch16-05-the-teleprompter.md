# 提词器：会长的字

字幕要是一次全亮出来，戏就没了悬念。场记提议上**提词器**：词一个字一个字地蹦上字幕框，像有人在后台现敲。

实现思路在这一章学完的人看来已经平平无奇：`Text2d` 是个普通组件，里面是个普通 `String`——**改它就是了**。配一个资源攥着整句词和节拍：

```rust
{{#include ../../code/ch16-text/examples/listing-16-08.rs:resource}}
```

<span class="caption">Listing 16-8（节选一）：提词器资源——词、进度、节拍各记各的（examples/listing-16-08.rs）</span>

```rust
{{#include ../../code/ch16-text/examples/listing-16-08.rs:type_out}}
```

<span class="caption">Listing 16-8（节选二）：节拍一到，`push` 一个字——就这么多（examples/listing-16-08.rs）</span>

两个细节：

- `script` 存的是 `Vec<char>` 而不是 `String`——中文一个字占三个字节，按字节切会切出半个字（Rust 会 panic），按 `char` 切才是按“字”切。`chars().collect()` 一步到位。
- `Text2d` 实现了 `DerefMut<Target = String>`，所以 `line.push(next)` 直接落在内部字符串上。`Single<&mut Text2d>` 是第 4 章的老朋友。

```console
cargo run -p ch16-text --example listing-16-08
```

```text
场记：提词就位。一拍一个字。
场记：整句递完，19 个字。
```

![动图:字幕框里的台词从无到有逐字浮现,写满一行后自动折到第二行](images/ch16/fig-16-09-typewriter.webp)

<span class="caption">Figure 16-9：提词器开工——注意词长到框边时自动折行，排版始终是新的</span>

值得多看一眼的是**没写的代码**：没有任何“请重新排版”的调用。第 4 章的变更检测在底层接了线——`bevy_text` 注册了一个侦察系统，盯着 `Text2d`、`TextFont`、`TextLayout`、`LineHeight` 和文本实体的子女名单，谁变了就把这块文本标记为“需要重排”，同一帧稍后排版系统照单重算。字符串、字号、颜色、行高——改哪个都是这个待遇，动画文字（比如数字滚动的计分牌）写起来毫无仪式感。

代价同样要心里有数：**每改一次就是一次完整的重排 + 可能的新字形光栅化**。提词器一秒七个字无关痛痒；但要是把一面墙的日志塞进一个 `Text2d` 然后每帧追加，重排的就是整面墙。大段频繁变动的文字，留给第 28 章的 UI 文本（同样的变更检测，但有更合适的布局容器）和第 33 章的诊断工具。

> **窗口出现≠开拍**：你可能注意到示例窗口刚出现的一两秒里提词器纹丝不动，之后才开始打字。这不是 Timer 失灵——首帧之前渲染管线还在编译着色器，Bevy 的 `Time` 从第一次更新才开始走表。所有按 `Time` 计时的逻辑都以“开拍”为零点，和窗口何时出现无关。

提词器管的是“字的多少”。下一节拆“字的妆容”——同一句词里，角色名、台词、舞台提示各穿各的行头。

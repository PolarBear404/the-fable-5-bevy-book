# 自记一笔

内置的账再全，也记不了你游戏里的数：台上还剩几片瓦、这一波刷了多少怪、寻路队列积压多长。好在账本机制全程开放——立账、记账、念账，三步都是公开 API。

实验道具选个热闹的：**撒纸屑**。空格一把撒 80 片，各自往下飘、寿数一到自行退场——台上纸屑的存量于是成了一个有涨有落、有尖峰有衰减的活数值，正好喂账本。

## 立账

```rust
{{#include ../../code/ch27-dev-tools/examples/listing-27-10.rs:register}}
```

<span class="caption">Listing 27-10（其一）：先立账——名目、单位后缀、史册长度（examples/listing-27-10.rs）</span>

- **`DiagnosticPath`** 是账目的名目：一个 `/` 分层的字符串（`"stage/confetti"`），要求全局唯一、非空、首尾无 `/`。`const_new` 是 `const fn`，所以名目可以像 Listing 里这样做成常量——立账、记账、念账三处引同一个常量，拼写错误在编译期就死了；分层前缀（`stage/`）纯粹是给人看的整理习惯，内置账的 `system/`、`process/` 同理；
- **`register_diagnostic`**（`RegisterDiagnostic` trait 加在 `App` 上的方法）把一本新账送进仓库。账本本体用 builder 现配：`Diagnostic::new(名目)` 打底，`.with_suffix(" 片")` 配单位（念账时缀在数值后），`.with_max_history_length(60)` 把史册从默认 120 裁到 60——纸屑存量变化快，太长的均值窗口只会钝化读数。还有一个 `.with_smoothing_factor()` 拨 EMA 的时间常数，默认值配合逐帧测量工作良好，一般不动；
- **先立账，后记账**——这是条纪律：往没立过的账上记数，测量会被静默丢弃（仓库查无此账）。下一节的小窗会把这种失误变成看得见的 `Missing`。

## 记账

```rust
{{#include ../../code/ch27-dev-tools/examples/listing-27-10.rs:measure}}
```

<span class="caption">Listing 27-10（其二）：记账系统与合账开关（examples/listing-27-10.rs）</span>

记账走系统参数 **`Diagnostics`**（注意复数，别跟单数的账本体 `Diagnostic` 混了）：`add_measurement(&名目, || 数值)` 添一笔测量，时间戳自动盖。两个设计值得停一拍：

- **数值是闭包，不是现成的数**。`|| confetti.iter().count() as f64` 只在这本账**开着**的时候才被调用——账合上了，连数都不数。对我们这个 `count()` 无所谓，但真实项目里的测量可能很贵（遍历一张大图、算一段统计），懒求值让“记账基础设施常驻、按需付费”成立；
- **`Diagnostics` 是延迟写入**（又是 `Commands` 同款机制）：测量攒在本系统的缓冲里，同步点统一入账。所以记账系统可以随便并行，不会为一本账打架。

合账开关则走仓库 `DiagnosticsStore`——直接改账本体的 `is_enabled` 字段。E 键一合一开，效果马上看。

## 念账实测

```console
cargo run -p ch27-dev-tools --example listing-27-10
```

撒两把、合一次账，终端里这本账的起伏全程在册（`LogDiagnosticsPlugin` 照常一秒一轮，`started_load_count` 那本引擎账省略）：

```text
INFO bevy_diagnostic: stage/confetti    :    0.000000 片 (avg 0.000000 片)
INFO bevy_diagnostic: stage/confetti    :   79.999056 片 (avg 78.666667 片)
INFO bevy_diagnostic: stage/confetti    :  129.655316 片 (avg 100.000000 片)
INFO bevy_diagnostic: stage/confetti    :  112.191214 片 (avg 121.866667 片)
INFO bevy_diagnostic: stage/confetti    :   22.191975 片 (avg 72.533333 片)
INFO bevy_diagnostic: stage/confetti    :    0.735694 片 (avg 11.733333 片)
```

对着台账读四件事：

1. **第一把撒完读数 `79.999` 而不是 80**——EMA 从 0 爬向 80 的最后一丝没爬完。平滑读数的“滞后”在台阶跳变上看得最清；
2. **第二把撒完瞬时存量是 160，念出来却是 `129.66`**——还是 EMA 在追。想看裸值？那是 `value()` 的口径，下一节的小窗里可以点单；
3. **中间断了两轮**——那是 E 键合账的两秒：`is_enabled = false` 时播报员直接跳过这本账（整行消失，不是打零），测量也一笔没记；
4. **重开账直接读到 `22.19`**——合账期间纸屑照常飘落凋零（游戏逻辑不受账本影响），账一开，读数直接续上现实。

单位后缀 `片` 在终端里安然无恙——**但请记住这个中文后缀**，本章收场它会在另一个场地（屏幕小窗）以豆腐块的形态再次登场，到时你就知道后缀该什么时候用中文、什么时候老实用 ASCII。

> 立账还有一条不显眼的通路：诊断这套机制**不依赖任何渲染**。`MinimalPlugins` 的无头程序（服务器、工具链）一样能立账念账——账本仓库只要 `DiagnosticsPlugin`，播报只要日志。第 33 章讲日志与远程调试时，还会见到让外部工具经 BRP 协议远程翻看世界状态的手法。

账立好了，念账的还是终端。下一站给它们一个体面的家：屏幕上的水牌和小窗。

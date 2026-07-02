# 开机进度条

一件一件等货，戏没法拍。真实游戏的做法是把一关用到的素材**整单装货**：列清单、全部开单、看着到货数爬升、全齐了才进关卡——玩家看到的就是那根进度条。这一节用已有的零件把它拼出来：清单是 `Vec` 里的一把 Handle，进度是“数到货”，切换是第 10 章的 States。

## 清单、状态与行头

```rust
{{#include ../../code/ch14-assets/examples/listing-14-06.rs:states}}
```

<span class="caption">Listing 14-6（节选一）：两段式开机——Loading 装货，Rolling 开拍（examples/listing-14-06.rs）</span>

清单里装的是 `UntypedHandle`——抹掉了类型的提货单。普通的 `Handle<Image>` 只能提图片，而一份真实的资产清单什么都有：图片、音频、剧本。`.untyped()` 把各色单子统一成一种，塞进同一个 `Vec`；它照样参与引用计数，**清单本身就是持单人**，装货期间谁也不会被回收。

```rust
{{#include ../../code/ch14-assets/examples/listing-14-06.rs:start}}
```

<span class="caption">Listing 14-6（节选二）：七件家当一次开单；进度条只活在 Loading 状态（examples/listing-14-06.rs）</span>

进度条就是两个色块 Sprite：深色底槽加金色填充——第 2 章的老手艺。两件行头都挂着 `DespawnOnExit(SetupPhase::Loading)`（第 10 章）：状态一切走，加载画面自动清场。

## 数到货

```rust
{{#include ../../code/ch14-assets/examples/listing-14-06.rs:track}}
```

<span class="caption">Listing 14-6（节选三）：清点、推进度条、全齐切状态（examples/listing-14-06.rs）</span>

核心一行是那个 `filter`：`is_loaded_with_dependencies(handle.id())` 对清单逐件发问“连配件全到了吗”，`count` 出到货数。进度条的几何是第 12 章的算术——金条宽度按比例伸长，同时把中心右移半个增量，让它看起来**从左端往右长**，而不是从中心向两边胀。

全齐的那帧把 `NextState` 设成 `Rolling`，装台系统在 `OnEnter(Rolling)` 里搭场：

```rust
{{#include ../../code/ch14-assets/examples/listing-14-06.rs:build}}
```

<span class="caption">Listing 14-6（节选四）：装台——此刻 load 同一路径，只是取回已在架的货（examples/listing-14-06.rs）</span>

注意装台系统取资产的姿势：**直接再 `load` 一遍同样的路径**。14.2 节的规矩在这里兑现——同路径同单，这几个 `load` 一两微秒就返回现成的 Handle，一个字节的磁盘都不碰。清单负责保活，用货处随取随用，两边不用传 Handle，只要约好路径。

```console
cargo run -p ch14-assets --example listing-14-06
```

```text
老顾：《长风渡》整本戏的家当，7 件，单子全开出去了。
老顾：（第 1 帧）到货 3/7。
老顾：（第 2 帧）到货 6/7。
老顾：（第 3 帧）到货 7/7。
老雷：全齐了？——装台，开机！
场务：夜渡幕布挂好，三件道具各就各位。
```

到货数分三帧爬完：第 1 帧到 3 件，第 2 帧 6 件，第 3 帧全齐——小图解码快、大图慢，货就是这样零零散散地进门（各帧的具体数字在你机器上会不同）。机制全部成立——只是这台机器装得太快，进度条在屏幕上一闪而过。真实项目里几百件资产、几百 MB 的体量，这根条要走上好几秒；本章总装（14.9 节）会给它加一道“最短亮相时间”的闸，这是真实游戏处理“加载太快”的标准手法——是的，加载条的烦恼有两个方向。

还有一笔账记在 14.3 节欠条上：万一清单里有件货 `Failed`，`is_loaded_with_dependencies` 永远是 `false`，进度条卡在 6/7 死等——这正是练习 1 要你补的洞。

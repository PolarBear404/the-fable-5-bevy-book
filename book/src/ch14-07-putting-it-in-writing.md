# 白纸黑字：把资产存回磁盘

上一节的方向是“纸面到台上”：秋白改文件，引擎把新词搬进内存。这一节把箭头倒过来。连排到半夜，老雷当场拍板改了幕名和末句——改动发生在**台上**，也就是货架上的那份 `Script`。可货架是内存，一关机就烟消云散；定稿要作数，就得落回纸面。到目前为止，库房只会进货：`load` 读盘、装载器解析，全是“盘到内存”的单行道。出货口在哪？

答案是一对搭档：**`AssetSaver`**（存档器——`AssetLoader` 的镜像，负责把资产写成字节）和 **`save_using_saver`**（库房的出货流程，负责把这些字节送到磁盘上的指定路径）。程序化生成的关卡、游戏内置的关卡编辑器、烘焙好的成品数据，走的都是这道门。

## 存档器：装载器的镜像

先看存档器本体。给 14.5 节的 `.script` 格式配上另一半：

```rust
{{#include ../../code/ch14-assets/examples/listing-14-09.rs:saver}}
```

<span class="caption">Listing 14-9（节选一）：ScriptSaver——逆着装载器的解析规则，把 Script 写回字节（examples/listing-14-09.rs）</span>

和 `AssetLoader` 逐项对照，处处都是镜像：

- `load` 收 `reader` 读字节，`save` 收 `writer` 写字节——同样是 `async fn`，同样跑在后台 IO 任务里；
- 装载器声明“我产出什么资产”，存档器声明“我保存什么资产”；
- 最有意思的是 **`OutputLoader`** 这个关联类型：它指名“存出去的文件由哪个装载器读回来”。这是格式契约的另一半——`save` 写出的字节，必须是 `OutputLoader` 能解析的格式。我们逆着 `ScriptLoader` 的规则拼行：`幕名：` 一行，台词一行一句，读写闭环由类型系统白纸黑字地记录在案；
- `save` 成功时返回的不是 `()` 的空胜利，而是 **`OutputLoader` 的设置**（`ScriptLoader` 的 `Settings` 恰好是 `()`）。这份设置会被写进存档旁边的 `.meta` 档案，告诉未来的读取方“用哪个装载器、什么设置”——档案是什么，下一节正好细说；
- 错误类型这回直接借 `std::io::Error`：写盘的错基本都是 IO 错，不像解析要分三类。

## 导演口述，后台执笔

存档器只是“会写”，还得有人喊它写。先备好三件家什，再看动手的系统：

```rust
{{#include ../../code/ch14-assets/examples/listing-14-09.rs:plumbing}}
```

<span class="caption">Listing 14-9（节选二）：三件家什——一稿的单子、存盘差事的回执、读回来的定稿（examples/listing-14-09.rs）</span>

```rust
{{#include ../../code/ch14-assets/examples/listing-14-09.rs:finalize}}
```

<span class="caption">Listing 14-9（节选三）：改货架上的本体，誊一份带进后台任务存盘（examples/listing-14-09.rs）</span>

前半段是 14.2 节承诺过的 `get_mut` 实战：改的是**货架上的资产本体**，所有持这张单的地方立刻见到新词（`Modified` 广播也会照章响一声）。后半段是出货，逐个参数过一遍 `save_using_saver`：

- **`AssetServer` 的克隆**——`save_using_saver` 要拿它找到目标来源（默认来源就是 `assets/` 文件夹）并借用写出口；`AssetServer` 本身是个轻量把手，克隆不复制库房；
- **存档器实例**与**目标路径**——路径和 `load` 同一套写法，相对 `assets/`，不带前缀；
- **`SavedAsset::from_asset(&fair_copy)`**——把待存资产打包成存档单。注意它**只借不夺**：包的是引用。而这个任务要在后台活到写盘结束，借世界里的数据活不了那么久，所以先 `clone` 出一份誊本，连誊本一起搬进 `async move` 块；
- 设置传 `&()`——`ScriptSaver` 没有可调选项。

整个调用是 `async` 的，交给 **`IoTaskPool`**（引擎自带的 IO 任务线程池，装载器的 `load` 也跑在这里）去执行。`spawn` 返回一张 `Task`——后台差事的**回执**，塞进 `SaveJob` 这个 Resource。磁盘照旧很慢、游戏照旧很快：写盘期间主循环一帧不停，这正是 14.1 节那条铁律的出货版。

## 回执与验货

回执在手，每帧问一声写完了没：

```rust
{{#include ../../code/ch14-assets/examples/listing-14-09.rs:await_save}}
```

<span class="caption">Listing 14-9（节选四）：check_ready 非阻塞查问回执；写完就把定稿读回来（examples/listing-14-09.rs）</span>

```rust
{{#include ../../code/ch14-assets/examples/listing-14-09.rs:proof}}
```

<span class="caption">Listing 14-9（节选五）：验货——盘上读回来的定稿，逐字对账（examples/listing-14-09.rs）</span>

`check_ready(&mut task)` 把任务戳一下：没完就 `None`，下一帧再来；完了就交出结果——**不阻塞、不等待**，和 14.3 节轮询状态牌是同一个姿势，这回轮询的是写而不是读。`Result` 两个分支都有人管：磁盘满、目录没权限，错误会老老实实带在回执里，这是 14.3 节“失败分支必须有人管”的又一次点名。若是完全不关心成败，`spawn` 之后调 `.detach()` 放手也行——但存盘这种事，建议留着回执。

存好之后的最后一步是本节的题眼：**用我们自己的装载器，把我们自己存的文件读回来**。`OutputLoader` 的契约在这里兑现——出货口写的格式，进货口原样认账。跑一遍看全程：

```console
cargo run -p ch14-assets --example listing-14-09
```

```text
老雷：把一稿拿来，今天当场定稿。
老雷：幕名与末句就这么改。定了，白纸黑字存下来！
场务：存好了——scripts/opening-final.script，旁边还多了一份 .meta。
场务：读回来了——《渡口夜话·定稿》，5 句词。
场务：末句是“阿燕：他会来。掌灯，今夜把戏排完。”，一字不差。
老雷：白纸黑字，收工。明天开机就用这份。
```

打开 `assets/scripts/`，多出来的 `opening-final.script` 里躺着定稿：

```text
幕名：渡口夜话·定稿

阿燕：二十年了，这把剑还认得回家的路。
梢公：客官，夜里风大，进舱吧。
阿燕：不了。我在等一个人。
梢公：那位贵客，怕是不会来喽。
阿燕：他会来。掌灯，今夜把戏排完。
```

对照一稿会发现开头的 `#` 批注没了。这不是丢失，是本分：存档器写的是**资产的内存形态**，而批注在 14.5 节就被装载器跳过、压根没进过 `Script`。一对 loader/saver 能保真到什么程度，取决于内存形态记下了什么——想让批注在往返中幸存，就得让 `Script` 结构收留它们。

旁边那份 `opening-final.script.meta` 是 `save_using_saver` 顺手写的档案：

```text
(
    meta_format_version: "1.0",
    asset: Load(
        loader: "listing_14_09::ScriptLoader",
        settings: (),
    ),
)
```

装载器的名字、`save` 返回的那份设置，全记在案——将来无论谁来 `load` 这个文件，库房翻开档案就知道怎么读。`.meta` 的完整故事，下一节展开。

> 两个知道就好的延伸：其一，引擎也自带存档器，比如把 `Image` 按扩展名写成 PNG 等格式的 `ImageSaver`——官方的 `asset_saving` 示例用它做了块可以 F5 存盘的小画板；其二，有的资产是“一文件多货”——主资产之下还挂着一串带标签的子资产（第 23 章的 glTF 就是典型），保存这种资产要用 `SavedAssetBuilder` 逐件登记子资产再打包，官方 `asset_saving_with_subassets` 示例是现成的样板，本书不展开。

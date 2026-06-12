# 资源的有无

组件没挂在某个实体上，查询自动跳过那一行，天经地义。资源不一样：`Res<Score>` 不是“筛选”，而是点名要——World 里没有这份资源时，引擎该怎么办？

## 缺席即事故

把 Listing 5-1 的 `insert_resource` 那行删掉：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-03.rs:main}}
```

<span class="caption">Listing 5-3：忘了注册资源——首帧 panic</span>

```console
cargo run -p ch05-resources --example listing-05-03
```

```text
Encountered an error in system `listing_05_03::announce`:
Parameter `Res<'_, Score>` failed validation: Resource does not exist
If this is an expected state, wrap the parameter in `Option<T>` and handle
`None` when it happens, or wrap the parameter in `If<T>` to skip the system
when it happens.
```

对比一下第 4 章的 `Single`：匹配数不对时它**静默跳过**系统，因为“本帧恰好没有目标”是游戏里的正常状态。`Res` 的态度截然相反——资源不存在被引擎视为**逻辑 bug**：你声明“必须有”，它就当真，缺了直接 panic 把问题钉在第一帧，而不是放任系统带病空转。

但报错信息自己也承认有例外：如果“时有时无”恰恰是设计的一部分呢？它给了两条出路：

- **`Option<Res<T>>`**：资源在就是 `Some`，不在就是 `None`，系统照常运行，由你分支处理；
- **`If<Res<T>>`**：资源不在时整个系统跳过这一帧——把 `Res` 的“必须有”软化成 `Single` 式的“没有就歇着”。

两者的分野在于**系统剩下的活还干不干**：还有别的事要做，用 `Option` 自己分支；这个系统离了它就没意义，用 `If` 整个让开。

## 存在性即信息：双倍卡

“时有时无”不只是要容忍的麻烦，它可以是主动的设计。打靶场的摊主会发**双倍得分卡**——卡在手里，下一枪翻倍；卡收回去，恢复原价。这张卡用一个不带任何数据的资源表示：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-04.rs:card}}
```

里面什么都没有——它的全部信息就是“在不在场”，像第 3 章的标记组件（`Wounded`）一样，只不过标记的不是某个实体，而是**整个游戏的状态**。发卡和收卡由摊主负责：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-04.rs:systems}}
```

<span class="caption">Listing 5-4（节选）：Option 探测有无，Commands 运行期插拔</span>

这里出现了资源的第三组操作入口：**`Commands` 也能 `insert_resource` / `remove_resource`**。App 上的同名方法是构建期一次性的；`Commands` 版本给了系统在**运行期**增删资源的能力——和第 3 章生成、销毁实体一样走命令队列，排队等同步点。资源的增删同样是“改 World 结构”的操作，第 3 章末尾那张表于是有了资源版：

| 操作 | 走哪条路 | 何时生效 |
|---|---|---|
| 读资源 | `Res<T>` | 即时 |
| 改资源的值 | `ResMut<T>` | 即时 |
| 增删资源 | `Commands` | 下一个同步点 |

运行：

```console
cargo run -p ch05-resources --example listing-05-04
```

```text
砰！+10 分（累计 10）
摊主：这张双倍卡送你，下一枪生效！
砰！+20 分（累计 30）
摊主：双倍卡到期，收回了。
砰！+10 分（累计 40）
```

逐帧对账。第 1 枪原价；摊主随后发卡——`shoot` 排在 `stall_keeper` 前面，发卡命令在本帧末尾的清算中才落地（第 3 章的规则：**一个调度跑完，攒下的命令必然已全部应用**），所以第 2 枪才吃到双倍。同理，第 2 枪后收卡的命令也是帧末生效，第 3 枪回到原价。`shoot` 全程不用改一行逻辑，`card.is_some()` 一问便知行情。

> 摊主的“下一枪生效”在系统顺序上是精确的：如果把 `stall_keeper` 排在 `shoot` **前面**，调度器会在两者之间插入同步点（第 3 章见过这一幕），发卡当枪就生效。命令应用时机的完整规则在第 6 章。

这个“以资源的存在与否表达全局开关”的模式在 Bevy 里随处可见——成就解锁、Boss 战进行中、调试模式开关。等到第 10 章你会见到它的工程化升级版：State。

资源的“有”与“无”都讲完了，还剩一个更早的问题：第一份值从哪来？`insert_resource` 要你亲手递一个值进去，但很多资源有显而易见的默认值，还有些资源的初始值要看别的资源脸色。下一节交给 `init_resource`。

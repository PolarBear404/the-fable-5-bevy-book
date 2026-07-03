# 打包参数：SystemState、SystemParam 与 QueryData

这一节解决三个“写着费劲”，工具各一件。三件的思路相同：**把常用的形状起成名字**。

## SystemState：在独占系统里开柜台

独占系统什么都能干，可前两节的代码你也看见了——`world.query_filtered::<(), With<Notice>>()` 这种长串每写一次都费劲，柜台时代一个 `Query` 参数的事。`SystemState` 把柜台搬进独占系统：在类型参数里写下你想要的**那组系统参数**，它替你按单配货。它本身就在独占系统参数的白名单上（11-1 节欠的名单），作为第二参数传 `&mut SystemState<...>` 即可：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-08.rs:census}}
```

<span class="caption">Listing 11-8：SystemState——独占系统里借出 Query 和 Commands</span>

`get_mut(world)` 借出参数包，解构出来的 `residents` 和 `commands` 与普通系统里的同名参数用法**完全一致**。留意它给的是 `Result`：借出之前有一道参数校验——普通系统跑不起来时替你把关的正是同一道（第 4 章 `Single` 的“不多不少”、缺货资源的报错，都出自它）——校验不过给 `Err`。名册查询加 `Commands` 这种组合不会失败，`unwrap` 即可；柜台里坐着 `Single` 这类挑剔参数时，这个 `Result` 要认真接。另有一处必须自己上心，运行：

```console
cargo run -p ch11-deep-ecs --example listing-11-08
```

```text
—— 第 1 帧 ——
  巡逻队：住户 3 人，公告牌 0 块。
—— 第 2 帧 ——
  巡逻队：住户 3 人，公告牌 0 块。
  艾达点名：罗兰、老蔫儿、杂货铺老板。
  公告牌：apply 之前 0 块，之后 1 块。
—— 第 3 帧 ——
  巡逻队：住户 3 人，公告牌 1 块。
```

“之前 0 块，之后 1 块”——**`SystemState` 借出的 `Commands` 依旧是缓冲的**，第 3 章的延迟语义原样保留；区别在于没有调度器替你清算，结账要自己来：`counter.apply(world)`。忘了 apply，命令不会丢，但会一直窝在 `SystemState` 里等下一次 apply——排查起来相当折磨人，养成“借了就结”的习惯。

为什么参数要写成 `&mut SystemState` 而不是每次现 `new` 一个？和 `Query` 参数背后缓存 `QueryState` 同理：`SystemState::new` 要登记访问、建查询缓存，每帧重建是纯浪费。挂在参数上，引擎就替你把它在两次运行之间保管好——和 `Local<T>` 的保管机制是同一套。顺带把第 4 章的一句旧话补全：`Local<T>` 的初始值“准确地说来自 `FromWorld`”——第 5 章资源那套 `FromWorld` 协议，对 `Local` 同样生效，默认实现仍是 `Default`。

## #[derive(SystemParam)]：参数打包

第二个费劲发生在普通系统：同一组参数——名册查询加镇库资源——艾达的每件公务都要原样抄一遍，签名越来越长。打包：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-09.rs:desk}}
```

<span class="caption">Listing 11-9（其一）：derive(SystemParam)——几个参数拼成一个新参数</span>

规则三条：结构体带 `<'w, 's>` 两个生命周期槽（world 借用和系统状态借用，照抄即可）；字段类型就是平时写在签名里的参数类型，`Query` 字段把两个生命周期填进去、数据位写 `&'static`（这个 `'static` 是占位写法，下一小节细说）；可以给它 `impl` 方法，把操作和数据捆在一起。用的时候，它就是一个普通参数：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-09.rs:seasons}}
```

<span class="caption">Listing 11-9（其二）：春盘秋盘，同一只柜台</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-09
```

```text
春盘：3 户造册，镇库现银 103 枚。
秋盘：3 户造册，镇库现银 106 枚。
```

并行调度毫不受影响——调度器看的是包里每个字段的访问集合，打包只是给人看的语法糖。第 4 章参数表的最后一行“自定义参数”，到此兑现。

## #[derive(QueryData)]：给表格的一行起名字

第三个费劲在查询本身：盘点表一行五列，元组写法 `(Entity, &Name, &Stock, Option<&Lodger>, Has<Registered>)` 解构起来全靠数位置，加一列全身动。同样的药方，打包成结构体：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-10.rs:row}}
```

<span class="caption">Listing 11-10（其一）：derive(QueryData)——一行一个结构体</span>

现在把 `&'static` 说清楚：QueryData 结构体里的引用字段**统一写 `&'static T`**，这是 derive 宏规定的占位拼法——它只声明“这一列是 `T` 的引用”，实际借用的生命周期由查询临场决定，绝不是真的 `'static`。`Option` 列、`Has` 列照常嵌进来。使用处按字段名取数：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-10.rs:read}}
```

<span class="caption">Listing 11-10（其二）：row.name、row.stock——不再数元组位置</span>

要写的列，给结构体标 `mutable`，字段写 `&'static mut`：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-10.rs:tax_row}}
```

<span class="caption">Listing 11-10（其三）：query_data(mutable)——可写的一行</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-10
```

```text
盘点表：
  15v0 杂货铺老板：存粮 40 袋（未盖章）
  14v0 老蔫儿：存粮 7 袋（已盖章）
  13v0 罗兰：存粮 3 袋，借宿于杂货铺老板家（未盖章）
收税：杂货铺老板缴 1 袋，余 39 袋
收税：老蔫儿缴 1 袋，余 6 袋
收税：罗兰缴 1 袋，余 2 袋
```

迭代拿到的不是 `TaxRow` 本身，而是宏生成的条目类型（`TaxRowItem`），可写字段在里面是 `Mut<Stock>`——所以 `row.stock.0 -= 1` 照常触发变更检测。标了 `mutable` 的结构体还附赠一个只读版本（`TaxRowReadOnly`），给只读借用时自动用上，不必自己写两份。

变更检测——上一段刚冒出来的词。`Mut` 是怎么知道你写没写的？`Changed` 过滤器的“自上次以来”到底量到哪根刻度？第 4 章欠的“更多机关”，下一节全部拆开。

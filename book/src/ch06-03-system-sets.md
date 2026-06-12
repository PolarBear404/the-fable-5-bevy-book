# SystemSet：成组排序

矿场要扩编了：备料的不止矿工，还有樵夫；结算的不止铸币机，还有账房。如果靠 `before`/`after` 两两点名，系统每多一个，约束就多一串——而且点名要求你**知道对方是谁**，跨 Plugin 的系统之间连这一点都做不到。

**SystemSet**（系统集合）解决的就是这个问题：先定义几个有名字的"工序"，把工序之间的顺序一次性排好；之后每个系统只声明"我属于哪道工序"，不点任何同事的名。

集合本身是个轻量的标签类型，派生一个 trait 就能用：

```rust
{{#include ../../code/ch06-schedules/examples/listing-06-06.rs:set}}
```

`derive(SystemSet)` 之外的那串 trait（`Debug`、`Clone`、`PartialEq`、`Eq`、`Hash`）是集合作为"可比较的标签"的硬性要求，照抄即可。组装：

```rust
{{#include ../../code/ch06-schedules/examples/listing-06-06.rs:main}}
```

<span class="caption">Listing 6-6：configure_sets 定工序，in_set 入伙</span>

两个新 API 各司其职：

- **`configure_sets(Update, ...)`**：配置集合本身。集合也接受 `.chain()`、`.before()`、`.after()`——排序语法对系统和集合一视同仁，这里把三道工序串成 `Produce` → `Process` → `Settle`。
- **`.in_set(...)`**：把系统挂进集合，集合的所有顺序约束自动落到它头上。挂接可以发生在任何注册点——三次 `add_systems` 互不相识，靠集合对上了暗号。

备料工序里有两个系统，故意不给它们排序：

```rust
{{#include ../../code/ch06-schedules/examples/listing-06-06.rs:produce}}
```

```console
cargo run -p ch06-schedules --example listing-06-06
```

```text
冶炼炉：出锭 1 根
铸币机：+1 金币
账房：金库共 1 枚金币
冶炼炉：出锭 1 根
铸币机：+1 金币
账房：金库共 2 枚金币
```

输出按工序走，分毫不乱。几个观察：

- **同一集合内部不排序**。矿工和樵夫之间没有任何约束——他们一个管矿一个管柴，访问不冲突，调度器可以让他们并行。集合划的是"阶段"的界，不是阶段内的队形。
- **集合约束与系统约束随意混用。**`Settle` 内部，账房用 `.after(mint_coins)` 加了一条细粒度约束；粗排靠集合、细排靠点名，两层叠加。
- **`configure_sets` 与 `add_systems` 谁先谁后无所谓**。集合是调度图上的节点，先挂系统后配顺序也一样。

集合的真正威力在工程组织：Plugin A 把自己的系统全挂进 `MintStage::Produce`，Plugin B 挂进 `MintStage::Settle`，两个 Plugin 的作者互不相识，顺序却由集合的一行 `configure_sets` 统一裁定。Bevy 引擎和第三方插件也都公开自己的集合（比如 `bevy_transform` 的 `TransformSystems`），供你的系统 `before`/`after`——后面章节用到时会一一指出。

顺序问题到此收齐。但调度还管着另一件事：有些系统**这一帧根本不该跑**。下一节给系统装开关。

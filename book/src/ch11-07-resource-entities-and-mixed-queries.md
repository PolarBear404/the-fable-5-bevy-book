# 资源实体与混合查询

本章一路数下来，账面一直有笔亏空：11-1 节艾达清点出 16 个实体，镇上明明只有 3 户，连沙盘上的 0 号门牌都早有了主；11-2 节预检官的合计是 17；11-3 节的空世界凭空多出一册档案。多出来的这批住户是谁，第 5 章其实已经交过底——**资源住在实体表里**，每个资源类型一行专属实体，值是组件，随行一件 `IsResource` 标记，`Res` 是“到那一行取组件”的语法糖。当时按下没表的是最后一句话：让资源实体和普通实体**同场接受查询**的玩法，留给第 11 章。现在人齐了、工具也齐了（广查询、`inspect_entity`、observer），把这笔账当场盘清。

## 全场点名：一个查询，两类实体

第 5 章的点名只报了名字，这次艾达连每行放了几件东西都要记。查询里什么组件都不点名（`Option` 不缩小匹配范围），World 里每一行都得到场——这就是**混合查询**：普通实体和资源实体走同一条流水线过审。

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-14.rs:roll_call}}
```

<span class="caption">Listing 11-14（其一）：混合查询——住户与资源实体同场过审</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-14
```

```text
艾达翻开全镇的账（count_spawned = 17）：
  0v0  资源  DefaultQueryFilters（2 件）
  1v0  资源  Schedules（2 件）
  2v0  资源  AppTypeRegistry（2 件）
  3v0  资源  MainScheduleOrder（1 件）
  4v0  资源  FixedMainScheduleOrder（2 件）
  5v0  资源  Messages<AppExit>（2 件）
  6v0  资源  MessageRegistry（2 件）
  7v0  资源  FrameCount（2 件）
  8v0  资源  Time（2 件）
  9v0  资源  Time<Real>（2 件）
  10v0  资源  Time<Virtual>（2 件）
  11v0  资源  Time<Fixed>（2 件）
  12v0  资源  TimeUpdateStrategy（2 件）
  13v0  资源  TownFunds（2 件）
  14v0  住户  罗兰（3 件）
  15v0  住户  老蔫儿（3 件）
  16v0  住户  杂货铺老板（3 件）
内账 14 行，民册 3 行；存粮共 50 袋，镇库 73 枚（Res 照常直达）。
```

本章的亏空一次结清。`MinimalPlugins` 的开机家底是 13 份资源：第 5 章见过的 7 份零插件家底，再加计帧的 `FrameCount`、`Time` 一家四口和 `TimeUpdateStrategy`；11-1 节的 16 = 13 + 3 户，本例和 11-2 节都再多一份你自己的 `TownFunds`，所以是 17。行号 0 到 13 按注册顺序落座，三户从 14v0 起步顺位领号——这批数字取决于装了哪些插件，别把它们写死在逻辑里。0 号门牌的主人也验明了：`DefaultQueryFilters`，上一节那份隐身名单——它是每个 World 出生自带的第一份资源，连 11-1 节不挂任何插件的裸沙盘也不例外，所以罗兰只能屈居 1v0。

再看每行的件数。资源实体清一色“2 件”：资源组件本身加 `IsResource`，与第 5 章的模型分毫不差——**只有一行例外**。

## 现场抓获：只剩 1 件的 MainScheduleOrder

`3v0 MainScheduleOrder（1 件）`——它的资源组件不见了，行上只剩孤零零的 `IsResource`。没人偷账本：此刻正是 `Main` 调度拿着 `MainScheduleOrder`（一份记录“调度按什么顺序跑”的资源）在安排你这个系统运行，而它借用的方式，就是 11-1 节的 `resource_scope`。那一节说 `resource_scope` 把资源“暂时摘下来”，这不是修辞：它对资源实体调的正是 11-2 节没收私酿酒的 `take::<R>()`——组件真的离开了那一行，闭包跑完再放回去。艾达在盘点时抓到引擎现行：只要 `Main` 调度在跑——你的每个系统都是它安排的——`MainScheduleOrder` 就处于“被摘走”状态，混合查询看到的就是这半空的一行。

这也顺手解释了一桩怪事：在普通系统里写 `Res<MainScheduleOrder>`，运行时报的是 “Resource does not exist”——明明它就在那儿。组件不在行上，按类型直达的路自然扑空，`Option<Res<...>>` 拿到的也是 `None`。引擎内务不劳你操心，但当你的检查器、序列化工具扫出一行“只有 `IsResource`”的实体时，别当成坏档案——多半是谁的 `resource_scope` 正开着。

## 划界：With、Without 与用不着划的

盘完总账要分册。`IsResource` 就是为划界准备的标记，往过滤器槽一放：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-14.rs:tally}}
```

<span class="caption">Listing 11-14（其二）：With 圈内、Without 圈外——只有广查询需要这一刀</span>

三个查询三种姿势，规则一句话：**广查询才需要划界，窄查询天生不沾内账**。`granary` 点名了 `Stock`，资源实体身上没有这件组件，对不上号就进不了名单——第 3 章到第 10 章写过的每一个查询都是这么天然免疫的，这也是资源搬进实体表而你毫无察觉的原因。要当心的只有 `Query<Entity>`、`Query<EntityRef>`、全 `Option` 这类什么都不点名的形状：清点数目、遍历全场、写检查器的时候，先想一想“内账要不要算进来”，不要就补一刀 `Without<IsResource>`。

11-2 节 B0002 报错里那句药方现在也全通了：`&World`、`Query<EntityRef>` 声明的“读一切组件”把资源组件也包了进去，才会和 `ResMut` 撞车；`Without<IsResource>` 把访问集合收窄成“一切组件，资源除外”，和任何资源参数都不再重叠——所以报错文案把它和 `ParamSet` 并列开在处方上。

还剩一个对照要摆正。上一节刚讲过 `Disabled`，同是“查询名单之外”，两者的机制完全相反：

| | `Disabled` | `IsResource` |
|---|---|---|
| 默认查询看得见吗 | 看不见——`DefaultQueryFilters` 给每个查询自动补 `Without<Disabled>` | 广查询看得见；窄查询只是对不上号 |
| 想看见 | 查询里显式提到它 | 本来就在场 |
| 想排除 | 默认已排除 | 手动 `Without<IsResource>` |

`IsResource` 不在 `DefaultQueryFilters` 的名单上。这是有意的：`Disabled` 的语义是“游戏层面暂时不存在”，藏起来是本分；而资源实体是引擎的正式户口，存档、检查器、网络同步这类工具恰恰指着“实体加组件就是全部数据”这条统一模型干活，默认藏起来反而坏事。最后一条纪律：资源实体的行归引擎管，**不要 despawn 它**——引擎会在日志里警告你，而那份资源也就没了。

## 资源上的 observer

资源既然是组件，第 8 章的那套即时响应就原样适用——生命周期事件、observer、钩子，一件不少。给镇库上个警报：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-15.rs:alarm}}
```

<span class="caption">Listing 11-15（其一）：观察资源的 Insert——警报挂在类型上，响铃时递来资源实体</span>

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-15.rs:script}}
```

<span class="caption">Listing 11-15（其二）：改值与入库，惊动的不是同一套机关</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-15
```

```text
  警报：镇库进账！现银 73 枚（响铃的是 14v0）
艾达收了 2 枚人头税，记在账上。（ResMut 改值，警报不响）
商队巨款入库！（insert_resource 覆盖写入）
  警报：镇库进账！现银 500 枚（响铃的是 14v0）
```

对账三条。其一，`insert_resource` 就是往资源实体上 `insert` 组件，所以首次插入 `Add` 和 `Insert` 都会触发，之后每次覆盖只触发 `Insert`——第 8 章的表格一格没变；两次响铃报的都是 `14v0`，“insert 永远覆盖原行”从事件侧再次验明。其二，`ResMut` 改值走的是变更检测（第 5 章的 `resource_changed` 那条线），不是生命周期事件——收人头税没惊动警报，机关各管各的。其三，observer 拿到 `on.entity` 后用 `Query<&TownFunds>` 按组件读值——`Res` 与查询，两条路通向同一个格子，这次是查询这条路顺手。

顺带补全待遇清单：`#[derive(Resource)]` 的类型同样能用第 8 章的钩子位（`#[component(on_add, ...)]` 写在类型上照常生效），也能标 `#[component(immutable)]`，让 `ResMut` 在编译期就被拒绝。资源不再有自己的一套规矩——它就是组件，规矩全在组件那边。

内账盘清，镇公所的秘密到此见底。还剩最后一件工具没交：11-3 节夸下的“列式连续存储”，除了让普通遍历跑得快，还能把整列直接借给你。

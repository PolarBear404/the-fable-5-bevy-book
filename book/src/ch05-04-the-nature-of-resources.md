# 资源的本质

第 3 章的实体清单欠着一笔账：打印出来的实体从 7v0 起步，0 到 6 号行“早有主了”，真面目押后再谈。现在可以揭晓了，因为那几行的主人正是本章的主角。

**资源不住在实体表之外——它就住在表里。** World 给每个资源类型开一行专属实体，资源的值就是那一行上的一个组件。`#[derive(Resource)]` 干的活也比表面上多：它同时把类型实现成 Component（`Resource` 本就是 `Component` 的子 trait——第一节说两者“一个待遇”，其实是一家人），还用第 3 章的 required components 机制给它配了件随行组件 **`IsResource`**——一个标记，里面记着“这一行装的是哪个资源类型”。于是每个资源实体身上恰好两件东西：资源组件本身，加一个 `IsResource`。至于本章开头担心的“全局数据的那一行要怎么找”，正是 `Res` 一族 API 替你包掉的活，稍后拆开看。

空口无凭，点个名：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-07.rs:setup}}
```

<span class="caption">Listing 5-7（节选）：一份资源、两个普通实体，同台备查</span>

```rust
{{#include ../../code/ch05-resources/examples/listing-05-07.rs:roll_call}}
```

<span class="caption">Listing 5-7（节选）：广查询全场点名，资源实体一并到场</span>

`roll_call` 的查询 D 槽位只有 `Entity` 和两个 `Option`——第 4 章说过，`Option<&T>` 不缩小匹配范围，`Entity` 更不挑人，所以这条查询什么都不筛，World 里每一行都会到场，不妨叫它**广查询**。`IsResource` 和 `Components` 平时藏在 prelude 之外，得点名导入：前者就是资源实体的随行标记，后者是引擎的组件注册表，拿着 `IsResource` 里记的编号能查出资源的类型名。运行：

```console
cargo run -p ch05-resources --example listing-05-07
```

```text
0v0  资源实体  DefaultQueryFilters
1v0  资源实体  Schedules
2v0  资源实体  AppTypeRegistry
3v0  资源实体  MainScheduleOrder
4v0  资源实体  FixedMainScheduleOrder
5v0  资源实体  Messages<AppExit>
6v0  资源实体  MessageRegistry
7v0  资源实体  Score
8v0  普通实体  外环
9v0  普通实体  红心
Res<Score> 照常直达：当前 0 分
```

对账。0 到 6 号行是 `App::new()` 的开机家底：引擎自己也靠资源过日子——存放全部调度的 `Schedules`（调度正是第 6 章的主题）、类型注册表、退出消息的队列（`Messages`，第 7 章见）……App 一造出来，这 7 份内部资源就按注册顺序落座，一人一行。行号 7 是你的 `Score`：`insert_resource` 排在它们之后，顺位领号。行号 8、9 才轮到 `Startup` 里 spawn 的靶子。第 3 章那批神秘房客至此验明正身：**World 从不为资源另设仓库，第 1 章那张实体表就是唯一的存储。**

那为什么第 3 章、第 4 章写了那么多查询，从没撞见过它们？不是引擎刻意隐藏，是匹配规则本来就放不进它们：你的查询点名的是 `Name`、`Health` 这类组件，而资源实体身上只有自己那件资源组件加 `IsResource`，对不上号，自然不进名单。第 3 章的怪物清单要的是 `&Name` 加 `&Health`，七个资源实体一件都没有，一行也混不进来。只有 Listing 5-7 这种什么都不点名的广查询，才会把它们和普通实体一起摊在你面前——顺带一提，将来你用 `Query<Entity>` 清点“实体总数”时，数出来的会比自己 spawn 的多，就是这个原因。

## insert_resource 背后

World 里有一份登记簿（`ResourceEntities`），记着每个资源类型住哪一行。`insert_resource` 每次都先查它：

- 该类型**没登记过**：spawn 一行新实体，把资源组件放上去——`IsResource` 作为 required component 自动跟上，这一行随即写进登记簿；
- **已登记**：直接覆盖那一行上的组件值，不另开新行。

这就是“insert 永远覆盖”的全部秘密：第一节所谓“`Score` 这个类型只有一个格子”，那个格子就是一行实体。`remove_resource` 也顺势看懂了——它只摘走资源组件，**行本身留着**：双倍卡收回去，那一行还在；哪天再发一张，还是住那儿。单例也不靠自觉：就算绕开资源 API、亲手把 `Score` 组件 spawn 到别的实体上，引擎也会当场把后来者清掉（日志里留一句警告）。一个类型一行，没有例外。

## Res 与 ResMut 是语法糖

读取走的也是同一份登记簿。`Res<Score>` 干的事拆开是三步：查登记簿找到 `Score` 住的那一行，到那一行上取 `Score` 组件，包好递给你——**`Res`/`ResMut` 就是“到专属实体上取组件”的语法糖**，把一次查询包装成报个类型名直达。Listing 5-7 顺手做了对照：广查询和 `Res<Score>` 在同一个系统里各取了一次分数，一个把那一行连同全场摊开，一个按类型直达那一行上的组件——两条路，同一个格子。向调度器登记的访问也是组件口径的：“读 `Score` 这一列，只限资源实体”。

现在回头看 B0002 报错里那句一直没解释的话——“using `Without<IsResource>` to create disjoint Queries”。既然资源是组件、住在实体表里，查询和资源访问就可能撞上同一列，而 `IsResource` 正是划界的标记：查询挂上 `Without<IsResource>`，就和资源实体彻底分家。至于让资源实体和普通实体同场接受查询、用 `IsResource` 精确圈定内外的玩法，留给第 11 章。

把模型收拢成一句话：普通组件挂在千千万万行上，资源组件只挂在自己那一行上，`Res`/`ResMut` 是为这一行配的按类型直达。底细清了，最后一块拼图是资源的变更检测——第 4 章 `Changed<T>` 的资源版本。

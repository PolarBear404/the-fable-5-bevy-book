# Disabled：隐身的实体

铁匠铺要冬歇了。把它 despawn？存粮数据没了，身上的组件没了，真有父子关系还会连孩子一起拆（第 9 章的级联）——开春重开等于从零重建。留着不拆？巡逻、收税、营业统计，每个系统都得加一条“铁匠铺除外”的判断，和第 10 章开头那种“每个系统自己把关”一个味道，漏一处出一处鬼。

要的效果其实是**隐身**：实体和数据原地保留，但所有日常系统视而不见。Bevy 的答案是一个普通组件：

## 挂牌与摘牌

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-13.rs:script}}
```

<span class="caption">Listing 11-13（其一）：insert(Disabled) 隐身，remove::&lt;Disabled&gt;() 现身</span>

`Disabled` 来自 `bevy::ecs::entity_disabling`，挂上即隐身、摘下即现身。妙处在受影响的一方——巡逻队**一个字都不用改**：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-13.rs:patrol}}
```

<span class="caption">Listing 11-13（其二）：巡逻队没改任何代码，却会自动失明</span>

而盘点册要求连冬歇的也入册。规则是：**查询只要显式提到 `Disabled`，就能看见它们**——`Has<Disabled>` 既放行又告诉你谁在冬歇：

```rust
{{#include ../../code/ch11-deep-ecs/examples/listing-11-13.rs:census}}
```

<span class="caption">Listing 11-13（其三）：提到 Disabled 的查询，隐身者照见</span>

```console
cargo run -p ch11-deep-ecs --example listing-11-13
```

```text
—— 第 1 帧 ——
  巡逻队（3 家亮灯）：杂货铺、铁匠铺、面包房
  盘点册：在册 3 家，其中冬歇 0 家
—— 第 2 帧 ——
  铁匠铺：入冬封炉，明春再会。（挂上 Disabled）
  巡逻队（2 家亮灯）：杂货铺、面包房
  盘点册：在册 3 家，其中冬歇 1 家（铁匠铺）
—— 第 3 帧 ——
  开春了，摘牌复工。（摘下 Disabled）
  巡逻队（3 家亮灯）：杂货铺、面包房、铁匠铺
  盘点册：在册 3 家，其中冬歇 0 家
```

第 2 帧挂牌，巡逻队当帧就少看见一家（`chain` 上游有 `Commands`，第 6 章规则二的自动同步点），盘点册照常数到 3 家并点出冬歇的是谁；第 3 帧摘牌，铁匠铺带着 2 袋存粮原样归队——这正是“隐身而非销毁”的全部价值。注意剧本里的细节：挂牌之后，剧本系统自己的 `shops` 查询也看不见铁匠铺了，摘牌的活只能交给 `reopen` 那种**专看隐身者**的查询（`With<Disabled>`）。

## 机制与细则

隐身的实现毫不神秘：每个 World 出生时自带一个 `DefaultQueryFilters`（默认查询过滤器）资源，里面登记着“隐身组件”名单（默认只有 `Disabled`）。构建查询时引擎查一眼名单：**访问集合没提到 `Disabled` 的查询，自动追加一条 `Without<Disabled>`**。所以“提到”是字面意思，三种姿势各有用途：

| 写法 | 角色 | 效果 |
|---|---|---|
| `With<Disabled>` | 过滤器 | 只看隐身的（reopen 用它） |
| `Has<Disabled>` | 数据 | 都看，且每行报告隐没隐（盘点册用它） |
| `Allow<Disabled>` | 过滤器 | 都看，不多给数据（“别给我过滤”的纯声明） |

细则四条，按踩坑概率排序：

- **隐身只罩查询**。`world.entity(e)`、`inspect_entity`、`entities().count_spawned()` 这些直接访问一概照见——上一节预检官数出的实体总数不会因冬歇而变少。盘点工具反而要利用这一点。
- **只隐身自己，不隐身孩子**。给载具挂 `Disabled`，乘员还露在外面；要整树隐身，用 `insert_recursive::<Children>(Disabled)`（第 9 章关系机制的衍生方法）。
- **可以自造隐身牌**。`world.register_disabling_component::<Hibernating>()` 把你自己的组件登记成隐身组件——但必须在 App 启动前登记：查询缓存建好之后才登记的，管不到已有查询。同理，这类组件会影响全 World 每一个查询，生态库之间互相不知道对方的隐身牌时会出怪事，自造之前三思。
- **状态作用域实体是邻居不是同类**。第 10 章的 `DespawnOnExit` 是“到点销毁”，`Disabled` 是“留人熄灯”——前者丢数据，后者保数据，按需选。

零件到此全部到齐：独占的钥匙、三档句柄、档案室的册与表、打包的参数、发条与印章、隐身牌。最后一节，把它们装配成艾达真正的盘点日。

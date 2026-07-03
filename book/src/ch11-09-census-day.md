# 盘点日

全章零件组装。分工照旧是本章的总纲：**日常归柜台，特权归钥匙**——巡逻和预检是普通系统（一个普通查询，一个 `&World` 只读），剧本用 `Commands` 排队制造变故，盘点本身是独占系统：`SystemState` 开柜台点名，`resource_scope` 写总册，`Allow<Disabled>` 全员点到，`inspect_entity` 逐户列家当，最后 `world.spawn` 当场立牌，同一帧里喇叭宣读。

```rust
{{#include ../../code/ch11-deep-ecs/src/main.rs}}
```

<span class="caption">Listing 11-17：完整示例——灰岩镇的盘点日（src/main.rs）</span>

```console
cargo run -p ch11-deep-ecs
```

```text
—— 第 1 帧 ——
  巡逻队（3 处亮灯）：杂货铺老板、铁匠铺、老蔫儿
  助手：预检——全镇实体 17 个，待盘点核对。
—— 第 2 帧 ——
  巡逻队（3 处亮灯）：杂货铺老板、铁匠铺、老蔫儿
  罗兰背着行李进镇：在杂货铺后屋搭个铺。（spawn）
  铁匠铺：入冬封炉，明春再会。（挂上 Disabled）
—— 第 3 帧 ——
  巡逻队（3 处亮灯）：杂货铺老板、老蔫儿、罗兰
  助手：预检——全镇实体 18 个，待盘点核对。
  艾达：盘点日，全镇静止！（接管 World）
  艾达点名（常住）：杂货铺老板、老蔫儿。
  艾达：归档 4 户，公告牌立讫。
  喇叭：宣读总册——
    14v0 杂货铺老板：存粮 40 袋 [Resident][Name][Stock][Shop]
    15v0 老蔫儿：存粮 7 袋 [Resident][Name][Stock]
    17v0 罗兰：存粮 3 袋 [Name][Stock][Lodger]
    16v0 铁匠铺（冬歇）：存粮 2 袋 [Disabled][Name][Stock][Shop]
（run() 返回，盘点日结束了）
```

三处值得回头多看一眼：

- **第 3 帧的三个数字是三种视角**。巡逻队报“3 处亮灯”——名单换了血：罗兰进来了、铁匠铺隐身了，可总数碰巧没变，光看它你不会知道镇上发生过什么；助手报“18 个实体”——`count_spawned` 不走查询，`Disabled` 瞒不过它，14 行内账（13 份引擎家底加总册资源 `TownLedger`）也一并在数，正是 11-7 节的口径；总册归档 4 户——`Allow<Disabled>` 让查询也看见冬歇的，`With<Name>` 又天然挡住了没名没姓的资源实体，一条查询同时用上了两节的划界。给游戏写统计或调试面板时，先想清楚要哪种视角。
- **总册那四行就是一个检查器**。事先没人声明过“要读 `Lodger`”或“要读 `Shop`”，是 `inspect_entity` 临场翻档案翻出来的——连 `[Disabled]` 都自报家门。把这些行写进文件就是存档的雏形，接上 UI 就是检查器；生态里的 inspector 类插件，地基与这几行无异。剧本第 2 帧排队的两条命令（投宿、挂牌），在第 3 帧帧首的清算里落地，恰好赶上盘点——而艾达自己的公告牌不排队，立讫即见，喇叭同帧宣读。
- **罗兰那行没有 `[Resident]`**。所以艾达点名常住时只有两人应到，总册上他照样入册——“点名”用窄查询，“归档”用全访问，一窄一宽各司其职。另外看门牌：罗兰是 `17v0`，比铁匠铺的 `16v0` 大——他是第 2 帧才 spawn 的，行号顺位领的。

## 钥匙的份量

本章的 API 在日常游戏逻辑里出场不多，这是设计使然：柜台参数能并行、能被调度器排布，钥匙不能。经验法则——**每帧跑的逻辑用 `Query`/`Commands`，低频特权活（存档、初始化、调试工具、测试）用 `World`**。但从读代码的角度，本章是道分水岭：官方仓库的 examples、生态插件的源码、乃至 Bevy 自身的实现，`SystemState`、`EntityWorldMut`、`QueryData` derive 俯拾皆是，现在它们对你不再是天书。

## 小结

- **独占系统**：参数为 `&mut World` 的系统，运行期间无任何并行；World 方法当场生效，没有命令队列。同伴只能是 `ExclusiveSystemParam`（`Local`、`&mut SystemState` 等），混入普通参数是编译错误
- **World 直接访问**：`resource`/`resource_mut`/`insert_resource`、`query`/`query_filtered`（给 `QueryState`，`iter(&world)` 取数据）、`write_message`、`trigger`（当场触发）；“资源加世界”的借用冲突用 `resource_scope` 解
- **实体句柄三档**：`EntityRef` 只读、`EntityMut` 读写数据、`EntityWorldMut` 全权（insert/remove/take/despawn）；多户同借自动降档——结构变更会搬家，搬家会作废邻座的句柄；`entity()` panic，`get_entity()` 给 `Result`
- **存储**：组件组合相同的实体同住一个 Archetype，数据默认在 Table 里列式连续——查询快的根源；insert/remove 即搬家；`#[component(storage = "SparseSet")]` 换“插拔快、遍历慢”；遍历顺序永远不可依赖
- **`&World` 与 `Query<EntityRef>`**：普通系统里的只读全访问；前者连资源也算读，后者只罩组件——同系统的写参数会撞出 B0002
- **打包三件套**：`SystemState` 在独占系统里借出参数（`Commands` 要手动 `apply`）；`#[derive(SystemParam)]` 打包参数；`#[derive(QueryData)]` 打包查询行（`mutable` 才许 `&'static mut`，`&'static` 是占位写法）
- **变更检测**：全局 `Tick` 每系统运行加一；组件带 `added`/`changed` 两印，`&mut` 解引用即盖章；`Changed` = 印落在 `(last_run, this_run]` 窗口；`Ref<T>` 看印不过滤；`set_if_neq` 防误报，`bypass_change_detection` 绕过盖章（慎用），`changed_by()` 报案发位置（需 `track_location`）
- **Disabled**：挂上即对“没提到它”的查询隐身（`DefaultQueryFilters` 自动补 `Without`）；`With`/`Has`/`Allow` 三种提法；World 直接访问不受影响；自定义隐身组件要在 App 启动前注册
- **资源实体**：资源一人一行住在实体表里、随行 `IsResource`（第 5 章的模型）；窄查询天生不沾，广查询默认看得见（`IsResource` **不在** `DefaultQueryFilters` 名单上）——清点、`EntityRef` 遍历、广查询 × 资源参数的 B0002 都会撞上，`With`/`Without<IsResource>` 划界；生命周期事件、observer、钩子、`immutable` 全套组件待遇照用；别 despawn 资源实体
- **连续访问**：`contiguous_iter`/`contiguous_iter_mut` 一次借出一张 Table 的整列切片；过滤器限 `With`/`Without`/`Or`（档案级），沾 SparseSet 的查询运行时给 `Err`；`ContiguousMut` 解引用即整表盖章，`bypass_change_detection()` 给裸切片全静默；大批量数值运算的专用钥匙

## 练习

1. **优雅的钉子户**：Listing 11-4 里艾达拆棚屋用的是 `world.entity_mut(shed).despawn()`——名单过期时这行会 panic。改用 `get_entity_mut` 重写拆房流程，对已注销的门牌打印一句“查无此户，跳过”，并故意拆两次验证两条分支都走到。
2. **档案室寻路**：在 Listing 11-7 末尾给罗兰 `remove::<Registered>()` 再 `remove::<Flagged>()`，先预测他会落回哪一册（新开一册，还是回到 `ArchetypeId(1)`？`archetypes().len()` 会不会减少？），运行验证。再想想：这对“频繁插拔组件”的成本意味着什么。
3. **错位的窗口**：把 Listing 11-11 的 `audit` 挪到 `script` 前面（`(banner, audit, script).chain()`），先预测每一帧账房的台词会怎么变——第 2 帧的误报还在吗，第 5 帧的入库哪一帧才被听见？运行对答案，并用“窗口 `(last_run, this_run]`”解释。
4. **自家的隐身牌**：定义 `Hibernating` 组件并 `register_disabling_component`（提示：`App` 阶段可以拿 `app.world_mut()` 调 World 方法），给面包房挂上。验证巡逻队同样失明，再验证 `Has<Hibernating>` 的查询能不能看见**铁匠铺**（它挂的是 `Disabled`）——由此体会“每个隐身组件各自独立”的含义。
5. **不在家的资源**：仿照 Listing 11-14 写一个只翻内账的系统（`With<IsResource>` 过滤，配 `inspect_entity` 数件数），在 `Update` 里跑。先预测：哪一行会“只有 1 件”、为什么？再想一想：你自己的资源会不会也有“不在家”的时刻——什么代码会造成它？（提示：`resource_scope`。）
6. **切片上的印章**：修改 Listing 11-16 第 2 帧——对铺面那张表（2 行）也调用一次 `DerefMut`（比如取 `&mut stocks[..]`）但一个字都不写。先预测这一帧账房会听到谁，再运行验证，并用“解引用即整表盖章”解释结果。

第二部分到此收官。从第 3 章的第一个 `#[derive(Component)]` 到今天的 `&mut World`，ECS 的地图已经画完整：数据怎么放、逻辑怎么跑、消息怎么传、世界怎么直接上手。下一章进入第三部分——给这个世界一个看得见的样子。先从最基本的问题开始：一个东西在哪儿、朝向哪儿、多大个儿——`Transform` 与坐标系统。

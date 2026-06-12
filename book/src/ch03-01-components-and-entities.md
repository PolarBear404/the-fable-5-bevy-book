# 组件与实体

本章的舞台是一个迷你地下城：骑士罗兰、几只怪物，全都只是控制台里的几行字。第一步，从定义本书的第一个自定义组件开始。

## 定义一个 Component

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-01.rs}}
```

<span class="caption">Listing 3-1：定义组件、生成实体、查询打印</span>

`Health` 是个普通的元组结构体，加一行 `#[derive(Component)]`，它就成了一种合法的“列”。这就是定义组件的全部仪式。组件可以是任何自有类型——有字段的结构体、空结构体、枚举都行（唯一的约束是 `Send + Sync + 'static`，日常类型几乎都满足）。Bevy 不要求你注册组件清单：第一次使用时引擎自动登记。

`spawn_slime` 你在第 2 章已经见过同款：`Commands` 是向 World 提交变更的指令队列，`spawn` 生成一个新实体并挂上给定的组件——给表添一行，行上有 `Health` 一列，值是 30。

`census` 用本章最朴素的 **Query** 形态：`Query<&Health>` 的意思是“所有带 `Health` 的实体，把这一列借我读”。`for ... in &creatures` 逐行遍历。Query 的完整能力是下一章的主题，本章只用它打印清单。

运行：

```console
cargo run -p ch03-entities-components --example listing-03-01
```

```text
发现一个实体，生命值：30
```

注意这个 App 没有装任何插件——连 `MinimalPlugins` 都没有。第 2 章说过，默认 runner 把所有调度跑一遍就退出：`Startup` 一轮、`Update` 一轮，打印完毕，进程结束。对纯逻辑实验来说，这个“跑一遍”恰好就是全部所需，本章所有示例都这么干。

## 标记组件与 Name

地下城需要区分敌我。身份不是数值，用“有没有这一列”本身来表达：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-02.rs:markers}}
```

不带任何字段的组件叫**标记组件**（marker component）：它不存数据，存在与否就是信息。第 2 章 `With<Sprite>` 之所以能把方块从相机里筛出来，本质就是按列筛选——标记组件是专为这种筛选而生的列。Bevy 自己也大量使用这个手法。

再认识一个内置组件：**`Name`**。它是 Bevy 提供的“名牌”，给实体挂一个人类可读的名字。调试打印、日志、外部检查器（第 33 章）都认它——比起每个项目自己发明一个 `struct PlayerName(String)`，用引擎统一的 `Name` 能让整个工具生态都看得懂你的实体。

把队伍生成出来：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-02.rs:spawn_party}}
```

<span class="caption">Listing 3-2（节选）：元组挂多个组件，spawn_batch 批量生成</span>

`spawn` 的参数从单个组件变成了元组——一次挂上多个组件。能这样传，是因为 `spawn` 接受的其实是 **Bundle**（组件包）：一组静态确定的组件。单个组件是 Bundle，至多 15 个成员的元组是 Bundle，元组还可以嵌套。你会在本章最后一节见到它的具名形式。

`spawn_batch` 负责批量：给它一个迭代器，每个元素是一个同类型的 Bundle，引擎预先分配好内存一口气生成。逐个 `spawn` 写循环也对，批量接口更快，意图也更清楚。

## 实体清单与 Entity 的真面目

队伍齐了，打印花名册。这次查询除了组件，还要出每行的“行号”：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-02.rs:roster}}
```

<span class="caption">Listing 3-2（节选）：在查询里要出 Entity 本身</span>

`Entity` 可以直接出现在查询元组里——它不是组件，而是行的 ID，查询会把每行的 ID 一并给你。运行：

```console
cargo run -p ch03-entities-components --example listing-03-02
```

```text
=== 实体清单 ===
1v0  史莱姆  HP 30
2v0  骷髅兵  HP 45
3v0  蝙蝠 1 号  HP 10
4v0  蝙蝠 2 号  HP 10
5v0  蝙蝠 3 号  HP 10
0v0  罗兰  HP 100
```

每个 `Entity` 打印成 `行号v世代号` 的形状。六个实体按生成先后拿到行号 0 到 5；**世代号**（generation）目前全是 0，它的作用下一节销毁实体时揭晓。记住这个事实：`Entity` 只是一个轻量 ID（行号 + 世代号，合计一个 u64），数据全在表里。它实现了 `Copy`，可以随手存进别的组件、塞进集合——这正是 ECS 里“引用一个游戏对象”的标准方式。

还有一个值得睁大眼睛看的细节：**打印顺序不是生成顺序**。罗兰第一个生成（行号 0），却排在最后。原因在于存储布局：World 把组件组合完全相同的实体放在同一张子表里——这种“组件组合”称为 **Archetype**（原型）——查询是按子表逐张扫过去的。罗兰的组合是 `Name + Player + Health`，独占一张子表；五只怪物共享 `Name + Monster + Health` 那张。子表内部按先来后到，子表之间的次序则是引擎的内务。结论只有一条：**永远不要依赖查询的遍历顺序**。需要顺序时自己排（存储细节在第 11 章）。

行也会造、清单也会打了。但有个第 2 章遗留的问题该算账了：这些 `spawn` 全都经过 `Commands`，而我们说过它“不当场生效”。不当场，那是何场？下一节专门拆它。

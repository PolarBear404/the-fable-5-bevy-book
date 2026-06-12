# 父子树：ChildOf 与 Children

商队的第一辆车要发车了。“老姜、小芙、罗兰在青篷车上”这件事，在 Bevy 里只需要给每个乘员挂一个组件：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-01.rs:board}}
```

<span class="caption">Listing 9-1（节选）：上车——给乘员插入 ChildOf（examples/listing-09-01.rs）</span>

**`ChildOf`**（父子关系组件）是个普通的单字段组件，里面装着父实体的 `Entity`。注意 `board` 里我们**只**写了 `ChildOf`，但查询时却能从两个方向看到这层关系：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-01.rs:roster}}
```

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-01.rs:whereami}}
```

<span class="caption">Listing 9-1（续）：同一层关系，两个方向各有一个组件可查</span>

```console
cargo run -p ch09-relationships --example listing-09-01
```

```text
== 从车往下看 ==
青篷车 载着 3 人：
  - 老姜
  - 小芙
  - 罗兰
== 从人往上看 ==
老姜 在 青篷车 上
小芙 在 青篷车 上
罗兰 在 青篷车 上
```

**`Children`**（子女名单组件）我们从头到尾没写过，它是哪来的？答案就是第 8 章的组件钩子：`ChildOf` 在 derive 时自带一个 `on_insert` 钩子——组件上身的瞬间，钩子跑到目标实体（车）上，没有 `Children` 就补一个，然后把新乘员登记进去。开篇抱怨的“两本账”问题，Bevy 的解法不是只记一本，而是**两本都记、但只让你写其中一本**：

- `ChildOf` 是**事实源**（source of truth）：想改变父子关系，永远操作它；
- `Children` 是引擎根据事实源自动维护的**镜像**：随便读（迭代、`len()`、按下标取），但不要手工增删——它对你唯一开放的写操作是重排顺序（`sort_by` 一族），名单里有谁完全由引擎做主。

`Children` 里的顺序就是登记顺序，这一点后面做 UI 时有用（子节点的排列顺序就是它）。

还有一条规则是白送的：第 3 章说过，一个实体上每种组件至多一份。`ChildOf` 是组件，所以**一个实体至多只有一个父亲**——“树”这个结构不靠任何额外检查，光凭组件模型就立住了。

## 建树三式

一个个 `spawn` 再挂 `ChildOf` 当然可行，但车队天天组装，Bevy 给了三种更顺手的写法：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-02.rs:assemble}}
```

<span class="caption">Listing 9-2（节选）：children! 宏、with_children 闭包、add_child 后补（examples/listing-09-02.rs）</span>

三种写法各有用武之地：

- **`children![...]` 宏**：声明式，树长什么样、代码就长什么样，支持任意嵌套（罗兰怀里还揣着铜灯）。代价是拿不到中途实体的 `Entity`——整棵树是一个表达式，没有插手的空隙。
- **`with_children(|car| ...)` 闭包**：闭包里的 `car.spawn(...)` 自动给孩子挂上 `ChildOf`，适合循环生成；闭包外的 `.id()` 照常能拿到父实体的编号。
- **`add_child(entity)`**：实体早已存在（路边捡的扁担），事后认亲。同族还有批量版 `add_children(&[...])`。

用一个递归函数把两棵树打出来验货：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-02.rs:print}}
```

<span class="caption">Listing 9-2（续）：沿 Children 递归下行，缩进打印整棵树</span>

```text
铁皮货车
  罗兰
    铜灯
  货物箱
青篷车
  老姜
  木桶 1 号
  木桶 2 号
  扁担
```

## 两桩闯祸

挂关系的时候手一抖，会发生什么？让一辆车自己拉自己，再让一位旅人登上早已报废的车：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-03.rs:mishaps}}
```

<span class="caption">Listing 9-3（节选）：自指的 ChildOf、指向已销毁实体的 ChildOf（examples/listing-09-03.rs）</span>

这次运行多挂了一个 `LogPlugin`——引擎的警告走日志系统，不挂它就什么都看不见：

```text
WARN bevy_ecs::relationship: The bevy_ecs::hierarchy::ChildOf(0v0) relationship on entity 0v0 points to itself. The invalid bevy_ecs::hierarchy::ChildOf relationship has been removed.
WARN bevy_ecs::relationship: The bevy_ecs::hierarchy::ChildOf(1v0) relationship on entity 2v0 relates to an entity that does not exist. The invalid bevy_ecs::hierarchy::ChildOf relationship has been removed.
== 事后清点 ==
青篷车：不挂在任何东西下面
倒霉旅人：不挂在任何东西下面
```

两条 `WARN` 各对应一桩祸：指向自己、指向不存在的实体。处置方式一样——**警告一声，把非法的 `ChildOf` 当场拆掉**，程序继续跑。事后清点也证实：两位当事实体都还活着，只是身上的关系组件没了。这是 Bevy 关系机制的一贯姿态：宁可拆掉关系自愈，也不让一笔坏账留在世界里。

但引擎只拦这两种明牌错误。A 当 B 的父亲、B 再当 A 的父亲这种**绕圈**它不查——环上的遍历会无限循环，这是下下节的注意事项。

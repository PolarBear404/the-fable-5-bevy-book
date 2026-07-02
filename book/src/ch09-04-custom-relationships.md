# 自定义关系：装备槽

上一节末尾的别扭，说破了是个硬约束：一个实体每种组件至多一份，所以**一个实体在每种关系里至多有一个目标**。长戟要是用 `ChildOf` 表示“在小芙手里”，那它就没法同时表达别的从属了；更糟的是语义被搅浑——“车上坐几个人”的遍历会把长戟也数进去，小芙下车时（`remove::<ChildOf>`）武器反倒留在了车上。

“拿在手里”和“坐在车上”就是两种关系，应该用两种组件。好消息是：`ChildOf`/`Children` 没有任何引擎特权，它们就是用公开的 `Relationship` 机制定义出来的。同一套 derive，你也可以用：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-08.rs:derive}}
```

<span class="caption">Listing 9-8（节选）：一对关系组件——EquippedBy 是事实源，Equipment 是镜像（examples/listing-09-08.rs）</span>

两个 derive 属性互相指认，把两个普通组件配成一对：

- `#[relationship(...)]` 标记**事实源**（对应 `ChildOf` 的角色）：单字段元组结构体，装着目标实体；
- `#[relationship_target(...)]` 标记**镜像**（对应 `Children` 的角色）：装着一个 `Vec<Entity>`，由引擎维护。镜像组件的字段必须保持私有——这正是“别手工改名单”的纪律在类型层面的体现。

配好之后，第 9-1、9-2 节学的每一招都原样适用，连语法糖都有对应物（`children!` 宏只是 `related!` 的缩写）：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-08.rs:setup}}
```

<span class="caption">Listing 9-8（续）：小芙同时身处两种关系——ChildOf 管乘坐，Equipment 管装备</span>

转手装备？跟换乘是同一个动作——insert 新的事实源：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-08.rs:handover}}
```

<span class="caption">Listing 9-8（续）：reparent 的装备版</span>

```console
cargo run -p ch09-relationships --example listing-09-08
```

```text
小芙 │ 在青篷车上 │ 装备：长戟、护身符
罗兰 │ 在青篷车上 │ 装备：（空手）

【小芙把长戟交给罗兰】
小芙 │ 在青篷车上 │ 装备：护身符
罗兰 │ 在青篷车上 │ 装备：长戟

【山路颠簸，护身符摔得粉碎】
罗兰 │ 在青篷车上 │ 装备：长戟
小芙 │ 在青篷车上 │ 装备：（空手）
```

最后一幕值得多看一眼：护身符被 `despawn`，小芙的清单**自动**除名，而且因为清单空了，`Equipment` 组件整个消失（所以清点系统查的是 `Option<&Equipment>`）。从插入、转手到销毁清账，自定义关系拿到的是和父子树完全相同的全套服务。

## linked_spawn：陪葬还是掉落

只有一件事，上面的 `Equipment` 没有照搬 `Children`：**级联销毁**。把主人 `despawn` 掉，装备会怎样？答案取决于镜像组件 derive 时的一个开关。这次给护卫配两件东西——商队配发的灯笼走普通关系，祖传的腰刀走带 `linked_spawn` 的关系：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-09.rs:derive}}
```

<span class="caption">Listing 9-9（节选）：同一套机制，一个开关的差别（examples/listing-09-09.rs）</span>

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-09.rs:setup}}
```

<span class="caption">Listing 9-9（续）：配发与认主，然后注销主人</span>

```text
== 清点 ==
公家灯笼（配发）→ 在 护卫老蔫儿 手里
祖传腰刀（认主）→ 绑定 护卫老蔫儿
护卫老蔫儿

【这趟跑完，老蔫儿领钱走人】
== 清点 ==
公家灯笼
```

老蔫儿注销后，两件装备命运分岔：

- **祖传腰刀**（`linked_spawn`）：随主人一起注销——目标实体销毁时，镜像名单上的源实体全部陪同销毁；
- **公家灯笼**（默认）：只是被解除关系，作为无主实体留在世界里，等下一任护卫来领。

现在回头看第 9-2 节就全通了：**`Children` 的级联销毁不是父子树的特权，只是它 derive 时开了 `linked_spawn`**。坠崖的货车带走全车人，和老蔫儿带走祖传腰刀，是同一行代码生效的结果。你设计自己的关系时按语义选边：部件、子弹、特效这类“皮之不存毛将焉附”的关系开它；装备、目标、阵营这类“关系断了各自活”的关系别开。

## allow_self_referential：允许自指

第 9-1 节的第一桩闯祸里，引擎拆掉了“青篷车自己拉自己”，那条警告末尾预告的 `allow_self_referential` 属性，就是给自定义关系准备的。`ChildOf` 那样要递归遍历的树形关系确实容不得自指，但你的关系未必是树：给自己疗伤、瞄准自己、雇主是本人——这些关系没人拿去遍历，“指向自己”在语义上站得住脚。想放行，在**事实源**一侧的 `#[relationship(...)]` 里补上这个属性：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-10.rs:derive}}
```

<span class="caption">Listing 9-10（节选）：allow_self_referential——为自指关系解除门禁（examples/listing-09-10.rs）</span>

山路颠簸，伤号不止一个，随队郎中自己也挂了彩：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-10.rs:injuries}}
```

<span class="caption">Listing 9-10（续）：郎中给自己上药——TreatedBy 指向自己</span>

```text
随队郎中 正在诊治：罗兰、随队郎中
```

这次特意挂了 `LogPlugin`，却一条警告也没有：自指的 `TreatedBy` 顺利上身，郎中安然出现在自己的病人名单里。要是去掉属性再跑一遍，下场就和青篷车一模一样——警告一声，关系当场拆除；在这条门禁上，自定义关系和 `ChildOf` 待遇相同。

放行的代价是防环的责任回到你头上：遍历方法不查环的规矩（第 9-3 节）在这里照旧，拿一个允许自指的关系去 `iter_ancestors`，走到自指实体就原地打转。所以这个属性只该给“本来就不遍历”的扁平关系——像病人名单这样只读一层的，随便开。

## 关系工具箱

自定义关系的配套 API 与父子版一一对应，规律是把 `child` 换成 `related` 并补上类型参数：

| 父子版 | 通用版 |
|---|---|
| `children![...]` | `related!(Equipment[...])` |
| `with_children(\|c\| ...)` | `with_related_entities::<EquippedBy>(\|c\| ...)` |
| `with_child(bundle)` | `with_related::<EquippedBy>(bundle)` |
| `add_child(e)` / `add_children(&[e])` | `add_one_related::<EquippedBy>(e)` / `add_related::<EquippedBy>(&[e])` |
| `detach_child(e)` 等 | `remove_related::<EquippedBy>(&[e])` 等 |
| `despawn_children()` | `despawn_related::<Equipment>()` |
| `iter_descendants(e)` 等遍历 | 同名方法，换用你的关系组件做查询数据即可 |

两个进阶开关，知道存在就好：

- 镜像的集合不是只能用 `Vec<Entity>`——`EntityHashSet`（不关心顺序、频繁增删）、`BTreeSet<Entity>` 都实现了所需的 trait；甚至可以直接用单个 `Entity`，得到**一对一关系**：新源头插入时，旧源头的关系组件会被自动移除（想想“决斗”——同一时刻每人只有一个对手）。
- 事实源上除了目标实体还可以带别的字段（须实现 `Default`），用 `#[relationship]` 属性标出哪个字段是目标。

下一节把全章人马拉上同一条山道，让两种关系的级联首尾相接。

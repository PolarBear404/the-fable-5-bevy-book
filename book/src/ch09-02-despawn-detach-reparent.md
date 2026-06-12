# 拆树：销毁、下车与换乘

树建好了，现在学怎么拆。拆法不止一种，差别全在“孩子的下场”上。商队这回有三辆车，各演一种：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-04.rs:acts}}
```

<span class="caption">Listing 9-4（节选）：despawn、despawn_children、remove::&lt;ChildOf&gt; 三种拆法（examples/listing-09-04.rs）</span>

清点系统在每一刀之间各跑一遍（注意 `Option<&Children>`——它的用意马上揭晓）：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-04.rs:muster}}
```

<span class="caption">Listing 9-4（续）：清点——车的名单、空车与下车的人</span>

```console
cargo run -p ch09-relationships --example listing-09-04
```

```text
== 出发点名 ==
铁皮货车 → 货物箱一、货物箱二
柴车 → 湿柴一捆、湿柴二捆
青篷车 → 老姜、小芙

【塌方！铁皮货车坠下山崖】
青篷车 → 老姜、小芙
柴车 → 湿柴一捆、湿柴二捆

【湿柴发了霉，连捆烧掉——车留下】
青篷车 → 老姜、小芙
柴车 →（空车）

【到家门口了，老姜下车】
青篷车 → 小芙
柴车 →（空车）
地面上站着：老姜
```

三刀三种下场：

- **`despawn()`——全家一起走**。坠崖之后，铁皮货车和两只货物箱都从清点里消失了。第 3 章你认识的 `despawn` 只销毁一个实体，但只要实体带着 `Children`，**整棵子树会级联销毁**，多少层都一样。开篇说的“手工递归”从此不用写。
- **`despawn_children()`——只销毁孩子**。湿柴没了，柴车还在。注意第二次清点起柴车显示“（空车）”：名单清空后 `Children` **整个组件**会被自动移除，而不是留一个空集合挂着——所以清点系统查的是 `Option<&Children>`，拿 `None` 当空车。这也解释了 Listing 9-1 为什么从不担心“车还没有乘员时 Children 是什么”：没有乘员，就没有这个组件。
- **`remove::<ChildOf>()`——只断关系**。老姜和青篷车都活着，只是名单上不再有他。一切修改都落在事实源 `ChildOf` 上，车那头的镜像自动更新。

顺序还提醒了一件第 4 章讲过的旧事：第二次清点里两辆车的输出顺序变了。查询的迭代顺序从来没有保证，`despawn` 之后存储搬动，顺序就可能洗牌——别把任何逻辑建立在它上面。

## 想直接改写 ChildOf？编译器拦下了

换乘看起来还有一条更直接的路：查出 `&mut ChildOf`，把里面的 `Entity` 改成新车。试试：

```rust
{{#include ../../code/ch09-relationships/no-compile/listing-09-05.rs:transfer}}
```

<span class="caption">Listing 9-5：直接改写关系组件——过不了编译（no-compile/listing-09-05.rs）</span>

```text
error[E0271]: type mismatch resolving `<ChildOf as Component>::Mutability == Mutable`
   |
   | fn transfer_everyone(mut crew: Query<&mut ChildOf>, new_wagon: Single<Entity, With<Wagon>>) {
   |                                ^^^^^^^^^^^^^^^^^^^ expected `Mutable`, found `Immutable`
   |
   = note: required for `&mut bevy::prelude::ChildOf` to implement `QueryData`
```

这是本书第一次见到**不可变组件**（immutable component）：`ChildOf` 在类型层面声明了“禁止可变借用”，`&mut ChildOf` 根本不是合法的查询。原因不难想通——关系的两本账靠钩子保持一致，而钩子只在组件**插入和移除**时运行（第 8 章的分界线：纯改值不触发任何钩子）。要是允许你拿到 `&mut ChildOf` 偷偷改目标，账房根本不知道发生过调动，旧车名单上的人就成了幽灵。所有用 derive 定义的关系组件一律不可变，引擎从根上堵死了“绕过账房改账本”这条路。

## 换乘的正确姿势

改不了字段，那就换整个组件——给乘客 `insert` 一个新的 `ChildOf`。插入会触发完整的钩子流程：先按 `Replace` 把旧关系的账清掉，再按 `Insert` 登记新关系：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-06.rs:transfer}}
```

<span class="caption">Listing 9-6（节选）：insert 新的 ChildOf——reparent 就这一行（examples/listing-09-06.rs）</span>

```text
青篷车 → 老姜、小芙
铁皮货车 → 罗兰

【小芙晕车，换乘铁皮货车】
青篷车 → 老姜
铁皮货车 → 罗兰、小芙
```

旧车除名、新车末尾入列，一行 `insert` 全办妥。给实体换父亲的操作行话叫 **reparent**——在 Bevy 里它没有专门的 API，因为不需要：插入新关系本身就是换。

## 拆树工具箱

本节的三刀加上几个没上场的亲戚，集中列一遍。它们都是 `EntityCommands` 上的方法（`EntityWorldMut` 上也有同名版本）：

| 方法 | 效果 |
|---|---|
| `despawn()` | 销毁实体及整棵子树（级联） |
| `despawn_children()` | 子树全灭，自己留下 |
| `remove::<ChildOf>()` | 从孩子端断开关系，双方都活着 |
| `detach_child(e)` / `detach_children(&[e])` | 从父亲端断开指定孩子，等效于对孩子 `remove::<ChildOf>()` |
| `detach_all_children()` | 断开全部孩子，孩子都活着 |
| `replace_children(&[e])` | 一步换成新名单：不在新名单的断开、新来的接上 |
| `insert(ChildOf(new))` | reparent：旧关系清账、新关系登记 |

记忆负担其实很小：**`despawn` 开头的杀实体，`detach` 开头的只断关系**；而所有“断关系”的方法，最终都落实为对事实源 `ChildOf` 的移除或替换。

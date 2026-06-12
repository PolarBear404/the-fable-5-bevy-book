# 沿树行走：层级遍历

Listing 9-2 里我们手写了递归来打印树。一两层的树这么干没问题，但“点名整支商队”“从一件装备追查它在哪辆车上”这类需求太常见，Bevy 直接把走法做成了 `Query` 的方法。这次把树搭满三层——大旗、车、人、随身物：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-07.rs:setup}}
```

<span class="caption">Listing 9-7（节选）：三层嵌套的 children!（examples/listing-09-07.rs）</span>

## 向下：iter_descendants

向下遍历靠 `Query<&Children>`——毕竟要沿着每个节点的名单往下走：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-07.rs:roll_call}}
```

<span class="caption">Listing 9-7（续）：广度优先与深度优先两种点名</span>

```console
cargo run -p ch09-relationships --example listing-09-07
```

```text
广度优先：青篷车、铁皮货车、老姜、小芙、罗兰、长戟、铜灯
深度优先：青篷车、老姜、小芙、长戟、铁皮货车、罗兰、铜灯
```

两个迭代器都吐出全部七个后代（不含起点自己），区别只在次序：

- **`iter_descendants`：广度优先**——先点完两辆车，再点车里的人，最后点随身物。一层层扫。
- **`iter_descendants_depth_first`：深度优先**——青篷车连人带物清完，才轮到铁皮货车。一支支捋。

多数时候你不在乎次序，随手用前者就行；要按“子树成块”处理时选后者。注意它们给的是**平铺的实体流**，不带“现在第几层”的信息——想要带缩进的树形输出，还是得像 Listing 9-2 那样自己递归。

## 向上：iter_ancestors 与 root_ancestor

向上遍历换 `Query<&ChildOf>`——沿着每个实体的父亲指针爬：

```rust
{{#include ../../code/ch09-relationships/examples/listing-09-07.rs:trace_up}}
```

<span class="caption">Listing 9-7（续）：失物招领——从长戟一路上溯</span>

```text
长戟在谁手里：长戟 ← 小芙 ← 青篷车 ← 商队大旗
它属于哪支商队：商队大旗
```

`iter_ancestors` 逐级吐出父亲、祖父……直到树根；`root_ancestor` 是它的快捷收尾，直接给出树根（如果实体本来就没有父亲，树根就是它自己）。

## 一条警告与一张清单

上一节末尾的伏笔在此兑现：这些遍历方法**不检测环**。引擎只拦“自己当自己父亲”，拦不住 A、B 互为祖先的绕圈——真出了环，`iter_descendants` 会在圈里永远转下去，程序原地卡死。环几乎总是 reparent 惹的祸：把某个实体挂到**它自己的后代**名下，树就拧成了圈。写关卡编辑器这类要任意拖拽层级的工具时，挂接前先用 `iter_ancestors` 查一遍“新父亲是不是我的后代”。

最后把没上场的几个遍历方法列齐——都是 `Query` 的方法，用哪种组件做查询数据，决定了你能往哪个方向走：

| 方法 | 查询数据 | 给你什么 |
|---|---|---|
| `iter_descendants(e)` | `&Children` | 全部后代，广度优先 |
| `iter_descendants_depth_first(e)` | `&Children` | 全部后代，深度优先 |
| `iter_ancestors(e)` | `&ChildOf` | 全部祖先，由近及远 |
| `root_ancestor(e)` | `&ChildOf` | 树根 |
| `iter_leaves(e)` | `&Children` | 子树里的全部叶子（没有孩子的末端实体） |
| `iter_siblings(e)` | `(Option<&ChildOf>, Option<&Children>)` | 同一个父亲名下的兄弟，不含自己 |

这章到现在，父子树的“增删走”三件事齐了。但你可能已经隐约觉得不对劲：长戟挂在小芙名下，用的也是 `ChildOf`——“拿在手里”和“坐在车上”真的是同一种关系吗？下一节就处理这个别扭。

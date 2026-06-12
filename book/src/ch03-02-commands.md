# Commands：排队修改世界

先回答“为什么不当场生效”。

World 是所有 System 共享的一张表。第 1 章说过，Bevy 靠每个 System 的签名判断它读写哪些列，从而把互不冲突的 System 扔到多个线程上并行跑。这套机制管得住“改格子里的值”：你在签名里声明了 `&mut Health`，调度器自然不会让别人同时碰这一列。

但 `spawn`、`despawn`、增删组件是另一码事：它们**改的是表的结构**。添一行、删一行、给某行换一套列组合，影响的是所有正在遍历这张表的人——这种操作必须独占整个 World，而 System 运行期间恰恰做不到独占。Bevy 的解法就是 `Commands`：结构性修改不立刻执行，先写成指令排进队列；等到调度中一个可以独占 World 的位置——称为**同步点**（sync point）——再统一应用。

## 眼见为实

下面这个程序让延迟语义自己开口说话：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-03.rs}}
```

<span class="caption">Listing 3-3：同一系统看不见自己刚下的命令，下一个系统看得见</span>

`.chain()` 是新面孔：不加约束时，同一调度里的系统没有固定执行顺序（Bevy 会把它们并行调度），`.chain()` 让元组里的系统按书写顺序执行。顺序控制的全貌在第 6 章，这里只需要它保证“先增援、后清点”。运行：

```console
cargo run -p ch03-entities-components --example listing-03-03
```

```text
下令前，查询到的怪物数：0
已下令生成增援，预分配的 Entity：0v0
下令后，查询到的怪物数：0
下一个系统查询到的怪物数：1
```

四行输出对应三个事实：

1. **`spawn` 立刻返回 Entity ID。**`.id()` 拿到的 `0v0` 是当场预留的行号——实体还不存在，号码先占住。所以你可以放心把这个 ID 存起来、传给后续命令用，不必等它“真的出生”。
2. **下令的系统自己看不见变化。**`spawn` 前后两次查询都数到 0 只怪——指令还躺在队列里，World 纹丝未动。
3. **下一个系统看见了**。调度器发现 `call_reinforcements` 攒着命令、而 `recount` 被 `.chain()` 排在它后面，就在两者之间自动插入了一个同步点，把队列清空。

另一条规则其实前面一直在默默使用：**一个调度跑完，攒下的命令必然已全部应用**。Listing 3-1、3-2 在 `Startup` 里 spawn、在 `Update` 里查询，靠的就是 `Startup` 结束时的清算。同步点的完整规则（自动插在哪、代价是什么）放在第 6 章。

## EntityCommands：对着某一行下指令

`commands.spawn(...)` 之外，`commands.entity(id)` 接受一个已有实体的 ID，返回 **`EntityCommands`**——绑定到那一行的指令柄。最常用的三个动作：

- `.insert(bundle)`：给这行加列（已有同名列则覆盖值）
- `.remove::<T>()`：摘掉列
- `.despawn()`：整行销毁

用一场战斗全部演示一遍。开场三只怪：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-04.rs:spawn}}
```

罗兰横扫，每只怪扣 50 点血：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-04.rs:sweep}}
```

<span class="caption">Listing 3-4（节选）：值修改当场生效，结构修改排队</span>

这个系统是本节核心对比的现场。`health.0 -= 50` 是**值修改**：通过 `&mut Health` 直接写格子，当场生效，不经过任何队列——紧接着的 `if health.0 <= 0` 读到的就是扣过血的新值。`despawn` 和 `insert` 是**结构修改**：排队，出了同步点才作数。一句话立此存照：**改值直接写，改结构走 Commands。**

死者销毁，伤者插上 `Wounded` 标记。下一个系统是随军牧师，按标记找伤员，治疗后摘掉标记：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-04.rs:triage}}
```

<span class="caption">Listing 3-4（节选）：按标记查询，remove 摘除组件</span>

`sweep` 插的标记，`triage` 能查到，依然是同步点的功劳（两者之间有 `.chain()` 约束）。`remove::<Wounded>()` 的泛型参数和 `insert` 的参数一样是 Bundle——也就是说 `remove::<(A, B)>()` 能一次摘一组。

战斗结束，增援抵达：

```rust
{{#include ../../code/ch03-entities-components/examples/listing-03-04.rs:reinforce}}
```

<span class="caption">Listing 3-4（节选）：despawn 之后的 spawn</span>

```console
cargo run -p ch03-entities-components --example listing-03-04
```

```text
史莱姆 倒下了
骷髅兵 倒下了
石巨人 负伤，剩 30 点生命
牧师治疗了 石巨人
增援抵达：1v1
=== 战后清点 ===
2v0  石巨人  HP 30
1v1  增援史莱姆  HP 30
```

看增援的 ID：**`1v1`**。行号 1 原本属于骷髅兵（`1v0`）——它阵亡后行号被回收，新实体复用同一行号，但**世代号加一**。这就是上一节埋的悬念：行号会复用，世代号保证复用不会张冠李戴。假如某处还存着骷髅兵的旧 ID `1v0`，拿它去访问 World，引擎一对世代号就知道“此 1 非彼 1”，旧 ID 作废——不会错把增援史莱姆当成还魂的骷髅兵。顺带一提，拿过期的 ID 再 `despawn` 一次不会让程序崩溃，Bevy 只是打一条警告，提醒你逻辑里残留着失效引用；但对已销毁实体 `insert` 默认按错误处理、直接 panic——错误处理策略可以定制，第 33 章细讲。

本节的全部规则浓缩成一张表：

| 操作 | 走哪条路 | 何时生效 |
|---|---|---|
| 读组件值 | `Query<&T>` | 即时 |
| 改组件值 | `Query<&mut T>` | 即时 |
| 生成/销毁实体 | `Commands` | 下一个同步点 |
| 增删组件 | `Commands` + `EntityCommands` | 下一个同步点 |

还剩最后一块拼图：第 2 章 `spawn(Camera2d)` 凭空冒出一整套组件的魔法。下一节揭晓。

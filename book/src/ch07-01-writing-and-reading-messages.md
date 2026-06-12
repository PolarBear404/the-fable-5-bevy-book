# 收发消息

用一个最小场景把全套机制走通：一颗弹球在走廊里来回弹，撞墙时发出一条消息，音效系统听到就响一声。需要三步——**定义**消息类型、向 App **注册**它、用一对系统参数**收发**它：

```rust
{{#include ../../code/ch07-messages/examples/listing-07-01.rs}}
```

<span class="caption">Listing 7-1：最小消息闭环——球撞墙，音效响</span>

```console
cargo run -p ch07-messages --example listing-07-01
```

```text
—— 第 1 帧 ——
球：位置 4
—— 第 2 帧 ——
球：位置 8
—— 第 3 帧 ——
球：位置 12
音效：砰！
—— 第 4 帧 ——
球：位置 8
—— 第 5 帧 ——
球：位置 4
—— 第 6 帧 ——
球：位置 0
音效：砰！
```

第 3、6 帧球撞墙，同一帧音效就响——还是熟悉的手动 `app.update()` 实验台。逐项对账：

- **定义**：`#[derive(Message)]` 把任意类型变成消息。和 Component、Resource 一样是纯数据，约束也最宽松——`Send + Sync + 'static` 即可，这里的 `WallHit` 干脆是个空结构体。
- **注册**：`add_message::<WallHit>()` 在 App 里为这个类型开出一条通道。忘了这步，程序会在收发系统第一次运行时 panic，错误信息是 `Message not initialized`。
- **写**：`MessageWriter<WallHit>` 是个普通的系统参数（第 4 章的清单又添一员）。`write` 把消息投进缓冲就返回，**不会当场触发任何人**——读者要等自己被调度到才看见。
- **读**：`MessageReader<WallHit>` 的 `read()` 返回一个迭代器，内容是**这个系统还没见过的**全部消息。本例每次撞墙只有一条，本章结尾的弹球馆里你会看到一帧多条的场面。

最值得盯住的是两个函数的签名：`move_ball` 里没有 `play_sound`，`play_sound` 里也没有 `move_ball`。两个系统素不相识，把它们连起来的是消息类型本身——**类型就是频道**。每个 `M` 各有一条独立通道，`MessageWriter<WallHit>` 写的消息，只有 `MessageReader<WallHit>` 收得到。

通道本身也没有任何魔法：`add_message::<M>()` 注册的其实是一个叫 `Messages<M>` 的**普通 Resource**，所有在途消息都存在里面。`MessageWriter<M>` 不过是 `ResMut<Messages<M>>` 的薄封装；`MessageReader<M>` 则是 `Res<Messages<M>>` 加一个 `Local` 游标——第 4、5 章的零件拼出来的。这个出身决定了它们的并发性质和生命周期规则，后面两节都会用到。

> **来自旧版本的改名**：这套机制在 Bevy 0.17 之前叫 `Event`——`EventReader`/`EventWriter`/`add_event`。网上的老教程、AI 给出的代码片段很可能还是旧名，照着写在 0.18 里编译不过，逐字替换成 `Message` 系列即可。`Event` 这个名字没有消失，它被让给了另一套即时响应机制——正是第 8 章的主角，两者的分工到时候细说。

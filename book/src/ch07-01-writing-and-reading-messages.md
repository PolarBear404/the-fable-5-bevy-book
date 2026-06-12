# 收发消息

碰碰车场开门第一天：新手阿莱上场，一脚油门踩到底；DJ 坐在台子后面，听到碰撞就放音效。让这两个素不相识的人协作起来，需要三步——**定义**消息类型、向 App **注册**它、用一对系统参数**收发**它：

```rust
{{#include ../../code/ch07-messages/examples/listing-07-01.rs}}
```

<span class="caption">Listing 7-1：最小消息闭环——阿莱撞护栏，DJ 放音效</span>

```console
cargo run -p ch07-messages --example listing-07-01
```

```text
—— 第 1 帧 ——
阿莱：撞上护栏！
DJ：砰！
—— 第 2 帧 ——
阿莱：撞上护栏！
DJ：砰！
—— 第 3 帧 ——
阿莱：撞上护栏！
DJ：砰！
```

直道只有 4 格、车速也是 4，阿莱每一帧都撞个正着；DJ 一声不落，而且**同一帧就响**——写者排在读者前面，`chain` 保证了这一点。逐项对账：

- **定义**：`#[derive(Message)]` 把任意类型变成消息。和 Component、Resource 一个待遇——普通结构体即可，约束只有 `Send + Sync + 'static`，这里的 `RailHit` 干脆是个空结构体。
- **注册**：`add_message::<RailHit>()` 在 App 里为这个类型开出一条通道。
- **写**：`MessageWriter<RailHit>` 是普通的系统参数——第 4 章那张参数清单里标着"第 7 章"的一行，今天兑现。`write` 把消息投进缓冲就返回，**不当场触发任何人**：读者要等自己被调度到才看见。
- **读**：`MessageReader<RailHit>` 的 `read()` 给出一个迭代器，内容是**这个系统还没见过的**全部消息；读过即翻篇，下一帧不会重读。

最值得盯住的是两个函数的签名：`drive` 里没有 `play_sound`，`play_sound` 里也没有 `drive`。把他们连起来的是消息类型本身——**类型就是频道**。每个类型一条独立通道，`MessageWriter<RailHit>` 写的消息，只有 `MessageReader<RailHit>` 收得到。

## 忘了注册会怎样

第 5 章忘了 `insert_resource` 有一出 panic 剧目，消息这边的对应剧目长这样：

```rust
{{#include ../../code/ch07-messages/examples/listing-07-02.rs}}
```

<span class="caption">Listing 7-2：忘记 add_message——写者就位，通道没开</span>

```console
cargo run -p ch07-messages --example listing-07-02
```

```text
Encountered an error in system `listing_07_02::drive`: Parameter
`MessageWriter<'_, RailHit>::messages` failed validation: Message not initialized
If this is an expected state, wrap the parameter in `Option<T>` and handle `None`
when it happens, or wrap the parameter in `If<T>` to skip the system when it happens.
```

报错把三件事一次说清：哪个系统（`drive`）、哪个参数（`MessageWriter` 内部那个 `messages`）、什么毛病（`Message not initialized`）。"failed validation"的口径来自第 4 章的参数校验家族——和 `Single` 找不到实体是同一套机制；报错末尾甚至附了 `Option`/`If` 的药方，第 5 章"资源的有无"那两件工具在这里原样可用。不过对消息来说，正确答案几乎总是补上那行 `add_message`，而不是把参数包成可选。

报错里那个 `messages` 参数顺带掀开了通道的底牌：`add_message::<M>()` 注册的其实是一个叫 **`Messages<M>`** 的普通 Resource，所有在途消息都存在里面。`MessageWriter<M>` 不过是 `ResMut<Messages<M>>` 的薄封装，`MessageReader<M>` 则是 `Res<Messages<M>>` 加一个 `Local` 游标——第 4、5 章的零件，拼出了一套新机制。这个出身决定了它的并发性质和生命周期规则，本章后面每一节都会回到这里；引擎还会亲口承认一次，见 Listing 7-8。

> **来自旧版本的改名**：这套机制在 Bevy 0.17 之前叫 `Event`——`EventReader`/`EventWriter`/`add_event`。网上的老教程、AI 给出的代码片段很可能还是旧名，照着写在 0.18 里编译不过，逐字替换成 `Message` 系列即可。`Event` 这个名字没有消失，它被让给了另一套即时响应机制——正是第 8 章的主角，两者的分工到时候细说。

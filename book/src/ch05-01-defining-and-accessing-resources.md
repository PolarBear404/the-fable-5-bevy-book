# 定义与访问 Resource

任何满足 `Send + Sync + 'static` 的类型，挂上 `#[derive(Resource)]` 就能当资源——和 `#[derive(Component)]` 一个待遇，普通结构体即可，没有别的仪式：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-01.rs:score}}
```

注册和使用一气呵成：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-01.rs:main}}
```

<span class="caption">Listing 5-1（节选）：insert_resource 把一份值放进 World</span>

**`insert_resource`** 把这份 `Score(0)` 放进 World。它发生在 App 构建期，第一帧开始前资源就已就位。“全局唯一”在这里有了确切含义：World 按**类型**存放资源，`Score` 这个类型只有一个格子——再 `insert_resource(Score(99))` 一次不会出现第二份分数，而是覆盖原值。

两个系统，一写一读：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-01.rs:systems}}
```

<span class="caption">Listing 5-1（节选）：ResMut 写，Res 读</span>

**`Res<T>`** 和 **`ResMut<T>`** 是资源版的 `&T` 和 `&mut T`：报上类型名，调度器把 World 里那份唯一的值递给你，只读用前者，要改用后者。运行：

```console
cargo run -p ch05-resources --example listing-05-01
```

```text
砰！+10 分
报靶员：累计 10 分
砰！+10 分
报靶员：累计 20 分
砰！+10 分
报靶员：累计 30 分
```

分数从 10 涨到 30——这份数据**跨帧存活**，这点 `Local` 也做得到；但它同时被 `shoot` 和 `announce` 两个系统读写——**跨系统共享**，这正是 `Local` 给不了的。第 4 章的缺口就此补上：只有自己关心的记忆用 `Local`，大家都要碰的数据用 Resource。

## 访问声明照旧管用

第 4 章的核心规则——**签名即访问声明**——对资源原样生效。`Res<Score>` 向调度器登记“读 `Score`”，`ResMut<Score>` 登记“写 `Score`”；多个系统同时读没问题，一写就必须独占。调度器据此安排并行：两个都只拿 `Res<Score>` 的系统可以同时跑，`shoot` 和 `announce` 则永远错开。

冲突的另一半也照旧：系统**之间**靠调度器错开，系统**自己的**两个参数撞上了就是另一回事。第 4 章两个 `&mut Hunger` 查询撞出了 B0001，资源版的对应剧目编号 **B0002**：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-02.rs:conflict}}
```

<span class="caption">Listing 5-2（节选）：同一资源一读一写——编译通过，首帧 panic</span>

```console
cargo run -p ch05-resources --example listing-05-02
```

```text
error[B0002]: ResMut<listing_05_02::Score> in system listing_05_02::impossible
conflicts with a previous system parameter. Consider removing the duplicate
access or using `Without<IsResource>` to create disjoint Queries or merging
conflicting Queries into a `ParamSet`. See: https://bevy.org/learn/errors/b0002
```

和 B0001 一样，判的是声明而不是数据，所以第一帧直接 panic。报错开的方子里混进了 Query 和一个陌生名字 `IsResource`——资源和查询的关系比表面上近得多，这句话要到“资源的本质”一节才能完全读懂，先按下。就这个系统而言，修法比查询冲突简单：同一资源一读一写几乎总是冗余声明——`ResMut` 本来就能读，删掉那个 `Res` 就完了。

## `Res<Time>` 归队

现在可以兑现第 2 章的承诺了。当时那个会动的 Sprite 靠 `Res<Time>` 拿到时钟，我们说“Resource 在第 5 章”——其实你已经看到了全部真相：`Time` 就是一个普通资源，由 `TimePlugin`（`DefaultPlugins` 和 `MinimalPlugins` 共有的成员）在构建期注册，每帧开头由引擎的系统刷新。你的 `Score` 和引擎的 `Time`，在 World 眼里是完全平等的两个格子。

这是个值得停一拍的事实：**引擎自己的全局状态也走这套机制**。窗口配置、帧计数、后面几章的消息队列，全是资源。学会 `Res`/`ResMut`，你就拿到了读取引擎大半内部状态的钥匙——第 18 章讲时间时，不会再有新的取数姿势，只有 `Time` 本身的细节。

下一节看资源的“无”：忘了注册会发生什么，以及怎么把“有没有”本身变成一种信息。

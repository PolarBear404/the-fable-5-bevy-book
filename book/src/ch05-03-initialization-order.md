# 初始化：init_resource 与 FromWorld

`insert_resource` 之外，App 还有一个 **`init_resource::<T>()`**——不接值，只接类型。它的行为分两支：

- World 里**没有** `T`：创建一份初始值放进去；
- World 里**已有** `T`：什么都不做，让位于现有值。

先看第一支的初始值从哪来。打靶场的配置有一份理所当然的默认值：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-05.rs:config}}
```

类型实现了 `Default`，`init_resource` 就用它。两种场景对比：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-05.rs:main}}
```

<span class="caption">Listing 5-5（节选）：init_resource——没有才创建，已有则让位</span>

```console
cargo run -p ch05-resources --example listing-05-05
```

```text
本场配置：5 发子弹，单价 2 元
本场配置：10 发子弹，单价 1 元
```

场景一拿到 `Default` 的 5 发 2 元；场景二里用户先 `insert` 了自定义配置，随后的 `init_resource` 发现位置有人，默认值根本没造出来。注意这与 `insert_resource` 的反差：`insert` 永远覆盖，`init` 永远谦让。

这份谦让正是它存在的理由。回想 `TimePlugin`——它在构建期 `init_resource::<Time>()`。假如它用的是 `insert`，你提前塞好的任何时钟配置都会被无情冲掉；用 `init`，插件提供默认值、用户的显式配置优先，两不相欠。**Plugin 注册自己依赖的资源用 `init_resource`，把覆盖的权力留给用户**——这是 Bevy 生态的通行礼数，你将来写 Plugin 也该如此。

## FromWorld：看着 World 算初始值

`init_resource` 的签名里藏着第二层："没有才创建"时调用的其实不是 `Default`，而是 **`FromWorld`**：

```rust
pub trait FromWorld {
    fn from_world(world: &mut World) -> Self;
}
```

它拿到**整个 World 的可变引用**，想看什么看什么。所有实现了 `Default` 的类型自动获得一份 `FromWorld` 实现（直接返回默认值），所以刚才的 `RangeConfig` 不用操心这事；但当初始值需要**根据 World 的现状算出来**时，就该亲手实现 `FromWorld` 了。

打靶场按难度定价：休闲场红心 10 分，职业场 25 分。记分规则不该拍脑袋写死，而是看难度资源的脸色：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-06.rs:difficulty}}
```

```rust
{{#include ../../code/ch05-resources/examples/listing-05-06.rs:rules}}
```

<span class="caption">Listing 5-6（节选）：FromWorld——初始值由另一份资源算出</span>

`world.resource::<Difficulty>()` 是 World 的直接访问方法（系统外没有 `Res` 可用，第 11 章细讲这一族 API），缺货时 panic。两种难度各开一场：

```rust
{{#include ../../code/ch05-resources/examples/listing-05-06.rs:main}}
```

```rust
{{#include ../../code/ch05-resources/examples/listing-05-06.rs:report}}
```

<span class="caption">Listing 5-6（节选）：难度一换，算出的规则跟着变</span>

```console
cargo run -p ch05-resources --example listing-05-06
```

```text
休闲场：红心一枪 10 分
职业场：红心一枪 25 分
```

`report` 顺手演示了系统读多个资源：再添一个参数就是，没有数量上的讲究。

## 顺序就是依赖

`ScoreRules` 的初始值依赖 `Difficulty`，这条依赖链对**书写顺序**提出了硬要求。`insert_resource` 和 `init_resource` 都是构建期立即执行的——不走 `Commands`、没有队列，一行一个脚印。把 Listing 5-6 的两行对调：

```rust
app.init_resource::<ScoreRules>() // from_world 此刻就跑——Difficulty 还没影
    .insert_resource(Difficulty::Casual)
```

```text
thread 'main' panicked at ch05-resources\examples\listing-05-06.rs:23:21:
Requested resource listing_05_06::Difficulty does not exist in the `World`.
                Did you forget to add it using `app.insert_resource` / `app.init_resource`?
```

panic 的位置就在 `from_world` 里那行 `world.resource::<Difficulty>()`，时机是 `init_resource` 执行的瞬间——`app.update()` 都还没开始，连第一帧都谈不上。**资源的初始化顺序，就是 App 构建代码的书写顺序**；谁依赖谁，谁就写在后面。

顺带收一条线：资源的注册其实有三条路径，时机各不相同——

| 路径 | 时机 |
|---|---|
| `App::insert_resource` / `init_resource` | 构建期，立即 |
| Plugin 的 `build` 里同名调用 | 构建期，随 `add_plugins` 的顺序 |
| `Commands::insert_resource` / `init_resource` | 运行期，下一个同步点 |

第二行正是 `Res<Time>` 第 2 章就能用的原因：`add_plugins(DefaultPlugins)` 在你的系统跑起来之前，早把 `Time` 备好了。

定义、读写、有无、初始化都齐了。最后一块拼图是资源的变更检测——第 4 章 `Changed<T>` 的资源版本。

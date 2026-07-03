# 组件钩子：长在组件上的规矩

公会有条铁规：武器入库必登记、出库必销册，一件不漏。用上一节的 observer 实现——`On<Add, Weapon>` 登记、`On<Remove, Weapon>` 销册——能跑，但总让账房不踏实：observer 是**外挂**的，挂多少个、什么时候挂、会不会被人 despawn 掉，组件自己一概不知情。账房想要的是把规矩**写进武器的出厂设定**：只要 `Weapon` 这个组件存在一天，登记销册就雷打不动。

这就是**组件钩子**（component hooks）的定位：声明在组件定义上的生命周期回调，相当于组件的构造函数与析构函数：

```rust
{{#include ../../code/ch08-events-observers/examples/listing-08-09.rs}}
```

<span class="caption">Listing 8-9：组件钩子——长在 Weapon 组件上的登记规矩</span>

```console
cargo run -p ch08-events-observers --example listing-08-09
```

```text
—— 第 1 帧 ——
老锤：打好两把武器。
  账房（钩子）：铁剑 登记入册。
  巡查员（observer）：看到 铁剑 入库。
  账房（钩子）：长戟 登记入册。
  巡查员（observer）：看到 长戟 入库。
—— 第 2 帧 ——
老锤：两把都出货了。出货前在册：["铁剑", "长戟"]
  巡查员（observer）：看到 铁剑 出库。
  账房（钩子）：铁剑 销册。
  巡查员（observer）：看到 长戟 出库。
  账房（钩子）：长戟 销册。
```

先对账写法，再看输出里藏的那条铁律。

- **声明**：`#[component(on_add = 函数名, on_remove = 函数名)]` 直接挂在 `derive(Component)` 旁边。五个钩子位与上一节的五个事件一一对应：`on_add`、`on_insert`、`on_discard`、`on_remove`、`on_despawn`，触发时机也完全相同。
- **签名是死的**：`fn(DeferredWorld, HookContext)`——一个普通函数指针。钩子不是系统，没有资格声明 `Query`、`Res` 那一套参数，也不能用捕获环境的闭包。
- **`DeferredWorld`**：受限版的世界访问。读写现成的组件和资源没问题（`world.get`、`world.resource_mut`），但结构性改动——增删组件、生成销毁实体——必须经 `world.commands()` 排队。毕竟钩子运行在世界变更的半道上，不能再当场大兴土木。
- **`HookContext`**：告诉你这次触发涉及哪个实体（`ctx.entity`）、哪个组件。

现在看输出顺序。第 1 帧添加时，**钩子先于 observer**；第 2 帧移除时，**observer 先于钩子**。这不是巧合而是设计：钩子既然是构造与析构函数，就理应**第一个到场、最后一个锁门**——构造时它先把组件的配套状态立好，observer 们才进场；析构时 observer 们先办完各自的事，它最后清场关灯。顺带留意第 2 帧没有出现 `Despawn` 字样的行——`despawn` 在这里是以 `Remove` 的身份被两边感知的，和上一节五幕剧的结论一致。

## 一个萝卜一个坑

钩子与 observer 的另一处分野：同一个组件的同一种钩子**只能有一个**。observer 爱挂几个挂几个，钩子的坑位是独占的——再塞一个，引擎当场翻脸：

```rust
{{#include ../../code/ch08-events-observers/examples/listing-08-10.rs}}
```

<span class="caption">Listing 8-10：一个组件、同种钩子只能有一个</span>

```console
cargo run -p ch08-events-observers --example listing-08-10
```

```text
thread 'main' (9880) panicked at ...\bevy_ecs-0.19.0\src\lifecycle.rs:187:14:
Component already has an on_add hook
```

`register_component_hooks` 是钩子的运行时注册入口（不改组件定义、给第三方组件补钩子时用得上），但它的两条前提都写在 panic 的剧本里：该种钩子还没被占坑——`derive` 上声明过的也算占坑，这正是上面这一幕的死因；以及该组件还没有被任何实体使用。这两条限制让钩子保持“出厂设定”的本色：它描述的是组件**固有**的行为，不该在世界已经跑起来之后被偷偷换掉。

两件工具放在一起，分工就清楚了：

| | 组件钩子 | Observer |
|---|---|---|
| 数量 | 每组件每种至多一个 | 不限 |
| 何时确定 | 组件定义时（或世界启动前） | 运行时随挂随卸 |
| 本职 | 维护组件的不变式：索引、登记、配套状态 | 业务联动：特效、音效、玩法逻辑 |

经验法则：**钩子管“这个组件自身必须成立的事”，observer 管“别人关心这个组件的事”**。登记簿跟着 `Weapon` 走天经地义，写成钩子；点火特效是火焰附魔的外延玩法，留给 observer。Bevy 自己也是这么分的：第 9 章要讲的父子关系，靠的正是关系组件上的钩子在维护双向链接——到时候你会认出这套手法。

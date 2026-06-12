# 一个会动的 Sprite

是时候兑现第 1 章结尾的承诺了。本节的程序会打开窗口，在屏幕上画一个方块，并让它动起来。代码里将出现几样还没正式介绍的工具——`Commands`、`Query`、`Res`——这是有意安排：先用直觉看懂全貌，第 3～6 章再逐个拆解时，你会带着“原来第 2 章那段代码是这个意思”的体感读完它们。

```rust
{{#include ../../code/ch02-first-app/src/main.rs}}
```

<span class="caption">Listing 2-7：相机、方块，与每帧驱动它的 System（src/main.rs）</span>

```console
cargo run -p ch02-first-app
```

窗口打开，一个天蓝色方块在窗口中央水平往返，六秒多一个来回。

## setup：把东西放进 World

`setup` 挂在 `Startup` 上，启动时跑一次。它的参数 **`Commands`** 是向 World 提交变更的指令队列：生成实体、销毁实体、增删组件，都通过它登记，稍后由引擎统一执行——为什么不当场生效、什么时候生效，是第 3 章的重点之一。

`commands.spawn(X)` 的意思是：生成一个新实体，挂上组件 X——给那张表添一行。这里添了两行。

第一行，**`Camera2d`**，2D 相机。**没有相机，就什么都看不见**——相机决定把世界的哪一块、以何种方式画到窗口上。细节在第 13 章。

第二行，**`Sprite`**（精灵——要绘制的一张 2D 图）。`Sprite::from_color` 不需要图片素材，给纯色和尺寸就行，先把“画面”这件事跑通；加载真正的图片要等第 14 章的 Asset 系统。

但表里的列比你写的多。`spawn(Camera2d)` 生成的实体，实际还自动带上了 `Camera`、投影方式等一整套组件；`Sprite` 同样自动带上了 `Transform`（位置、旋转、缩放）和 `Visibility`（可见性）。这个机制叫 **required components**（必需组件）：组件可以声明“有我就必须有它们”，缺的由引擎自动补齐——0.15 版以来 Bevy 最核心的惯用法之一，第 3 章正式讲。

此刻的 World 大致是这样（相机一行省略了 `Camera`、投影等更多列）：

| | `Camera2d` | `Sprite` | `Transform` | … |
|---|:---:|:---:|:---:|:---:|
| 相机 | ✓ | | ✓ | … |
| 方块 | | ✓ | ✓ | … |

## move_sprite：每帧改写数据

`move_sprite` 挂在 `Update` 上，每帧跑一次。它的参数就是第 1 章说的“System 声明自己关心哪些列”：

- `Res<Time>`：要读名为 `Time` 的 **Resource**（资源——World 里全局唯一、不属于任何实体的数据）。`Time` 是引擎维护的时钟，`elapsed_secs()` 给出启动以来的秒数。Resource 在第 5 章，时间在第 18 章。
- `Query<&mut Transform, With<Sprite>>`：一个 **Query**（查询）——“给我所有**带 `Sprite`** 的实体的 `Transform`，我要改写它”。`With<Sprite>` 是过滤条件；Query 的完整能力在第 4 章。

函数体把 x 坐标写成 `sin(启动秒数) × 200`：正弦值在 -1 到 1 之间摆，x 就在 -200 到 +200 像素之间往返（顺带交代坐标系：2D 世界的原点在窗口中心，x 向右为正，y 向上为正，第 12 章细讲）。

注意这里没有任何“让它动”的指令。**所谓动画，就是每帧改一次数据**：`move_sprite` 改写 `Transform`，渲染插件在同一帧稍后照着新数据画，仅此而已。

### 为什么需要 `With<Sprite>`

把查询改成 `Query<&mut Transform>`（去掉过滤）再运行，你会看到一个有趣的结果：**方块纹丝不动**。

原因：相机也有 `Transform`——它也是表里的一行。去掉过滤后，System 作用于所有带 `Transform` 的实体：相机和方块被同一个公式驱动，同进同退，相对位置永远不变，画面看起来就是静止的。

这是 ECS 与“对某个对象调方法”思维最大的不同：**System 默认面向一类实体，而不是某一个**。精确圈定“哪一类”，正是 Query 过滤器存在的意义。

## 回看那张表

第 1 章的模型，现在每个格子都有了着落：

- **Entity**：两行——相机和方块，`spawn` 添加的；
- **Component**：`Camera2d`、`Sprite`、`Transform`……挂在行上的列；
- **System**：`setup` 和 `move_sprite`——声明要哪些列，引擎喂给它匹配的行；
- **控制反转**：全程没写一个 `loop`，只是把函数登记到 `Startup` 和 `Update`，循环由插件提供、引擎驱动。

往后的章节，无非是让这张表越来越宽（更多种类的组件）、筛选越来越准（查询与过滤器）、时机越来越细（调度、消息与事件）。骨架你已经全部见过了。

## 小结

- **App** 是 Bevy 程序的容器：World（数据）+ Schedule（时机）+ runner（怎么跑）；`run()` 交出控制权，默认 runner 只把调度跑一遍
- **System 是普通函数**，用 `add_systems(时机, 函数)` 登记；`Startup` 启动时跑一次，`Update` 每帧跑一次
- **App 的能力来自 Plugin**：`MinimalPlugins` 给无窗口循环，`DefaultPlugins` 给含窗口与渲染的完整引擎；`.set()` 定制组内成员；实现 `Plugin` trait 打包你自己的功能
- 窗口是 `WindowPlugin` 里的数据（标题、分辨率……），默认所有窗口关闭即退出；开窗后的主循环由 winit 事件循环驱动
- 画面三件套：相机（`Camera2d`）+ 内容（`Sprite`）+ 每帧改写数据的 System——**动画 = 每帧改数据**

## 练习

1. **窗口**：给 Listing 2-7 加上标题“会动的方块”和 1024×768 的分辨率（手法见 Listing 2-5）。
2. **运动**：让方块改为上下往返；再让它绕圈——提示：x 用 `sin`，y 用 `cos`，半径随意。
3. **插件**：把 `setup` 和 `move_sprite` 打包成 `MovingSquarePlugin`，让 `main` 里只剩 `add_plugins`（手法见 Listing 2-6）。

下一章潜入 Bevy 的心脏：Entity 与 Component。你会定义自己的组件、批量生成实体，并真正理解 `spawn` 与 required components 背后发生了什么。

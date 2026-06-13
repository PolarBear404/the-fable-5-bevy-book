# 忘了贴标签

那一脚现在可以踩了。既然不带标签 `load` 出来的是 `Handle<Gltf>`，要是把它直接交给 `SceneRoot` 呢？

```rust
{{#include ../../code/ch23-gltf/no-compile/listing-23-03.rs:wrong}}
```

<span class="caption">Listing 23-3（编译失败）：把整份 glTF 当场景用（no-compile/listing-23-03.rs）</span>

编译器当场拦下，错得明明白白：

```text
error[E0308]: mismatched types
  --> ch23-gltf\no-compile\listing-23-03.rs:19:30
   |
19 |     commands.spawn(SceneRoot(gltf));
   |                    --------- ^^^^ expected `Handle<Scene>`, found `Handle<Gltf>`
   |                    |
   |                    arguments to this struct are incorrect
   |
note: tuple struct defined here
  --> ...\bevy_scene-0.18.1\src\components.rs:18:12
   |
18 | pub struct SceneRoot(pub Handle<Scene>);
   |            ^^^^^^^^^
```

`SceneRoot` 要的是 `Handle<Scene>`，你给的是 `Handle<Gltf>`。一份 glTF 文件不是「一个场景」，是「一箱东西」；`SceneRoot` 要的是箱子里那**一个**场景，所以非得 `Scene(0)` 点名不可。这层区分，类型系统替你顶在了脸上。

把几种写法摆在一处，就清楚了：

| 写法 | 编译器看到的类型 | 什么时候出错 |
|---|---|---|
| `asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/puppet.gltf"))` | `Handle<Scene>` | 标签写成 `Scene(999)` 这种不存在的编号时，运行时报「没有这个标签」 |
| `asset_server.load("models/puppet.gltf#Scene0")` | 由上下文推断，放进 `SceneRoot` 时是 `Handle<Scene>` | 字符串拼错时运行时报错；大小写也得一字不错 |
| `let gltf: Handle<Gltf> = asset_server.load("models/puppet.gltf")` | `Handle<Gltf>` | 直接塞进 `SceneRoot`，编译期 E0308 |
| `SceneRoot(asset_server.load("models/puppet.gltf"))` | 被 `SceneRoot` 反推成 `Handle<Scene>` | 编译能过，但没写标签，运行时加载失败 |

后两种最容易把人绕住。其一：显式写 `let gltf: Handle<Gltf>`，编译器当场知道你拿的是整份 glTF，不让它进 `SceneRoot`。其二：一气呵成 `SceneRoot(asset_server.load("models/puppet.gltf"))`，编译反而**过得去**，因为 `load` 的返回类型被 `SceneRoot` 反推成了 `Handle<Scene>`；可这条没贴标签的路径加载不成，运行时在控制台打出一行 `ERROR`。标签**拼错**也是运行时才暴露，比如把 `Scene0` 敲成 `scene0`，报错会把文件里所有能用的标签列出来给你对照：

```text
ERROR bevy_asset::server: The file at 'models/puppet.gltf' does not contain
the labeled asset 'scene0'; it contains the following 19 assets: 'Animation0',
'Material0', 'Material1', 'Mesh0', 'Mesh0/Primitive0', …, 'Node6', 'Scene0'
```

两个变体撞的是同一条规矩，区别只在**什么时候**被发现：`Scene(0)` 这种带类型的写法，错了在**编译期**就被挡下（上面那个 E0308）；手搓字符串只能等到**运行时**才暴露。能在编译期解决的，何必拖到运行时——所以**优先用带类型的 `GltfAssetLabel` 点名，少手搓字符串路径**。

阿福站上了台，可它眼下还是「铁板一块」。其实加载那一刻，它早被拆成了一棵实体树：躯干、头、四肢各是一个实体，还各自带着名字。下一节就去点它们的名。

# glTF 与 3D 资产

得月楼灯也亮了、影子也落了地，可台上立着的还是箱笼、立柱、绣球——一色儿是 `Cuboid`、`Sphere` 手工拼的布景。戏要开，总得有角儿。真正的角儿不是几个图元拼得出来的：它有头有四肢、会比划、会做身段，这种东西在 Blender、Maya 这些建模软件里捏，捏好打包成一份文件请进门。这份文件的格式，就是 **glTF**（GL Transmission Format，3D 世界通行的传输格式）——上一章末尾说的那张「通用提货单」。

老雷先请来个探路的：杖头木偶「阿福」。挑木偶不挑真人是有讲究的——木偶浑身是**关节**：头、躯干、四条肢，各是一块独立的、有名有姓的部件，靠几根杆子各自摆动。这正好对上本章要学的东西。真正有血有肉、皮肉随骨头一起变形的角色（那叫**蒙皮**），机巧更深，留到第 30 章动画再请。

第 14 章讲过：一个 `Handle` 是一件资产的「提货单」。glTF 把这事放大了——一份文件是一整箱货，箱里装着场景、节点、网格、材质、动画好几样，提货单上每一样都编了号。本章就学怎么照着这张单子，把整箱、或单件，提进 Bevy 的世界。

跟着阿福走一遍：

- **请角儿上台**——`SceneRoot` 配上 `GltfAssetLabel::Scene(0)`，加载一份 glTF 场景、自动展开成实体；
- **拆开提货单**——一份 glTF 里到底装了什么（场景、节点、网格、材质、动画），标签 `#Scene0`、`#Animation0` 怎么点名单件；加载整份 `Gltf` 翻它的目录；
- **忘了贴标签**——漏掉标签会撞上的 `Handle<Gltf>` / `Handle<Scene>` 类型错，亲眼看一眼；
- **点谁的名**——场景展开成一棵实体树，节点的名字变成 `Name`；靠 `SceneInstanceReady` 等它就位，再按名字找到某根胳膊、给它挂上道具；
- **让角儿动起来**——加载并循环播放 glTF 里的一段动画（只取最简单的一路，动画的全本留到第 30 章）；
- **从 Blender 到 Bevy**——一个真实模型从建模软件导出、进到 Bevy 的完整工作流，以及阿福这份手写 glTF 的来历。

glTF 的加载归 `bevy_gltf` 管。最常用的两样——`Gltf`（整份文件的资产）和 `GltfAssetLabel`（点名文件里某样东西的标签）——都在 `bevy::prelude` 里；场景组件 `SceneRoot`、以及动画那几样（`AnimationGraph`、`AnimationPlayer`、`AnimationGraphHandle`）也都在 prelude。只有一样要显式引入：等场景展开完毕的信号 `SceneInstanceReady`，从 `bevy::scene` 点名。

配套 crate 是 `code/ch23-gltf`，不需要任何新依赖——glTF 加载与动画都在 Bevy 的默认特性里。本章唯一的美术资产就是木偶阿福（`assets/models/puppet.gltf`）。它不是从 Blender 导出的，而是 `scripts/make_ch23_assets.py` 用 Python 标准库**手写**的一份 glTF：几块带名字的部件拼成身架，再加一段四肢摆动的动画。这么做有意为之——一来零依赖、一条命令就能重建，二来正文里能把 glTF 的真身（一份 JSON）摊开给你看。一键生成：

```console
py -3.11 scripts/make_ch23_assets.py
```

请角儿。

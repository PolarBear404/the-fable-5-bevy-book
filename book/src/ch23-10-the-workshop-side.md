# 作坊那头：DCC 工作流

本章到现在都站在收货的一侧。最后补上寄货的一侧——不是要你现学建模，而是收货人得懂发货人的规矩：知道箱子里的每样东西在作坊里叫什么、从哪个工序来，协作时才说得上话。

做模型的软件统称 **DCC**（Digital Content Creation）工具，最常见的是免费开源的 **Blender**，商业阵营有 Maya、3ds Max、Houdini 等。它们的共同点：都能导出 glTF（Blender 的导出器尤其成熟，Khronos 官方参与维护）。所以工作流是一条直线：**美术在 DCC 里建模、上材质、做动画 → 导出 `.glb` → 丢进 `assets/` → 你按本章的手艺开箱**。

两头的名词一一对应，这张图值得贴在墙上：

![手绘示意图：左栏是 Blender 侧的概念——物体与层级、物体名、材质名、动画名（Action）、自定义属性（Custom Properties）、摆好的相机与灯；右栏是 Bevy 侧的落点——实体树、Name 组件、GltfMaterialName 与 Material N/std、named_animations 与 AnimationClip、GltfExtras、Camera3d 与 PointLight；中间一列 glTF 字段（nodes、name、materials、animations、extras、cameras/KHR_lights_punctual）作为桥，箭头两两相连](images/ch23/fig-23-13-dcc-pipeline.svg)

<span class="caption">Figure 23-13：从 Blender 到 ECS——名字、材质、动画、附注，各走各的桥，全部有迹可循</span>

逐条过桥，每条都在本章亲手验证过：

- **物体层级 → 节点树 → 实体树**。Blender 大纲视图里的父子关系，导出成 `nodes`，落地成 `ChildOf`/`Children`（23.6 的树）；
- **物体名 → `name` → `Name` 组件**。挂灯笼凭的就是它；
- **材质名 → `materials[].name` → `GltfMaterialName` + 两本账**。换漆凭前者，借漆凭 `/std` 标签（23.7）；
- **动画（Blender 里叫 Action）→ `animations` → `AnimationClip`**。按名取走 `named_animations`，按序取走 `Animation{N}`（23.8）；
- **自定义属性 → `extras` → `GltfExtras`**。美术在物体上随手加的键值对，就是你在 23.6 念的货单——挂点、交互标记、掉落表，都能走这条道；
- **相机与灯 → `cameras` / KHR 扩展 → `Camera3d` / 各式 Light**。默认照单全收，23.4 的重影与 23.5 的开关都源于此。

glTF 本体之外的能力（灯就是一例）由 **KHR 扩展**补充，装箱时写进 `extensionsUsed`——23.2 节 JSON 里那行 `KHR_lights_punctual` 就是。Bevy 支持一批常用扩展：灯、自发光强度、透光、清漆之类的高级材质参数——其中几种要开对应的 cargo feature（如 `pbr_multi_layer_material_textures`，清单见附录 B）；不支持的扩展若被文件标为**必需**，加载会报错。网格还能携带规范之外的**自定义顶点属性**（下划线开头，如 `_BARYCENTRIC`），在 `GltfPlugin` 上用 `add_custom_vertex_attribute` 注册后即可随箱进来——那是自定义 shader 的原料，第 36 章的地界。

## 名字就是接口

上面每条桥几乎都途经“名字”，这不是巧合，值得单独立一节说透：**在 glTF 工作流里，名字是美术与程序之间事实上的 API。**

- 挂点靠 `Name` 匹配——`hang_lantern` 做的是逐字的字符串比对，美术把 `LeftArm` 改名成 `Arm.L`（Blender 的惯用后缀风格），比对必然落空，而且**无声**（`iter_descendants` 匹配不到就是匹配不到，第 4 章的老话）；
- 换漆靠 `GltfMaterialName`——材质改名同理；
- 动画的绑定更隐蔽：每条动画通道找目标，靠的是**从动画根到目标节点的名字路径**算出的一个稳定 ID——装卸工把它作为 `AnimationTargetId` 组件直接写在各节点实体上，旁边配一枚指回播放器的 `AnimatedBy`。ID 是名字路径的哈希，名字一变 ID 就变——**给节点改名等于剪断已烤好的动画连线**，这在 DCC 里改起来毫无手感，在游戏里表现为“某个部位突然不动了”，还是哑的。

所以成熟团队都有一份**命名规约**：挂点前缀、左右后缀、材质命名格式，写成文档，两边共同遵守；关键标记不走名字走 `extras`（结构化数据总比字符串约定可靠）。我们的巧手斋规约压缩成一句话就是：节点名 PascalCase、材质名带 Afu 前缀、挂点写 extras——你在 23.3 的花名册里看到的那份整齐，是规约的产物，不是运气。

## 骨骼：这箱没有的东西

细心的话你会发现装箱单上有一类货全程零出场：`Skin`。阿福是**杖头木偶**——头、袖、杆都是刚性部件，动画拧的是节点的 `Transform`，部件自己不变形。而人形角色的“皮肉”是一整张网格，要跟着一副**骨架**（skeleton）弯曲拉伸，这门手艺叫**蒙皮**（skinning）：glTF 里对应 `skins` 数组、`Skin{N}` 与 `InverseBindMatrices{N}` 标签，Bevy 侧对应 `GltfSkin` 资产与 `SkinnedMesh` 组件。原理、权重、与动画系统的合演，全是第 30 章的正戏——到时候你会发现本章的三件套（clip、graph、player）原样通用，蒙皮只是“动画写进去的地方”从 Transform 换成了骨骼矩阵。

最后一句给还没装 Blender 的读者：**不装一点也不耽误本章**。阿福证明了 glTF 就是一份格式公开的数据——`scripts/make_ch23_assets.py` 用四百行 Python 徒手写出了它。哪天你从网上下载模型（Sketchfab、Poly Haven 这类站点都直供 glTF），或者接到美术同事的第一只箱子，开箱的手艺一模一样。

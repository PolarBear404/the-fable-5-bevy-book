# glTF 与 3D 资产

光配齐了，得月楼却还缺角儿。老鲁凿的胶囊木人顶了两章的缸——没脸、没手、不会动，看客已经开始编排它。老雷一咬牙，托府城的**巧手斋**木偶作坊定制了一尊杖头木偶，取名**阿福**：圆头戏曲脸、朱红大袍、一双甩袖、一根主杆，还随箱附一折挥袖的身段。今天，箱子到货了。

这只“箱子”就是 **glTF**——3D 资产的标准交付格式。它不是一张图片那样的单件货，而是一整箱：网格、材质、贴图、节点树、相机、灯、动画，全在一个文件家族里。做模型的人在 Blender 这类建模软件里干活，导出成 glTF；你的游戏把它加载进来，箱子里的东西就变成一棵实体树，落进 ECS，跟你手搓的实体平起平坐。本章从收货开始，把开箱这门手艺一件件过手：

- **第一次开箱**——一个组件把阿福请上台；顺手看一眼漏写标签时那条著名的谜语报错；
- **箱子解剖**——glTF 文件里到底装了什么：JSON 主档、二进制几何、贴图，以及把三件套打成单件的 `.glb`；
- **装箱单与标签**——`Gltf` 总资产的花名册，`GltfAssetLabel` 的提货单语法，只提一件货的取法；
- **换场**——同一箱里的第二个场景，换景不重启；顺带撞上一次“作坊的相机抢镜头”；
- **开箱的讲究**——`GltfLoaderSettings` 逐个旋钮：灯请不请、相机请不请，以及“同一箱只认头一回规矩”的坑；
- **回执与按名找人**——`WorldInstanceReady` 观察者、打印整棵实体树、凭 `Name` 给阿福的袖口挂灯笼；
- **材质两本账**——`GltfMaterial` 与 `#Material1/std`：一罐漆记两本账，换漆与借漆各走哪本；
- **让阿福动起来**——动画三件套开锣，再亲手踩一次“谱子抽了、台上冻住、日志无声”的哑巴坑；
- **转向**——glTF 的“前”是 +Z，Bevy 的“前”是 −Z：让甲乙两尊木偶各走两步，谁在倒退一目了然；
- **作坊那头**——Blender 侧的工作流：名字、材质、动画、自定义数据是怎么一路流进 ECS 的；
- **木偶戏开演**——全章合龙：开箱、挂灯、放动画、拖着转台看。

门牌先报一遍：本章的类型几乎全出自 `bevy_gltf`（`Gltf`、`GltfAssetLabel`、`GltfMaterial`、`GltfLoaderSettings`、`GltfExtras`、`GltfMaterialName`……）；把场景挂进世界的 `WorldAssetRoot` 与就绪事件 `WorldInstanceReady` 住在 `bevy_world_serialization`（第 32 章的主场，本章只用它开箱）；动画三件套（`AnimationPlayer`、`AnimationGraph`、`AnimationGraphHandle`）出自 `bevy_animation`，第 30 章深谈。常用的几位都在 `bevy::prelude`，其余从 `bevy::gltf`、`bevy::world_serialization` 显式引入，用到时正文点名。

配套 crate 是 `code/ch23-gltf`，`Cargo.toml` 没有新花样——glTF 加载与动画都在默认 feature 里：

```toml
{{#include ../../code/ch23-gltf/Cargo.toml:deps}}
```

主角阿福本尊是一份**纯 Python 手写的 glTF**，由脚本一键合成：

```console
py -3.11 scripts/make_ch23_assets.py
```

别的章拿脚本合成贴图和音频，这章直接合成整只箱子——不是炫技：glTF 的主档本来就是一份 JSON 文本，亲手写过一遍，你对“箱子里有什么”的理解会比任何示意图都扎实。23.2 节就把它摊开看。

开工。

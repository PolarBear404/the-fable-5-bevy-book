# 请角儿上台

阿福的提货单就摆在 `assets/models/puppet.gltf`。把它请上台，核心只有一句：

```rust
{{#include ../../code/ch23-gltf/examples/listing-23-01.rs:load}}
```

<span class="caption">Listing 23-1：用 `SceneRoot` 加载一份 glTF 场景（examples/listing-23-01.rs）</span>

剩下的地面、主光、机位都是第 21、22 章的老相识，搁在 `stage` 里不抢戏。开台：

```console
cargo run -p ch23-gltf --example listing-23-01
```

![一座灰青色的空台子上，立着一个红袍、米白脑袋的方块小人：一块躯干、一颗头、两条垂着的胳膊、两条腿，脚边在地上拖出一道斜影](images/ch23/fig-23-01-puppet-on-stage.png)

<span class="caption">Figure 23-1：木偶阿福上台了——一份 glTF 场景加载、展开，落在台面上</span>

阿福站住了。就这一句 `spawn`，做成了一连串事：

`SceneRoot` 是个**组件**，裹着一张 `Handle<Scene>`——一份「场景」资产的提货单。把它挂到实体上，引擎看见了，就在后台把这份场景**展开**成一串实体（阿福的躯干、头、四肢各是一个），全都挂在这个实体名下做子实体。你 spawn 的这一个实体，是它们的根。

关键在那行标签。`asset_server.load(...)` 要的不是文件路径，而是**文件里某样东西**的地址。`GltfAssetLabel::Scene(0)` 就是「第 0 号场景」这张标签，`.from_asset("models/puppet.gltf")` 把它贴到文件上，合起来正是资产路径 `"models/puppet.gltf#Scene0"`——`#Scene0` 就是那张标签的文字形态。

那能不能省掉标签、直接 `load("models/puppet.gltf")` 把整份文件摆上台？不能，而且这是新手最常踩的第一脚。下一节先把提货单拆开看清楚，再回头踩这一脚。

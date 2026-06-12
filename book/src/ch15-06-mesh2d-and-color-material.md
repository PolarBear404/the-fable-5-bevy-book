# Mesh2d 与 ColorMaterial：不用画的道具

《渡口夜话》还缺一轮满月。小棠提笔要画，老雷拦下：“一个正圆还要劳驾画师？”确实不用——圆、环、三角这类几何形状，引擎可以现场“铸”出来，根本不需要图片文件。

干这活的是另一套渲染管线。`Sprite` 本质上是“一张矩形的图”；而 **Mesh**（网格——由顶点和三角形拼成的几何形状）想是什么形状就是什么形状。3D 世界里所有模型都是 Mesh（第 21 章的正题），2D 里它同样能用——拍扁了用，像桌上的剪纸。

一只 2D 网格实体由两个组件拼成，各管一半：

- **`Mesh2d`**——形状。内里是 `Handle<Mesh>`：网格也是资产，住 `Assets<Mesh>` 货架；
- **`MeshMaterial2d<ColorMaterial>`**——皮相。**Material**（材质——“这个表面怎么着色”的完整说明书）也是资产，2D 的标配材质是 `ColorMaterial`：一块颜色，可选再贴一张图。

形状从哪来？第 12 章几何课上的那批图元（`Circle`、`Annulus`、`Rectangle`、`Triangle2d`……）在这儿兑现第二重身份：它们都能 `From` 成 `Mesh`，所以 `meshes.add(Circle::new(64.0))` 一句话就把“数学上的圆”铸成“可渲染的圆盘”。月亮、光晕、远山、江面、星斗，全场布景一张图片不用：

```rust
{{#include ../../code/ch15-sprites/examples/listing-15-11.rs:moon}}
```

<span class="caption">Listing 15-11（节选一）：月亮与光晕——形状铸成 Mesh、颜色调成 ColorMaterial，两张提货单各管各的（examples/listing-15-11.rs）</span>

```rust
{{#include ../../code/ch15-sprites/examples/listing-15-11.rs:shapes}}
```

<span class="caption">Listing 15-11（节选二）：远山、江面与十四颗星——星星共用一份网格一份材质，只是提货单多发了几张（examples/listing-15-11.rs）</span>

两处细节值得圈出来。其一，`materials.add(Color::srgb(...))` 能直接收颜色，因为 `ColorMaterial` 实现了 `From<Color>`——这是便捷写法，全写开是 `ColorMaterial { color, ..default() }`。其二，十四颗星**共用同一份网格与材质**，循环里只克隆提货单——第 14 章“clone 只加计数不复制货”的省钱诀窍，在渲染资产上同样成立，而且渲染器看到同料同形的实体还能批量处理。

## 忘了上料

小棠铸伴月星盘时漏了一步——只给了形状，没给材质。猜猜会发生什么？按第 3 章的直觉，`Mesh2d` 是组件，挂上就该生效；可渲染管线的答案是：

```rust
{{#include ../../code/ch15-sprites/examples/listing-15-11.rs:accident}}
```

<span class="caption">Listing 15-11（节选三）：一只忘了配材质的 Mesh2d，与一只挂着默认材质的（examples/listing-15-11.rs）</span>

```console
cargo run -p ch15-sprites --example listing-15-11
```

```text
老雷：伴月盘呢？西天怎么空了一块？
小棠：形状铸了，料忘了配……旁边那块洋红的挂了默认料，等于贴了张催料单。
```

![夜空场景：右上满月带光晕，远山江面星斗俱全；左上角一块刺眼的洋红圆盘，它右边本该有伴月盘的位置空无一物](images/ch15/fig-15-13-moon-and-magenta.png)

<span class="caption">Figure 15-13：右上的月亮一切正常；左上洋红的圆挂了默认材质，它右边那只“裸网格”干脆没画出来</span>

**没有材质的 `Mesh2d` 什么也不画**——不报错、不警告，安静地隐身。这跟 `Sprite` 的脾气不同：`Sprite` 一个组件包圆了形状与皮相，而网格渲染管线认的是“形状 + 材质”这对组合，缺谁都不上场。“我的圆怎么不见了”属于 2D 新手高频问题，答案十有八九是忘了 `MeshMaterial2d`。

旁边那块洋红是另一课：`MeshMaterial2d::<ColorMaterial>::default()` 给的是**默认材质句柄**，Bevy 把它预设成了刺眼的洋红色（`srgb(1.0, 0.0, 1.0)`）——游戏行业的传统信号色，意思是“这儿缺正经材质”。日后在哪儿见到洋红，第一反应都该是：有人忘了上料。

## ColorMaterial 还能贴图

`ColorMaterial` 一共四个字段：`color` 之外还有 `texture: Option<Handle<Image>>`（贴一张图上去，颜色继续当染色乘上来——相当于 `Sprite` 的 image + color 组合）、`alpha_mode`（半透明怎么混：`Blend` 默认、`Opaque` 不透明、`Mask` 镂空）、`uv_transform`（贴图坐标的仿射变换，能滚动、缩放贴图）。也就是说，一张贴了图的矩形网格能干 `Sprite` 的大部分活——那什么时候用谁？

| | `Sprite` | `Mesh2d` + `ColorMaterial` |
|---|---|---|
| 形状 | 永远是矩形 | 任意几何形状 |
| 图集/九宫格/翻面 | 全套内建 | 无，自己动 uv |
| 批量渲染 | 引擎深度优化，万级无压力 | 同网格同材质可合批 |
| 自定义着色 | 不行 | 换自定义 Material 即可（第 36 章） |

经验法则：**带图的矩形用 `Sprite`，几何形状或要上特效的用 Mesh2d**。第 20 章的打砖块两边都会用到。3D 那边的 `Mesh3d` + `MeshMaterial3d<StandardMaterial>` 是同一个模式的立体版——第 13 章结尾你已经瞥见过，第 21 章正式开讲。

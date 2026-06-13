# Picking 与相机控制

第 24 章的材质球画廊只能「点画面拨一个开关」：程序知道鼠标按下了，却不知道你按的是哪颗球。真正的 3D 交互要再进一步：鼠标落在画面上，沿相机射出一条线，打到哪个实体，就把 `Click`、`Over`、`Drag` 这样的指针事件交给那个实体处理。

这就是 `bevy_picking`（拾取系统）的工作：把鼠标、触摸、笔这样的指针输入，翻译成「指针正在悬停哪个实体」「刚刚点击了哪个实体」「正在拖拽哪个实体」。它不是只给 3D mesh 用；Bevy 0.18.1 里有 mesh、sprite、UI 三套内置拾取后端，可以一起工作。

本章配套 crate 是 `code/ch25-picking-camera-control`。它第一次用到 `bevy_camera_controller` 里的现成相机控制器，所以依赖要开 `free_camera` feature：

```toml
{{#include ../../code/ch25-picking-camera-control/Cargo.toml:deps}}
```

<span class="caption">Listing 25-0：第 25 章额外开启 `free_camera` feature（Cargo.toml）</span>

本章要走六步：

- 看懂 picking 管线：输入、后端、HoverMap、指针事件、Observer；
- 给 3D mesh 加 `MeshPickingPlugin`，用 `Pointer<Over>` / `Pointer<Click>` 响应点选；
- 写一次拖拽，顺手踩明 `Pointer<Drag>::delta` 是屏幕像素，不是世界坐标；
- 看 sprite 与 UI 后端如何并存，以及 `Pickable` 怎样决定遮挡和穿透；
- 用 `FreeCameraPlugin` 给 3D 场景接上现成自由视角相机；
- 把它们合成一个小舞台：左键点选/拖拽物体，右键控制相机。

![一张 picking 管线图：PointerInput 进入 Backends，mesh、sprite、UI 后端产生命中结果，HoverMap 按深度和 Pickable 规则整理，最终生成 Over、Click、Drag 等 Pointer 事件并交给实体上的 observe 回调](images/ch25/fig-25-01-picking-pipeline.svg)

<span class="caption">Figure 25-1：Picking 不是一个系统，而是一条管线——输入先进来，后端做命中测试，最后才变成实体可观察的指针事件</span>

本章只讲 Bevy 自带的拾取与相机控制器。更重的选择，例如物理引擎射线、编辑器 gizmo、运行时检查器，会在第 27 章和后面的工程化章节再碰。


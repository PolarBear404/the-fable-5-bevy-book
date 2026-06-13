# Solari：实验性的实时光追

`bevy_solari` 是 Bevy 0.18.1 里实验性的实时光追方向。它不属于本章画质面板的可运行主例子，因为它需要额外 feature、GPU ray tracing 能力和更严格的渲染配置；但它在“画质”这个范围里必须被点名。

源码里 `bevy_solari` 通过 `bevy_internal` 在 `bevy_solari` feature 下导出。核心入口是：

- `SolariPlugins`：插件组，默认加入实时 Solari lighting；
- `SolariLighting`：挂在相机上的实时光追光照组件；
- `RaytracingMesh3d`：给参与 Solari 场景的 mesh 建立 ray tracing 数据；
- `PathtracingPlugin`：非实时 path tracer，主要用于验证。

官方示例 `vendor/bevy/examples/3d/solari.rs` 还展示了几个硬条件：相机需要 `CameraMainTextureUsages::default().with(TextureUsages::STORAGE_BINDING)`，需要 `Msaa::Off`，参与光追的 mesh 要挂 `RaytracingMesh3d`。源码注释也写明：这是一个正在演进的实验功能，不是普通项目默认应该打开的后处理选项。

所以本书在这里给出边界，而不把 Solari 塞进画质开关面板：

- 本章主线关注稳定、默认 feature 下能编译运行的相机画质组件；
- Solari 涉及 GPU feature 检查、资源绑定、ray tracing mesh 数据和不同的性能预算；
- 它和 Bloom、TAA、Tonemapping 不是同一层开关：Solari 改的是光照求解方式，后处理改的是相机输出。

等本书进入更底层的渲染扩展、平台能力检测或实验功能附录时，再把 Solari 作为单独主题更合适。现在你只需要知道：它存在，名字在 `bevy::solari` 下，但不要把它当成“把画质调高”的普通按钮。

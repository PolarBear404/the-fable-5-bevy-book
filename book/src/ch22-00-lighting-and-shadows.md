# 光照与阴影

得月楼的立体布景立起来了，可那盏堂灯从第 21 章开工到散场一动没动：挂在哪、多亮、什么颜色、投不投影子，全是没拆的封。材质墙左上角那颗镜面金属球，也还黑着脸等一个值得照的世界。老雷请来一位**掌灯的**——管的就是这一摊：天光、堂灯、追光、影子。他要装的不是一盏灯，是一整套**昼夜光照切换台**：一按空格，从黎明翻到正午、黄昏、入夜，整座园子的光随之改天换地。

本章把第 21 章欠下的三笔债一次还清：**灯的全家**（平行光、点光、聚光、环境光）、灯的**量纲**（21 章卖的关子——亮度到底拿什么单位记），以及**影子**。还会给那颗金属球补上它等了一章的环境光照，最后捎带一层雾。

跟着掌灯的调台子，一档一档来：

- **把太阳请上台**——`DirectionalLight` 当太阳，认识三种灯各自的亮度量纲（勒克斯、流明），看清平行光的方向由「旋转」而非「位置」决定；
- **开影子**——一行 `shadows_enabled` 让布景投下影子；再拆开影子的三把旋钮（贴图分辨率、级联、bias），踩一脚 shadow acne 的坑再填上；
- **点光与聚光**——`PointLight` 当夜里的灯笼、`SpotLight` 当台口的追光，看流明、射程、光锥角各管什么；
- **环境光**——把第 21 章那层「兜底的轮廓光」`AmbientLight` 讲全，顺手踩一脚「组件当资源插」的编译错误；
- **给金属一个世界**——`EnvironmentMapLight` 环境光照，把一张立方体贴图当周遭，镜面金属终于照出东西；
- **雾**——`DistanceFog` 给远处蒙一层，再概述体积雾与光探针这两件进阶家什；
- **昼夜切换台**——四档天色合龙，空格轮换。

这些类型分住两个 crate：灯的全家、阴影配置、环境光照都在 `bevy_light`；雾 `DistanceFog` 在 `bevy_pbr`。常用的灯（`DirectionalLight`、`PointLight`、`SpotLight`、`AmbientLight`、`GeneratedEnvironmentMapLight`）和 `DistanceFog` 都在 `bevy::prelude` 里；只有几样配置项要显式引入：`DirectionalLightShadowMap`、`CascadeShadowConfigBuilder` 从 `bevy::light`，装配立方体贴图要用的 `TextureViewDescriptor`、`TextureViewDimension` 与相机的 `Hdr` 从 `bevy::render`。用到时再点名。

配套 crate 是 `code/ch22-lighting`，不需要任何新依赖。本章唯一的美术资产是一张程式化的「暖阁」立方体贴图（`assets/textures/skybox.png`，给金属当反射的世界），由 `scripts/make_ch22_assets.py` 用 PIL 现场绘制，一键就位：

```console
py -3.11 scripts/make_ch22_assets.py
```

掌灯，开台。

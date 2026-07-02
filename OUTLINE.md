# The Fable 5 Bevy Book 全书大纲（已审定）

> 基于 Bevy 0.19.0（vendor/bevy-0.19 @ c6f634c）的 59 个子 crate 与 420 个官方示例（`[[example]]` 条目）盘点而成。
**状态：已审定（2026-06-12）；2026-07-02 依 0.19 迁移评估修订（migration/0.19-outline-assessment.md，P-1～P-13 全部批准）。** 本文件是章节范围的唯一依据：调整章节须由用户明确发起，改动后同步 PROGRESS.md。
每章「覆盖」为模块级范围，精确 API 在写作时按 vendor/bevy-0.19 源码逐项核实。
附录 D（第三方生态）是否纳入，留到附录动工时由用户决定。

---

## 第一部分　起步

### 第 1 章　认识 Bevy
- **目标**：理解游戏引擎做什么、ECS 是什么思维模型、Bevy 的模块化设计与版本节奏；装好工具链
- **覆盖**：引擎全景；ECS vs 面向对象；workspace 与快速编译配置（profile、可选 dynamic_linking）
- **示例**：无代码量，环境验证 `cargo check`

### 第 2 章　第一个 Bevy App
- **目标**：跑起第一个窗口程序，建立"App = Plugin 的容器、逻辑 = System"的心智模型
- **覆盖**：bevy_app（App、Plugin、DefaultPlugins/MinimalPlugins）、bevy_winit 窗口初见、游戏主循环
- **示例**：hello world → 开窗口 → 屏幕上一个会动的 Sprite（钩子，细节后述）

## 第二部分　ECS——Bevy 的心脏

### 第 3 章　Entity 与 Component
- **目标**：会定义组件、生成/销毁实体；理解 required components 这一 0.15+ 核心惯用法
- **覆盖**：bevy_ecs：Entity、#[derive(Component)]、required components、Bundle（简述）、Commands 的延迟语义
- **示例**：纯逻辑 demo（无渲染），打印实体清单

### 第 4 章　System 与 Query
- **目标**：掌握"函数就是 System"；用 Query 精确取数据
- **覆盖**：系统参数、Query 与过滤器（With/Without/Or/Changed/Added）、Single、借用冲突与 ParamSet、Local
- **示例**：多组件查询的小型模拟（生物群体属性更新）

### 第 5 章　Resource——全局唯一数据
- **目标**：区分"每实体数据（Component）"与"全局数据（Resource）"
- **覆盖**：#[derive(Resource)]、Res/ResMut、init_resource、资源的初始化顺序、资源的本质——单例实体上的组件（0.19 起），Res/ResMut 是其语法糖
- **示例**：计分板/游戏配置

### 第 6 章　Schedule 与执行顺序
- **目标**：搞清系统何时跑、按什么顺序跑——Bevy 程序行为的根源
- **覆盖**：Main schedule 全家（Startup/Update/FixedUpdate…）、before/after/.chain()、SystemSet、run_if、同步点与命令应用时机
- **示例**：可观察执行顺序的打印实验 + 条件运行

### 第 7 章　Message——缓冲消息
- **目标**：用消息解耦系统间通信
- **覆盖**：bevy_ecs message 模块：Message、MessageReader/MessageWriter、双缓冲与清理时机（注意：0.17 起由旧 Event 改名）
- **示例**：碰撞消息驱动音效/计分的解耦演示

### 第 8 章　Event 与 Observer
- **目标**：掌握即时响应式编程：观察者、实体生命周期事件
- **覆盖**：bevy_ecs observer 模块：Event/EntityEvent、add_observer、observer 运行条件（run_if）、实体生命周期事件（插入/移除/销毁）、组件钩子
- **示例**：装备系统——装上/卸下组件即时触发联动

### 第 9 章　实体关系与层级
- **目标**：父子树与自定义关系
- **覆盖**：ChildOf/Children、Relationship 机制、自引用关系（allow_self_referential）、层级遍历、级联销毁、Transform 传播预告
- **示例**：载具与乘员/装备槽

### 第 10 章　State——游戏状态机
- **目标**：菜单/游戏中/暂停的工程化表达
- **覆盖**:bevy_state：States/SubStates/ComputedStates、OnEnter/OnExit、转换调度、state-scoped entities
- **示例**：菜单 ↔ 游戏 ↔ 暂停三态切换

### 第 11 章　深入 ECS
- **目标**：补齐直接操作 World 的能力，看懂高级代码
- **覆盖**：EntityRef/EntityMut、exclusive systems、World API、自定义 SystemParam、QueryData derive、变更检测细节、Disabled 实体、资源实体与混合查询（IsResource、资源上的 hooks/observers）、Query 连续访问（contiguous_iter）
- **示例**：小型存档/检查器工具片段

## 第三部分　画面、资产与交互（2D 路线）

### 第 12 章　Transform 与坐标系统
- **覆盖**：bevy_transform（Transform/GlobalTransform、传播）、bevy_math（Vec2/Vec3/Quat/Dir、Rect、primitives）、坐标系约定
- **示例**：太阳系轨道（官方 movement/transforms 类示例改编）

### 第 13 章　Camera 与视口
- **覆盖**：bevy_camera：Camera2d/Camera3d、正交/透视投影、viewport、多相机分屏、清屏色、RenderLayers
- **示例**：双相机分屏 + 小地图

### 第 14 章　Asset 系统
- **覆盖**：bevy_asset：AssetServer、Handle、加载状态与异步性、嵌入资产、热重载、自定义 AssetLoader、运行时资产保存（save_using_saver/SavedAsset）、asset processing 概述
- **示例**：加载进度条 + 自定义文本格式 loader

### 第 15 章　2D 渲染：Sprite 与图集
- **覆盖**：bevy_sprite(_render)：Sprite、TextureAtlas、九宫格切片、anchor、2D Mesh 与 ColorMaterial；bevy_color 系统性介绍
- **示例**：精灵帧动画 + 图集

### 第 16 章　文本与字体
- **覆盖**：bevy_text：Text2d（世界内文本）、字体资产、TextLayout/TextFont、FontSource 与语义字体族、可变字体（FontWeight/FontWidth/FontStyle）、系统字体发现（system_font_discovery）、FontSize 响应式单位、LetterSpacing 字距；UI 文本预告
- **示例**：伤害飘字

### 第 17 章　输入处理
- **覆盖**：bevy_input：ButtonInput<KeyCode/MouseButton>、鼠标移动/滚轮、触摸、Gamepad（bevy_gilrs）；输入映射的工程模式
- **示例**：键鼠手柄三合一控制同一角色

### 第 18 章　时间、定时器与 FixedUpdate
- **覆盖**：bevy_time：Time、Virtual/Real、Timer/Stopwatch、延迟命令（commands.delayed()）、fixed timestep 语义与渲染插值
- **示例**：子弹冷却 + 固定步进物理对比实验

### 第 19 章　音频
- **覆盖**：bevy_audio：AudioPlayer、PlaybackSettings、音量/暂停控制、空间音频
- **示例**：BGM + 音效 + 距离衰减

### 第 20 章　项目实战 I：完整的 2D 游戏
- **目标**：综合 3–19 章，从空目录做出带菜单、计分、音效的 Breakout（打砖块）
- **覆盖**：综合应用 + 项目组织（插件划分模块）
- **示例**：对标官方 showcase/breakout.rs（0.19 起 games/ 更名 showcase/）但按本书体系重构、分步成章

## 第四部分　三维世界

### 第 21 章　3D 入门：Mesh 与 Material
- **覆盖**：bevy_mesh（Mesh3d、内置 primitives、顶点数据）、MeshMaterial3d<StandardMaterial> 初见、PBR 直觉
- **示例**：基础几何体场景

### 第 22 章　光照与阴影
- **覆盖**：bevy_light：Directional/Point/Spot/RectLight（0.19 新增矩形面光源）、AmbientLight、阴影贴图参数与接触阴影、EnvironmentMapLight、Skybox 与 Atmosphere 大气散射（0.19 起归 bevy_light）、反射探针（含 PCCM 视差校正）与光探针混合、体积雾概述
- **示例**：昼夜光照切换台（含大气散射与天空盒）

### 第 23 章　glTF 与 3D 资产
- **覆盖**：bevy_gltf：GltfAssetLabel、WorldAssetRoot（0.19 起 SceneRoot 更名）、GltfMaterial 与 #Material0/std 材质标签、按命名取实体、与 DCC 工具（Blender）的工作流
- **示例**：加载带动画的角色模型
- **0.20 前瞻**：官方已预告 glTF 加载将接入 BSN，届时本章需一次跟进

### 第 24 章　PBR 材质深入
- **覆盖**：bevy_pbr：StandardMaterial 全参数（金属度/粗糙度/法线/自发光/透明/clearcoat…以源码为准）、双面、深度偏移
- **示例**：材质球画廊

### 第 25 章　Picking 与相机控制
- **覆盖**：bevy_picking（mesh/sprite/UI 拾取、指针事件与 Observer 集成、连击计数 Pointer&lt;Click&gt;::count）、bevy_camera_controller（现成相机控制器；PanCamera 0.19 起默认鼠标平移）
- **示例**：点选/拖拽 3D 物体 + 自由视角相机

### 第 26 章　画质：后处理与抗锯齿
- **覆盖**：bevy_core_pipeline（HDR、tonemapping；Skybox/大气见第 22 章）、bevy_post_process（bloom、景深、运动模糊、Vignette 暗角、LensDistortion 镜头畸变…）、bevy_anti_alias（MSAA/FXAA/TAA…）、bevy_solari 实验性光追概述
- **示例**：画质开关面板（逐项对比）

### 第 27 章　Gizmos、诊断与开发工具
- **覆盖**：bevy_gizmos(_render)（即时/保留模式调试绘制、文本 Gizmos、Transform Gizmo）、bevy_dev_tools（FPS overlay、无限网格、诊断浮层等）、bevy_diagnostic
- **示例**：给 Breakout 加调试层

## 第五部分　UI

### 第 28 章　UI 基础：Node 与布局
- **覆盖**：bevy_ui(_render)：Node、Flexbox/CSS Grid 布局、尺寸单位、层叠与 z、UI 相机/渲染目标、ImageNode、UI 文本
- **示例**：响应式 HUD 布局

### 第 29 章　UI 交互与控件（含项目实战 II）
- **覆盖**：Interaction、bevy_input_focus、滚动、bevy_ui_widgets（无样式控件，0.19 起默认启用）、EditableText 文本输入（光标/选区/多击/IME/多行）、bevy_settings 设置持久化（非默认 feature）、bevy_clipboard 提及、bevy_feathers（仍实验性，概述；其 BSN 写法见第 32 章）、UI 材质
- **示例**：完整的设置界面（音量滑条、键位重绑定、分辨率下拉、玩家名文本框）＋ 设置持久化到磁盘

## 第六部分　动画、架构与工程化

### 第 30 章　动画系统
- **覆盖**：bevy_animation：AnimationPlayer、AnimationGraph、骨骼动画、变形目标、动画事件、可动画属性与曲线
- **示例**：角色动画状态切换（idle/walk/run 混合）

### 第 31 章　Reflect——运行时反射
- **覆盖**：bevy_reflect：derive(Reflect)、TypeRegistry、动态读写、asset handle 的序列化（HandleSerializeProcessor）、为什么 scene/inspector/remote 都依赖它（0.19 起根模块拆分为 structs/enums 等子模块）
- **示例**：通用属性查看器

### 第 32 章　BSN——场景系统的现在与未来
- **覆盖**：bevy_scene（0.19 起即 BSN）：bsn!/bsn_list!、Scene/SceneList、patch 式组合与分层、SceneComponent、Template/FromTemplate、spawn_scene/queue_spawn_scene、on() 观察者与 #Name 实体引用；bevy_world_serialization（旧场景系统更名）：DynamicWorld、世界序列化与存档、与 Reflect 的关系
- **示例**：双示例——BSN 组合式场景（对标 scene/bsn.rs）＋ 关卡存档（DynamicWorld 保存当前世界→重启加载，对标 scene/world_serialization.rs）
- **0.20 前瞻**：.bsn 资产加载器落地后本章扩一节资产工作流

### 第 33 章　日志、错误处理与远程调试
- **覆盖**：bevy_log/tracing、系统返回 Result 与错误处理策略、bevy_diagnostic 自定义诊断、bevy_remote（BRP 协议 + 外部检查器、BRP 驱动的集成测试）
- **示例**：接入 BRP 用外部工具实时改组件

### 第 34 章　异步与并行
- **覆盖**：bevy_tasks（线程池、AsyncComputeTaskPool 模式）、并行查询迭代、non-send data（0.19 起 NonSend 资源改称）、与 async 生态（reqwest 等）桥接
- **示例**：后台生成地形 + 异步 HTTP 请求

### 第 35 章　窗口与平台细节
- **覆盖**：bevy_window/bevy_winit 全参数、多窗口、低功耗模式、无头模式、bevy_clipboard 剪贴板、bevy_a11y 无障碍概述（含 AccessibleLabel）、bevy_platform 与 no_std 概述
- **示例**：多窗口工具 + 省电模式

### 第 36 章　自定义 Material 与 Shader
- **覆盖**：bevy_shader（WGSL 入门、ShaderType）、Material trait（3D/2D/UI 自定义材质）、shader 资产与热重载
- **示例**：溶解效果材质

### 第 37 章　渲染架构导览
- **覆盖**：bevy_render：RenderApp 与 extract/prepare/queue 各阶段、渲染 schedule（Core2d/Core3d）与作为普通 system 的渲染 pass（0.19 起取代 render graph）、RenderStartup 与管线初始化、渲染错误恢复（RenderErrorHandler）、遮挡剔除、自定义渲染管线的地图（目标：能读懂 shader_advanced 示例）
- **示例**：导读式，配最小自定义 pipeline

### 第 38 章　发布你的游戏
- **覆盖**：发布构建优化、性能分析（stress_tests 用法）、Wasm/Web 部署、移动端概述（bevy_android/iOS、安卓 activity feature 显式选择）、资产打包、CI
- **示例**：把 Breakout 发布到 Web

## 附录

- **附录 A**　编译加速与安装疑难（linker、dynamic_linking、增量编译、目标盘符）
- **附录 B**　Cargo features 完整清单（按 0.19 根 Cargo.toml 整理；0.19 起 feature 体系重构为 profile 式，default = ["2d", "3d", "ui", "audio"]）
- **附录 C**　ECS 速查表（SystemParam、查询过滤器、调度 API 一页通）
- **附录 D**　生态系统地图（候选：avian/bevy_rapier 物理、bevy_egui、leafwing-input-manager、tilemap 等——**是否纳入由用户决定**）
- **附录 E**　版本迁移方法论（如何读官方迁移指南；以本书 0.18.1→0.19 迁移全程为实例）
- **附录 F**　中英术语对照表

---

## 覆盖对照 I：59 个子 crate → 章节

| crate | 章节 |
|---|---|
| bevy_app | 2, 6 |
| bevy_ecs | 3–11（全书根基） |
| bevy_state | 10 |
| bevy_transform | 12 |
| bevy_math | 12, 20, 附C |
| bevy_camera | 13 |
| bevy_camera_controller | 25 |
| bevy_asset | 14 |
| bevy_image | 14, 15 |
| bevy_sprite / bevy_sprite_render | 15 |
| bevy_color | 15（系统介绍），全书使用 |
| bevy_text | 16, 28 |
| bevy_input | 17 |
| bevy_gilrs | 17 |
| bevy_time | 18 |
| bevy_audio | 19 |
| bevy_mesh | 21, 36 |
| bevy_pbr | 21, 22, 24 |
| bevy_material（0.19 自 bevy_pbr/render 拆出，多为 re-export） | 24, 36 |
| bevy_light | 22 |
| bevy_gltf | 23 |
| bevy_picking | 25 |
| bevy_core_pipeline | 13, 26 |
| bevy_post_process | 26 |
| bevy_anti_alias | 26 |
| bevy_solari | 26（实验性概述） |
| bevy_gizmos / bevy_gizmos_render | 27 |
| bevy_dev_tools | 27 |
| bevy_diagnostic | 27, 33 |
| bevy_ui / bevy_ui_render | 28, 29 |
| bevy_ui_widgets | 29 |
| bevy_input_focus | 29 |
| bevy_feathers | 29（实验性概述） |
| bevy_settings（0.19 新增，非默认 feature） | 29 |
| bevy_clipboard（0.19 新增） | 29, 35 |
| bevy_animation | 30 |
| bevy_reflect | 31 |
| bevy_scene（0.19 起即 BSN 新场景系统） | 32 |
| bevy_world_serialization（旧 bevy_scene 更名） | 23, 32 |
| bevy_log | 33 |
| bevy_remote | 33 |
| bevy_tasks | 34 |
| bevy_window / bevy_winit | 2, 35 |
| bevy_a11y | 35 |
| bevy_platform | 34, 35 |
| bevy_android | 38 |
| bevy_shader | 36 |
| bevy_render | 36, 37 |
| bevy_derive / bevy_macro_utils / bevy_ptr / bevy_encase_derive / bevy_utils / bevy_internal / bevy_dylib | 内部实现 crate，不单独成章（dylib 见附 A） |

## 覆盖对照 II：官方示例 37 类 → 章节

| 示例目录 | 章节 |
|---|---|
| ecs | 3–11 |
| app | 2, 6 |
| state | 10 |
| transforms / movement | 12, 18 |
| camera | 13, 25 |
| asset | 14 |
| 2d | 15, 16 |
| input | 17 |
| time | 18 |
| audio | 19 |
| showcase（0.19 起 games 更名） | 20（Breakout 等） |
| 3d | 21–26 |
| gltf | 23 |
| picking | 25 |
| gizmos | 27 |
| dev_tools / diagnostics | 27, 33 |
| ui（0.19 起按 images/layout/text/widgets 等子目录组织） | 28, 29 |
| animation | 30 |
| reflection | 31 |
| scene（bsn.rs + world_serialization.rs） | 32 |
| remote | 33 |
| async_tasks | 34 |
| window | 35 |
| no_std | 35 |
| shader | 36 |
| shader_advanced | 37 |
| wasm / mobile | 38 |
| tools / stress_tests | 38 |
| large_scenes（0.19 新增渲染基准场景） | 教学辅助，按需引用 |
| math | 12, 附C |
| usage / helpers / testbed | 教学辅助，按需引用 |

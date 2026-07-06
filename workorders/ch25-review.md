# ch25 交叉审阅报告（只读审阅，未改动任何文件）

> 审阅人：新鲜眼 agent（2026-07-06）。对象：book/src/ch25-00～ch25-14 共 15 节、
> code/ch25-picking 全部代码（Cargo.toml、src/main.rs、examples/listing-25-01～14、
> no-compile/listing-25-02）、book/src/images/ch25 全部 14 图。
> 方法：正文 API 断言逐条对 vendor/bevy（v0.19.0）源码抽核（bevy_picking 全模块、
> bevy_sprite/bevy_ui picking_backend、bevy_camera_controller 两控制器全文、bevy_ecs
> observer、bevy_math ray、bevy 根 Cargo.toml feature 表）；输出块对 workorders/ch25.md §3
> 台账抽 6 处；插图抽 5 张（SVG×3 读源码、PNG×2 看图）；跨章引用 grep 落点 17 处；
> 体例扫描（无痕/引号/SUMMARY/编号/include）。
>
> **总计 22 条发现：高 1、中 7、低 14。** 无一条动摇本章骨架；高与中项集中在
> 「个别 API 断言写错」「个别内部引用指错」「一处硬规违例」，均为局部可修。

---

## 一、按严重度排序的发现清单

### 高

**H1. 「MeshRayCastSettings 没有 Default」是错的**
- 位置：`book/src/ch25-09-mesh-ray-cast.md:19`（「三个字段一个不能少（它没有 `Default`，就是要你逐项表态）」）
- 问题：`vendor/bevy/crates/bevy_picking/src/mesh_picking/ray_cast/mod.rs:82-90` 明确有
  `impl Default for MeshRayCastSettings`（visibility: VisibleInView、filter 恒 true、
  **early_exit_test 恒 true**＝默认「最近一件就收队」）。正文括号里的立论（引擎逼你逐项表态）
  不成立；且默认 early_exit_test 恒 true 这个事实本身对读者有用（`..default()` 出来的是
  「拾取管线口味」的单发射线）。
- 严重度：高（可核查的 API 事实断言写反，且用它支撑了一个设计动机叙事）
- 建议：改为「它有 `Default`（VisibleInView + 不筛人 + 最近一件即收队），本例三个字段全
  手填，是为了把每一项的含义摆上台面」——顺手把「默认恒早退」写成一个认知点。

### 中

**M1. Press/Release 的字段清单与源码不符**
- 位置：`book/src/ch25-04-press-click-release.md:15-16`
- 问题：正文声称「三封信的正文各有几个字段，**逐个交代**」，但 events.rs:288-305 里
  `Press { button, hit, count }`、`Release { button, hit }`。正文写「`Release` **只带**
  `button`」（漏 hit，「只带」为误断），`Press` 漏 `hit`。同节的 Drag 三兄弟字段倒是全对。
- 严重度：中（技术准确性 + 逐参数硬规双重失分）
- 建议：两行各补上 `hit`；「只带 button」改「带 `button` 与 `hit`」。

**M2. 手写 Rust 代码块违反 {{#include}} 铁律**
- 位置：`book/src/ch25-04-press-click-release.md:73-77`（`if click.count == 2 { // 双击的戏 }`）
- 问题：CLAUDE.md「禁止在 .md 里手写 Rust 代码块（shell 命令、程序输出示例除外）」。
  全章唯一一处；其余代码块均为 include，构建零失败。
- 严重度：中（硬规违例，虽只三行）
- 建议：要么删块改行文（「拿 `click.count == 2` 分个叉就够」），要么在 main.rs 的分流
  observer 上开 ANCHOR 截同义片段（25.14 本来就有现成的 `count == 2` 分流）。

**M3. Cargo.toml 注释把 feature 报错实验指到「25.9」，实为 25.12**
- 位置：`code/ch25-picking/Cargo.toml:13`（「25.9 的报错实验用 --no-default-features 复现」）——
  该注释经 `{{#include ...Cargo.toml:deps}}` 原样进入正文（Listing 25-13 其一，ch25-12 节）
- 问题：`--no-default-features` 的 E0433 实验在 **25.12 节**（ch25-12-free-camera.md:15-25）。
  「25.9」是编号重排前的遗留（工单 §3 首行明言「编号最终版与初版规划有出入」），且与
  ch25-00:34 的预告「25.12 节会讲……不开门会撞上什么报错」自相矛盾。
- 严重度：中（错误引用被 include 进正文，读者会翻错小节）
- 建议：注释改「25.12」。改的是 code/ 注释，正文自动跟着对。

**M4. 「25.1 节官方示例给地面挂的就是它（IGNORE）」——内部引用错位**
- 位置：`book/src/ch25-06-pickable.md:16`
- 问题：25.1 节（listing-25-01.rs）的台面**没有**挂 `Pickable::IGNORE`（也不能挂——25.1 的
  「点锣中孔无输出」解释依赖「射线落在台面上、台面没挂观察者」）。作者想指的是**官方
  `mesh_picking.rs` 示例**给地面挂 IGNORE（25.14 节的写法「官方 mesh_picking 示例给地面的
  同款」才是对的）。读者按图索骥翻回 25.1 会找不到。
- 严重度：中（内部引用指错，直接可证伪）
- 建议：改「官方 `mesh_picking` 示例给地面挂的就是它（我们的收场戏 25.14 也这么干）」。

**M5. 「加 `Added` 过滤器反选即可」——Bevy 查询过滤器写不出「非 Added」**
- 位置：`book/src/ch25-03-state-components.md:54`
- 问题：正文说想跳过 `Changed` 把「领牌上岗」也算变化的开场白，「加 `Added` 过滤器反选
  即可」。Bevy 的查询过滤器没有对 `Added` 取反的组合（`Without` 只对组件、无 `Not<Added>`）；
  正确姿势是查询里拿 `Ref<Hovered>` 在系统里 `!hovered.is_added()` 跳过。按字面写不出可编译
  的代码。
- 严重度：中（可操作性建议实际不可操作）
- 建议：改「查询里改拿 `Ref<Hovered>`，跳过 `is_added()` 的那帧即可」。

**M6. FreeCamera「这里只改了三个数」与代码不符；run_speed: 9.0 零交代**
- 位置：`book/src/ch25-12-free-camera.md:39-43` 对照 `examples/listing-25-13.rs:26-31`
- 问题：代码实际改的是 `walk_speed: 3.0`、`run_speed: 9.0`、`friction: 25.0` 三项；正文列的
  「三个数」却是 walk_speed、friction、**sensitivity 0.2**——sensitivity 0.2 是出厂默认值
  （free_camera.rs:142），listing 里根本没写它；真正改了的 `run_speed: 9.0`（为何从 15 降到 9）
  一个字没提。「逐参数讲解」硬规在本章执行最好的一节里恰好破了这一处。
- 严重度：中（图文不一致 + 逐参数硬规）
- 建议：第三条改讲 run_speed（「出厂 15 是勘景冲刺，画廊里 9 就够；Shift 的意义是 3→9 的
  三倍差」），sensitivity 挪进「其余没动的字段」段落。

**M7. Figure 25-5 图注承诺四种走向，SVG 只画了三种**
- 位置：`book/src/ch25-05-bubbling.md:73`（caption「四种账单的走向——冒泡三连、拦截、
  **从中层起头**、空处直达」）对照 `images/ch25/fig-25-05-bubbling.svg`
- 问题：SVG 内容为三条路径（点锣冒泡三连、点盏拦截、点空处直达），图内标题也是「冒泡、
  拦截与空处直达」；「点货架板从中层起头」未画。alt 文本倒是诚实描述了三种——是 caption
  相对图与 alt 都超编了。打印读者对着图数不出第四种。
- 严重度：中（图文一致性；插图规范「与正文描述不符时修文或修图」）
- 建议：改 caption 为三种，或 SVG 补一支从货架起头的两连箭头（工单 §5 规格表写的本来就是
  「四种账单走向」，看来是绘制时缩水、caption 没跟着改）。

### 低

**L1. 「流水线整个跑在 PreUpdate」——Input 段其实在 First**
- 位置：`book/src/ch25-00-picking-and-camera-control.md:26`、fig-25-01 底部脚注「四段全部跑在
  PreUpdate」
- 问题：lib.rs：`PickingSystems::Input/PostInput` 挂在 **First**；ProcessInput/Backend/Hover/
  PostHover/Last 才在 PreUpdate（工单 §1 自己记的就是「Input（First）→ 其余都在 PreUpdate」）。
  教学结论（Update 看到的是处理完的世界）不受影响。
- 建议：「整个/全部」松成「主体四段都在 PreUpdate（原始输入的采集更早，在 First）」，或不改
  （容忍简化），但 SVG 脚注与正文至少别用「全部」。

**L2. 「HitData……四个字段」实为五个**
- 位置：`book/src/ch25-01-first-pick.md:53`
- 问题：backend.rs:135-156 还有第五个 `extra: Option<Arc<dyn HitDataExtra>>`（后端自定义附加
  数据）。刻意略过可以，写死「四个」不妥。
- 建议：删数量词，或补一句「还有一格后端自留的 `extra`，本书用不上」。

**L3. 「窗口永远以垫底命中的身份挂在悬停名单末尾」措辞过强**
- 位置：`book/src/ch25-05-bubbling.md:42`
- 问题：窗口后端是把命中**报进候选队列**垫底（order = NEG_INFINITY）；能不能进「悬停名单」
  （HoverMap）还要过「挡下家」裁决——25.10 节自己演示了吸音档把窗口也挡出名单（「连台口的
  『落空』都没有」）。两节表述打架，25.5 的「永远……挂在名单末尾」不准确。
- 建议：25.5 改「永远垫底**报一发候选**；只要上面没人收队，点空处就落在窗口头上」。

**L4. 台账一致性：25-05 第二批三击的 duration 无台账出处；25-02 报错块路径行经过改写**
- 位置：`book/src/ch25-04-press-click-release.md:58-66` 对照 `workorders/ch25.md §3 25-05`；
  `book/src/ch25-01-first-pick.md:92-101` 对照 §3 25-02
- 问题：抽查 6 处输出块，4 处逐字吻合（25-01 三件货、25-03 悬停、25-13 FreeCamera、25-14
  PanCamera；25-06/25-07/25-10/25-11/25-12 顺带目测也吻合）。两处例外：
  ① 25-04 正文拨表后的三个单击报了 duration 48/49/53 ms，台账只记「（同节奏 180ms 间隔三击：
  count=1,1,1）」无时长——这三个毫秒数在台账里找不到出处；
  ② 25-02 的 E0277 报错，台账记的是草稿文件 `...zz-nc.rs:30:26`，正文改写为
  `ch25-picking\no-compile\listing-25-02.rs:30:26`——与真实文件行列号自洽（listing-25-02.rs 第
  30 行确是 `.observe(|click: On<Click>| {`），改写合理，但严格「逐字一致」不成立。
- 建议：①往台账补记那次实测的三行（或正文换用台账已有数字）；②台账加一行「正文报错块按
  最终文件名/行号重排过、已核对」备案。

**L5. Cancel 的触发例子「窗口失焦一类」未在源码找到依据**
- 位置：`book/src/ch25-14-grand-inspection.md:64`（小结）
- 问题：0.19 输入插件只在 `TouchPhase::Canceled` 时发 `PointerAction::Cancel`
  （bevy_picking/input.rs:263-267）；窗口失焦不产 Cancel。「一类」有兜底但例子举错。
- 建议：只留「触摸被系统打断一类」。

**L6. friction 的「速度每秒衰减为原来的 e 分之几」数学不严**
- 位置：`book/src/ch25-12-free-camera.md:42`
- 问题：实现是 `velocity.smooth_nudge(&ZERO, friction, dt)`＝指数衰减，每秒余量 e^(−friction)
  ——friction=1 时才是「e 分之一」。「设 0 就是冰面」倒是对的（衰减因子恒 1）。
- 建议：改「衰减速率系数：每过 1/friction 秒余 1/e，数值越大刹得越急」。

**L7. PanCamera「字段全是 `Option<KeyCode>`」以偏概全**
- 位置：`book/src/ch25-13-pan-camera.md:11`
- 问题：只有八个键位字段是 `Option<KeyCode>`；enabled/zoom_factor/min_zoom/max_zoom/
  zoom_speed/pan_speed/rotation_speed/mouse_pan_settings 都不是。
- 建议：「键位字段全是 `Option<KeyCode>`」。

**L8. FreeCamera「WASD 平面移动」与两段后的实测自相抵触**
- 位置：`book/src/ch25-12-free-camera.md:37`
- 问题：W/S 沿**相机视线**（能扎地入天），A/D 沿右轴——正文自己随后实测出「前进一秒 y 从
  2.8 掉到 1.9」并解释「前是相机自己的前方」。开头的「平面移动」误导第一印象。
- 建议：改「WASD 沿视线与横轴移动」。

**L9. 「sprite 后端把命中先全数报上去」略过了后端内部的截断**
- 位置：`book/src/ch25-10-sprite-picking.md:77`
- 问题：sprite 后端内部同样按 `should_block_lower` 截断下层（picking_backend.rs:271，被挡的
  下层 sprite 根本不上报）。与 mesh 的真正差异是：**不收（is_hoverable=false）的 sprite 本身
  仍会上报、仍能挡**，而 mesh 在射线阶段就把它整个剔了。结论（sprite 四档全、mesh 吸音退化）
  与实验都对，只是「全数报上去」三字过宽。
- 建议：改「把『不收』的实体照样报上去（挡不挡另算）」。

**L10. 练习 3 用到正文未讲的 require_markers/MeshPickingCamera/MeshPickingSettings**
- 位置：`book/src/ch25-14-grand-inspection.md:75`
- 问题：这三个名字全章正文零次出现，首见即练习。题面自带完整配方（插件照加、资源覆写、
  挂哪两个标记都写明了），预测所需的机制（backend/Pickable/悬停）都讲过，勉强自足；但与
  「练习所需知识都已讲过」的标准打擦边。工单 §2 微实验清单第 13 条本计划「25-6 或 25-9 带」
  一句，正文成稿时掉了。
- 建议：在 25.9（讲 filter 的地方最顺——mesh 后端的 filter 闭包里就有 marker_requirement）补
  一两句「还有一档全局开关 require_markers：拾取全体改为白名单制」，练习即完全落地。

**L11. 练习 4 的提示盖不住滚轮路径**
- 位置：`book/src/ch25-14-grand-inspection.md:76`
- 问题：题意「罩住期间……也不许点穿到转台」。「守门 + 空观察者」能吞掉点击与拖拽（转台的
  Drag 观察者有 original_event_target 判定，纱幕起头的账单不会转台），但收场戏窗口的
  **Scroll 观察者没有 original 判定**（main.rs:218-220）——纱幕悬停下滚轮事件照样冒泡到窗口、
  推拉照样动。严格达成题意还需在纱幕上挂一个 `Scroll` 观察者 `propagate(false)`。题目可解，
  提示不完备。
- 建议：提示补半句「（滚轮那路呢？想想账单还会不会冒泡）」——反而变成更好的题。

**L12. `Scroll` 的 unit 字段全章未交代，「滚轮格数」只对 Line 单位成立**
- 位置：`book/src/ch25-14-grand-inspection.md:36`（「`scroll.y` 是滚轮格数」）；`Scroll` 事件
  在小结中列为家族成员但字段（unit/x/y/hit/phase）无一处展开
- 问题：events.rs:457-470，`Scroll` 带 `unit: MouseScrollUnit`——鼠标滚轮是 Line（格数），
  **触控板是 Pixel（像素值，一次能到几十上百）**。收场戏 demo 面向网页读者，触控板极常见：
  `scroll.y * 0.6` 在 Pixel 单位下一划到底（好在 clamp(3,12) 兜底，行为是「猛」而非「坏」）。
  本章唯一一个正文消费过却没逐字段交代的事件。
- 建议：36 行补一句「触控板报的是像素单位（`scroll.unit` 区分），跨设备要除以单位换算——
  FreeCamera 源码里正有现成写法（SCROLL_UNIT_CONVERSION_FACTOR）」。

**L13. 收场戏两个魔数（0.008 rad/px、0.6）未按逐参数标准给理由**
- 位置：`book/src/ch25-14-grand-inspection.md:34-36` 对照 `src/main.rs:213-219`
- 问题：转台的 `rig.yaw -= drag.delta.x * 0.008` ——这个 0.008 的量纲是**弧度/像素**（拖满
  1280px 转约 587°），与 25.7 节推导的 0.008（**米/像素**）只是数值巧合，正文未说明，读者易
  误以为沿用了 25.7 的推导；滚轮步长 0.6（米/格）也无一字。25.7 对 0.008 的示范推导恰恰抬高
  了预期。
- 建议：各给半句：「0.008 rad/px≈拖满一屏转半圈多，转台手感；0.6 米/格≈行程 (3,12) 里滚
  十五格走完」。

**L14. 「这份账贵」的归因与源码注释相左**
- 位置：`book/src/ch25-03-state-components.md:59`
- 问题：hover.rs:334 注释原话是维护 `Hovered` 的开销 "relatively cheap, and linear in the
  number of entities that have the Hovered component inserted"。「只给声明了『我要』的实体算」
  完全正确（选入机制的因），但「这份账贵」是引申（正因选入才便宜）。
- 建议：弱化为「这份账要沿树向上聚合，不值得给全场默认开」。

---

## 二、四个维度逐项结论

### ① 技术准确性——查过，除上列 H1/M1/M3-M6/L1-L3/L5-L9/L12 外，其余抽核全部与源码一致

对 vendor/bevy 逐条核实且**无误**的断言（正面清单，供后续免复查）：
- `Pointer<E>`：entity/pointer_id/pointer_location/event 四公共字段 + `Deref` 直达内层
  （events.rs:74-87,136-142）✓；同时是 Message 与 Component ✓；`PointerId::Mouse/Touch(u64)/
  Custom(Uuid)` ✓；`PointerTraversal` 父链到头跳窗口实体再停 ✓；Enter/Leave 以
  propagate=false 造出不冒泡 ✓
- 派发顺序（events.rs:619-660 文档 + 台账实证一致）：帧内 Out→Leave→DragLeave、
  DragEnter→Enter→Over、每按键 **Press 或 Click→Release**→DragDrop→DragEnd→DragLeave ✓
  ——25.4「Click 先于 Release」、25.5「Enter 先于 Over」、25.8 成交四连顺序全对
- `Click { button, hit, duration, count }`「四个字段」✓；`Drag { button, distance, delta }` ✓；
  `DragEnd { button, distance }` ✓；DragStart 带 hit ✓；四件套 dragged/dropped ✓；
  Move 带 delta 且屏幕系 y 向下 ✓（Press/Release 见 M1）
- 连击：`multi_click_interval` 默认 Duration::from_millis(500)（lib.rs:354）✓；计数
  per-entity（PointerButtonState.clicking: EntityHashMap）✓；PickingSettings 四开关 +
  不在 prelude ✓
- `Pickable` 两字段语义、默认 true/true、`IGNORE`、无组件默认「能收也能挡」
  （lib.rs:196-254 + hover.rs:206-209 else 分支）✓；悬停裁决层四象限齐全（hover.rs:198-210）✓
- **mesh 吸音失灵机制归因精确**：mesh_picking/mod.rs:107 filter 闭包在射线阶段剔除
  `is_hoverable=false`，should_block_lower 轮不到出场 ✓；sprite 后端把不收的实体照样上报
  （picking_backend.rs 查询含 `&Pickable`、命中判定不看 is_hoverable）→ 吸音真吸音 ✓
- sprite：`&Pickable` 是查询必备项（无牌不参检）✓；`AlphaThreshold(0.1)` 出厂 ✓；判定
  `color.alpha() > cutoff`（严格大于）✓；压缩纹理读不出→warn + 判失手 ✓；
  SpritePickingPlugin 随 SpritePlugin 自动注册（bevy_sprite/lib.rs:108）✓
- UI：UiPickingPlugin 随 UiPlugin 自动注册（bevy_ui/lib.rs:179）✓；节点无 Pickable 也参与
  （查询里 `Option<&Pickable>`，缺省 block）✓；**order = camera.order + 0.5**
  （picking_backend.rs:272-276，模块注释同）✓
- 窗口后端：order = f32::NEG_INFINITY、depth 0、position=屏幕坐标 extend(0.0)、normal None
  （window.rs:42-47）✓；is_window_picking_enabled 默认 true ✓
- 状态牌：`PickingInteraction` 由 update_interactions 对命中实体 **try_insert 自动发**
  （hover.rs:268-270）、Pressed=2/Hovered=1/None=0 ✓；`Hovered(pub bool)`
  `#[component(immutable)]`、只更新已挂的、含子孙（CSS :hover 语义）✓；`PointerInteraction`
  经 `#[require]` 天生在指针实体上、sorted、`get_nearest_hit()` 取首 ✓
- `MeshRayCast::cast_ray(ray, settings) -> &[(Entity, RayMeshHit)]` 近到远排序 ✓；AABB 粗筛
  再逐三角形 ✓；`RayMeshHit` point/normal/barycentric_coords/distance/triangle/uv（另有
  triangle_index，正文未列、未称穷尽，无碍）✓；`always_early_exit()` 存在 ✓；
  `Ray3d::intersect_plane(Vec3, InfinitePlane3d)` 存在（练习 2 提示成立）✓
- FreeCamera（free_camera.rs:140-165 + 控制器全文）：sensitivity 0.2、WASD+E 升 Q 降、
  ShiftLeft、右键按住抓/M 切换、Numpad 1/3/7 轴吸附（Ctrl 反向）、walk 5.0/run 15.0、
  scroll_factor 0.04879016≈ln 1.05 且 `speed_multiplier *= exp(factor×scroll)`、friction 40、
  vertical_movement_axis 默认 World（源码注释原话就是 Bevy/Unreal/Blender vs Unity/Godot）、
  `#[require(FreeCameraState)]`、`single_mut()` 多台即罢工、pitch clamp ±PI/2、
  CursorGrabMode::Locked + 隐藏光标、文件 485 行（「五百行」成立）✓；
  「抄源码自改」为 crate 文档原话（lib.rs 头注）✓；feature 默认全关 ✓
- PanCamera（pan_camera.rs）：无 require、一件组件全包（zoom_factor 就存组件上）✓；默认
  zoom 1.0/min 0.1/max 5.0/zoom_speed 0.1/=/−/pan 500/W 上 S 下 A 左 D 右/π rad/s/Q 逆 E 顺/
  左键拖平移 ✓；缩放线性、写 `Transform.scale` ✓；**DragStart 冒泡到窗口时 spawn 一个盯
  `Pointer<Drag>` 的 Observer 挂窗口、DragEnd despawn**（add_window_observer/
  remove_window_observer/handle_mouse_pan）——25.13 的源码揭底与练习 5 Q2（换算走
  world_to_viewport→delta 取反→viewport_to_world_2d，天然含缩放）全部对得上 ✓
- 插件格局：DefaultPickingPlugins=PointerInputPlugin+PickingPlugin+InteractionPlugin、
  **不含 backend**；`picking` feature = ["bevy_picking","mesh_picking","sprite_picking",
  "ui_picking"] 且在默认 profile 里 ✓；17 门 `Pointer<…>` 消息 add_message ✓；
  `On::propagate` / `On::original_event_target` / `EntityEvent::event_target` 均实在
  （bevy_ecs observer/system_param.rs:139,156；event/mod.rs:329）✓
- 报错样例：25-02 的 E0277 与 no-compile 文件 30 行 26 列自洽 ✓；25-12 的 E0433 与
  feature 门机制一致 ✓
- 正文数字抽查：500ms ✓、0.1 门槛 ✓、order+0.5 ✓、NEG_INFINITY ✓、walk 5.0/run 15.0/
  friction 40/0.0488≈ln1.05 ✓、pan 500/zoom 0.1/夹 0.1-5.0/π ✓、0.008 的 45°/6.4m/720px
  推导复算成立（2×6.4×tan22.5°≈5.30m→5.3/720≈0.00736）✓、701≈700×1s ✓、
  0.70=1.00−3×0.1 ✓、313≈0.35×1280×0.7 ✓、91°≈π×0.5s ✓

### ② 两条硬规——逐节查过，总体达标，破例见 M1/M2/M6/L12/L13

- **微实验入文**：15 节每节都有「问题→实验→结论」弧线且配真实输出/插图——25.1 点锣中孔、
  25.2 瞬移对照、25.3 三货对照组（一挂两不挂）、25.4 拨表 500→120、25.5 四种账单+Enter/Leave
  补测、25.6 四档纱幕+第三档坏账、25.7 生搬 y 对照、25.8 注释掉让路的哑火实验、25.9 V 档
  隐身现形+放空、25.10 六步（未挂牌落空→挂牌→重叠区认像素/认框→吸音全场沉默）、25.11 开洞
  一点两账、25.12 前进掉高+惯性滑行、25.13 逐行对账四路输入、25.14 收场。**无一节是纯 API
  罗列**。「亲眼看报错」两处（E0277、E0433）+ 静默坑两处（忘加插件、忘让路）成色好。
- **逐参数讲解**：绝大多数首现 API/字段有交代，非显然取值多有理由（0.008 有推导、500ms
  有出处、0.1 有「什么时候要动」段、clamp(3,12) 有「不穿模不失联」、120ms 拨表值服务于
  180ms 手速的实验设计）。破例：M1（Press/Release 漏 hit）、M6（run_speed 9.0 零交代、
  sensitivity 张冠李戴）、L12（Scroll.unit 未提）、L13（收场 0.008/0.6 无理由）。

### ③ 体例一致性——查过，除 M2/M3 外干净

- 无痕原则：ch25 全部 15 文件 grep「0.19/新增/更名/改名/迁移/旧版」零命中 ✓
- 中文引号：正文 ASCII 直引号零命中（代码块除外）✓；「」与破折号风格与 ch01 一致 ✓
- 文件名全部 kebab-case ✓；SUMMARY.md 210-224 行 15 条目与文件一一对应 ✓
- 代码块：除 M2 一处外全部 `{{#include}}`，shell/输出块合规 ✓
- Listing 25-1～25-15 连续、文件名 listing-25-XX 与 Listing 编号对齐（编号≠小节号，一致）✓；
  Figure 25-1～25-14 按出现顺序连续 ✓；caption 格式统一（`<span class="caption">`，每个
  listing 首段标文件路径）✓
- 术语首现处理：picking（拾取）/冒泡（bubbling）/拖放（drag & drop）等中文注解齐；旧术语
  （observer、ChildOf、Handle…）一律回指章号不复述，符合「已讲不重复」✓
- 跨章引用抽查 17 处全部落点存在：ch02-02（窗口是实体）、ch04（Changed/Added/系统即函数/
  With-Without/Single）、ch05-05（资源变更检测）、ch07（MessageReader）、**ch08-02（伏笔
  原文里确有「第 25 章」预告）**、ch09-01（ChildOf）、ch12（坐标系 y 翻转/Visibility）、
  ch13（order/RenderLayers/正交缩放）、ch14-02（句柄提货单）/ch14-08（KTX2 压缩纹理）、
  ch16（Text2d 与中文字体）、ch17-03（指哪打哪三步走）/ch17-04（AccumulatedMouseMotion+
  CursorGrabMode 导播摇臂）、ch18（RunFixedMainLoop）、ch20-07（canvas/fit_canvas_to_parent）、
  ch21（UV/图元）、ch23-06（Name 按名取实体）、ch24-08（anisotropy 门）✓
- 语气对标 ch01：问题驱动、先跑后写、台词有人味、无空话——达标；45k 量级（40.3k）符合
  试行标尺区间

### ④ 图文一致性抽查（5 张）——查过，除 M7 外全部相符

- fig-25-01（SVG）：四段横排、鼠标+手指→PointerLocation、三车间各配 PointerHits 小票、
  漏斗「按深度排序/被挡的筛掉」→HoverMap、信封 Over/Click/Drag 沿树上飘——与 alt 逐项吻合；
  #f7f5f0 圆角卡底 ✓、中文全部可读、配色与 ch13 以来的 SVG 家族一致。唯脚注「四段全部跑在
  PreUpdate」同 L1。
- fig-25-05（SVG）：树形结构、金色冒泡三连（带 1/2/3 序号与敲响记号）、红八角「停」牌 +
  propagate(false)、虚线空处直达、底注 entity/original_event_target 对比——画面本身质量佳、
  与 alt 一致；**与 caption 的出入见 M7**。
- fig-25-09（SVG）：侧视剖面、相机+viewport_to_world 注记、虚线射线依次穿盏（4.38m）、
  锣（6.39m）、虚线轮廓隐身盒（7.99m，旁注「Any 档才串上」+Visibility::Hidden）、扎进台面
  （9.29m），四枚串珠——距离与台账/正文输出逐字一致 ✓。
- fig-25-10（PNG）：左右两联、同一十字点位（阿燕包围盒右上透明角=灯笼灯体）、左联灯笼描
  金边、右联阿燕整框描金边、底部中文标注「认像素（AlphaThreshold 0.1）/认框（BoundingBox）」
  ——与 alt 及正文实验逐项吻合 ✓。
- fig-25-14（PNG）：转台转过的视角、漆盒与琉璃盏在左、光标按在鎏金锣上拖向右侧朱漆托盘、
  **锣无高亮**（拖起让路——alt 修订后的表述与画面一致，工单 §7 记录的「修文不修图」处置
  正确）、托盘原色待命 ✓。光标为按注入坐标补画（工单已披露，非伪造）✓。

### 台账逐字比对（抽 6 处）——4 处逐字一致，2 处偏差见 L4

### 练习可解性——1/2/5 完全可解；3/4 见 L10/L11

- 练 1（长按端详）：Click::duration 已讲，「为什么不需要计时器」的思考题指向 duration 随
  Click 送达——可解 ✓
- 练 2（像素级跟手）：viewport_to_world 已讲（25.9），`Ray3d::intersect_plane` 存在且提示
  点名——可解 ✓
- 练 5（读源码答两问）：pan_camera.rs 335 行、两问均能在 add_window_observer 的
  `entity == drag_start.entity` 判定与 handle_mouse_pan 的 viewport 往返换算里找到答案；
  Q2 的 313 像素与换算机制核实相符——可解 ✓

---

## 三、修复优先级建议（如决定处置）

1. 先修 H1、M1、M4、M5（四处「读者会学错」的断言，均为几行文字改动）
2. 再修 M3（code 注释一行）、M6（正文一段）、M2（删块或补 anchor）、M7（改 caption 一行
   最省）
3. 低项里 L4（台账补记）与 L12（Scroll 单位）建议顺手；其余可留待存量回炉

无任何文件在本次审阅中被修改（本报告文件除外，系协调人指定落盘路径）。

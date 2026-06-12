# 输入处理

《渡口夜话》连演七场，场场满座。散戏后总有看客赖在台口不走，比划阿燕那记“拨云”。老雷看在眼里，谢幕那晚拍了板：“戏不能光演给人看。开个体验场，把阿燕交出去——键盘的、攥鼠标的、抱手柄的、伸手指头戳屏幕的，来者不拒。”

这一章把玩家的手接进游戏。到目前为止，本书的每个示例都是上了发条自己走的：节拍器拨帧、定时器换景，运行起来就没人插得上手。现在补上交互程序的另一半。Bevy 把所有输入设备纳进同一套秩序——设备事件先以**消息**的形式流进来（第 7 章的 Message），引擎在 `PreUpdate` 把它们折叠成**本帧快照**（第 6 章那张调度表的伏笔），你的系统在 `Update` 里要么问快照、要么读流水。一路换着设备练：

- **键盘上手**——`ButtonInput<KeyCode>` 三分钟接管阿燕的脚，顺手踩一个旧教程埋的坑：`KeyCode::A` 这个变体早没了；
- **一下、按住与松手**——`just_pressed`／`pressed`／`just_released` 三种问法三种招式；按住空格做一回实验，看清“快照”与“流水账”各记了什么；
- **指哪打哪**——鼠标按键同一套快照，光标位置走 `cursor_position` 反算世界坐标，兑现第 13 章 `viewport_to_world_2d` 的预告；
- **导播摇臂**——光标撞了屏幕边怎么办：`AccumulatedMouseMotion` 的原始位移、滚轮的两种计量单位、`CursorGrabMode` 抓光标；
- **手柄进场**——每只手柄是一个**实体**：连接消息、摇杆模拟量、死区滤波与震动回敬；没有手柄也能跟完这一节；
- **触摸**——一窝带编号的“光标”：`Touches` 资源与多指追踪；
- **三合一**——输入映射的工程模式：设备各自翻译成“意图”，动作系统只认意图——四路看客使唤同一个阿燕。

这些类型住在 `bevy_input` 这个 crate 里，常用的（`ButtonInput`、`KeyCode`、`MouseButton`、`Touches`、`Gamepad` 一家）都从 `bevy::prelude` 来；消息类型与累计资源（`KeyboardInput`、`MouseMotion`、`AccumulatedMouseMotion`……）要从 `bevy::input` 的子模块显式引入。手柄的硬件后端是 `bevy_gilrs`（封装跨平台手柄库 gilrs，`DefaultPlugins` 自带）；光标与窗口打交道的部分（`cursor_position`、`CursorOptions`）在 `bevy_window`。

配套 crate 是 `code/ch17-input`，不需要任何新依赖。`assets/` 里的家当——阿燕的十二格连环画、桥板、木人桩、中文字体——全部复用前两章的脚本化资产，由 `scripts/make_ch17_assets.py` 一键就位，本章不新画一笔。

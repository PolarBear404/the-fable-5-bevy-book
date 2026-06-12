# 时间、定时器与 FixedUpdate

体验场连开十几个夜场，老雷攒下一肚子疑问。散场后他把场记、鼓师都留下，要把时间的账算清楚：阿燕的步速、剑光的寿命、镜头的跟拍，全都乘着一个 `time.delta_secs()` 在走——这个从第 2 章用到现在的数到底是什么，凭什么它能让快慢不一的机器走出同样的戏？还有第 17 章实验里那句警告——`just_pressed` 搬进 `FixedUpdate` 会丢拍——机制是什么，怎么救？

这一章把 Bevy 的时间系统从头到尾盘一遍。出场的角色还是戏班三人：老雷拍板、场记拿怀表记账、新登台的**鼓师**掌一面鼓——他就是固定时间步的人形写照：不管台上戏多快多慢，鼓点按自己的节拍落。一路要算的账：

- **一帧多长**——帧的间隔天生不均匀，`delta` 是引擎每帧量好递给你的尺子；不乘它的代价用两台“机器”当场对比；
- **两面钟**——`Time` 其实是一族资源：走人间的 `Real`、走戏里的 `Virtual`；中场暂停、慢动作回放，旋钮都在 `Virtual` 上，顺便踩一个“给通用钟按暂停”的编译错误；
- **Timer 与 Stopwatch**——袖箭冷却（只走一程）、箭匣补给（循环走）、运劲掐表（正着数），三件计时家什各司其职；
- **鼓师的账本**——`FixedUpdate` 攒时间、整拍结算的全套账目：`Time<Fixed>`、步长、`overstep`，以及“在 `FixedUpdate` 里读 `Res<Time>` 读到的是哪只钟”；
- **丢拍与重复**——第 17 章欠的账：快照按帧翻篇、鼓点不按帧落，`just_pressed` 在鼓点上要么看不见要么看几遍；官方解法是把瞬时输入缓存成意图；
- **鼓点之间**——压轴《赶月》：走位结算搬上鼓点后画面一顿一顿，用两本账加一个 `overstep_fraction()` 把它抹平——渲染插值。

这些类型住在 `bevy_time` 这个 crate 里。常用的（`Time`、`Fixed`、`Real`、`Virtual`、`Timer`、`TimerMode`）都从 `bevy::prelude` 来；`Stopwatch` 与定时器系的运行条件（`on_timer`、`on_real_timer`、`paused`）要从 `bevy::time` 显式引入。调度侧的 `FixedUpdate`、`RunFixedMainLoop`、`RunFixedMainLoopSystems` 在 `bevy_app`，也都进了 prelude。

配套 crate 是 `code/ch18-time`，不需要任何新依赖；`assets/` 里的家当与第 17 章完全一致——阿燕的连环画、桥板、木人桩、中文字体，由 `scripts/make_ch18_assets.py` 一键就位，本章不新画一笔。

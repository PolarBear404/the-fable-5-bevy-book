# 音频

《长风渡》排到了合成阶段，老雷却在审片时坐立难安：台上的戏齐了，台下静得像默片。这不是第一次欠下声音的账——第 7 章碰碰车场的 DJ 听到碰撞消息就喊“砰！”，那是一行 `println!` 在替音箱站岗；第 16 章夜战结案时也说过“想加打击音效，再挂个 Observer 就是”。账都记着，本章一并清偿：文武场进驻戏班——**琴师**掌文场，管那支循环不歇的曲子；**鼓师**掌武场，锣鼓点一声是一声；**阿燕**照旧登台，这回提着梆子巡夜；**场记**盯着后台，把每一声的来龙去脉记在册上。

Bevy 的音频住在 `bevy_audio` 这个 crate 里，而它的用法你其实已经会了一大半：声音文件是 **Asset**（第 14 章的提货单那一套原样适用），发出一个声音是 spawn 一个实体，控制一个正在播的声音是查询它身上的组件。没有“音频管理器”这类新角色——还是 ECS 那张桌子，多摆了几件乐器。一路要过的手：

- **第一声**——`AudioPlayer` 把一个 `Handle<AudioSource>` 变成声音；顺路撞上本章的真实报错：合成的 `.wav` 默认播不了，要开 feature；
- **锣鼓点**——一次性音效的正确姿势：`PlaybackSettings` 与它的四种 `PlaybackMode`，先看见“敲一声留一具空壳”的堆积现场，再请 `DESPAWN` 自动拆台；
- **AudioSink**——开播之后的缰绳：它哪一帧才上岗、怎么暂停恢复，以及一个好实验——按第 18 章的法子暂停戏台钟，曲子为什么照奏不误；
- **音量**——`Volume` 的 Linear 与 Decibels 两把尺子，`GlobalVolume` 总闸，和总闸“只管新声”的坑；
- **空间音频**——`SpatialListener` 的一对耳朵、移动声源、距离衰减的真实公式，以及 2D 里必须自己定的那把 `SpatialScale` 尺；
- **首演之夜**——BGM、消息驱动的锣鼓、随人走的更声合成一台完整的戏，第 7 章的解耦模式终于真的发声。

常用类型（`AudioPlayer`、`PlaybackSettings`、`AudioSink`、`SpatialAudioSink`、`SpatialListener`、`GlobalVolume`，还有 sink 操作所需的 trait `AudioSinkPlayback`）都从 `bevy::prelude` 来；`Volume`、`PlaybackMode`、`AudioPlugin`、`SpatialScale` 要从 `bevy::audio` 显式引入。

配套 crate 是 `code/ch19-audio`。它的 `Cargo.toml` 比前几章多一个 feature，为什么要开、不开会怎样，是下一节开场的戏：

```toml
{{#include ../../code/ch19-audio/Cargo.toml:deps}}
```

`assets/` 里的音频——序曲、锣、鼓、梆子——全部由 `scripts/make_ch19_assets.py` 用 Python 标准库现场合成（正弦波加包络，参数都写在注释里），一行下载都没有；字体与贴图照旧复用前几章的家当。`py -3.11 scripts/make_ch19_assets.py` 一键重建。

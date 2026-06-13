# HDR 与 Tonemapping

`Hdr` 是挂在相机上的 marker component。没有它时，相机主纹理按普通显示范围工作；有它时，主纹理能保存大于 1.0 的亮度，让自发光材质、强灯光和后处理还有空间可用。`Bloom` 在 Bevy 0.18.1 里通过 required component 自动要求 `Hdr`，本章的键盘逻辑也显式保持这个关系：打开 Bloom 时顺手打开 HDR，关掉 HDR 时也关掉 Bloom。

HDR 不是最终画面。屏幕仍然要显示有限范围的颜色，所以还需要 `Tonemapping` 把高动态范围压回显示范围。Bevy 0.18.1 的 `Tonemapping` 枚举包括 `None`、`Reinhard`、`AcesFitted`、`AgX`、`TonyMcMapface`、`BlenderFilmic` 等选项；默认是 `TonyMcMapface`。这些算法不是“谁绝对正确”，而是不同的色调和高光压缩风格。

相机初始化时直接挂上 `Hdr`、`Tonemapping`、`DebandDither` 和本章默认的后处理组件：

```rust
{{#include ../../code/ch26-post-processing-aa/src/main.rs:camera_bundle}}
```

<span class="caption">Listing 26-4：画质相关能力大多是相机实体上的组件</span>

这里有几个边界要记住：

- `Hdr` 决定相机主纹理是否以 HDR 方式工作；
- `Tonemapping` 决定 HDR 画面如何压回显示范围；
- `DebandDither` 用抖动减少渐变色带，不会替你增加真实细节；
- Bloom、景深、运动模糊和抗锯齿也都是围绕相机输出工作的组件。

运行示例后按 `1` 切 HDR，按 `2` 循环 Tonemapping。`Tonemapping::None` 在发光很强的场景里容易让高光直接截断；`TonyMcMapface`、`AgX`、`AcesFitted` 这类选项会把高光压得更顺。你不需要在每个项目里立刻调完所有曲线，但应该尽早确定项目的默认 tonemapping，因为它会影响材质、灯光和截图的观感。

下一步把 HDR 的亮度信息用起来：打开 Bloom。

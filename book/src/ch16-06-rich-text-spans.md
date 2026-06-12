# 富文本：一块文本，各管各的妆

剧本的排版规矩比字幕复杂：角色名烫金加粗，舞台提示小一号、灰着，台词正色。要是一段文字只能一副行头，就得拆成三个 `Text2d` 再手动对位——排版引擎白养了。

bevy_text 的答案是**文本段**（`TextSpan`）：一块文本是一棵小树——根上挂 `Text2d`（兼任第一段），子实体挂 `TextSpan`，每段是一个独立实体、各配各的 `TextFont` 与 `TextColor`；**排版按整块算**（换行、对齐、地界都看根上的 `TextLayout` 与 `TextBounds`），**妆容按段算**。秋白改词那页手稿，正好五脏俱全：

```rust
{{#include ../../code/ch16-text/examples/listing-16-09.rs:setup}}
```

<span class="caption">Listing 16-9：根管开头，六个 TextSpan 子实体各管一段——字体、字号、颜色、装饰互不干涉（examples/listing-16-09.rs）</span>

```console
cargo run -p ch16-text --example listing-16-09
```

```text
秋白：‘孤舟’划了，换‘秋水’——水比船有戏。
```

![一块居中的剧本文字:金色加粗的角色名阿燕,灰色小字的舞台提示,白色台词中划掉的灰色孤舟与带金色下划线的红色秋水,末行带黑色底签的等宽英文,整块字背后有阴影](images/ch16/fig-16-10-rich-text.png)

<span class="caption">Figure 16-10：一棵文本树的全套妆容——五种字色、三种字号、两副字模、删除线、下划线、底色与阴影</span>

这棵树上每类零件的归属，是富文本不出怪相的关键：

- **逐段生效**（挂在根或任意 span 上，只管自己那段）：
  - `TextFont`、`TextColor`——**每个 span 都要自己配**。span 不继承根的字体：`TextSpan` 的 required components 给它补的是*默认*的 `TextFont`，也就是那副只认 ASCII 的内置字模。中文 span 忘了指字体，那一段就单独变豆腐——整块其他段都正常，就它不正常，现场相当迷惑；
  - **`Underline`**／**`Strikethrough`**——下划线与删除线，标记组件，挂谁谁有线。线色默认随字色，想分开配就加 `UnderlineColor`／`StrikethroughColor`（Listing 16-9 的“秋水”：红字配金线）；
  - **`TextBackgroundColor`**——这一段文字的底色块，签条、高亮都是它。
- **整块生效**（只认根上的）：
  - `TextLayout`、`TextBounds`、`Anchor`、`Transform`——块只有一个排版、一个地界、一枚钉子；
  - **`Text2dShadow`**——整块字的影子，`offset` 定方向（默认右下 4 像素）、`color` 定色。它是 `Text2d` 专属（UI 文本另有一个 `TextShadow`，第 28 章见）。

改词的“孤舟”划掉、“秋水”顶上，两段比肩而立——这种“保留修改痕迹”的排版，三个独立 `Text2d` 拼起来要对齐到天荒地老，spans 里只是两个相邻子实体。

> **span 不能独闯**：`TextSpan` 只有作为 `Text2d`（或 UI `Text`）实体的后代才有意义——它自己没有 `TextLayout`，单独 spawn 出去没有任何东西负责排它，引擎会在日志里点名警告。
>
> **读写一棵树**：运行时想改某一段，沿着 `Children` 爬树取 `&mut TextSpan` 当然可以，但 bevy_text 备了专门的系统参数 `Text2dReader`／`Text2dWriter`——按“块内第几段”直接存取：0 号是根，1 号起是 span 按树序排队。下一节的连击牌就用它。

行头到这里配齐了。出戏之前还剩一个本质问题：这些字到底活在哪个世界？

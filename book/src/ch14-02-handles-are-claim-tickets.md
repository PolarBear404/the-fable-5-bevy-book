# Handle：提货单

上一节的提货单只用来交给 Sprite。这一节把它翻过来看看背面的小字：单子可以复印吗？两张单子怎么知道指的是同一件货？没人要的货什么时候回炉？

## 同路径同单，克隆不花钱

```rust
{{#include ../../code/ch14-assets/examples/listing-14-02.rs:tickets}}
```

<span class="caption">Listing 14-2（节选一）：开两次单、复印一次，三张“单子”同指一件货（examples/listing-14-02.rs）</span>

```console
cargo run -p ch14-assets --example listing-14-02
```

```text
老顾：灯笼的单子开了两回，是同一张单吗？——true
老顾：复印一份给布景组，指的还是那盏灯吗？——true
老顾：货号——AssetId<bevy_image::image::Image>{ index: 7, generation: 0}
老顾：货址——Some(props/lantern.png)
```

四条规矩：

- **同一路径，永远同一张单**。第二次 `load("props/lantern.png")` 不会进第二遍货，库房认出这条路径已有登记，把同一张 Handle 还给你。所以“先在 `Startup` 里 load，之后哪里需要哪里再 load 同一路径”是完全合法的取单方式——14.4 节会用到；
- **克隆是复印，不是复制货**。`Handle` 内部是 `Arc`（标准库的原子引用计数指针），`clone` 只把计数加一，图片本体在内存里永远只有一份。两个 Sprite 各持一张单，画的是同一盏灯；
- **`id()` 是货号**——`AssetId`，一个实现了 `Copy` 的纯编号，比较、做键都方便；
- **`path()` 是货址**——从文件加载的资产记得自己的来路；用 14.5 节的 `Assets::add` 手工上架的没有来路，返回 `None`。

窗口里左右各一盏灯笼——两张单，一件货。

## 货架也是 Resource

`Assets<Image>` 没有任何神秘之处：它就是个 Resource，泛型参数区分货架——`Assets<Image>` 摆图片，以后还会见到 `Assets<Mesh>`（网格）、`Assets<StandardMaterial>`（材质）。常用的架上操作一共四个：`get(&handle)` 取货、`get_mut(&handle)` 改货（会触发资产的变更广播，14.6 节见）、`add(asset)` 不经库房直接上架、`remove(&handle)` 下架。

货架上还住着一位常驻房客。`Handle` 实现了 `Default`，而默认单子指向一件引擎自带的货：

```rust
{{#include ../../code/ch14-assets/examples/listing-14-02.rs:default_handle}}
```

<span class="caption">Listing 14-2（节选二）：默认 Handle 指向常驻的 1×1 纯白图（examples/listing-14-02.rs）</span>

```text
老顾：另有一块 1 × 1 的百搭白布常年在架，染个色就能当色块使。
```

这块白布解开一个从第 2 章悬到现在的谜：`Sprite::from_color` 凭什么不要图片？看一眼它的源码——它只填了 `color` 和 `custom_size`，`image` 字段留给 `..Default::default()`。**色块 Sprite 一直都有贴图**，贴的就是这块 1×1 白图，拉伸到你要的尺寸再染上颜色。Bevy 里“看起来没用资产”的地方，往往只是用了默认资产。

## 谁攥着单子，货就活多久

提货单还有最后一条规矩，也是最重要的一条：**强引用计数**。`load` 返回的是 `Handle::Strong`——只要世界上还有一张强单（不管攥在组件、Resource 还是局部变量手里），货就保证在架。最后一张强单销毁，货被自动下架回收。

这正是 Listing 14-1 里把 Handle 塞进 `Sprite` 就完事的原因：组件本身就是持单人，实体活着，图就活着；实体 `despawn`，单子随之销毁，没人要的图自动回炉——下一节会拿广播亲眼验证这一幕。

反过来，这也是新手最常踩的坑：在 `Startup` 里 `load` 完，单子只存在局部变量里，函数一结束强单当场销毁——这件货等于白进，到货即回收，谁也用不上。`load` 完必须把单子**存进活得够久的地方**：组件字段、Resource，或者 14.4 节的资产清单。只想记“是哪件货”而不想保活，存 `id()` 拿到的 `AssetId`——纯编号不参与计数，Listing 14-4 马上演示这个差别。

> 细心的读者会发现 `Handle` 还有第二个变体 `Handle::Uuid`：不指向加载流程，只是个固定编号，引擎用它安置“白布”这类内置资产。日常代码几乎只和 `Strong` 打交道，知道有这位亲戚即可。

# 现场改词：热重载

连排进行到第三遍，秋白觉得最后一句不对味。流程本该是：改文件、关游戏、重新 `cargo run`、等编译、等加载、再看效果——为一个字，两分钟。美术调一张贴图的颜色、关卡师挪一个摆设，全是同样的循环。**迭代速度就是内容质量**：试错一次的成本越低，敢试的次数就越多。

热重载（hot reloading）把这个循环砍到一秒以内：游戏不停，文件一存盘，引擎察觉变化、自动重新装载、当场生效。

## 打开监工模式

热重载靠 `file_watcher` 这个 cargo feature——本章 `Cargo.toml` 里早就备好的那行现在揭晓用途：

```toml
{{#include ../../code/ch14-assets/Cargo.toml:deps}}
```

开了 feature，`AssetServer` 默认就监视 `assets/` 目录（想精细控制，`AssetPlugin` 的 `watch_for_changes_override` 字段可以强行开关）。注意这是**开发期装备**：发布构建别带——玩家的机器上没有热重载的须要，文件监视白吃资源。

代码侧要做的事少得出奇。所谓“响应热重载”，就是收听 14.3 节的 `Modified` 广播：

```rust
{{#include ../../code/ch14-assets/examples/listing-14-08.rs:setup}}
```

<span class="caption">Listing 14-8（节选一）：台上一把剑、手里一本剧本，等着被改（examples/listing-14-08.rs）</span>

```rust
{{#include ../../code/ch14-assets/examples/listing-14-08.rs:watch}}
```

<span class="caption">Listing 14-8（节选二）：剧本一变，回到第一句重新对词（examples/listing-14-08.rs）</span>

## 三场实验

启动连排，让它把第一稿念完：

```console
cargo run -p ch14-assets --example listing-14-08
```

**实验一：改词**。游戏别关。用任何编辑器打开 `assets/scripts/opening.script`，把幕名改成“渡口夜话·二稿”，最后一句换成“长风不熄，故人不散。”，存盘。一秒之内：

```text
场务：秋白改稿送到！从头对词——
老雷：《渡口夜话·二稿》，5 句词。对词！
阿燕：二十年了，这把剑还认得回家的路。
…
阿燕：他会来。长风不熄，故人不散。
```

幕后链条：文件监视器发现 `opening.script` 变了 → 自动重跑 `ScriptLoader`（我们 14.5 节写的那个，热重载对自定义装载器一视同仁）→ 新 `Script` **顶替架上旧货**（Handle 不变！）→ 广播 `Modified` → 我们的系统把进度归零。持单人什么都不用换，货架上的货本体被偷换了——所有拿这张单的地方，下次 `get` 到的就是新词。

**实验二：改坏**。在剧本末尾加一行忘了写冒号的词，比如“这一行忘了写角色名”，存盘。控制台立刻出现红色 ERROR——14.5 节精心编写的错误信息在此兑现：

```text
ERROR bevy_asset::server: Failed to load asset 'scripts\opening.script' with asset loader
'listing_14_08::ScriptLoader': 第 11 行不是“角色：台词”的格式："这一行忘了写角色名"
```

要紧的是**没发生**的事：游戏没崩，台上没冷场——重载失败时，货架上的旧货原样保留，`Modified` 也不广播。演员继续念二稿，秋白看着报错改文件。这张安全网让“边跑边改”敢放开手脚：改坏了，最多是改动不生效。

**实验三：换图**。把那行坏词删掉存盘（台词恢复），然后偷天换日——运行中把灯笼的图盖到剑的文件上：

```powershell
Copy-Item assets/props/lantern.png assets/props/qingshuang-sword.png
```

台面上的“青霜剑”原地变成一盏灯笼：

![贴图热替换前后对比：左图台上是青霜剑，右图同一位置变成红灯笼](images/ch14/fig-14-04-hot-swap.png)

<span class="caption">Figure 14-4：文件一换，画面即换——实体、组件、Handle 谁都没动</span>

这场实验里**一行配合代码都没有**：Sprite 持的是 Handle，Handle 指着货架上的格子，格子里的 `Image` 被监视器换成了新内容——渲染下一帧照单取货，取到什么画什么。这正是“共享一份资产”架构的全部回报：换一处，处处生效。

> 看完实验记得收拾现场：`git checkout -- code/ch14-assets/assets` 或重跑 `py -3.11 scripts/make_ch14_assets.py`，把剑请回来。
>
> 另外两个实诚的注脚：其一，编辑器存盘动作可能触发两次文件事件，于是偶尔会看到连报两条 ERROR 或连念两遍——这是文件系统的脾气，不是 bug；其二，嵌入资产（14.7 节）默认不受监视，要热重载它们得另开 `embedded_watcher` feature。

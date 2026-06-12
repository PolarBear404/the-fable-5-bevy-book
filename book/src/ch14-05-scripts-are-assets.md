# 剧本也是资产

秋白交稿了。《长风渡》的开场戏躺在 `assets/scripts/opening.script` 里——这是剧组自己定的格式，纯文本，一行一句词：

```text
{{#include ../../code/ch14-assets/assets/scripts/opening.script}}
```

<span class="caption">剧组的 .script 格式：`#` 是批注，`幕名：` 定标题，其余行是“角色：台词”（assets/scripts/opening.script）</span>

把它 `load` 进来会怎样？库房一脸茫然：PNG 它认识（引擎内置了 `ImageLoader`），`.script` 是哪门子货？**Asset 系统的全部内置能力，对自定义格式一视同仁地开放**——只要办两件事：声明一种资产类型，再教库房怎么从字节变出它。

## 声明资产：derive(Asset)

```rust
{{#include ../../code/ch14-assets/examples/listing-14-07.rs:asset}}
```

<span class="caption">Listing 14-7（节选一）：剧本的内存形态——一个普通 struct 加一行 derive（examples/listing-14-07.rs）</span>

`Script` 就是个普普通通的 struct。`#[derive(Asset)]` 给它发资格证：能住进 `Assets<Script>` 货架、能开 `Handle<Script>` 提货单、有自己的 `AssetEvent<Script>` 广播频道。旁边的 `TypePath` 是 Asset 的前置要求——给类型发一张全局唯一的“名片”（如 `listing_14_07::Script`），日志与诊断靠它点名；第 31 章讲反射时它还会再登场。

## 教库房读格式：AssetLoader

装载器先把“可能出的错”说清楚——这正是 `thiserror` 的用武之地，每个变体配一条人话错误信息：

```rust
{{#include ../../code/ch14-assets/examples/listing-14-07.rs:error}}
```

<span class="caption">Listing 14-7（节选二）：错误类型——IO 错、编码错、格式错，各有各的说法（examples/listing-14-07.rs）</span>

然后是装载器本体，一个实现了 `AssetLoader` trait 的类型：

```rust
{{#include ../../code/ch14-assets/examples/listing-14-07.rs:loader}}
```

<span class="caption">Listing 14-7（节选三）：ScriptLoader——从字节流到 Script 的全部手艺（examples/listing-14-07.rs）</span>

逐项过一遍这个 trait：

- 三个关联类型表明身份：产出 `Asset = Script`；`Settings = ()`（本装载器没有可调选项——有选项长什么样，14.7 节见）；错误是刚才的 `ScriptLoaderError`；
- **`load` 是 `async fn`**——它运行在后台 IO 任务里，正是 14.1 节那条“后台时间线”的真身。`reader` 是文件内容的异步读取口，`read_to_end` 把字节全部收进来时用 `.await` 让出线程，一帧都不堵主循环；
- 解析部分是纯粹的 Rust 字符串功夫，没有任何 Bevy 知识：逐行 `trim`、跳过批注、按全角冒号 `split_once`。认不出的行**报错带行号**——错误信息的质量，决定了三天后排查问题的你对今天的你是感激还是怨恨；
- `extensions` 认领扩展名：今后凡是 `.script` 结尾的路径都派给本装载器，`load` 调用处不需要任何额外说明。

最后在 App 上登记这两样东西：

```rust
{{#include ../../code/ch14-assets/examples/listing-14-07.rs:register}}
```

<span class="caption">Listing 14-7（节选四）：init_asset 建货架，init_asset_loader 上岗装载器（examples/listing-14-07.rs）</span>

`init_asset::<Script>` 给新资产上户口（建 `Assets<Script>` 货架、开广播频道），`init_asset_loader` 让装载器上岗。两行登记，`.script` 从此与 PNG 平起平坐。顺带一提，装载器自己也得 `derive(TypePath)`——失败日志里“哪个装载器出的错”就靠它署名。

## 两条进货路

```rust
{{#include ../../code/ch14-assets/examples/listing-14-07.rs:fetch}}
```

<span class="caption">Listing 14-7（节选五）：load 走库房异步到货；Assets::add 手工上架当场可取（examples/listing-14-07.rs）</span>

这里故意并排走了两条路。`load` 是熟路：开单、后台解析、异步到货。`scripts.add(...)` 是新路：**程序运行中生成的数据直接上架**——不读盘、不经装载器、同步完成，下一行 `get` 立刻有货。运行时生成的纹理、代码拼出来的网格，走的都是这条路；它发的 Handle 与 `load` 的一视同仁，照样计数、照样能交给别人。

```rust
{{#include ../../code/ch14-assets/examples/listing-14-07.rs:recite}}
```

<span class="caption">Listing 14-7（节选六）：对词系统——货没到就下次再来，到了按节拍念（examples/listing-14-07.rs）</span>

`recite` 开头那个 `let Some(script) = scripts.get(...) else { return }` 值得划线：这是**资产驱动系统的标准开场白**——拿不到货就安静退场，下一帧再试，既不堵也不慌。对 `Assets<Script>` 的用法与 `Assets<Image>` 别无二致：自定义资产是系统的一等公民。

```console
cargo run -p ch14-assets --example listing-14-07
```

```text
老顾：手抄的垫场词直接上架，当场可取——true。
老雷：剧本到了——《渡口夜话》，5 句词。对词！
阿燕：二十年了，这把剑还认得回家的路。
梢公：客官，夜里风大，进舱吧。
阿燕：不了。我在等一个人。
梢公：那位贵客，怕是不会来喽。
阿燕：他会来。风往北吹，他就往南走。
老雷：第一稿就这个样，咔。
```

第一行先到（`add` 同步），《渡口夜话》随后到货，五句词按两秒一句的节拍过完。秋白的文字第一次跑在引擎里——而引擎压根不知道什么叫剧本，它只知道：这是件登记过的资产，照章办事。

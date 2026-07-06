# WASM 内嵌 demo 操作手册（给接活的 agent）

政策与判定看 CLAUDE.md「交互演示规范」；本文是操作事实。现存范本：ch20 打瓦——
`scripts/build_ch20_wasm.py` + `book/src/demos/ch20/index.html`（手写入库）+ 正文
`<figure class="bevy-demo">` 嵌入段。交付物：`scripts/build_chXX_wasm.py`、宿主页、
正文嵌入段、preview 实跑验证记录。

## 构建

- 流程：`cargo build --profile wasm-release --target wasm32-unknown-unknown -p <crate>
  [--example <名>]` → `wasm-bindgen --target web --out-name <名>` 输出到
  `book/src/demos/chXX/`。产物路径按目标分：bin 在 `target/<t>/<p>/<crate>.wasm`，
  example 在 `target/<t>/<p>/examples/<名>.wasm`。out_name 必须是合法标识符（无连字符）。
- **0.19 必须加 `--features bevy/web`**（浏览器 RAF 主循环等的总闸，不在默认集）。
- wasm-bindgen-cli 版本必须对齐 code/Cargo.lock（当前 0.2.126），脚本内置一致性检查，
  不符即退。安装时 schannel 握手失败：`$env:CARGO_HTTP_CHECK_REVOKE="false";
  $env:CARGO_HTTP_MULTIPLEXING="false"` 再 `cargo install`。
- `wasm-release` profile 已在 code/Cargo.toml（opt-level="z" + lto + strip，70 MB→20 MB 级）。
- 产物（*.js/*.wasm/assets）已 gitignore；禁止手工构建，一切进脚本。
- 脚本顶部 `sys.stdout.reconfigure(encoding="utf-8")`（GBK 控制台坑）。

## 同源代码

- demo 必须是本章真实可编译示例：`WindowPlugin { primary_window: Window {
  canvas: Some("#bevy-chXX"), fit_canvas_to_parent: true, .. } }`——这些字段非 web
  平台无效，桌面与网页同一份代码。
- 把某个 listing 编成 demo：web 专用代码（WindowPlugin、点击系统）放在 `// ANCHOR`
  截取区**之外**，正文 include 出的代码一字不变。构建用 `--example`，canvas id 与
  import 名要对上。

## 宿主页与嵌入

- 懒加载模板已就位：`book/theme/demo.{css,js}` + book.toml 挂接。正文只写
  `<figure class="bevy-demo" data-src="demos/chXX/index.html">` + 占位图 + figcaption；
  占位图用本章真实截图（或既有动图），无 JS/打印时凭它自足。
- 2D 固定几何画幅：canvas 外包固定设计尺寸的 `#stage`（如 920×720），CSS
  `transform: scale()` 等比缩放——transform 不改 content box，fit_canvas_to_parent
  读到的仍是设计尺寸。3D 透视相机自适应，不需要。
- 音频三件套（bevy_audio 对浏览器 autoplay 零处理，全靠宿主页）：①启动层——点按钮
  才 import wasm，AudioContext 在手势链内创建即 running；init 后 `canvas.focus()`；
  ②兜底——module script 顶部劫持 AudioContext 构造器收集实例，document capture 阶段
  keydown/pointerdown 一律 resume suspended；③散场——AppExit 后 rodio drop 输出流、
  AudioContext 转 `closed`，statechange 监听到 closed 盖「已散场」幕布，点击
  `location.reload()`。
- 资产 `.meta` 404 是 AssetServer 正常探测，无害。

## web 输入铁则

- 鼠标拖动交互**禁用 `AccumulatedMouseMotion`**——浏览器里 raw 位移要 pointer lock
  才有数据，iframe 内恒为 0、桌面却正常，极易漏判。改用 `Window::cursor_position()`
  + `Local<Option<Vec2>>` 存上一帧算 delta（ch17 手法）。

## preview 验证（先跑后写）

- `.claude/launch.json` 已有 py http.server 8234 指 `book/book`——先 `mdbook build book`
  再起 server。无头浏览器走真 GPU（ANGLE/D3D11），能真渲染 Bevy。
- `preview_click` 合成事件**穿不进 iframe**：iframe 内按钮用
  `preview_eval` + `contentDocument...click()`。preview_eval 走 CDP 自带 userGesture，
  AudioContext 直接 running——测不出 suspended 路径，别据此判兜底失效。
- 键盘：合成 `KeyboardEvent`（带 `code`，dispatch 到 canvas）winit 照收，全键路可
  自动验证。`preview_click` 点 canvas 能驱动 `MouseButton::Left` just_pressed。
- 导航别写进 `preview_eval`（`location.href=` 杀掉 eval 上下文，还会把后续 screenshot
  卡到超时）——单独导航、再纯查询。
- WebGL canvas 无 preserveDrawingBuffer，`toDataURL`/`readPixels` 拿不到像素；判断
  「在动还是冻住」用前后两张 `preview_screenshot` 对比。
- 重型 demo（多球 PBR + IBL 级别）screenshot 会次次 30s 超时且重启不救：改看
  `preview_console_logs` 出现 `AdapterInfo {...WebGL 2.0...}` 且无 error 证渲染活着，
  画面长相用同源 main.rs 的桌面截图核对，交互在桌面同路径按键验证 + 网页 click 无
  error 旁证。
- 发布前把全部交互路径（按键/点击/暂停/退出重开）走一遍，记录进回报。
- **合成 pointermove 必须补 coalesced**（ch25 教训）：winit 0.30 在 Chrome 上只从
  `getCoalescedEvents()` 取 move 位置，合成 PointerEvent 该列表为空会被静默丢弃——
  dispatch 前给事件实例覆写 `e.getCoalescedEvents = () => [e]`。down/up/wheel 不受影响。
  用 iframe 自己 realm 的构造器（`new frame.contentWindow.PointerEvent(...)`），
  move 记得 `button: -1`（缺省 0 会被 winit 当按键弦事件）。
- **tab 必须真前台，否则全是假象**（ch25 教训）：后台 tab RAF 停摆，Bevy 一帧不跑，
  合成事件全部积压、回前台同帧涌入——press 到达时 hover map 还是旧位置的，「拖货」
  会被误判成拖空处，且 preview_screenshot 30s 超时。preview 面板不可见时换真 Chrome
  （claude-in-chrome）验证，并先用 Windows-MCP `App switch` 把 Chrome 窗口带到系统前台
  （`document.visibilityState` 变 visible 才算数）；zoom/screenshot 只带前台一瞬，不够跑帧。
- **多步交互用 RAF 分帧队列**：按下→拖→松手这类序列别用 setTimeout 定时发（后台被
  节流到 1Hz），把步骤数组交给 iframe 的 `requestAnimationFrame` 链、每 ~6 帧发一步，
  与 Bevy 帧序天然对齐；队列置完成 flag，eval 轮询 flag 确认跑完再验证下一段，
  绝不并行两条队列（交错序列会把状态搅脏）。
- **合成 KeyboardEvent 不保证被收**（ch26 教训，与上文 ch23 经验相反）：同为 winit 0.30，
  ch26 的 demo 里 dispatch 到 canvas 的合成 KeyboardEvent 全程无效（疑与键盘监听挂点/
  isTrusted 检查的路径差异有关）。合成键失灵时改走 claude-in-chrome `computer` 的 `key`
  动作（CDP 真实按键，需 tab 真前台），实测全键路可靠命中。先试合成、不行换 CDP，两条
  路都记进回报。
- **Claude Preview 面板对重型 demo 可能恒 `document.hidden=true`**（ch26 实测）：面板内
  RAF 不跑、screenshot 必超时，与 demo 本身无关。重型 demo 验证直接走真 Chrome tab
  （claude-in-chrome + 系统前台），preview 面板只用来起 server。
- **WebGL2 两条硬红线**（ch26 实测，正文 26.13 已入册）：①MSAA≠Off 时创建带
  TEXTURE_BINDING 的 prepass 纹理 → glow 层 `Tex storage 2D multisample is not supported`
  当场 panic——摘效果组件不够，required 补票的 prepass 组件要一并手摘；②TAA 管线
  naga 译 GLSL ES 报 `A image was used with multiple samplers` → RenderError 直接
  AppExit。两者桌面均无恙；web 构建里用 `cfg!(target_arch = "wasm32")` 把危险挡位封存。
- **隐藏 tab 卡管线预热的无干扰解法**（ch26 实测）：后台 tab 被 Chrome 停发 RAF 时，Bevy
  会卡在窗口创建/管线预热阶段一百多秒不动。连续调 CDP `Page.captureScreenshot`（computer
  的 screenshot 动作）会强制 Chrome 给隐藏 tab 出帧——两三张就能把 Bevy 泵过预热直到出图，
  不必抢用户前台。验证结论仍以状态牌文字/帧间变化为准。

# 插图流水线操作手册（给接活的 agent）

体例与政策看 CLAUDE.md「插图规范」；本文是踩过坑的操作事实。你的输入是本章工单
（workorders/chXX.md）里的插图规格清单，交付物是 `scripts/make_chXX_figures.py` 与
`book/src/images/chXX/` 下的全部图，且每张图要与规格逐条对照过。

## 环境

- 图脚本一律 `py -3.11` 运行（该解释器带 Pillow；裸 `python` 是 hermes venv，没有 pip）。
- 控制台默认 GBK：脚本顶部 `sys.stdout.reconfigure(encoding="utf-8")`（stderr 同），
  否则 print 到 `✓ → ≈` 直接 UnicodeEncodeError 崩全场。
- 骨架别从零写：抄最近一章的 `make_chXX_figures.py`（make_ch22_figures.py 最新最全，
  含 SendInput 三重保险；make_ch20_figures.py 是自动定标范本）。

## 截窗口（scripts/capture.py）

- 必须走 PrintWindow（PW_RENDERFULLCONTENT），不抓屏——别的窗口抢前台不污染截图。
- 渲染器初始化的几秒里 winit 只有一个 0×0 的「Winit Thread Event Target」假窗口：
  找窗口必须过滤「客户区非零」；采样时间零点以**真窗口出现**为准，不以进程启动为准。
- 采样时刻 ≥1.5s：窗口出现后渲染管线还要编译约 1 秒，期间清屏色已出、Sprite 全无
  （t=0.15 截到过空场景）。
- 截出来的是**物理像素**，且本机显示器缩放会来回变（见过 100% 与 125%）：一切裁剪、
  标注、bot 打靶几何按截图实际宽度自动定标 k=宽/1280，禁止写死 ×1.25 之类倍数；
  或先 `img.resize((1280,720), LANCZOS)` 归一化再处理。

## 驱动示例进程

- **驱动脚本禁用 `subprocess.PIPE`**：ICU4X 对每次中文文本重排刷 stderr，4KB 管道塞满
  → 子进程阻塞 → winit 消息泵冻结（窗口未响应、按键全丢）。临时驱动用文件重定向，
  正式 make 脚本 DEVNULL。
- SendInput 三重保险（make_ch22_figures.py 已内置）：①每次 tap/hold 前
  force_foreground（带 Alt 空击解锁前台限制，失败即 raise）；②按住期间穿插抓帧时
  每 3 帧重发一次 keydown（PrintWindow 会吃掉抬键/按住状态）；③段落收尾 keyup 发
  两次（间隔 50ms）。首个键击安排在窗口出现 ≥2.5s 后。
- SendInput 是否可用取决于会话环境：先跑再说；若注入失效（返回 0 或画面无响应），
  退路是**环境变量初始态钩子**——示例读 env（如 `CHXX_PRESET=0..3`）选开场状态，
  脚本逐态重启截帧。设计新交互示例时优先留 env 钩子，比注入稳。

## 动图（WebP）

- 对比动图：两个示例分别录、逐帧并排合成（走位由 elapsed_secs 决定，轨迹确定，相位差小）。
- WebP q70、10fps、12s ≈ 360 KB；上限 2 MB。
- 验证帧内容别用 `ImageSequence.Iterator` 直接 `list()`——它逐次返回同一个被原地 seek
  的对象，收集到的全是末帧；要 `seek(i)` + `copy()`。WebP 有损，ImageChops 差分数字
  仅作参考。

## 概念示意图（手绘 SVG）

- 浅色卡片底：#f7f5f0 圆角矩形打底，明暗主题都可读。
- 自检：无头 Edge `--headless --screenshot` 渲染后 Read 目检。**必须传 `file:///C:/...`
  绝对 URL（空格转 %20）并加 `--no-proxy-server`**——相对路径会被当 URL 走系统代理
  （127.0.0.1:7897），截回 502 错误页。
- 文字与虚线相撞用 `stroke="#f7f5f0" paint-order="stroke"` 挖空；✓/✗ 这类细笔画字符
  会被描边吃掉，别用这招。

## 交付前

- 每张产物 Read 目检一遍；逐条对照工单规格（编号、画面内容、静/动）。
- 截图与正文同受「先跑后写」约束：画面与规格/正文描述不符时，回报差异（修代码或修
  正文是主会话的决定），不准 P 图。

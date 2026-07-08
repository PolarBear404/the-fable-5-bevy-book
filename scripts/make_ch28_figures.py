# -*- coding: utf-8 -*-
"""一键重建第 28 章全部 20 张插图（16 张运行截图/拼图 + 4 张手绘 SVG）。

    py -3.11 scripts/make_ch28_figures.py [编号筛选...]
    例：py -3.11 scripts/make_ch28_figures.py 05 12   # 只重建 fig-28-05 与 fig-28-12

运行图全部由本章 example / main 实拍。注入手法沿用 ch27 的实测结论
（本机 SendInput 会被前台完整性级别挡下，见 workorders/ch27.md）：
- 键盘：PostMessage WM_KEYDOWN/WM_KEYUP（scratchpad/probe_ch28.py 全场景验证过）；
- 拖窗：SetWindowPos 改窗口外框（物理像素），触发真实 resize。

几何约定：截图是物理像素、显示器缩放会变——1280×720 设计窗先归一到逻辑像素
（k = 截图宽/1280）再裁切标注；被拖过的窗口按同一 k 归一，禁止写死倍数。
"""

import ctypes
import os
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

sys.stdout.reconfigure(encoding="utf-8")
sys.stderr.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch28-ui-layout"
EXAMPLES = CODE / "target" / "debug" / "examples"
MAIN_EXE = CODE / "target" / "debug" / "ch28-ui-layout.exe"
OUT = ROOT / "book" / "src" / "images" / "ch28"

# 示例读字体/皮子等资产：跟实测台账一致，资产根钉在本章 crate
os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import find_main_window, grab_window, _set_dpi_aware  # noqa: E402

user32 = ctypes.windll.user32

FONT_PATH = str(CRATE / "assets" / "fonts" / "book-sans-sc-bold.otf")
FONT = ImageFont.truetype(FONT_PATH, 20)       # 拼图标注条
BAR_FONT = ImageFont.truetype(FONT_PATH, 22)   # 画面内合成标注（fig-28-03）
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36

# ---------------------------------------------------------------- PostMessage 注入

WM_CLOSE = 0x0010
WM_KEYDOWN, WM_KEYUP = 0x0100, 0x0101

# 名字 -> (virtual-key, scancode, extended)
KEYS = {
    "A": (0x41, 0x1E, False), "B": (0x42, 0x30, False), "D": (0x44, 0x20, False),
    "G": (0x47, 0x22, False), "H": (0x48, 0x23, False), "I": (0x49, 0x17, False),
    "J": (0x4A, 0x24, False), "N": (0x4E, 0x31, False), "O": (0x4F, 0x18, False),
    "P": (0x50, 0x19, False), "S": (0x53, 0x1F, False), "T": (0x54, 0x14, False),
    "U": (0x55, 0x16, False), "W": (0x57, 0x11, False), "Z": (0x5A, 0x2C, False),
    "SPACE": (0x20, 0x39, False),
    "F3": (0x72, 0x3D, False),
}


def _post_key(hwnd: int, name: str, down: bool) -> None:
    vk, scan, ext = KEYS[name]
    lparam = 1 | (scan << 16)
    if ext:
        lparam |= 1 << 24
    if not down:
        lparam |= (1 << 30) | (1 << 31)
    user32.PostMessageW(hwnd, WM_KEYDOWN if down else WM_KEYUP, vk, lparam)


class Run:
    """一个受测示例：stdout/stderr 进 DEVNULL（ICU4X 刷 stderr 会塞死管道），
    PostMessage 注键、SetWindowPos 拖窗、PrintWindow 截图。"""

    def __init__(self, name: str, exe: Path | None = None):
        _set_dpi_aware()
        self.name = name
        exe = exe or (EXAMPLES / f"{name}.exe")
        self.proc = subprocess.Popen(
            [str(exe)], cwd=str(CRATE),
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        )
        self.hwnd = find_main_window(self.proc.pid)
        self.t0 = time.perf_counter()
        user32.SetForegroundWindow(self.hwnd)

    def wait_until(self, t_since_window: float) -> None:
        remain = self.t0 + t_since_window - time.perf_counter()
        if remain > 0:
            time.sleep(remain)

    def grab(self) -> Image.Image:
        return grab_window(self.hwnd)

    def tap(self, key: str, settle: float = 0.35) -> None:
        _post_key(self.hwnd, key, True)
        time.sleep(0.06)
        _post_key(self.hwnd, key, False)
        time.sleep(settle)

    def resize(self, w: int, h: int, settle: float = 1.2) -> None:
        """把窗口外框改成 w×h（物理像素），触发真实 resize。"""
        SWP_NOMOVE, SWP_NOZORDER = 0x0002, 0x0004
        user32.SetWindowPos(self.hwnd, 0, 0, 0, int(w), int(h), SWP_NOMOVE | SWP_NOZORDER)
        time.sleep(settle)

    def close(self) -> None:
        user32.PostMessageW(self.hwnd, WM_CLOSE, 0, 0)
        try:
            self.proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            self.proc.kill()

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        self.close()


# ---------------------------------------------------------------- 通用排版

def cargo(*args: str) -> None:
    subprocess.run(["cargo", *args], cwd=CODE, check=True)


def logical(img: Image.Image, size=(1280, 720)) -> Image.Image:
    """物理像素 → 逻辑像素（显示器缩放会变，一律归一后再裁切/标注）。"""
    if img.size == size:
        return img
    return img.resize(size, Image.LANCZOS)


def normalize(img: Image.Image, k: float) -> Image.Image:
    """按本次运行实测的缩放系数 k 归一到逻辑像素（用于拖过的窗口）。"""
    return img.resize((round(img.width / k), round(img.height / k)), Image.LANCZOS)


def shrink(img: Image.Image, k: float) -> Image.Image:
    return img.resize((round(img.width * k), round(img.height * k)), Image.LANCZOS)


def label_bar(width: int, texts: list[str]) -> Image.Image:
    bar = Image.new("RGB", (width, LABEL_H), LABEL_BG)
    draw = ImageDraw.Draw(bar)
    cell = width / len(texts)
    for i, text in enumerate(texts):
        w = draw.textlength(text, font=FONT)
        draw.text((cell * i + (cell - w) / 2, 6), text, font=FONT, fill=LABEL_FG)
    return bar


def hstack_below(images: list[Image.Image], labels: list[str]) -> Image.Image:
    """多联横排，每联下方各自一条标注条（各联可不等高，顶对齐）。"""
    h = max(im.height for im in images)
    w = sum(im.width for im in images) + GAP * (len(images) - 1)
    canvas = Image.new("RGB", (w, h + LABEL_H), GAP_COLOR)
    x = 0
    for im, text in zip(images, labels):
        canvas.paste(im, (x, 0))
        canvas.paste(label_bar(im.width, [text]), (x, im.height))
        x += im.width + GAP
    return canvas


def vstack_below(images: list[Image.Image], labels: list[str]) -> Image.Image:
    """多联竖排，每联下方各自一条标注条。"""
    w = max(im.width for im in images)
    h = sum(im.height + LABEL_H for im in images) + GAP * (len(images) - 1)
    canvas = Image.new("RGB", (w, h), GAP_COLOR)
    y = 0
    for im, text in zip(images, labels):
        x = (w - im.width) // 2
        canvas.paste(im, (x, y))
        canvas.paste(label_bar(im.width, [text]), (x, y + im.height))
        y += im.height + LABEL_H + GAP
    return canvas


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def save_svg(text: str, filename: str) -> None:
    path = OUT / filename
    path.write_text(text, encoding="utf-8")
    print(f"{filename}：{path.stat().st_size // 1024} KB")


# ---------------------------------------------------------------- Figure 28-1：第一块牌子

def fig_01_first_node() -> None:
    """Figure 28-1：左上角 320×140 朱漆牌 + 默认灰清屏底（全窗单图）。"""
    with Run("listing-28-01") as r:
        r.wait_until(2.2)
        save_png(logical(r.grab()), "fig-28-01-first-node.png")


# ---------------------------------------------------------------- Figure 28-3：单位阅兵

def fig_03_units_parade() -> None:
    """Figure 28-3：五条彩条开场帧，右侧合成单位名标注（画面内容不动）。"""
    with Run("listing-28-03") as r:
        r.wait_until(2.2)
        shot = logical(r.grab())
    # 五块牌：padding 16、高 44、gap 12；宽（逻辑）按台账 300/624/640/360/360
    bars = [
        ("Px(300)", 300), ("Percent(50)", 624), ("Vw(50)", 640),
        ("Vh(50)", 360), ("VMin(50)", 360),
    ]
    d = ImageDraw.Draw(shot)
    for i, (label, width) in enumerate(bars):
        y = 16 + i * 56
        d.text((16 + width + 14, y + 9), label, font=BAR_FONT, fill=(225, 225, 228))
    save_png(shot.crop((0, 0, 1280, 300)), "fig-28-03-units-parade.png")


# ---------------------------------------------------------------- Figure 28-5：透视镜

def fig_05_box_xray() -> None:
    """Figure 28-5：F3 透视镜下两块牌三层描框。

    验收点：两牌之间的缝（margin 16+16 逻辑）≈ 40 物理像素——在开镜前的
    原始物理截图上实测（开镜后缝两侧多出描线，量不准）。
    """
    with Run("listing-28-04") as r:
        r.wait_until(2.5)
        raw = r.grab()                       # 开镜前：量缝用
        r.tap("F3", settle=0.9)
        shot = logical(r.grab())
    # 量缝：两块牌边框盒之间（逻辑 592..624，中心 608）的背景连续段
    k = raw.width / 1280
    y = round(360 * k)
    bg = raw.getpixel((round(8 * k), round(8 * k)))
    px = raw.load()

    def is_bg(x: int) -> bool:
        c = px[x, y]
        return all(abs(c[i] - bg[i]) <= 12 for i in range(3))

    cx = round(608 * k)
    if not is_bg(cx):
        raise RuntimeError("fig-28-05：牌缝中心采样点不是背景色，布局有变，先核对画面")
    x0, x1 = cx, cx
    while x0 > 0 and is_bg(x0 - 1):
        x0 -= 1
    while x1 < raw.width - 1 and is_bg(x1 + 1):
        x1 += 1
    gap = x1 - x0 + 1
    if not 34 <= gap <= 46:
        raise RuntimeError(f"fig-28-05：实测牌缝 {gap} 物理像素，偏离 ≈40 的验收点")
    print(f"  两牌之间实测缝 {gap} 物理像素（验收点 ≈40）")
    save_png(shot.crop((300, 215, 980, 505)), "fig-28-05-box-xray.png")


# ---------------------------------------------------------------- Figure 28-7/8：对齐板

# 对齐板：640×360 金框居中——裁它周围一圈
STAGE_BOX = (304, 164, 976, 556)


def fig_07_alignment_grid() -> None:
    """Figure 28-7：六档 2×3 拼图。上排拨 justify_content，下排拨 align_items。

    一次运行连拨：J 序 FlexStart→Center→…→SpaceEvenly→回 FlexStart，再 A 序。
    """
    shots: list[tuple[str, Image.Image]] = []
    with Run("listing-28-05") as r:
        r.wait_until(2.5)
        shots.append(("justify_content: FlexStart", logical(r.grab())))
        r.tap("J")
        shots.append(("justify_content: Center", logical(r.grab())))
        r.tap("J"); r.tap("J")
        shots.append(("justify_content: SpaceBetween", logical(r.grab())))
        r.tap("J"); r.tap("J")
        shots.append(("justify_content: SpaceEvenly", logical(r.grab())))
        r.tap("J")                          # 绕回 FlexStart，孤立 align 变量
        r.tap("A")
        shots.append(("align_items: Center", logical(r.grab())))
        r.tap("A"); r.tap("A")
        shots.append(("align_items: Stretch", logical(r.grab())))
    cells = [shrink(img.crop(STAGE_BOX), 0.66) for _, img in shots]
    labels = [label for label, _ in shots]
    top = hstack_below(cells[:3], labels[:3])       # 档名条在每格下方，不压画面
    bottom = hstack_below(cells[3:], labels[3:])
    canvas = Image.new("RGB", (top.width, top.height + GAP + bottom.height), GAP_COLOR)
    canvas.paste(top, (0, 0))
    canvas.paste(bottom, (0, top.height + GAP))
    save_png(canvas, "fig-28-07-alignment-grid.png")


def fig_08_align_self() -> None:
    """Figure 28-8：S×2（align_self: FlexEnd）——一号三号贴顶，二号独沉板底。"""
    with Run("listing-28-05") as r:
        r.wait_until(2.5)
        r.tap("S", settle=0.5)
        r.tap("S", settle=0.5)
        shot = logical(r.grab())
    save_png(shot.crop(STAGE_BOX), "fig-28-08-align-self.png")


# ---------------------------------------------------------------- Figure 28-9：收缩 vs 换行

def fig_09_shrink_vs_wrap() -> None:
    """Figure 28-9：窄窗（外框 560×500 物理）上下两联——NoWrap 压缩 vs Wrap 三行。

    正文的实验顺序里 W 之前 G 已把大户 grow 拨成 0（Wrap 后大户缩在底数 220，
    与正文报数一致）；G 在亏空局面下不影响上联（分摊只看 shrink）。
    """
    with Run("listing-28-06") as r:
        r.wait_until(2.5)
        k = r.grab().width / 1280           # 拖窗前定标：物理/逻辑
        r.tap("G", settle=0.4)              # 复现正文档位：大户 grow 2 → 0
        r.resize(560, 500)
        nowrap = normalize(r.grab(), k)
        r.tap("W", settle=0.7)
        wrap = normalize(r.grab(), k)
    w = nowrap.width
    save_png(
        vstack_below(
            [nowrap.crop((0, 0, w, 140)), wrap.crop((0, 0, w, 340))],
            ["NoWrap（默认）——三户挤一行，两户被压扁", "W 拨 Wrap——一户一行，佃户独占整行"],
        ),
        "fig-28-09-shrink-vs-wrap.png",
    )


# ---------------------------------------------------------------- Figure 28-10：隐身 vs 离场

def fig_10_hidden_vs_none() -> None:
    """Figure 28-10：上下两联——H 后留空位 vs N 后合拢（四角告示+横幅全在画）。"""
    with Run("listing-28-07") as r:
        r.wait_until(2.5)
        r.tap("H", settle=0.6)
        hidden = logical(r.grab())
        r.tap("H", settle=0.4)              # 请老二回来
        r.tap("N", settle=0.6)
        gone = logical(r.grab())
    save_png(
        vstack_below(
            [shrink(hidden, 0.55), shrink(gone, 0.55)],
            ["H：Visibility::Hidden——老二不画了，席位还留着",
             "N：Display::None——老二从布局除名，两侧合拢"],
        ),
        "fig-28-10-hidden-vs-none.png",
    )


# ---------------------------------------------------------------- Figure 28-11：叠放三联

def fig_11_zindex() -> None:
    """Figure 28-11：初始丙顶 / Z 后甲顶 / G 后横幅盖全场。"""
    with Run("listing-28-08") as r:
        r.wait_until(2.5)
        base = logical(r.grab())
        r.tap("Z", settle=0.6)
        z_up = logical(r.grab())
        r.tap("G", settle=0.6)
        g_up = logical(r.grab())
    box = (20, 0, 600, 410)                 # 三张告示 + 横幅所在的左半场
    panels = [shrink(img.crop(box), 0.76) for img in (base, z_up, g_up)]
    save_png(
        hstack_below(
            panels,
            ["初始：树序说了算，丙在最上", "Z：甲 ZIndex 拨到 2，兄弟里登顶",
             "G：横幅 GlobalZIndex 拨到 1，全局登顶"],
        ),
        "fig-28-11-zindex.png",
    )


# ---------------------------------------------------------------- Figure 28-13/14：Grid

def fig_13_seating_chart() -> None:
    """Figure 28-13：座位表开场——包厢 2×2 描金、乐池横贯底行、散座填空。"""
    with Run("listing-28-09") as r:
        r.wait_until(2.5)
        shot = logical(r.grab())
    save_png(shot.crop((288, 168, 992, 552)), "fig-28-13-seating-chart.png")


def fig_14_forgot_display_grid() -> None:
    """Figure 28-14：上下两联——Flex 挤一排 vs D 后 4×2 归格。"""
    with Run("listing-28-11") as r:
        r.wait_until(2.5)
        flex = logical(r.grab())
        r.tap("D", settle=0.7)
        grid = logical(r.grab())
    box = (348, 258, 932, 462)
    save_png(
        vstack_below(
            [flex.crop(box), grid.crop(box)],
            ["display 还是默认的 Flex——格子字段被静默无视", "D 拨 display: Grid——同一份图纸当场归格"],
        ),
        "fig-28-14-forgot-display-grid.png",
    )


# ---------------------------------------------------------------- Figure 28-15：四种绷法

def fig_15_image_modes() -> None:
    """Figure 28-15：上排 Auto/Stretch/Sliced/Tiled，下排四签+半透明第五签（全景）。"""
    with Run("listing-28-12") as r:
        r.wait_until(2.6)                   # 皮子与图集要等资产加载
        shot = logical(r.grab())
    save_png(shot.crop((0, 210, 1280, 510)), "fig-28-15-image-modes.png")


# ---------------------------------------------------------------- Figure 28-16：字捅出板外

def fig_16_text_overflow() -> None:
    """Figure 28-16：上下两联——NoWrap 字捅出右板 vs O 后裁齐（匾额带影子在画）。"""
    with Run("listing-28-13") as r:
        r.wait_until(2.6)                   # 字体加载 + CJK 排版
        visible = logical(r.grab())
        r.tap("O", settle=0.7)
        clipped = logical(r.grab())
    box = (280, 10, 1180, 200)
    save_png(
        vstack_below(
            [visible.crop(box), clipped.crop(box)],
            ["LineBreak::NoWrap——字从板子右缘捅出去", "O 拨 Overflow::clip()——沿板边裁齐"],
        ),
        "fig-28-16-text-overflow.png",
    )


# ---------------------------------------------------------------- Figure 28-17：跟哪台相机

def fig_17_ui_target_camera() -> None:
    """Figure 28-17：左右两联——默认挤右下墨绿画幅 vs T 点名后横贯全景。"""
    with Run("listing-28-14") as r:
        r.wait_until(2.5)
        default = logical(r.grab())
        r.tap("T", settle=0.7)
        assigned = logical(r.grab())
    save_png(
        hstack_below(
            [shrink(default, 0.55), shrink(assigned, 0.55)],
            ["默认：跟 order 高的角落相机（640×360 画幅）",
             "T 点名 UiTargetCamera(全景)——铺满 1280×720"],
        ),
        "fig-28-17-ui-target-camera.png",
    )


# ---------------------------------------------------------------- Figure 28-18/19/20：《前厅》

def fig_18_19_20_front_hall() -> None:
    """Figure 28-18/19/20：一次运行连拍——
    空格×5 + H（比分 250、第三球褪色、五签亮）→ P 中场横幅 → 拖窄 760×620。"""
    with Run("main", MAIN_EXE) as r:
        r.wait_until(2.6)
        for _ in range(5):
            r.tap("SPACE", settle=0.25)
        r.tap("H", settle=0.5)
        wide_raw = r.grab()
        k = wide_raw.width / 1280
        wide = logical(wide_raw)
        save_png(wide, "fig-28-18-front-hall.png")

        r.tap("P", settle=0.9)
        save_png(logical(r.grab()), "fig-28-19-intermission.png")
        r.tap("P", settle=0.5)              # 收横幅再拖窗

        r.resize(760, 620, settle=1.4)
        narrow = normalize(r.grab(), k)

    left = shrink(wide, 0.62)
    right = shrink(narrow, 0.62)
    w = left.width + GAP + right.width
    h = max(left.height, right.height) + LABEL_H
    canvas = Image.new("RGB", (w, h), GAP_COLOR)
    canvas.paste(left, (0, 0))
    canvas.paste(label_bar(left.width, ["宽窗 1280×720"]), (0, left.height))
    canvas.paste(right, (left.width + GAP, 0))
    canvas.paste(
        label_bar(right.width, ["拖窄后——阵型不变、格子仍方"]),
        (left.width + GAP, right.height),
    )
    save_png(canvas, "fig-28-20-responsive.png")


# ---------------------------------------------------------------- 手绘 SVG
# 内容即代码：落盘即重建（插图规范）。#f7f5f0 圆角卡片底，明暗主题均可读。

SVG_02_LAYOUT_TIMING = """<svg viewBox="0 0 880 470" xmlns="http://www.w3.org/2000/svg" font-family="system-ui, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr28a" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#9a9280"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="880" height="470" rx="10" fill="#f7f5f0"/>
  <text x="440" y="32" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">布局在一帧里的位置——三把量尺，两种读数</text>

  <!-- ============ 调度带 ============ -->
  <!-- Startup -->
  <rect x="36" y="118" width="140" height="60" rx="7" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.6"/>
  <text x="106" y="143" text-anchor="middle" font-size="13.5" fill="#4a463f" font-weight="bold">Startup</text>
  <text x="106" y="162" text-anchor="middle" font-size="10.5" fill="#7a7468">开台，只跑一次</text>
  <path d="M180,148 L200,148" stroke="#9a9280" stroke-width="1.8" fill="none" marker-end="url(#arr28a)"/>

  <!-- 第 1 帧 -->
  <rect x="204" y="92" width="420" height="100" rx="9" fill="none" stroke="#b8862e" stroke-width="1.8"/>
  <text x="220" y="110" font-size="12" fill="#8a6a1e" font-weight="bold">第 1 帧</text>
  <rect x="220" y="118" width="118" height="60" rx="7" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.6"/>
  <text x="279" y="143" text-anchor="middle" font-size="13.5" fill="#4a463f" font-weight="bold">Update</text>
  <text x="279" y="162" text-anchor="middle" font-size="10.5" fill="#7a7468">游戏逻辑改意图</text>
  <path d="M342,148 L360,148" stroke="#9a9280" stroke-width="1.8" fill="none" marker-end="url(#arr28a)"/>
  <rect x="364" y="118" width="244" height="60" rx="7" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.6"/>
  <text x="486" y="134" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">PostUpdate</text>
  <rect x="376" y="142" width="164" height="28" rx="5" fill="#efe0c4" stroke="#b8862e" stroke-width="1.8"/>
  <text x="458" y="156" text-anchor="middle" font-size="10.5" fill="#8a6a1e" font-family="Consolas, monospace">UiSystems::Layout</text>
  <text x="458" y="167" text-anchor="middle" font-size="9" fill="#8a6a1e">布局结算</text>
  <text x="574" y="160" text-anchor="middle" font-size="12" fill="#9a9280">…</text>

  <!-- 第 2 帧 -->
  <path d="M628,148 L648,148" stroke="#9a9280" stroke-width="1.8" fill="none" marker-end="url(#arr28a)"/>
  <rect x="652" y="92" width="196" height="100" rx="9" fill="none" stroke="#9a9280" stroke-width="1.5" stroke-dasharray="6 4"/>
  <text x="668" y="110" font-size="12" fill="#7a7468" font-weight="bold">第 2 帧</text>
  <rect x="668" y="118" width="96" height="34" rx="6" fill="none" stroke="#9a9280" stroke-width="1.3" stroke-dasharray="4 3"/>
  <text x="716" y="140" text-anchor="middle" font-size="12" fill="#7a7468">Update</text>
  <text x="782" y="140" font-size="12" fill="#9a9280">……</text>
  <text x="758" y="172" text-anchor="middle" font-size="10" fill="#7a7468">这儿再量：读到的是</text>
  <text x="758" y="186" text-anchor="middle" font-size="10" fill="#7a7468">上一帧的结算</text>

  <!-- ============ 三把量尺 ============ -->
  <circle cx="106" cy="178" r="3.2" fill="#b3402e"/>
  <path d="M106,181 L106,238" stroke="#b3402e" stroke-width="1.5" stroke-dasharray="4 3" fill="none"/>
  <rect x="30" y="240" width="182" height="84" rx="8" fill="#ffffff" stroke="#b3402e" stroke-width="1.6"/>
  <text x="121" y="262" text-anchor="middle" font-size="12" fill="#b3402e" font-weight="bold">量尺 ①　开台量一次</text>
  <text x="121" y="284" text-anchor="middle" font-size="13" fill="#b3402e" font-family="Consolas, monospace">size = 0×0</text>
  <text x="121" y="308" text-anchor="middle" font-size="10.5" fill="#7a7468">布局一次都还没跑</text>

  <circle cx="279" cy="178" r="3.2" fill="#b3402e"/>
  <path d="M279,181 L279,238 L300,238 L300,336" stroke="#b3402e" stroke-width="1.5" stroke-dasharray="4 3" fill="none"/>
  <rect x="222" y="338" width="182" height="84" rx="8" fill="#ffffff" stroke="#b3402e" stroke-width="1.6"/>
  <text x="313" y="360" text-anchor="middle" font-size="12" fill="#b3402e" font-weight="bold">量尺 ②　Update 里量</text>
  <text x="313" y="382" text-anchor="middle" font-size="13" fill="#b3402e" font-family="Consolas, monospace">size = 0×0</text>
  <text x="313" y="406" text-anchor="middle" font-size="10.5" fill="#7a7468">当帧结算还没开始</text>

  <circle cx="560" cy="178" r="3.2" fill="#1d6b40"/>
  <path d="M560,181 L560,238" stroke="#1d6b40" stroke-width="1.5" stroke-dasharray="4 3" fill="none"/>
  <rect x="446" y="240" width="252" height="98" rx="8" fill="#ffffff" stroke="#1d6b40" stroke-width="1.8"/>
  <text x="572" y="262" text-anchor="middle" font-size="12" fill="#1d6b40" font-weight="bold">量尺 ③　排在 Layout 之后</text>
  <text x="572" y="286" text-anchor="middle" font-size="13" fill="#1d6b40" font-family="Consolas, monospace">size = 400×175（物理）</text>
  <text x="572" y="306" text-anchor="middle" font-size="10.5" fill="#4a463f">× inverse_scale_factor(0.8)</text>
  <text x="572" y="322" text-anchor="middle" font-size="10.5" fill="#4a463f">= 320×140 逻辑像素</text>

  <text x="440" y="450" text-anchor="middle" font-size="11.5" fill="#7a7468">要读“本帧最新”的账，就把系统排进 PostUpdate、.after(UiSystems::Layout)；在 Update 里读，永远晚一拍</text>
</svg>
"""

SVG_04_BOX_MODEL = """<svg viewBox="0 0 780 500" xmlns="http://www.w3.org/2000/svg" font-family="system-ui, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <rect x="0" y="0" width="780" height="500" rx="10" fill="#f7f5f0"/>
  <text x="390" y="32" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">盒模型——从里到外四层皮</text>

  <!-- margin：虚线外圈（不属于节点本体） -->
  <rect x="90" y="100" width="420" height="300" rx="4" fill="none" stroke="#9a9280" stroke-width="1.8" stroke-dasharray="7 5"/>
  <!-- border：有厚度的一圈 -->
  <rect x="130" y="140" width="340" height="220" fill="#c99f47" stroke="#8a6a1e" stroke-width="1.2"/>
  <!-- padding -->
  <rect x="146" y="156" width="308" height="188" fill="#efe6d2"/>
  <!-- content -->
  <rect x="190" y="200" width="220" height="100" fill="#8fbdb9" stroke="#5f938f" stroke-width="1.2"/>
  <text x="300" y="243" text-anchor="middle" font-size="13.5" fill="#274b48" font-weight="bold">content 内容区</text>
  <text x="300" y="264" text-anchor="middle" font-size="10.5" fill="#3c625f">子节点和文字住这儿</text>

  <!-- 四层的名字（画在各自的皮上） -->
  <text x="108" y="126" font-size="11.5" fill="#7a7468" font-weight="bold">margin</text>
  <text x="300" y="151.5" text-anchor="middle" font-size="11" fill="#5c4712" font-weight="bold">border</text>
  <text x="300" y="184" text-anchor="middle" font-size="11.5" fill="#8a7a52" font-weight="bold">padding</text>

  <!-- 右侧字段注记 -->
  <circle cx="496" cy="120" r="3" fill="#7a7468"/>
  <path d="M496,120 L556,120" stroke="#9a9280" stroke-width="1.3" fill="none"/>
  <text x="562" y="117" font-size="12.5" fill="#4a463f" font-weight="bold">margin（外距）</text>
  <text x="562" y="134" font-size="10.5" fill="#7a7468">Node.margin——让给邻居的</text>
  <text x="562" y="149" font-size="10.5" fill="#7a7468">空地，不算自家尺寸</text>

  <circle cx="462" cy="205" r="3" fill="#8a6a1e"/>
  <path d="M462,205 L556,205" stroke="#9a9280" stroke-width="1.3" fill="none"/>
  <text x="562" y="202" font-size="12.5" fill="#4a463f" font-weight="bold">border（边框）</text>
  <text x="562" y="219" font-size="10.5" fill="#7a7468">厚度归 Node.border，</text>
  <text x="562" y="234" font-size="10.5" fill="#7a7468">颜色归 BorderColor 组件</text>

  <circle cx="430" cy="290" r="3" fill="#8a7a52"/>
  <path d="M430,290 L556,290" stroke="#9a9280" stroke-width="1.3" fill="none"/>
  <text x="562" y="287" font-size="12.5" fill="#4a463f" font-weight="bold">padding（内衬）</text>
  <text x="562" y="304" font-size="10.5" fill="#7a7468">Node.padding——内容与</text>
  <text x="562" y="319" font-size="10.5" fill="#7a7468">边框之间的衬垫，算自家地皮</text>

  <!-- 底注 -->
  <text x="390" y="446" text-anchor="middle" font-size="11.5" fill="#7a7468">margin 画成虚线：它只是四邻之间让出的空地，不属于节点本体</text>
  <text x="390" y="470" text-anchor="middle" font-size="11.5" fill="#7a7468">width/height 量到哪层皮，归 box_sizing 说了算——默认 BorderBox，量到边框外沿</text>
</svg>
"""

SVG_06_TWO_AXES = """<svg viewBox="0 0 880 440" xmlns="http://www.w3.org/2000/svg" font-family="system-ui, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr28m" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto">
      <path d="M0,0 L9,5 L0,10 z" fill="#b8862e"/>
    </marker>
    <marker id="arr28c" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto">
      <path d="M0,0 L9,5 L0,10 z" fill="#33586e"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="880" height="440" rx="10" fill="#f7f5f0"/>
  <text x="440" y="32" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">Flexbox 的两根轴——flex_direction 定主轴，交叉轴与它垂直</text>

  <!-- ============ 左：Row ============ -->
  <text x="210" y="76" text-anchor="middle" font-size="13.5" fill="#4a463f" font-family="Consolas, monospace" font-weight="bold">flex_direction: Row（默认）</text>
  <rect x="50" y="92" width="320" height="230" rx="6" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.8"/>
  <rect x="66" y="108" width="64" height="60" rx="4" fill="#b3402e"/>
  <rect x="144" y="108" width="64" height="95" rx="4" fill="#2e8ba8"/>
  <rect x="222" y="108" width="64" height="130" rx="4" fill="#1d6b40"/>
  <text x="98" y="144" text-anchor="middle" font-size="14" fill="#ffffff" font-weight="bold">1</text>
  <text x="176" y="144" text-anchor="middle" font-size="14" fill="#ffffff" font-weight="bold">2</text>
  <text x="254" y="144" text-anchor="middle" font-size="14" fill="#ffffff" font-weight="bold">3</text>

  <!-- 主轴：水平向右 -->
  <path d="M58,346 L356,346" stroke="#b8862e" stroke-width="3" fill="none" marker-end="url(#arr28m)"/>
  <text x="60" y="372" font-size="12.5" fill="#8a6a1e" font-weight="bold">主轴 main axis</text>
  <text x="60" y="390" font-size="10.5" fill="#8a6a1e">justify_content 管这根轴上怎么分空地</text>

  <!-- 交叉轴：垂直向下 -->
  <path d="M394,100 L394,300" stroke="#33586e" stroke-width="3" fill="none" marker-end="url(#arr28c)"/>
  <text x="406" y="150" font-size="12.5" fill="#33586e" font-weight="bold" transform="rotate(90 406 150)">交叉轴 cross axis</text>

  <!-- ============ 右：Column ============ -->
  <text x="660" y="76" text-anchor="middle" font-size="13.5" fill="#4a463f" font-family="Consolas, monospace" font-weight="bold">flex_direction: Column</text>
  <rect x="500" y="92" width="320" height="230" rx="6" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.8"/>
  <rect x="516" y="108" width="64" height="52" rx="4" fill="#b3402e"/>
  <rect x="516" y="172" width="110" height="52" rx="4" fill="#2e8ba8"/>
  <rect x="516" y="236" width="156" height="52" rx="4" fill="#1d6b40"/>
  <text x="548" y="140" text-anchor="middle" font-size="14" fill="#ffffff" font-weight="bold">1</text>
  <text x="548" y="204" text-anchor="middle" font-size="14" fill="#ffffff" font-weight="bold">2</text>
  <text x="548" y="268" text-anchor="middle" font-size="14" fill="#ffffff" font-weight="bold">3</text>

  <!-- 主轴：垂直向下 -->
  <path d="M844,100 L844,300" stroke="#b8862e" stroke-width="3" fill="none" marker-end="url(#arr28m)"/>
  <text x="856" y="150" font-size="12.5" fill="#8a6a1e" font-weight="bold" transform="rotate(90 856 150)">主轴 main axis</text>

  <!-- 交叉轴：水平向右 -->
  <path d="M508,346 L806,346" stroke="#33586e" stroke-width="3" fill="none" marker-end="url(#arr28c)"/>
  <text x="510" y="372" font-size="12.5" fill="#33586e" font-weight="bold">交叉轴 cross axis</text>
  <text x="510" y="390" font-size="10.5" fill="#33586e">align_items 管这根轴上怎么站</text>

  <text x="440" y="424" text-anchor="middle" font-size="11.5" fill="#7a7468">记法：justify＝主轴、align＝交叉轴；Reverse 只是把主轴倒过来放</text>
</svg>
"""

SVG_12_GRID_LINES = """<svg viewBox="0 0 900 380" xmlns="http://www.w3.org/2000/svg" font-family="system-ui, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <rect x="0" y="0" width="900" height="380" rx="10" fill="#f7f5f0"/>
  <text x="450" y="32" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">8 列地有 9 条格线——正数从头数，负数从尾数</text>

  <!-- 8 个格子 -->
  <g fill="#ffffff" stroke="#c9c2b2" stroke-width="1.2">
    <rect x="90" y="150" width="90" height="90"/><rect x="180" y="150" width="90" height="90"/>
    <rect x="270" y="150" width="90" height="90"/><rect x="360" y="150" width="90" height="90"/>
    <rect x="450" y="150" width="90" height="90"/><rect x="540" y="150" width="90" height="90"/>
    <rect x="630" y="150" width="90" height="90"/><rect x="720" y="150" width="90" height="90"/>
  </g>

  <!-- start_span(4, 2)：格线 4 → 格线 6 的色块 -->
  <rect x="366" y="156" width="168" height="78" rx="3" fill="#e3c88a" stroke="#b8862e" stroke-width="2"/>
  <text x="450" y="190" text-anchor="middle" font-size="14" fill="#5c4712" font-family="Consolas, monospace" font-weight="bold">start_span(4, 2)</text>
  <text x="450" y="212" text-anchor="middle" font-size="10.5" fill="#5c4712">从格线 4 起步，跨 2 格</text>

  <!-- 9 条竖格线（4 与 6 描金加粗） -->
  <g stroke="#4a463f" stroke-width="2">
    <line x1="90" y1="128" x2="90" y2="262"/><line x1="180" y1="128" x2="180" y2="262"/>
    <line x1="270" y1="128" x2="270" y2="262"/>
    <line x1="450" y1="128" x2="450" y2="262"/>
    <line x1="630" y1="128" x2="630" y2="262"/>
    <line x1="720" y1="128" x2="720" y2="262"/><line x1="810" y1="128" x2="810" y2="262"/>
  </g>
  <line x1="360" y1="122" x2="360" y2="268" stroke="#b8862e" stroke-width="3.5"/>
  <line x1="540" y1="122" x2="540" y2="268" stroke="#b8862e" stroke-width="3.5"/>

  <!-- 正数编号（上） -->
  <g font-size="14" fill="#4a463f" font-weight="bold" text-anchor="middle">
    <text x="90" y="114">1</text><text x="180" y="114">2</text><text x="270" y="114">3</text>
    <text x="360" y="110" fill="#8a6a1e" font-size="15">4</text>
    <text x="450" y="114">5</text><text x="540" y="114" fill="#8a6a1e">6</text><text x="630" y="114">7</text>
    <text x="720" y="114">8</text><text x="810" y="114">9</text>
  </g>

  <!-- 负数编号（下） -->
  <g font-size="13" fill="#7a7468" text-anchor="middle">
    <text x="90" y="288">−9</text><text x="180" y="288">−8</text><text x="270" y="288">−7</text>
    <text x="360" y="288">−6</text><text x="450" y="288">−5</text><text x="540" y="288">−4</text>
    <text x="630" y="288">−3</text><text x="720" y="288">−2</text><text x="810" y="288">−1</text>
  </g>

  <text x="450" y="330" text-anchor="middle" font-size="11.5" fill="#8a6a1e">色块＝座位表里的包厢：从第 4 条格线起步、跨 2 个格子，到格线 6 为止</text>
  <text x="450" y="358" text-anchor="middle" font-size="11.5" fill="#7a7468">n 列地有 n+1 条格线；−1 永远是最后一条——start_end(1, −1) 不管地有几列都横贯到底</text>
</svg>
"""


def fig_02_layout_timing_svg() -> None:
    """Figure 28-2：一帧时间线上的三把量尺（手绘 SVG）。"""
    save_svg(SVG_02_LAYOUT_TIMING, "fig-28-02-layout-timing.svg")


def fig_04_box_model_svg() -> None:
    """Figure 28-4：盒模型同心四层（手绘 SVG）。"""
    save_svg(SVG_04_BOX_MODEL, "fig-28-04-box-model.svg")


def fig_06_two_axes_svg() -> None:
    """Figure 28-6：Row 与 Column 的主轴/交叉轴（手绘 SVG）。"""
    save_svg(SVG_06_TWO_AXES, "fig-28-06-two-axes.svg")


def fig_12_grid_lines_svg() -> None:
    """Figure 28-12：格线编号与 start_span(4,2)（手绘 SVG）。"""
    save_svg(SVG_12_GRID_LINES, "fig-28-12-grid-lines.svg")


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_01_first_node,
    fig_02_layout_timing_svg,
    fig_03_units_parade,
    fig_04_box_model_svg,
    fig_05_box_xray,
    fig_06_two_axes_svg,
    fig_07_alignment_grid,
    fig_08_align_self,
    fig_09_shrink_vs_wrap,
    fig_10_hidden_vs_none,
    fig_11_zindex,
    fig_12_grid_lines_svg,
    fig_13_seating_chart,
    fig_14_forgot_display_grid,
    fig_15_image_modes,
    fig_16_text_overflow,
    fig_17_ui_target_camera,
    fig_18_19_20_front_hall,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    picks = sys.argv[1:]
    todo = [f for f in ALL
            if not picks or any(p in f.__name__ for p in picks)]
    if not todo:
        print(f"没有图名匹配 {picks}；可选：")
        for f in ALL:
            print(f"  {f.__name__}")
        return
    if any("svg" not in f.__name__ for f in todo):
        print("构建本章二进制……")
        cargo("build", "-p", "ch28-ui-layout", "--bins", "--examples")
    for fig in todo:
        fig()
        time.sleep(0.5)


if __name__ == "__main__":
    main()

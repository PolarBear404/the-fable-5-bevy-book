# -*- coding: utf-8 -*-
"""一键重建第 27 章全部 17 张插图（14 张运行截图 + 1 张 WebP 动图 + 2 张手绘 SVG）。

    py -3.11 scripts/make_ch27_figures.py [编号筛选...]
    例：py -3.11 scripts/make_ch27_figures.py 04 13   # 只重建 fig-27-04 与 fig-27-13

运行图全部由本章 example 实拍。注入手法是本章实测结论（与 ch25/26 的 SendInput 不同，
见 workorders/ch27.md「实测台账」与 scratchpad/run_ch27_tests.py）：
- 键盘：PostMessage WM_KEYDOWN/WM_KEYUP（本机 SendInput 会被前台完整性级别挡下）；
- 鼠标移动：ClientToScreen + SetCursorPos 真移光标（合成 WM_MOUSEMOVE 会被系统按真实
  光标位置补发的 WM_MOUSELEAVE 吃掉，悬停撑不住）；
- 鼠标按键：PostMessage WM_LBUTTONDOWN/WM_LBUTTONUP（lparam 带客户区物理像素坐标）；
- 拖拽：先把光标移到起点悬停 0.6s 预热（冷启动首拖会被吞），再按下、逐步移动、抬起。

坐标约定：键鼠打靶按设计尺寸给（listing 窗口 1600×900、main 1600×1000 物理像素，
即 125% 缩放下的客户区），运行时按实际客户区等比换算——显示器缩放变了也不用改脚本。
截图先归一到逻辑像素（1280×720 / 1280×800）再裁切标注。
"""

import ctypes
import subprocess
import sys
import time
from ctypes import wintypes
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

sys.stdout.reconfigure(encoding="utf-8")
sys.stderr.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch27"

sys.path.insert(0, str(ROOT / "scripts"))
from capture import find_main_window, grab_window, _set_dpi_aware  # noqa: E402

user32 = ctypes.windll.user32

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36

# ---------------------------------------------------------------- PostMessage 注入

WM_CLOSE = 0x0010
WM_KEYDOWN, WM_KEYUP = 0x0100, 0x0101
WM_LBUTTONDOWN, WM_LBUTTONUP = 0x0201, 0x0202
MK_LBUTTON = 0x0001

# 名字 -> (virtual-key, scancode, extended)
KEYS = {
    "1": (0x31, 0x02, False), "2": (0x32, 0x03, False), "3": (0x33, 0x04, False),
    "A": (0x41, 0x1E, False), "B": (0x42, 0x30, False), "C": (0x43, 0x2E, False),
    "J": (0x4A, 0x24, False), "L": (0x4C, 0x26, False), "P": (0x50, 0x19, False),
    "U": (0x55, 0x16, False),
    "SPACE": (0x20, 0x39, False),
    "F3": (0x72, 0x3D, False), "F4": (0x73, 0x3E, False),
    "LEFT": (0x25, 0x4B, True), "UP": (0x26, 0x48, True),
    "RIGHT": (0x27, 0x4D, True), "DOWN": (0x28, 0x50, True),
}


def _post_key(hwnd: int, name: str, down: bool) -> None:
    vk, scan, ext = KEYS[name]
    lparam = 1 | (scan << 16)
    if ext:
        lparam |= 1 << 24
    if not down:
        lparam |= (1 << 30) | (1 << 31)
    user32.PostMessageW(hwnd, WM_KEYDOWN if down else WM_KEYUP, vk, lparam)


def _mouse_lparam(x: float, y: float) -> int:
    return (int(y) << 16) | (int(x) & 0xFFFF)


class Run:
    """一个受测示例：stdout/stderr 进 DEVNULL，键鼠注入与截图在手。

    design：该示例窗口的设计尺寸（物理像素，125% 缩放基准）。
    所有打靶坐标按 design 给，内部按实际客户区等比换算。
    """

    def __init__(self, name: str, exe: Path | None = None, design=(1600, 900)):
        _set_dpi_aware()
        self.name = name
        self.design = design
        exe = exe or (EXAMPLES / f"{name}.exe")
        self.proc = subprocess.Popen(
            [str(exe)], cwd=str(CODE),
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        )
        self.hwnd = find_main_window(self.proc.pid)
        self.t0 = time.perf_counter()
        user32.SetForegroundWindow(self.hwnd)

    # ---- 几何换算 ----

    def client_size(self) -> tuple[int, int]:
        rect = wintypes.RECT()
        user32.GetClientRect(self.hwnd, ctypes.byref(rect))
        return rect.right, rect.bottom

    def _scale(self, x: float, y: float) -> tuple[float, float]:
        w, h = self.client_size()
        return x / self.design[0] * w, y / self.design[1] * h

    # ---- 时序与截图 ----

    def wait_until(self, t_since_window: float) -> None:
        remain = self.t0 + t_since_window - time.perf_counter()
        if remain > 0:
            time.sleep(remain)

    def grab(self) -> Image.Image:
        return grab_window(self.hwnd)

    # ---- 键盘 ----

    def key_down(self, key: str) -> None:
        _post_key(self.hwnd, key, True)

    def key_up(self, key: str) -> None:
        _post_key(self.hwnd, key, False)

    def tap(self, key: str, hold: float = 0.06, settle: float = 0.35) -> None:
        self.key_down(key)
        time.sleep(hold)
        self.key_up(key)
        time.sleep(settle)

    def hold(self, key: str, dur: float, settle: float = 0.2) -> None:
        """按住：持续重发 keydown（自动重复），最后抬键。"""
        steps = max(1, int(dur / 0.05))
        for _ in range(steps):
            self.key_down(key)
            time.sleep(0.05)
        self.key_up(key)
        time.sleep(settle)

    # ---- 鼠标（坐标按 design 给） ----

    def move_cursor(self, dx: float, dy: float) -> None:
        """真移系统光标到客户区（design 坐标）。"""
        x, y = self._scale(dx, dy)
        pt = wintypes.POINT(int(x), int(y))
        user32.ClientToScreen(self.hwnd, ctypes.byref(pt))
        user32.SetCursorPos(pt.x, pt.y)
        time.sleep(0.03)

    def drag(self, x0, y0, x1, y1, steps: int = 30, pause: float = 0.03) -> None:
        """拖拽：悬停 0.6s 预热 → 按下 → 逐步移动 → 抬起（design 坐标）。"""
        for _ in range(6):
            self.move_cursor(x0, y0)
        time.sleep(0.6)
        sx0, sy0 = self._scale(x0, y0)
        user32.PostMessageW(self.hwnd, WM_LBUTTONDOWN, MK_LBUTTON, _mouse_lparam(sx0, sy0))
        time.sleep(0.2)
        for i in range(1, steps + 1):
            t = i / steps
            self.move_cursor(x0 + (x1 - x0) * t, y0 + (y1 - y0) * t)
            time.sleep(pause)
        time.sleep(0.1)
        sx1, sy1 = self._scale(x1, y1)
        user32.PostMessageW(self.hwnd, WM_LBUTTONUP, 0, _mouse_lparam(sx1, sy1))
        time.sleep(0.4)

    # ---- 收尾 ----

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


def label_bar(width: int, texts: list[str]) -> Image.Image:
    bar = Image.new("RGB", (width, LABEL_H), LABEL_BG)
    draw = ImageDraw.Draw(bar)
    cell = width / len(texts)
    for i, text in enumerate(texts):
        w = draw.textlength(text, font=FONT)
        draw.text((cell * i + (cell - w) / 2, 6), text, font=FONT, fill=LABEL_FG)
    return bar


def hstack(images: list[Image.Image], labels: list[str] | None = None) -> Image.Image:
    """多联横排，顶部一条整跨标注（每格一段文字）。"""
    h = max(im.height for im in images)
    w = sum(im.width for im in images) + GAP * (len(images) - 1)
    top = LABEL_H if labels else 0
    canvas = Image.new("RGB", (w, h + top), GAP_COLOR)
    if labels:
        canvas.paste(label_bar(w, labels), (0, 0))
    x = 0
    for im in images:
        canvas.paste(im, (x, top))
        x += im.width + GAP
    return canvas


def hstack_below(images: list[Image.Image], labels: list[str]) -> Image.Image:
    """多联横排，每联下方各自一条标注条。"""
    h = max(im.height for im in images)
    w = sum(im.width for im in images) + GAP * (len(images) - 1)
    canvas = Image.new("RGB", (w, h + LABEL_H), GAP_COLOR)
    x = 0
    for im, text in zip(images, labels):
        canvas.paste(im, (x, 0))
        canvas.paste(label_bar(im.width, [text]), (x, h))
        x += im.width + GAP
    return canvas


def logical(img: Image.Image, size=(1280, 720)) -> Image.Image:
    """物理像素 → 逻辑像素（显示器缩放会变，一律归一后再裁切/标注）。"""
    if img.size == size:
        return img
    return img.resize(size, Image.LANCZOS)


def shrink(img: Image.Image, k: float = 0.5) -> Image.Image:
    return img.resize((int(img.width * k), int(img.height * k)), Image.LANCZOS)


def crop_zoom(img: Image.Image, box: tuple, out_w: int) -> Image.Image:
    """裁切放大：等比、最近邻（保持粉线的像素观感）。"""
    c = img.crop(box)
    scale = out_w / c.width
    return c.resize((out_w, int(c.height * scale)), Image.NEAREST)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def save_svg(text: str, filename: str) -> None:
    path = OUT / filename
    path.write_text(text, encoding="utf-8")
    print(f"{filename}：{path.stat().st_size // 1024} KB")


# ---------------------------------------------------------------- Figure 27-1：第一道粉线

def fig_01_first_chalk() -> None:
    """Figure 27-1：粉线在场 vs 按住空格消失（双联，裁滑轨区）。"""
    with Run("listing-27-01") as r:
        r.wait_until(2.0)
        on = logical(r.grab())
        r.key_down("SPACE")           # 按住空格：画线系统歇手
        time.sleep(0.3)
        r.key_down("SPACE")           # 中途补一枪自动重复，保住按住状态
        time.sleep(0.3)
        off = logical(r.grab())
        r.key_up("SPACE")
    box = (560, 200, 1200, 560)       # 滑轨区：两个时刻的箱子都在框内
    save_png(
        hstack_below(
            [on.crop(box), off.crop(box)],
            ["每帧都画", "按住空格：这一帧没画，线就没了"],
        ),
        "fig-27-01-first-chalk.png",
    )


# ---------------------------------------------------------------- Figure 27-3/4：粉线字典

def fig_03_vocabulary() -> None:
    """Figure 27-3：2D 粉线词汇一屏画全（全景单图）。"""
    with Run("listing-27-02") as r:
        r.wait_until(2.0)
        save_png(logical(r.grab()), "fig-27-03-vocabulary.png")


def fig_04_resolution() -> None:
    """Figure 27-4：站位圈 resolution 4（菱形）vs 64（光滑），裁圈区。"""
    with Run("listing-27-02") as r:
        r.wait_until(2.0)
        for _ in range(4):            # 32→16→8→6→4
            r.tap("DOWN")
        res4 = logical(r.grab())
        for _ in range(5):            # 4→6→8→16→32→64
            r.tap("UP")
        res64 = logical(r.grab())
    box = (280, 180, 520, 420)        # 站位圈居中，避开左侧走位弧
    save_png(
        hstack_below(
            [crop_zoom(res4, box, 440), crop_zoom(res64, box, 440)],
            [".resolution(4)——4 段", ".resolution(64)——64 段"],
        ),
        "fig-27-04-resolution.png",
    )


# ---------------------------------------------------------------- Figure 27-5/6：规格与分组

def fig_05_width_and_joints() -> None:
    """Figure 27-5：加粗走位线的 Miter/Round(8)/Bevel 拐角三联（裁含两拐点的中段）。"""
    with Run("listing-27-03") as r:
        r.wait_until(2.0)
        r.hold("RIGHT", 1.2)          # 线宽拨到 ~20px，拐角才看得出讲究
        time.sleep(0.3)
        miter = logical(r.grab())
        r.tap("J")                    # Miter -> Round(8)
        rnd = logical(r.grab())
        r.tap("J")                    # -> Bevel
        bevel = logical(r.grab())
    box = (330, 100, 770, 540)        # 左峰 + 谷底两个拐点，顶部带进安全框的虚线沿
    save_png(
        hstack_below(
            [crop_zoom(miter, box, 440), crop_zoom(rnd, box, 440),
             crop_zoom(bevel, box, 440)],
            ["Miter", "Round(8)", "Bevel"],
        ),
        "fig-27-05-width-and-joints.png",
    )


def fig_06_line_styles() -> None:
    """Figure 27-6：Dotted 全线（方点串）vs Dashed 安全框特写。"""
    with Run("listing-27-03") as r:
        r.wait_until(2.0)
        r.hold("RIGHT", 1.2)          # 复现 fig-5 的加粗状态
        time.sleep(0.3)
        r.tap("J")
        r.tap("J")
        r.tap("U")                    # 走位线 Solid -> Dotted
        dotted = logical(r.grab())
        r.tap("1")                    # 走位线全下，只剩 Dashed 安全框
        dashed = logical(r.grab())
    dashed_crop = crop_zoom(dashed, (580, 390, 1220, 710), 720)  # 右下角：横竖两段虚线
    save_png(
        hstack_below(
            [shrink(dotted), dashed_crop],
            ["Dotted", "Dashed { gap_scale: 2.0, line_scale: 4.0 }"],
        ),
        "fig-27-06-line-styles.png",
    )


# ---------------------------------------------------------------- Figure 27-7：会写字的粉线

def fig_07_text_gizmos() -> None:
    """Figure 27-7：四块描线字牌 + 中文空位（t≈2.0，x≈+168 构图佳）。"""
    with Run("listing-27-05") as r:
        r.wait_until(2.0)
        save_png(logical(r.grab()), "fig-27-07-text-gizmos.png")


# ---------------------------------------------------------------- Figure 27-8：慢时钟动图

def fig_08_two_clocks() -> None:
    """Figure 27-8（动图）：绿框跟手 vs 橙框步进驻留，录 2 秒、20fps。

    箱子在 ±320 之间折返（240 px/s，周期 5.33s）；取 t≈5.45~7.45 这一段，
    箱子全程向右滑，不掺折返。
    """
    fps, dur = 20, 2.0
    with Run("listing-27-06") as r:
        raw = []
        for i in range(int(dur * fps)):
            r.wait_until(5.45 + i / fps)
            raw.append(r.grab())

    legend = [("绿 = Update", (70, 225, 70)), ("橙 = FixedUpdate（4 Hz）", (235, 115, 45))]

    def stamp(img: Image.Image) -> Image.Image:
        d = ImageDraw.Draw(img)
        x = 14
        pad_y, chip = 10, 16
        total = sum(chip + 8 + d.textlength(t, font=FONT) + 24 for t, _ in legend)
        d.rectangle((6, 4, 6 + total + 4, 40), fill=(16, 17, 20))
        for text, color in legend:
            d.rectangle((x, pad_y, x + chip, pad_y + chip), fill=color)
            d.text((x + chip + 8, 6), text, font=FONT, fill=LABEL_FG)
            x += chip + 8 + d.textlength(text, font=FONT) + 24
        return img

    band = [stamp(logical(f).crop((0, 250, 1280, 470))) for f in raw]
    path = OUT / "fig-27-08-two-clocks.webp"
    band[0].save(path, save_all=True, append_images=band[1:],
                 duration=int(1000 / fps), loop=0, method=4, quality=85)
    print(f"fig-27-08-two-clocks.webp：{len(band)} 帧 {band[0].size[0]}x{band[0].size[1]}，"
          f"{path.stat().st_size // 1024} KB")


# ---------------------------------------------------------------- Figure 27-9：保留模式

def fig_09_retained_grid() -> None:
    """Figure 27-9：保留线网+即时光标圈 vs 方向键整体右移（双联）。"""
    with Run("listing-27-07") as r:
        r.wait_until(2.0)
        r.move_cursor(1120, 360)      # 0.7w, 0.4h：即时光标圈落点
        time.sleep(0.3)
        a = logical(r.grab())
        r.hold("RIGHT", 1.0)          # 平移 Gizmo 实体的 Transform
        b = logical(r.grab())
    save_png(
        hstack_below(
            [shrink(a), shrink(b)],
            ["保留线网 + 即时光标圈", "按住 →：实体右移，整张网随行"],
        ),
        "fig-27-09-retained-grid.png",
    )


# ---------------------------------------------------------------- Figure 27-10/11：现成的描边

def fig_10_aabb_draw_all() -> None:
    """Figure 27-10：B 主箱金框 + A 全场描框（斜鼓正盒、地面扁盒）。"""
    with Run("listing-27-08") as r:
        r.wait_until(2.0)
        r.tap("B")                    # 主箱描金框
        r.tap("A")                    # draw_all：一件不落
        save_png(logical(r.grab()), "fig-27-10-aabb-draw-all.png")


def fig_11_light_gizmos() -> None:
    """Figure 27-11：四种灯形 ByLightType 配色，无 AABB 干扰。"""
    with Run("listing-27-08") as r:
        r.wait_until(2.0)
        r.tap("L")                    # 挂灯形牌（默认 MatchLightColor）
        r.tap("C")                    # -> ByLightType：点红/聚绿/平行蓝/矩形褐
        save_png(logical(r.grab()), "fig-27-11-light-gizmos.png")


# ---------------------------------------------------------------- Figure 27-13：开发工具箱

def fig_13_overlays() -> None:
    """Figure 27-13：水牌+帧时图+两扇小窗（拖开），红框圈出 Missing 行。

    两扇小窗都出生在 (32,32) 叠成一摞：先拖走上层的 Stage ledger 到台中，
    再把露出的 Fps 窗往下挪一点（留在左上角、避开帧时图）。
    拖完等几秒，让拖拽卡顿的红色尖刺滚出帧时图。
    """
    with Run("listing-27-12") as r:
        r.wait_until(2.5)
        r.drag(100, 62, 640, 420)     # Stage ledger -> 台中
        time.sleep(0.4)
        r.drag(100, 62, 97, 184)      # Fps 窗 -> 左上角、帧时图正下方
        r.move_cursor(1400, 700)      # 光标挪去空场，别悬停在窗上
        time.sleep(5.0)               # 尖刺滚出帧时图
        shot = logical(r.grab())

    # 红色圆角框圈住 stage/confetti Missing 行（补画标注，不改画面内容）。
    # 小窗是逻辑像素 UI：ledger 落点由拖拽决定，行位随 DiagnosticsOverlayStyle 固定。
    d = ImageDraw.Draw(shot)
    red = (235, 70, 55)
    d.rounded_rectangle((460, 374, 613, 398), radius=7, outline=red, width=3)
    note = "这条名目从未 register 过"
    d.line((613, 386, 648, 386), fill=red, width=3)
    d.text((656, 373), note, font=FONT, fill=(255, 120, 100))
    save_png(shot, "fig-27-13-overlays.png")


# ---------------------------------------------------------------- Figure 27-14/15：迷你检修间

def fig_14_mini_workshop() -> None:
    """Figure 27-14：无限网格 + 三箱 + 0 号箱 Translate 把手（全景）。"""
    with Run("listing-27-13") as r:
        r.move_cursor(1520, 72)       # 光标停到空处，避免误悬停提亮把手
        r.wait_until(2.5)
        save_png(logical(r.grab()), "fig-27-14-mini-workshop.png")


def fig_15_gizmo_modes() -> None:
    """Figure 27-15：同一箱上 Translate/Rotate/Scale 三种把手（裁 0 号箱区）。"""
    with Run("listing-27-13") as r:
        r.move_cursor(1520, 72)
        r.wait_until(2.5)
        translate = logical(r.grab())     # 开场即 Translate
        r.tap("2")
        rotate = logical(r.grab())
        r.tap("3")
        scale = logical(r.grab())
    box = (264, 264, 560, 560)            # 0 号箱 + 把手/三环的活动范围
    save_png(
        hstack_below(
            [crop_zoom(translate, box, 444), crop_zoom(rotate, box, 444),
             crop_zoom(scale, box, 444)],
            ["1：Translate", "2：Rotate", "3：Scale"],
        ),
        "fig-27-15-gizmo-modes.png",
    )


# ---------------------------------------------------------------- Figure 27-16/17：《检场》总成

MAIN_EXE = CODE / "target" / "debug" / "ch27-dev-tools.exe"


def fig_16_stagehand_on_off() -> None:
    """Figure 27-16：调试层全开 vs F3 粉线全下（球待发，游戏画面纹丝不动）。"""
    with Run("main", MAIN_EXE, design=(1600, 1000)) as r:
        r.wait_until(2.5)
        on = logical(r.grab(), (1280, 800))
        r.tap("F3", settle=0.4)       # 两组粉线 enabled 一起拨下
        off = logical(r.grab(), (1280, 800))
    save_png(
        hstack_below(
            [shrink(on, 0.55), shrink(off, 0.55)],
            ["调试层全开", "按过 F3：粉线全下"],
        ),
        "fig-27-16-stagehand-on-off.png",
    )


def _find_ball(img: Image.Image) -> tuple[int, int] | None:
    """在归一化 1280×800 图里找红球（限定瓦阵下方的开阔区，避开沟线/帧时图/箭头）。

    区域上沿收到 y=465：再高的话球头顶的 v 380 描线字就会压进第 7 排瓦。
    """
    region = (240, 465, 1050, 640)
    crop = img.crop(region)
    px = crop.load()
    hits = []
    for y in range(0, crop.height, 2):
        for x in range(0, crop.width, 2):
            r, g, b = px[x, y][:3]
            if r > 185 and g < 140 and b < 140 and r - g > 70 and r - b > 70:
                hits.append((x, y))
    if len(hits) < 15:
        return None
    cx = sum(p[0] for p in hits) // len(hits) + region[0]
    cy = sum(p[1] for p in hits) // len(hits) + region[1]
    return cx, cy


# 瓦阵 8 列 × 7 排的格心（归一化 1280×800 逻辑像素）。
# 素瓦填色 sum(RGB)≈486、釉瓦≈329、缺瓦露出的台底≈93——阈值 200 二分。
_BRICK_COLS = [290, 389, 488, 588, 687, 786, 886, 985]
_BRICK_ROWS = [188, 222, 256, 290, 324, 358, 392]


def _missing_bricks(img: Image.Image) -> list[tuple[int, int]]:
    """逐格采样瓦心颜色，返回缺瓦的 (排, 列) 清单；空列表 = 56 片全在。"""
    px = img.load()
    return [
        (r + 1, c + 1)
        for r, y in enumerate(_BRICK_ROWS)
        for c, x in enumerate(_BRICK_COLS)
        if sum(px[x, y][:3]) <= 200
    ]


def fig_17_frozen_inspect() -> None:
    """Figure 27-17：SPACE 发球 → 球触瓦之前 P 定格，球钉半空带圈/箭头/v 380 描线字。

    球升到第一片瓦约在发球后 0.74s——定格必须抢在这之前：账本小窗的重建节拍
    走虚拟时钟，P 之后账面冻在 56/60/478，若已有瓦碎，账面就比台面旧一拍。
    逐个等待时长重试，验收双条件：球在瓦阵下方开阔区（v 380 字牌不压瓦）
    且 56 片瓦一片不缺。
    """
    shot = None
    for wait in (0.48, 0.42, 0.54, 0.36, 0.58, 0.30):
        with Run("main", MAIN_EXE, design=(1600, 1000)) as r:
            r.wait_until(2.5)
            r.tap("SPACE", settle=0.0)    # 发球
            time.sleep(wait)
            r.tap("P", settle=0.0)        # 定格：虚拟时钟一停，FixedUpdate 停拍
            time.sleep(0.5)
            img = logical(r.grab(), (1280, 800))
        ball = _find_ball(img)
        missing = _missing_bricks(img)
        if ball and not missing:
            print(f"  发球后 {wait}s 定格：球在 {ball}、56 瓦全在，采用")
            shot = img
            break
        print(f"  发球后 {wait}s 定格：球位 {ball or '不佳'}、缺瓦 {missing or '无'}，重试")
    if shot is None:
        raise RuntimeError("六次定格都不合格（球位/缺瓦）——重跑一次，或调整等待时长")
    # 裁掉沟线以下的空带：上中横带（水牌/账本/瓦阵/球/凳都在）
    save_png(shot.crop((0, 0, 1280, 720)), "fig-27-17-frozen-inspect.png")


# ---------------------------------------------------------------- 手绘 SVG
# 内容即代码：落盘即重建（插图规范）。#f7f5f0 圆角卡片底，明暗主题均可读。

SVG_02_IMMEDIATE = """<svg viewBox="0 0 860 540" xmlns="http://www.w3.org/2000/svg" font-family="system-ui, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr27a" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#7a7468"/>
    </marker>
    <marker id="arr27b" markerWidth="8" markerHeight="8" refX="6" refY="4" orient="auto">
      <path d="M0,0 L7,4 L0,8 z" fill="#9a9280"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="860" height="540" rx="10" fill="#f7f5f0"/>
  <text x="430" y="30" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">即时模式：粉线只活一帧</text>

  <!-- ============ 帧时间轴 ============ -->
  <rect x="150" y="52" width="150" height="32" rx="6" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
  <text x="225" y="73" text-anchor="middle" font-size="12" fill="#7a7468">第 N-1 帧</text>
  <rect x="330" y="52" width="150" height="32" rx="6" fill="#efe0c4" stroke="#b8862e" stroke-width="2"/>
  <text x="405" y="73" text-anchor="middle" font-size="12.5" fill="#8a6a1e" font-weight="bold">第 N 帧</text>
  <rect x="510" y="52" width="150" height="32" rx="6" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
  <text x="585" y="73" text-anchor="middle" font-size="12" fill="#7a7468">第 N+1 帧</text>
  <path d="M304,68 L326,68" stroke="#9a9280" stroke-width="1.6" fill="none" marker-end="url(#arr27b)"/>
  <path d="M484,68 L506,68" stroke="#9a9280" stroke-width="1.6" fill="none" marker-end="url(#arr27b)"/>
  <path d="M664,68 L700,68" stroke="#9a9280" stroke-width="1.6" fill="none" marker-end="url(#arr27b)"/>
  <text x="726" y="73" font-size="11" fill="#9a9280">时间</text>

  <!-- 把第 N 帧展开 -->
  <path d="M334,86 L28,128" stroke="#b8862e" stroke-width="1.4" stroke-dasharray="5 4" fill="none"/>
  <path d="M476,86 L832,128" stroke="#b8862e" stroke-width="1.4" stroke-dasharray="5 4" fill="none"/>

  <!-- ============ 一帧之内的四步 ============ -->
  <rect x="24" y="130" width="812" height="298" rx="10" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.6"/>

  <!-- 步骤一：系统们画线 -->
  <text x="126" y="158" text-anchor="middle" font-size="13" fill="#4a463f" font-weight="bold">① 系统们画线</text>
  <rect x="52" y="176" width="44" height="30" rx="5" fill="#eef1f4" stroke="#4a463f" stroke-width="1.4"/>
  <rect x="104" y="176" width="44" height="30" rx="5" fill="#eef1f4" stroke="#4a463f" stroke-width="1.4"/>
  <rect x="156" y="176" width="44" height="30" rx="5" fill="#eef1f4" stroke="#4a463f" stroke-width="1.4"/>
  <text x="74" y="196" text-anchor="middle" font-size="10" fill="#4a463f">系统</text>
  <text x="126" y="196" text-anchor="middle" font-size="10" fill="#4a463f">系统</text>
  <text x="178" y="196" text-anchor="middle" font-size="10" fill="#4a463f">系统</text>
  <path d="M74,206 C74,232 100,240 112,252" stroke="#7a7468" stroke-width="1.6" fill="none" marker-end="url(#arr27a)"/>
  <path d="M126,206 L126,250" stroke="#7a7468" stroke-width="1.6" fill="none" marker-end="url(#arr27a)"/>
  <path d="M178,206 C178,232 152,240 140,252" stroke="#7a7468" stroke-width="1.6" fill="none" marker-end="url(#arr27a)"/>
  <!-- 公共桶 -->
  <path d="M84,258 L168,258 L156,330 L96,330 Z" fill="#fbf1de" stroke="#4a463f" stroke-width="1.8"/>
  <line x1="100" y1="286" x2="126" y2="274" stroke="#b8862e" stroke-width="3" stroke-linecap="round"/>
  <line x1="118" y1="304" x2="148" y2="296" stroke="#2e8ba8" stroke-width="3" stroke-linecap="round"/>
  <line x1="104" y1="316" x2="132" y2="318" stroke="#1d6b40" stroke-width="3" stroke-linecap="round"/>
  <text x="126" y="352" text-anchor="middle" font-size="11" fill="#8a6a1e" font-family="Consolas, monospace">GizmoStorage</text>
  <text x="126" y="376" text-anchor="middle" font-size="10.5" fill="#7a7468">每一笔线段都丢进</text>
  <text x="126" y="392" text-anchor="middle" font-size="10.5" fill="#7a7468">这只公共桶</text>

  <path d="M216,282 L242,282" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr27a)"/>

  <!-- 步骤二：帧末汇总 -->
  <text x="330" y="158" text-anchor="middle" font-size="13" fill="#4a463f" font-weight="bold">② 帧末汇总</text>
  <path d="M262,258 L306,258 L300,296 L268,296 Z" fill="#fbf1de" stroke="#4a463f" stroke-width="1.6"/>
  <line x1="272" y1="272" x2="288" y2="266" stroke="#b8862e" stroke-width="2.4" stroke-linecap="round"/>
  <line x1="278" y1="284" x2="296" y2="282" stroke="#2e8ba8" stroke-width="2.4" stroke-linecap="round"/>
  <path d="M310,276 L332,276" stroke="#7a7468" stroke-width="1.8" fill="none" marker-end="url(#arr27a)"/>
  <rect x="336" y="250" width="72" height="52" rx="6" fill="#eef1f4" stroke="#33586e" stroke-width="1.8"/>
  <line x1="346" y1="266" x2="398" y2="266" stroke="#33586e" stroke-width="2"/>
  <line x1="346" y1="276" x2="398" y2="276" stroke="#33586e" stroke-width="2"/>
  <line x1="346" y1="286" x2="386" y2="286" stroke="#33586e" stroke-width="2"/>
  <text x="372" y="322" text-anchor="middle" font-size="10.5" fill="#33586e">一份顶点数据</text>
  <text x="330" y="376" text-anchor="middle" font-size="10.5" fill="#7a7468">桶里的线段合成一份，</text>
  <text x="330" y="392" text-anchor="middle" font-size="10.5" fill="#7a7468">交给渲染器</text>

  <path d="M424,282 L450,282" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr27a)"/>

  <!-- 步骤三：渲染上屏 -->
  <text x="534" y="158" text-anchor="middle" font-size="13" fill="#4a463f" font-weight="bold">③ 渲染上屏</text>
  <rect x="472" y="230" width="124" height="84" rx="8" fill="#2a2c31" stroke="#4a463f" stroke-width="2"/>
  <polyline points="486,292 516,254 544,286 572,248" fill="none" stroke="#e8c34a" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
  <rect x="522" y="314" width="24" height="10" fill="#c9c2b2"/>
  <rect x="508" y="324" width="52" height="6" rx="3" fill="#c9c2b2"/>
  <text x="534" y="376" text-anchor="middle" font-size="10.5" fill="#7a7468">和场景一起</text>
  <text x="534" y="392" text-anchor="middle" font-size="10.5" fill="#7a7468">画到屏幕上</text>

  <path d="M612,282 L638,282" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr27a)"/>

  <!-- 步骤四：清空桶 -->
  <text x="732" y="158" text-anchor="middle" font-size="13" fill="#4a463f" font-weight="bold">④ 清空桶</text>
  <g transform="rotate(64 726 292)">
    <path d="M684,264 L768,264 L756,336 L696,336 Z" fill="#ffffff" stroke="#4a463f" stroke-width="1.8"/>
  </g>
  <path d="M700,238 C716,224 744,224 760,238" stroke="#9a9280" stroke-width="1.6" fill="none" stroke-dasharray="4 3" marker-end="url(#arr27b)"/>
  <line x1="676" y1="330" x2="662" y2="342" stroke="#c9c2b2" stroke-width="2.4" stroke-linecap="round"/>
  <line x1="690" y1="342" x2="682" y2="354" stroke="#c9c2b2" stroke-width="2.4" stroke-linecap="round"/>
  <text x="732" y="376" text-anchor="middle" font-size="10.5" fill="#7a7468">桶一倒而空——</text>
  <text x="732" y="392" text-anchor="middle" font-size="10.5" fill="#7a7468">这帧的线就此谢幕</text>

  <!-- ============ 下一帧：虚线重复 ============ -->
  <rect x="24" y="444" width="812" height="42" rx="8" fill="none" stroke="#9a9280" stroke-width="1.6" stroke-dasharray="7 5"/>
  <text x="44" y="470" font-size="12" fill="#7a7468" font-weight="bold">下一帧</text>
  <rect x="110" y="452" width="76" height="26" rx="5" fill="none" stroke="#9a9280" stroke-width="1.3" stroke-dasharray="4 3"/>
  <text x="148" y="470" text-anchor="middle" font-size="11" fill="#7a7468">画</text>
  <path d="M190,465 L212,465" stroke="#9a9280" stroke-width="1.4" fill="none" marker-end="url(#arr27b)"/>
  <rect x="216" y="452" width="76" height="26" rx="5" fill="none" stroke="#9a9280" stroke-width="1.3" stroke-dasharray="4 3"/>
  <text x="254" y="470" text-anchor="middle" font-size="11" fill="#7a7468">汇总</text>
  <path d="M296,465 L318,465" stroke="#9a9280" stroke-width="1.4" fill="none" marker-end="url(#arr27b)"/>
  <rect x="322" y="452" width="76" height="26" rx="5" fill="none" stroke="#9a9280" stroke-width="1.3" stroke-dasharray="4 3"/>
  <text x="360" y="470" text-anchor="middle" font-size="11" fill="#7a7468">上屏</text>
  <path d="M402,465 L424,465" stroke="#9a9280" stroke-width="1.4" fill="none" marker-end="url(#arr27b)"/>
  <rect x="428" y="452" width="76" height="26" rx="5" fill="none" stroke="#9a9280" stroke-width="1.3" stroke-dasharray="4 3"/>
  <text x="466" y="470" text-anchor="middle" font-size="11" fill="#7a7468">清空</text>
  <text x="540" y="470" font-size="11" fill="#9a9280">……桶重新从空开始</text>

  <text x="430" y="518" text-anchor="middle" font-size="11.5" fill="#7a7468">想让线常驻，就每帧都画——或者用 27.6 的保留模式</text>
</svg>
"""

SVG_12_THREE_READINGS = """<svg viewBox="0 0 900 540" xmlns="http://www.w3.org/2000/svg" font-family="system-ui, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr27t" markerWidth="8" markerHeight="8" refX="6" refY="4" orient="auto">
      <path d="M0,0 L7,4 L0,8 z" fill="#9a9280"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="900" height="540" rx="10" fill="#f7f5f0"/>
  <text x="450" y="30" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">一本 Diagnostic 账，三种读数</text>

  <!-- ============ 上半部：测量历史队列 ============ -->
  <text x="60" y="62" font-size="12" fill="#4a463f">最近 120 笔测量，附时间戳：</text>
  <g stroke="#4a463f" stroke-width="1.2" fill="#ffffff">
    <rect x="60"  y="72" width="24" height="24" rx="3"/><rect x="88"  y="72" width="24" height="24" rx="3"/>
    <rect x="116" y="72" width="24" height="24" rx="3"/><rect x="144" y="72" width="24" height="24" rx="3"/>
    <rect x="172" y="72" width="24" height="24" rx="3"/><rect x="200" y="72" width="24" height="24" rx="3"/>
    <rect x="228" y="72" width="24" height="24" rx="3"/><rect x="256" y="72" width="24" height="24" rx="3"/>
    <rect x="284" y="72" width="24" height="24" rx="3"/><rect x="312" y="72" width="24" height="24" rx="3"/>
    <rect x="340" y="72" width="24" height="24" rx="3"/><rect x="368" y="72" width="24" height="24" rx="3"/>
    <rect x="396" y="72" width="24" height="24" rx="3"/><rect x="424" y="72" width="24" height="24" rx="3"/>
    <rect x="452" y="72" width="24" height="24" rx="3"/><rect x="480" y="72" width="24" height="24" rx="3"/>
    <rect x="508" y="72" width="24" height="24" rx="3"/><rect x="536" y="72" width="24" height="24" rx="3"/>
    <rect x="564" y="72" width="24" height="24" rx="3"/><rect x="592" y="72" width="24" height="24" rx="3"/>
    <rect x="620" y="72" width="24" height="24" rx="3"/><rect x="648" y="72" width="24" height="24" rx="3"/>
  </g>
  <rect x="676" y="72" width="24" height="24" rx="3" fill="#efe0c4" stroke="#b8862e" stroke-width="2.2"/>
  <text x="688" y="116" text-anchor="middle" font-size="10.5" fill="#8a6a1e">最新一笔</text>
  <path d="M716,84 L764,84" stroke="#9a9280" stroke-width="1.6" fill="none" marker-end="url(#arr27t)"/>
  <text x="790" y="89" font-size="11" fill="#9a9280">时间</text>
  <text x="60" y="116" font-size="10.5" fill="#9a9280">（史册长度 with_max_history_length 可调，装满就挤掉最旧的）</text>

  <!-- ============ 下半部：三条读数曲线 ============ -->
  <line x1="70" y1="150" x2="70" y2="450" stroke="#c9c2b2" stroke-width="1.4"/>
  <line x1="70" y1="450" x2="660" y2="450" stroke="#c9c2b2" stroke-width="1.4"/>
  <text x="56" y="230" text-anchor="end" font-size="10.5" fill="#9a9280">大</text>
  <text x="56" y="390" text-anchor="end" font-size="10.5" fill="#9a9280">小</text>
  <text x="660" y="468" text-anchor="end" font-size="10.5" fill="#9a9280">时间 →</text>

  <!-- 原始信号（粗浅灰）：前段低平 + 一根尖刺，中段跳上台阶，后段又两根尖刺 -->
  <path d="M70,390 L160,390 L170,300 L180,390 L300,390 L306,230 L430,230 L440,310 L450,230 L560,230 L570,160 L580,230 L660,230"
        fill="none" stroke="#c9c2b2" stroke-width="6" stroke-linejoin="round" stroke-linecap="round"/>
  <!-- value()：细红线，完全贴着原始信号（连尖刺一起） -->
  <path d="M70,390 L160,390 L170,300 L180,390 L300,390 L306,230 L430,230 L440,310 L450,230 L560,230 L570,160 L580,230 L660,230"
        fill="none" stroke="#b3402e" stroke-width="1.8" stroke-linejoin="round"/>
  <!-- average()：滑窗均值——最平缓，尖刺摊平，台阶后慢慢爬 -->
  <path d="M70,388 L160,388 C185,388 190,380 215,383 L300,384 C340,380 370,290 430,252 C455,240 465,250 490,246 C520,242 545,238 575,236 C605,228 635,232 660,231"
        fill="none" stroke="#33586e" stroke-width="2.4" stroke-linejoin="round"/>
  <!-- smoothed()：EMA——快速跟上台阶，又滤掉毛刺 -->
  <path d="M70,386 L162,386 C172,380 176,376 186,382 C196,387 240,387 300,387 C312,330 330,250 366,236 C392,228 420,229 436,236 C446,240 452,234 470,231 C500,228 552,228 566,222 C576,218 582,226 600,228 C620,229 645,228 660,228"
        fill="none" stroke="#1d6b40" stroke-width="2.4" stroke-linejoin="round"/>

  <text x="120" y="416" font-size="10.5" fill="#9a9280">原始信号（带尖刺）</text>

  <!-- 右侧：三个方法名 -->
  <line x1="660" y1="230" x2="700" y2="196" stroke="#b3402e" stroke-width="1.4" stroke-dasharray="3 3"/>
  <rect x="694" y="184" width="14" height="14" rx="3" fill="#b3402e"/>
  <text x="716" y="196" font-size="12.5" fill="#b3402e" font-family="Consolas, monospace" font-weight="bold">value()</text>
  <text x="716" y="212" font-size="10.5" fill="#7a7468">敏捷——每笔照录，连尖刺一起</text>

  <line x1="660" y1="228" x2="700" y2="262" stroke="#1d6b40" stroke-width="1.4" stroke-dasharray="3 3"/>
  <rect x="694" y="250" width="14" height="14" rx="3" fill="#1d6b40"/>
  <text x="716" y="262" font-size="12.5" fill="#1d6b40" font-family="Consolas, monospace" font-weight="bold">smoothed()</text>
  <text x="716" y="278" font-size="10.5" fill="#7a7468">折中——跟得上台阶，滤得掉毛刺</text>

  <line x1="660" y1="231" x2="700" y2="328" stroke="#33586e" stroke-width="1.4" stroke-dasharray="3 3"/>
  <rect x="694" y="316" width="14" height="14" rx="3" fill="#33586e"/>
  <text x="716" y="328" font-size="12.5" fill="#33586e" font-family="Consolas, monospace" font-weight="bold">average()</text>
  <text x="716" y="344" font-size="10.5" fill="#7a7468">平稳——尖刺摊平，但整体慢半拍</text>

  <text x="450" y="512" text-anchor="middle" font-size="11.5" fill="#7a7468">同一段测量史，三种口径：播报与水牌默认念 smoothed，翻旧账用 values()/duration 自己算</text>
</svg>
"""


def fig_02_immediate_svg() -> None:
    """Figure 27-2：即时模式一帧四步（手绘 SVG）。"""
    save_svg(SVG_02_IMMEDIATE, "fig-27-02-immediate-mode.svg")


def fig_12_three_readings_svg() -> None:
    """Figure 27-12：一本账三种读数（手绘 SVG）。"""
    save_svg(SVG_12_THREE_READINGS, "fig-27-12-three-readings.svg")


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_01_first_chalk,
    fig_02_immediate_svg,
    fig_03_vocabulary,
    fig_04_resolution,
    fig_05_width_and_joints,
    fig_06_line_styles,
    fig_07_text_gizmos,
    fig_08_two_clocks,
    fig_09_retained_grid,
    fig_10_aabb_draw_all,
    fig_11_light_gizmos,
    fig_12_three_readings_svg,
    fig_13_overlays,
    fig_14_mini_workshop,
    fig_15_gizmo_modes,
    fig_16_stagehand_on_off,
    fig_17_frozen_inspect,
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
        cargo("build", "-p", "ch27-dev-tools", "--bins", "--examples")
    for fig in todo:
        fig()
        time.sleep(0.5)


if __name__ == "__main__":
    main()

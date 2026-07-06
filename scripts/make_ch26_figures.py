# -*- coding: utf-8 -*-
"""一键重建第 26 章全部 25 张插图（21 张运行截图 + 1 张动图 + 3 张手绘 SVG）。

    py -3.11 scripts/make_ch26_figures.py [图名筛选]

运行图全部由真实键盘驱动（SendInput scancode；方向键 KEYEVENTF_EXTENDEDKEY），
窗口截取走 capture.Example（PrintWindow，物理像素，统一归一到 1280×720 逻辑像素）。
键序照 scratchpad/run_ch26_tests.py 的剧本（该脚本是本章实测台账的驱动脚本，
逐 listing 的按键时序与本文件一一对应）。

要点（见 scripts/figures-ops.md 与 workorders/ch26.md「实测台账」）：
- BEVY_ASSET_ROOT 必须指到 crate 目录，main.rs 要加载字体；
- 首个键击安排在窗口出现 ≥2.5s 后（渲染管线要热身）；
- listing-26-01 开场即 TonyMcMapface（RECIPES[6]），空格轮换序：
  Blender→Khronos→None→Reinhard→ReinhardLum→Aces→AgX→SBDT→TMM；
- listing-26-11 的 TAA+MSAA 每帧 warn 刷屏，M 键关闭后噪点约 1~1.5s 内熔平，
  截图前需要等待收敛；
- fig-26-12 走 capture.record 录帧 + Pillow 存 WebP 动图（≤2MB）。
"""

import ctypes
import os
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
CRATE = CODE / "ch26-quality"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch26"

# 直接跑 exe 时没有 CARGO_MANIFEST_DIR，Bevy 靠它找 assets/（main.rs 要加载字体）
os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402

user32 = ctypes.windll.user32
kernel32 = ctypes.windll.kernel32

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36

# ---------------------------------------------------------------- SendInput

ULONG_PTR = ctypes.POINTER(ctypes.c_ulong)


class KEYBDINPUT(ctypes.Structure):
    _fields_ = [
        ("wVk", ctypes.c_ushort),
        ("wScan", ctypes.c_ushort),
        ("dwFlags", ctypes.c_ulong),
        ("time", ctypes.c_ulong),
        ("dwExtraInfo", ULONG_PTR),
    ]


class _INPUTunion(ctypes.Union):
    _fields_ = [("ki", KEYBDINPUT), ("pad", ctypes.c_ubyte * 32)]


class INPUT(ctypes.Structure):
    _fields_ = [("type", ctypes.c_ulong), ("union", _INPUTunion)]


INPUT_KEYBOARD = 1
KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYEVENTF_EXTENDEDKEY = 0x2, 0x8, 0x1

SCAN = {
    "1": 0x02, "2": 0x03, "3": 0x04, "4": 0x05, "5": 0x06,
    "0": 0x0B, "MINUS": 0x0C, "EQUAL": 0x0D,
    "Q": 0x10, "W": 0x11, "E": 0x12, "R": 0x13, "T": 0x14, "P": 0x19,
    "S": 0x1F, "G": 0x22, "H": 0x23, "F": 0x21,
    "C": 0x2E, "V": 0x2F, "B": 0x30, "M": 0x32,
    "SPACE": 0x39,
    "UP": (0x48, True), "DOWN": (0x50, True),
    "LEFT": (0x4B, True), "RIGHT": (0x4D, True),
}


def _key(name: str, up: bool) -> INPUT:
    entry = SCAN[name]
    scan, ext = entry if isinstance(entry, tuple) else (entry, False)
    flags = KEYEVENTF_SCANCODE | (KEYEVENTF_KEYUP if up else 0)
    if ext:
        flags |= KEYEVENTF_EXTENDEDKEY
    inp = INPUT()
    inp.type = INPUT_KEYBOARD
    inp.union.ki = KEYBDINPUT(0, scan, flags, 0, None)
    return inp


def _key_vk(vk: int, up: bool) -> INPUT:
    inp = INPUT()
    inp.type = INPUT_KEYBOARD
    inp.union.ki = KEYBDINPUT(vk, 0, KEYEVENTF_KEYUP if up else 0, 0, None)
    return inp


def _send(*inputs: INPUT) -> None:
    array = (INPUT * len(inputs))(*inputs)
    if user32.SendInput(len(inputs), array, ctypes.sizeof(INPUT)) != len(inputs):
        raise RuntimeError("SendInput 未全部送达")


def force_foreground(hwnd: int, tries: int = 10) -> None:
    """AttachThreadInput 绕开前台锁（带 Alt 空击解锁前台切换限制）。"""
    for _ in range(tries):
        if user32.GetForegroundWindow() == hwnd:
            return
        _send(_key_vk(0x12, False))
        _send(_key_vk(0x12, True))
        tid_fg = user32.GetWindowThreadProcessId(user32.GetForegroundWindow(), None)
        tid_us = kernel32.GetCurrentThreadId()
        user32.AttachThreadInput(tid_us, tid_fg, True)
        user32.BringWindowToTop(hwnd)
        user32.SetForegroundWindow(hwnd)
        user32.AttachThreadInput(tid_us, tid_fg, False)
        time.sleep(0.2)
    raise RuntimeError("示例窗口拿不到前台焦点，键击会落空——关掉抢焦点的程序再试")


def tap(ex: Example, name: str, hold: float = 0.06, settle: float = 0.3) -> None:
    force_foreground(ex.hwnd)
    _send(_key(name, False))
    time.sleep(hold)
    _send(_key(name, True))
    time.sleep(settle)


def hold_key(ex: Example, name: str, dur: float) -> None:
    """按住期间每隔一小段重发一次 keydown（PrintWindow 采样时抓帧会吃掉按住状态）。"""
    force_foreground(ex.hwnd)
    steps = max(1, int(dur / 0.1))
    for _ in range(steps):
        _send(_key(name, False))
        time.sleep(0.1)
    _send(_key(name, True))
    time.sleep(0.05)
    _send(_key(name, True))  # 抬键补一枪


# ---------------------------------------------------------------- 通用排版

def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch26-quality.exe"
    return EXAMPLES / f"{name}.exe"


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
    """多联横排，顶部一条整跨标注（每格一段文字，ch22 惯例）。"""
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
    """多联横排，每联下方各自一条标注条（ch25 惯例的下置变体）。"""
    h = max(im.height for im in images)
    w = sum(im.width for im in images) + GAP * (len(images) - 1)
    canvas = Image.new("RGB", (w, h + LABEL_H), GAP_COLOR)
    x = 0
    for im, text in zip(images, labels):
        canvas.paste(im, (x, 0))
        canvas.paste(label_bar(im.width, [text]), (x, h))
        x += im.width + GAP
    return canvas


def vstack(rows: list[Image.Image]) -> Image.Image:
    w = max(im.width for im in rows)
    h = sum(im.height for im in rows) + GAP * (len(rows) - 1)
    canvas = Image.new("RGB", (w, h), GAP_COLOR)
    y = 0
    for im in rows:
        canvas.paste(im, (0, y))
        y += im.height + GAP
    return canvas


def logical(img: Image.Image) -> Image.Image:
    """物理像素 → 1280×720 逻辑像素（显示器缩放会变，按宽自动归一）。"""
    if img.size == (1280, 720):
        return img
    return img.resize((1280, 720), Image.LANCZOS)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def save_svg(text: str, filename: str) -> None:
    path = OUT / filename
    path.write_text(text, encoding="utf-8")
    print(f"{filename}：{path.stat().st_size // 1024} KB")


THIRD = 1 / 3   # 三/四联单帧缩放
QUARTER = 0.5


def shrink(img: Image.Image, k: float = 0.5) -> Image.Image:
    return img.resize((int(img.width * k), int(img.height * k)), Image.LANCZOS)


def crop_zoom(img: Image.Image, box: tuple, out_w: int) -> Image.Image:
    """裁切放大：等比、最近邻（保持像素观感，用于细几何/锯齿裁切）。"""
    c = img.crop(box)
    scale = out_w / c.width
    return c.resize((out_w, int(c.height * scale)), Image.NEAREST)


def crop_lanczos(img: Image.Image, box: tuple, out_w: int) -> Image.Image:
    """裁切放大：等比、LANCZOS（用于柔光/景深这类连续色调裁切）。"""
    c = img.crop(box)
    scale = out_w / c.width
    return c.resize((out_w, int(c.height * scale)), Image.LANCZOS)


def corner_label(img: Image.Image, text: str, pos: str = "tl") -> Image.Image:
    """画面角落叠字（九宫格/预设格用：每格标名字）。"""
    img = img.copy()
    draw = ImageDraw.Draw(img)
    w = draw.textlength(text, font=FONT)
    pad = 8
    if pos == "tl":
        xy = (pad, pad)
    else:
        xy = (img.width - w - pad, pad)
    draw.rectangle((xy[0] - 6, xy[1] - 4, xy[0] + w + 6, xy[1] + 26), fill=(10, 10, 12, 200))
    draw.text(xy, text, font=FONT, fill=(240, 235, 220))
    return img


# ---------------------------------------------------------------- Figure 26-2：九宫格冲印

def fig_02_tonemapping_grid() -> None:
    """Figure 26-2：listing-26-01 九配方九宫格（Space 轮换 8 次 + 开场 TMM）。"""
    names = ["TonyMcMapface", "BlenderFilmic", "KhronosPbrNeutral", "None",
             "Reinhard", "ReinhardLuminance", "AcesFitted", "AgX",
             "SomewhatBoringDisplayTransform"]
    shots = []
    with Example(exe("listing-26-01"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        shots.append(logical(ex.grab()))          # 开场 TMM
        for _ in range(8):                          # 轮换 8 次覆盖剩余 8 种
            tap(ex, "SPACE", settle=0.5)
            shots.append(logical(ex.grab()))
    cells = [corner_label(shrink(s, THIRD), n) for s, n in zip(shots, names)]
    rows = [
        hstack([cells[0], cells[1], cells[2]]),
        hstack([cells[3], cells[4], cells[5]]),
        hstack([cells[6], cells[7], cells[8]]),
    ]
    save_png(vstack(rows), "fig-26-02-tonemapping-grid.png")


def fig_03_hdr_vs_ldr() -> None:
    """Figure 26-3：HDR+TMM / LDR+TMM（H 拔）/ HDR+None 三联。"""
    with Example(exe("listing-26-01"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        hdr_tmm = logical(ex.grab())                # 开场：HDR + TMM
        tap(ex, "H", settle=0.5)                     # 抽底片 → LDR + TMM
        ldr_tmm = logical(ex.grab())
        tap(ex, "H", settle=0.5)                     # 插回 HDR
        for _ in range(2):                            # 轮换到 None（TMM→Blender→Khronos→None）
            tap(ex, "SPACE", settle=0.4)
        tap(ex, "SPACE", settle=0.4)
        hdr_none = logical(ex.grab())
    save_png(
        hstack_below([shrink(hdr_tmm), shrink(ldr_tmm), shrink(hdr_none)],
                     ["HDR + TonyMcMapface", "LDR + TonyMcMapface（H 拔）", "HDR + None（对照）"]),
        "fig-26-03-hdr-vs-ldr.png",
    )


# ---------------------------------------------------------------- Figure 26-4/5：Bloom

def fig_04_bloom_intensity() -> None:
    """Figure 26-4：listing-26-02 intensity 0.00/0.15/0.30 三联。"""
    with Example(exe("listing-26-02"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        mid = logical(ex.grab())                     # 默认 0.15
        for _ in range(6):
            tap(ex, "DOWN", settle=0.15)
        zero = logical(ex.grab())                    # 0.00
        for _ in range(3):
            tap(ex, "UP", settle=0.15)
        back = logical(ex.grab())                    # 回 0.15（核对）
        for _ in range(3):
            tap(ex, "UP", settle=0.15)
        high = logical(ex.grab())                    # 0.30
    save_png(
        hstack_below([shrink(zero), shrink(back), shrink(high)],
                     ["intensity 0.00：无晕", "intensity 0.15：默认", "intensity 0.30：全场发蒙"]),
        "fig-26-04-bloom-intensity.png",
    )


def fig_05_emissive_vs_intensity() -> None:
    """Figure 26-5：左 intensity 0.30 全场蒙 vs 右 0.15+金灯 emissive×4.1。"""
    # 左联：intensity 拉到 0.30
    with Example(exe("listing-26-02"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        for _ in range(3):
            tap(ex, "UP", settle=0.15)
        left = logical(ex.grab())
    # 右联：回到默认 crate 再单独调金灯 emissive（intensity 保持 0.15）
    with Example(exe("listing-26-02"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        for _ in range(3):
            tap(ex, "EQUAL", settle=0.15)
        right = logical(ex.grab())
    save_png(
        hstack_below([shrink(left), shrink(right)],
                     ["intensity 0.30：三盏灯一起发蒙", "intensity 0.15 + 金灯 emissive ×4.1：只中灯变亮"]),
        "fig-26-05-emissive-vs-intensity.png",
    )


# ---------------------------------------------------------------- Figure 26-6：四预设

def fig_06_bloom_presets() -> None:
    """Figure 26-6：listing-26-03 四预设（NATURAL/OLD_SCHOOL/ANAMORPHIC/SCREEN_BLUR）。"""
    names = ["NATURAL", "OLD_SCHOOL", "ANAMORPHIC", "SCREEN_BLUR"]
    shots = []
    with Example(exe("listing-26-03"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        shots.append(logical(ex.grab()))
        for _ in range(3):
            tap(ex, "SPACE", settle=0.5)
            shots.append(logical(ex.grab()))
    cells = [corner_label(shrink(s), n) for s, n in zip(shots, names)]
    save_png(
        vstack([hstack([cells[0], cells[1]]), hstack([cells[2], cells[3]])]),
        "fig-26-06-bloom-presets.png",
    )


# ---------------------------------------------------------------- Figure 26-7：哑巴坑

def fig_07_silent_bloom_death() -> None:
    """Figure 26-7：listing-26-04 晕在/H 拔晕灭/H 插复活三联。"""
    with Example(exe("listing-26-04"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        glow = logical(ex.grab())
        tap(ex, "H", settle=0.6)
        dead = logical(ex.grab())
        tap(ex, "H", settle=0.6)
        alive = logical(ex.grab())
    save_png(
        hstack_below([shrink(glow), shrink(dead), shrink(alive)],
                     ["Hdr 在：金灯带饱满光晕", "H 拔掉 Hdr：晕当场熄灭（零警告）", "H 插回：晕原样复活"]),
        "fig-26-07-silent-bloom-death.png",
    )


# ---------------------------------------------------------------- Figure 26-8/9/10：景深

def fig_08_focus_rack() -> None:
    """Figure 26-8：listing-26-05 f/1.4 Bokeh 下对焦 1/2/3 三档。"""
    with Example(exe("listing-26-05"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        tap(ex, "G", settle=0.5)                     # 先切 Bokeh
        near = logical(ex.grab())                     # 默认已对焦 1（近）
        tap(ex, "2", settle=0.5)
        mid = logical(ex.grab())
        tap(ex, "3", settle=0.5)
        far = logical(ex.grab())
    save_png(
        hstack_below([shrink(near), shrink(mid), shrink(far)],
                     ["对焦琉璃盏（近）", "对焦堂鼓（中）", "对焦锦旗（远）"]),
        "fig-26-08-focus-rack.png",
    )


def fig_09_aperture_ladder() -> None:
    """Figure 26-9：对焦近档，光圈 Q/W/E 三档（f/1.4 → f/5.6 → f/16）。"""
    with Example(exe("listing-26-05"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        f14 = logical(ex.grab())                      # 默认已对焦近、f/1.4
        tap(ex, "W", settle=0.5)
        f56 = logical(ex.grab())
        tap(ex, "E", settle=0.5)
        f16 = logical(ex.grab())
    save_png(
        hstack_below([shrink(f14), shrink(f56), shrink(f16)],
                     ["f/1.4：背景重糊", "f/5.6：糊度收敛", "f/16：全场基本清晰"]),
        "fig-26-09-aperture-ladder.png",
    )


def fig_10_bokeh_vs_gaussian() -> None:
    """Figure 26-10：远排珠灯离焦形态 Gaussian vs Bokeh（对焦 1、f/1.4，裁珠灯排）。"""
    with Example(exe("listing-26-05"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        gauss = logical(ex.grab())                    # 默认 Gaussian，对焦近、f/1.4
        tap(ex, "G", settle=0.5)
        bokeh = logical(ex.grab())
    # 珠灯排挂在画面上沿（z=-8.5、y=2.4 的一排小亮点）——只裁右侧三颗、放大一倍，
    # 光斑的边缘形态（渐隐 vs 平盘）才看得真切
    box = (660, 20, 1140, 220)
    save_png(
        hstack_below([crop_lanczos(gauss, box, 800), crop_lanczos(bokeh, box, 800)],
                     ["Gaussian：糊成雾", "Bokeh：摊成盘"]),
        "fig-26-10-bokeh-vs-gaussian.png",
    )


# ---------------------------------------------------------------- Figure 26-11~14：运动模糊

def fig_11_shutter_angle() -> None:
    """Figure 26-11：listing-26-07 快门角 0/0.25/1.0/2.0 静帧四联。

    轮换序 0.5(开场)→1→2→0→0.25→0.5……需要拨到目标档再截。
    """
    with Example(exe("listing-26-07"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        tap(ex, "SPACE", settle=0.4)                  # 0.5→1.0
        one = logical(ex.grab())
        tap(ex, "SPACE", settle=0.4)                  # 1.0→2.0
        two = logical(ex.grab())
        tap(ex, "SPACE", settle=0.4)                  # 2.0→0.0
        zero = logical(ex.grab())
        tap(ex, "SPACE", settle=0.4)                  # 0.0→0.25
        quarter = logical(ex.grab())
    cells = [corner_label(shrink(s), n) for s, n in
             zip([zero, quarter, one, two], ["快门角 0", "快门角 0.25", "快门角 1.0", "快门角 2.0"])]
    save_png(vstack([hstack([cells[0], cells[1]]), hstack([cells[2], cells[3]])]),
             "fig-26-11-shutter-angle.png")


def fig_12_carousel_strobe() -> None:
    """Figure 26-12（动图）：快门角 0 与 0.5 各录一段，拼接循环，角落标档位。"""
    fps, dur = 10, 1.2

    # 快门角 0：开场 0.5→SPACE→1.0→SPACE→2.0→SPACE→0.0（3 次）
    with Example(exe("listing-26-07"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        for _ in range(3):
            tap(ex, "SPACE", settle=0.4)
        elapsed = time.perf_counter() - ex.t0
        frames0 = ex.record(elapsed + 0.1, dur, fps, size=(1280, 720))

    # 快门角 0.5：全新进程，开场就是 0.5，不用拨
    with Example(exe("listing-26-07"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        elapsed = time.perf_counter() - ex.t0
        frames_half = ex.record(elapsed + 0.1, dur, fps, size=(1280, 720))

    def stamp(img: Image.Image, text: str) -> Image.Image:
        im = logical(img).copy()
        draw = ImageDraw.Draw(im)
        w = draw.textlength(text, font=FONT)
        draw.rectangle((im.width - w - 26, im.height - 40, im.width - 6, im.height - 6),
                       fill=(10, 10, 12))
        draw.text((im.width - w - 16, im.height - 34), text, font=FONT, fill=(240, 235, 220))
        return im

    seq = [stamp(f, "快门角 0") for f in frames0] + [stamp(f, "快门角 0.5") for f in frames_half]
    path = OUT / "fig-26-12-carousel-strobe.webp"
    seq[0].save(path, save_all=True, append_images=seq[1:],
               duration=100, loop=0, method=4, quality=70)
    print(f"fig-26-12-carousel-strobe.webp：{len(seq)} 帧，{path.stat().st_size // 1024} KB")


def fig_13_blur_samples() -> None:
    """Figure 26-13：快门角 2.0 下 samples 1 vs 4 双联。"""
    with Example(exe("listing-26-07"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        for _ in range(2):                             # 0.5→1.0→2.0
            tap(ex, "SPACE", settle=0.4)
        s1 = logical(ex.grab())                        # samples 默认 1
        tap(ex, "S", settle=0.4)
        s4 = logical(ex.grab())                        # samples 4
    save_png(
        hstack_below([shrink(s1), shrink(s4)],
                     ["快门角 2.0，samples 1：断成分身", "快门角 2.0，samples 4：连贯拖影"]),
        "fig-26-13-blur-samples.png",
    )


def fig_14_camera_pan_blur() -> None:
    """Figure 26-14：samples 4、按住方向键横摇机位时抓一帧全屏糊。"""
    with Example(exe("listing-26-07"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        tap(ex, "S", settle=0.3)                       # samples → 4
        force_foreground(ex.hwnd)
        _send(_key("RIGHT", False))
        time.sleep(0.55)
        shot = logical(ex.grab())
        _send(_key("RIGHT", True))
    save_png(shot, "fig-26-14-camera-pan-blur.png")


# ---------------------------------------------------------------- Figure 26-15/16：镜头三件套

def fig_15_vignette_distortion() -> None:
    """Figure 26-15（2×2）：上排暗角开/关对照（畸变 0），下排桶形/枕形（暗角复位）。"""
    with Example(exe("listing-26-08"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        vign_on = logical(ex.grab())                    # 开场：暗角开、畸变 0——直线基准
        tap(ex, "V", settle=0.4)
        vign_off = logical(ex.grab())                   # 暗角摘下，四角亮回来
        tap(ex, "V", settle=0.3)                        # 装回，后两格保持三件套完整
        for _ in range(2):
            tap(ex, "RIGHT", settle=0.3)                # 0→0.2→0.4
        barrel = logical(ex.grab())
        for _ in range(6):
            tap(ex, "LEFT", settle=0.15)                # 0.4→...→-0.8
        pincushion = logical(ex.grab())
    top = hstack_below(
        [shrink(vign_on), shrink(vign_off)],
        ["暗角开（默认档），畸变 0：直线基准", "暗角摘下：四角亮回来"],
    )
    bottom = hstack_below(
        [shrink(barrel), shrink(pincushion)],
        ["畸变 +0.4：桶形外鼓", "畸变 -0.8：枕形内凹"],
    )
    canvas = Image.new(
        "RGB",
        (max(top.width, bottom.width), top.height + GAP + bottom.height),
        GAP_COLOR,
    )
    canvas.paste(top, (0, 0))
    canvas.paste(bottom, (0, top.height + GAP))
    save_png(canvas, "fig-26-15-vignette-and-distortion.png")


def fig_16_chromatic_aberration() -> None:
    """Figure 26-16：色差 0.02 vs 0.4+64samples，裁灯笼角。"""
    with Example(exe("listing-26-08"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        low = logical(ex.grab())                       # 默认 0.02
        tap(ex, "C", settle=0.4)
        high = logical(ex.grab())                      # 0.4 + 64 samples
    # 灯笼在画面上方左右两侧——裁右侧灯笼角（预演图核实的区域）
    box = (900, 150, 1280, 500)
    save_png(
        hstack_below([crop_lanczos(low, box, 640), crop_lanczos(high, box, 640)],
                     ["色差 0.02：镜头本色", "色差 0.4 + 64 采样：撞击特效"]),
        "fig-26-16-chromatic-aberration.png",
    )


# ---------------------------------------------------------------- Figure 26-17：自动曝光

def fig_17_auto_exposure_ramp() -> None:
    """Figure 26-17：P 摇暗后 t+0.3/t+1.5/t+3.5 三联。"""
    with Example(exe("listing-26-09"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(3.0)
        tap(ex, "P", hold=0.06, settle=0.0)
        t_p = time.perf_counter()

        def grab_after(dt: float) -> Image.Image:
            remain = t_p + dt - time.perf_counter()
            if remain > 0:
                time.sleep(remain)
            return logical(ex.grab())

        t03 = grab_after(0.3)
        t15 = grab_after(1.5)
        t35 = grab_after(3.5)
    save_png(
        hstack_below([shrink(t03), shrink(t15), shrink(t35)],
                     ["摇暗后 t+0.3s：近黑", "t+1.5s：半亮", "t+3.5s：完全适应"]),
        "fig-26-17-auto-exposure-ramp.png",
    )


# ---------------------------------------------------------------- Figure 26-19：MSAA 阶梯

def fig_19_msaa_ladder() -> None:
    """Figure 26-19：listing-26-10 素颜/4x/8x，裁栏杆+斜旗杆区等比放大。"""
    with Example(exe("listing-26-10"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        default4x = logical(ex.grab())                 # 出厂 MSAA 4x
        tap(ex, "1", settle=0.5)
        noaa = logical(ex.grab())
        tap(ex, "2", settle=0.4)
        tap(ex, "E", settle=0.5)                        # MSAA 上场后 E 拨到 8x
        msaa8 = logical(ex.grab())
    # 栏杆 + 斜旗杆区域：画面中偏左的细几何带（按预演图核实）
    box = (100, 260, 780, 620)
    save_png(
        hstack_below([crop_zoom(noaa, box, 640), crop_zoom(default4x, box, 640),
                     crop_zoom(msaa8, box, 640)],
                     ["素颜：粗硬阶梯", "MSAA 4x：阶梯基本抹平", "MSAA 8x：边际收益"]),
        "fig-26-19-msaa-ladder.png",
    )


def fig_20_fxaa() -> None:
    """Figure 26-20：FXAA High vs Extreme（键 3；T 到 Extreme），裁栏杆区。"""
    with Example(exe("listing-26-10"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        tap(ex, "3", settle=0.5)                        # FXAA 上场，默认 High
        high = logical(ex.grab())
        tap(ex, "T", settle=0.5)                        # 灵敏度拨到 Extreme（Q/W/E/R/T 五档）
        extreme = logical(ex.grab())
    box = (100, 260, 780, 620)
    save_png(
        hstack_below([crop_zoom(high, box, 640), crop_zoom(extreme, box, 640)],
                     ["FXAA High：快而略糊", "FXAA Extreme：涂抹感明显"]),
        "fig-26-20-fxaa.png",
    )


def fig_21_smaa_vs_fxaa() -> None:
    """Figure 26-21：SMAA High（键 4）vs FXAA High（键 3）同区域。"""
    with Example(exe("listing-26-10"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        tap(ex, "3", settle=0.5)
        fxaa = logical(ex.grab())
        tap(ex, "4", settle=0.5)
        smaa = logical(ex.grab())
    box = (100, 260, 780, 620)
    save_png(
        hstack_below([crop_zoom(smaa, box, 640), crop_zoom(fxaa, box, 640)],
                     ["SMAA High：磨边不糊图", "FXAA High：整体略糊（对比）"]),
        "fig-26-21-smaa-vs-fxaa.png",
    )


def fig_23_taa_melts_noise() -> None:
    """Figure 26-23：listing-26-11 MSAA 开（噪点）vs M 后 TAA 熔平（等待收敛）。"""
    with Example(exe("listing-26-11"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(3.0)                              # 先攒够警告/噪点
        noisy = logical(ex.grab())
        tap(ex, "M", settle=1.6)                         # Msaa::Off，等 TAA 收敛
        smooth = logical(ex.grab())
    # 堂鼓 + 鹅卵石接触阴影区（画面中央偏下，按预演图核实）
    box = (420, 380, 1180, 720)
    save_png(
        hstack_below([crop_lanczos(noisy, box, 640), crop_lanczos(smooth, box, 640)],
                     ["MSAA 开、TAA 罢工：接触阴影一地砂子", "Msaa::Off、TAA 熔平：噪点净尽"]),
        "fig-26-23-taa-melts-noise.png",
    )


def fig_24_cas_sharpen() -> None:
    """Figure 26-24：listing-26-10 TAA（键 5）vs TAA+CAS0.8，裁栏杆+瓷柱。"""
    with Example(exe("listing-26-10"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        tap(ex, "5", settle=1.3)                         # TAA 上场，多等几帧收敛
        taa = logical(ex.grab())
        tap(ex, "0", settle=0.4)                          # 锐化开（0.6）
        tap(ex, "EQUAL", settle=0.4)                      # 强度 0.8
        cas = logical(ex.grab())
    box = (100, 260, 900, 620)
    save_png(
        hstack_below([crop_zoom(taa, box, 700), crop_zoom(cas, box, 700)],
                     ["TAA，未锐化：边缘平滑略软", "TAA + CAS 0.8：棱线找回来了"]),
        "fig-26-24-cas-sharpen.png",
    )


def fig_25_final_shot() -> None:
    """Figure 26-25：main.rs 默认档全景（demo 占位图），需状态牌清晰可读。"""
    with Example(exe("main"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        shot = logical(ex.shot(3.0))
    save_png(shot, "fig-26-25-final-shot.png")


# ---------------------------------------------------------------- 手绘 SVG
# 内容即代码：落盘即重建（插图规范）。浅色卡片底，明暗主题均可读。

SVG_01_PIPELINE = """<svg viewBox="0 0 1180 420" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr26a" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#7a7468"/>
    </marker>
    <marker id="arr26b" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto">
      <path d="M0,0 L9,5 L0,10 z" fill="#b8862e"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="1180" height="420" rx="10" fill="#f7f5f0"/>
  <text x="590" y="32" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">后处理流水线：主 pass 到屏幕之间的一条龙</text>

  <!-- ============ HDR 段（左，暖底色区分） ============ -->
  <rect x="20" y="56" width="560" height="300" rx="10" fill="#fbf1de" stroke="#d9c194" stroke-width="1.6" stroke-dasharray="0"/>
  <text x="300" y="80" text-anchor="middle" font-size="13" fill="#8a6a1e" font-weight="bold">HDR 段——数值可以比 1.0 大，后处理吃的是这批原料</text>

  <!-- 主 pass -->
  <rect x="36" y="102" width="118" height="60" rx="8" fill="#ffffff" stroke="#4a463f" stroke-width="1.6"/>
  <text x="95" y="128" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">主 pass</text>
  <text x="95" y="146" text-anchor="middle" font-size="10" fill="#7a7468">场景画进中间画布</text>
  <path d="M158,132 L180,132" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>

  <!-- TAA -->
  <rect x="184" y="102" width="94" height="60" rx="8" fill="#ffffff" stroke="#4a463f" stroke-width="1.6"/>
  <text x="231" y="128" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">TAA</text>
  <text x="231" y="146" text-anchor="middle" font-size="9.5" fill="#7a7468">历史帧混合</text>
  <path d="M282,132 L302,132" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>

  <!-- 运动模糊 -->
  <rect x="306" y="102" width="94" height="60" rx="8" fill="#ffffff" stroke="#4a463f" stroke-width="1.6"/>
  <text x="353" y="122" text-anchor="middle" font-size="12" fill="#4a463f" font-weight="bold">运动</text>
  <text x="353" y="138" text-anchor="middle" font-size="12" fill="#4a463f" font-weight="bold">模糊</text>
  <text x="353" y="154" text-anchor="middle" font-size="9.5" fill="#7a7468">shutter_angle</text>
  <path d="M404,132 L424,132" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>

  <!-- 辉光 -->
  <rect x="428" y="102" width="94" height="60" rx="8" fill="#ffffff" stroke="#4a463f" stroke-width="1.6"/>
  <text x="475" y="128" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">辉光</text>
  <text x="475" y="146" text-anchor="middle" font-size="9.5" fill="#7a7468">Bloom</text>

  <path d="M95,166 L95,196" stroke="#7a7468" stroke-width="2" fill="none"/>
  <path d="M95,196 L510,196" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>

  <!-- 景深 -->
  <rect x="36" y="216" width="94" height="58" rx="8" fill="#ffffff" stroke="#4a463f" stroke-width="1.6"/>
  <text x="83" y="242" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">景深</text>
  <text x="83" y="259" text-anchor="middle" font-size="9.5" fill="#7a7468">DoF</text>
  <path d="M130,245 L150,245" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>

  <!-- 镜头三件套 -->
  <rect x="154" y="216" width="118" height="58" rx="8" fill="#ffffff" stroke="#4a463f" stroke-width="1.6"/>
  <text x="213" y="238" text-anchor="middle" font-size="12" fill="#4a463f" font-weight="bold">镜头三件套</text>
  <text x="213" y="255" text-anchor="middle" font-size="9" fill="#7a7468">暗角/畸变/色差</text>
  <path d="M272,245 L292,245" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>

  <!-- 自动测光 -->
  <rect x="296" y="216" width="118" height="58" rx="8" fill="#ffffff" stroke="#4a463f" stroke-width="1.6"/>
  <text x="355" y="238" text-anchor="middle" font-size="12" fill="#4a463f" font-weight="bold">自动测光</text>
  <text x="355" y="255" text-anchor="middle" font-size="9" fill="#7a7468">AutoExposure</text>

  <path d="M475,162 L475,190" stroke="#7a7468" stroke-width="2" fill="none"/>
  <path d="M475,190 L355,190 L355,216" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>
  <path d="M414,245 L560,245" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>

  <!-- ============ 分水岭：冲印 ============ -->
  <rect x="596" y="56" width="120" height="300" rx="10" fill="#efe0c4" stroke="#b8862e" stroke-width="2.4"/>
  <text x="656" y="90" text-anchor="middle" font-size="13.5" fill="#8a6a1e" font-weight="bold">冲印</text>
  <text x="656" y="108" text-anchor="middle" font-size="11" fill="#8a6a1e" font-family="Consolas, monospace">tonemapping</text>
  <text x="656" y="180" text-anchor="middle" font-size="10.5" fill="#8a6a1e">HDR 数值</text>
  <text x="656" y="196" text-anchor="middle" font-size="10.5" fill="#8a6a1e">压回</text>
  <text x="656" y="212" text-anchor="middle" font-size="10.5" fill="#8a6a1e">0.0~1.0</text>
  <text x="656" y="260" text-anchor="middle" font-size="10" fill="#a3752a">分水岭</text>
  <text x="656" y="276" text-anchor="middle" font-size="10" fill="#a3752a">HDR ↔ LDR</text>

  <path d="M96,300 L96,336 L636,336" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>
  <path d="M656,356 L656,300" stroke="#b8862e" stroke-width="3" fill="none" marker-end="url(#arr26b)"/>

  <!-- ============ LDR 段（右，冷底色区分） ============ -->
  <rect x="736" y="56" width="424" height="300" rx="10" fill="#eef1f4" stroke="#b6c2cc" stroke-width="1.6"/>
  <text x="948" y="80" text-anchor="middle" font-size="13" fill="#33586e" font-weight="bold">LDR 段——在 0~1 的显示范围里做形状/边缘的活儿</text>

  <rect x="752" y="102" width="94" height="60" rx="8" fill="#ffffff" stroke="#4a463f" stroke-width="1.6"/>
  <text x="799" y="128" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">FXAA</text>
  <text x="799" y="146" text-anchor="middle" font-size="9.5" fill="#7a7468">/SMAA</text>
  <path d="M850,132 L870,132" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>

  <rect x="874" y="102" width="94" height="60" rx="8" fill="#ffffff" stroke="#4a463f" stroke-width="1.6"/>
  <text x="921" y="128" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">锐化</text>
  <text x="921" y="146" text-anchor="middle" font-size="9.5" fill="#7a7468">CAS</text>
  <path d="M968,132 L988,132" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr26a)"/>

  <rect x="992" y="102" width="140" height="60" rx="8" fill="#ffffff" stroke="#4a463f" stroke-width="2"/>
  <text x="1062" y="128" text-anchor="middle" font-size="13" fill="#4a463f" font-weight="bold">屏幕</text>
  <text x="1062" y="146" text-anchor="middle" font-size="9.5" fill="#7a7468">最终显示</text>

  <text x="948" y="200" text-anchor="middle" font-size="11" fill="#33586e">MSAA/TAA 抢在冲印前的</text>
  <text x="948" y="218" text-anchor="middle" font-size="11" fill="#33586e">主 pass 里就采过样了——</text>
  <text x="948" y="236" text-anchor="middle" font-size="11" fill="#33586e">它们不在这条队伍里，是</text>
  <text x="948" y="254" text-anchor="middle" font-size="11" fill="#33586e">另一种「反锯齿」的活法</text>

  <text x="590" y="398" text-anchor="middle" font-size="11.5" fill="#7a7468">本章大多数工序排在冲印前面：辉光要认哪盏灯比 1.0 亮多少倍，自动测光要读真实亮度——这些账算完，才轮到冲印把结果摁回屏幕能显示的范围</text>
</svg>
"""

SVG_18_ALIASING = """<svg viewBox="0 0 980 460" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <rect x="0" y="0" width="980" height="460" rx="10" fill="#f7f5f0"/>
  <text x="490" y="34" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">锯齿的病理与 MSAA 的药理：一票定黑白 vs 四票分深浅</text>

  <!-- ============ 左：一像素一票 ============ -->
  <g>
    <rect x="30" y="60" width="430" height="360" rx="8" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
    <text x="245" y="88" text-anchor="middle" font-size="13.5" fill="#c05a2e" font-weight="bold">一像素一票——非黑即白</text>

    <!-- 像素网格 10x10，格宽 34 -->
    <g stroke="#d8d2c4" stroke-width="1">
      <!-- 生成网格线（写死坐标，14 行列覆盖 60~440） -->
    </g>
    <!-- 手绘网格：11 条竖线 + 11 条横线，覆盖 60..440（380px，34.5 步） -->
    <g stroke="#ded8ca" stroke-width="1">
      <line x1="60" y1="110" x2="440" y2="110"/><line x1="60" y1="145" x2="440" y2="145"/>
      <line x1="60" y1="180" x2="440" y2="180"/><line x1="60" y1="215" x2="440" y2="215"/>
      <line x1="60" y1="250" x2="440" y2="250"/><line x1="60" y1="285" x2="440" y2="285"/>
      <line x1="60" y1="320" x2="440" y2="320"/><line x1="60" y1="355" x2="440" y2="355"/>
      <line x1="95" y1="110" x2="95" y2="355"/><line x1="130" y1="110" x2="130" y2="355"/>
      <line x1="165" y1="110" x2="165" y2="355"/><line x1="200" y1="110" x2="200" y2="355"/>
      <line x1="235" y1="110" x2="235" y2="355"/><line x1="270" y1="110" x2="270" y2="355"/>
      <line x1="305" y1="110" x2="305" y2="355"/><line x1="340" y1="110" x2="340" y2="355"/>
      <line x1="375" y1="110" x2="375" y2="355"/><line x1="410" y1="110" x2="410" y2="355"/>
    </g>
    <rect x="60" y="110" width="380" height="245" fill="none" stroke="#4a463f" stroke-width="1.6"/>

    <!-- 斜边：从左上到右下穿过网格 -->
    <line x1="80" y1="110" x2="420" y2="355" stroke="#274a91" stroke-width="3"/>
    <text x="250" y="100" text-anchor="middle" font-size="10" fill="#274a91">真实几何边</text>

    <!-- 阶梯着色：斜边下方格子涂深色，模拟每格只在中心取一票 -->
    <g fill="#3b3733">
      <rect x="60" y="145" width="35" height="35"/>
      <rect x="60" y="180" width="70" height="35"/>
      <rect x="60" y="215" width="105" height="35"/>
      <rect x="60" y="250" width="140" height="35"/>
      <rect x="60" y="285" width="175" height="35"/>
      <rect x="60" y="320" width="210" height="35"/>
      <rect x="60" y="355" width="245" height="0"/>
    </g>
    <!-- 采样点：每格中心一个点 -->
    <g fill="#c05a2e">
      <circle cx="77.5" cy="127.5" r="3"/><circle cx="112.5" cy="162.5" r="3"/>
      <circle cx="147.5" cy="197.5" r="3"/><circle cx="182.5" cy="232.5" r="3"/>
      <circle cx="217.5" cy="267.5" r="3"/><circle cx="252.5" cy="302.5" r="3"/>
      <circle cx="287.5" cy="337.5" r="3"/>
    </g>
    <text x="245" y="380" text-anchor="middle" font-size="10.5" fill="#7a7468">每格只在中心问一次「在不在三角形里」</text>
    <text x="245" y="396" text-anchor="middle" font-size="10.5" fill="#c05a2e" font-weight="bold">答案非 0 即 1——边缘只能是台阶</text>
  </g>

  <!-- ============ 右：MSAA 4x ============ -->
  <g>
    <rect x="520" y="60" width="430" height="360" rx="8" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
    <text x="735" y="88" text-anchor="middle" font-size="13.5" fill="#1d6b40" font-weight="bold">MSAA 4x——四票分深浅</text>

    <g stroke="#ded8ca" stroke-width="1">
      <line x1="550" y1="110" x2="930" y2="110"/><line x1="550" y1="145" x2="930" y2="145"/>
      <line x1="550" y1="180" x2="930" y2="180"/><line x1="550" y1="215" x2="930" y2="215"/>
      <line x1="550" y1="250" x2="930" y2="250"/><line x1="550" y1="285" x2="930" y2="285"/>
      <line x1="550" y1="320" x2="930" y2="320"/><line x1="550" y1="355" x2="930" y2="355"/>
      <line x1="585" y1="110" x2="585" y2="355"/><line x1="620" y1="110" x2="620" y2="355"/>
      <line x1="655" y1="110" x2="655" y2="355"/><line x1="690" y1="110" x2="690" y2="355"/>
      <line x1="725" y1="110" x2="725" y2="355"/><line x1="760" y1="110" x2="760" y2="355"/>
      <line x1="795" y1="110" x2="795" y2="355"/><line x1="830" y1="110" x2="830" y2="355"/>
      <line x1="865" y1="110" x2="865" y2="355"/><line x1="900" y1="110" x2="900" y2="355"/>
    </g>
    <rect x="550" y="110" width="380" height="245" fill="none" stroke="#4a463f" stroke-width="1.6"/>

    <line x1="570" y1="110" x2="910" y2="355" stroke="#274a91" stroke-width="3"/>

    <!-- 过渡灰阶：沿斜边的格子按覆盖比例上色（25/50/75%），呈现渐变而非硬阶梯 -->
    <g>
      <rect x="550" y="145" width="35" height="35" fill="#3b3733" fill-opacity="0.25"/>
      <rect x="585" y="145" width="35" height="35" fill="#3b3733" fill-opacity="0.75"/>
      <rect x="585" y="180" width="35" height="35" fill="#3b3733" fill-opacity="0.25"/>
      <rect x="620" y="180" width="35" height="35" fill="#3b3733" fill-opacity="0.75"/>
      <rect x="620" y="215" width="35" height="35" fill="#3b3733" fill-opacity="0.25"/>
      <rect x="655" y="215" width="35" height="35" fill="#3b3733" fill-opacity="0.75"/>
      <rect x="655" y="250" width="35" height="35" fill="#3b3733" fill-opacity="0.25"/>
      <rect x="690" y="250" width="35" height="35" fill="#3b3733" fill-opacity="0.75"/>
      <rect x="690" y="285" width="35" height="35" fill="#3b3733" fill-opacity="0.25"/>
      <rect x="725" y="285" width="35" height="35" fill="#3b3733" fill-opacity="0.75"/>
      <rect x="725" y="320" width="35" height="35" fill="#3b3733" fill-opacity="0.25"/>
      <rect x="760" y="320" width="35" height="35" fill="#3b3733" fill-opacity="0.75"/>
      <rect x="550" y="110" width="35" height="35" fill="#3b3733" fill-opacity="0.0"/>
    </g>
    <!-- 每格四采样点示意（放大展示其中一格） -->
    <g fill="#1d6b40">
      <circle cx="561" cy="156" r="2.4"/><circle cx="574" cy="156" r="2.4"/>
      <circle cx="561" cy="169" r="2.4"/><circle cx="574" cy="169" r="2.4"/>
    </g>
    <text x="735" y="380" text-anchor="middle" font-size="10.5" fill="#7a7468">每格四个采样点各自问一次，按命中数上色</text>
    <text x="735" y="396" text-anchor="middle" font-size="10.5" fill="#1d6b40" font-weight="bold">25%/50%/75% 的过渡灰阶——阶梯变渐变</text>
  </g>

  <text x="490" y="440" text-anchor="middle" font-size="11.5" fill="#7a7468">MSAA 只在几何边上加采样点，纹理内部的颜色、光照一次搞定——这也是它比全屏超采样便宜得多的原因</text>
</svg>
"""

SVG_22_TAA = """<svg viewBox="0 0 1080 420" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr26taa" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#7a7468"/>
    </marker>
    <marker id="arr26taar" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#b3402e"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="1080" height="420" rx="10" fill="#f7f5f0"/>
  <text x="540" y="32" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">TAA 三件事：抖动采样、按运动向量对齐历史、加权混合</text>

  <!-- ============ 格一：亚像素抖动 ============ -->
  <g>
    <rect x="24" y="56" width="330" height="308" rx="8" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
    <text x="189" y="82" text-anchor="middle" font-size="13.5" fill="#4a463f" font-weight="bold">① 亚像素抖动</text>

    <rect x="60" y="140" width="220" height="150" fill="none" stroke="#4a463f" stroke-width="1.6"/>
    <line x1="170" y1="140" x2="170" y2="290" stroke="#ded8ca" stroke-width="1"/>
    <line x1="60" y1="215" x2="280" y2="215" stroke="#ded8ca" stroke-width="1"/>

    <!-- 连续四帧的采样点，各偏移不到半个像素，围绕格心 -->
    <circle cx="165" cy="210" r="4" fill="#274a91"/>
    <circle cx="178" cy="207" r="4" fill="#b8862e"/>
    <circle cx="172" cy="221" r="4" fill="#1d6b40"/>
    <circle cx="160" cy="217" r="4" fill="#8a4a91"/>
    <path d="M165,210 L178,207 L172,221 L160,217 Z" fill="none" stroke="#9a9280" stroke-width="1" stroke-dasharray="3 2"/>

    <text x="189" y="322" text-anchor="middle" font-size="10.5" fill="#7a7468">同一像素格，连续四帧</text>
    <text x="189" y="338" text-anchor="middle" font-size="10.5" fill="#7a7468">各偏移不到半个像素取样</text>
    <text x="189" y="354" text-anchor="middle" font-size="10" fill="#9a9280">TemporalJitter 负责这份偏移</text>
  </g>

  <!-- ============ 格二：运动向量 ============ -->
  <g>
    <rect x="374" y="56" width="330" height="308" rx="8" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
    <text x="539" y="82" text-anchor="middle" font-size="13.5" fill="#4a463f" font-weight="bold">② 运动向量找历史</text>

    <rect x="410" y="140" width="260" height="150" fill="none" stroke="#4a463f" stroke-width="1.6"/>
    <!-- 上一帧位置（虚） -->
    <rect x="430" y="230" width="34" height="34" fill="none" stroke="#9a9280" stroke-width="1.6" stroke-dasharray="4 3"/>
    <text x="447" y="278" text-anchor="middle" font-size="9.5" fill="#9a9280">上一帧</text>
    <!-- 当前帧位置（实） -->
    <rect x="586" y="166" width="34" height="34" fill="#efe0c4" stroke="#b8862e" stroke-width="1.8"/>
    <text x="603" y="216" text-anchor="middle" font-size="9.5" fill="#8a6a1e">当前帧</text>
    <!-- 箭头：从当前指回上一帧 -->
    <path d="M590,196 C 540,210 490,222 466,240" stroke="#274a91" stroke-width="2.2" fill="none"
          stroke-dasharray="6 4" marker-end="url(#arr26taa)"/>
    <text x="540" y="200" text-anchor="middle" font-size="10" fill="#274a91">到上一帧去找我自己</text>

    <text x="539" y="322" text-anchor="middle" font-size="10.5" fill="#7a7468">MotionVectorPrepass 记下每个像素</text>
    <text x="539" y="338" text-anchor="middle" font-size="10.5" fill="#7a7468">这一帧相对上一帧挪了多远</text>
  </g>

  <!-- ============ 格三：历史混合 + ghosting ============ -->
  <g>
    <rect x="724" y="56" width="332" height="308" rx="8" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
    <text x="890" y="82" text-anchor="middle" font-size="13.5" fill="#4a463f" font-weight="bold">③ 历史混合</text>

    <rect x="760" y="132" width="260" height="96" fill="none" stroke="#4a463f" stroke-width="1.4"/>
    <text x="890" y="122" text-anchor="middle" font-size="10.5" fill="#7a7468">当前帧 + 历史帧 按权重叠加</text>
    <!-- 边缘平滑输出：锯齿变过渡 -->
    <path d="M770,220 L820,220 L820,175 L870,175 L870,220 L1010,220" stroke="#1d6b40" stroke-width="3" fill="none"/>
    <path d="M770,220 L845,220 L845,175 L1010,175" stroke="#9a9280" stroke-width="1.6" stroke-dasharray="4 3" fill="none"/>
    <text x="890" y="246" text-anchor="middle" font-size="10" fill="#1d6b40">输出：平滑的边缘</text>

    <!-- 副作用：ghosting -->
    <rect x="760" y="264" width="260" height="76" rx="6" fill="#fbeee8" stroke="#b3402e" stroke-width="1.6"/>
    <text x="890" y="284" text-anchor="middle" font-size="11.5" fill="#b3402e" font-weight="bold">副作用：ghosting</text>
    <!-- 快速移动物体：实体+多层残影 -->
    <rect x="800" y="296" width="22" height="30" fill="#c9776a" fill-opacity="0.35"/>
    <rect x="812" y="296" width="22" height="30" fill="#c9776a" fill-opacity="0.55"/>
    <rect x="824" y="296" width="22" height="30" fill="#8a2a1e"/>
    <path d="M850,311 L900,311" stroke="#b3402e" stroke-width="1.6" fill="none" marker-end="url(#arr26taar)"/>
    <text x="960" y="316" font-size="9.5" fill="#b3402e">快速移动时旧帧</text>
    <text x="960" y="330" font-size="9.5" fill="#b3402e">残影拖尾</text>
  </g>

  <text x="540" y="398" text-anchor="middle" font-size="11.5" fill="#7a7468">三步连起来：每帧的采样点都不一样，靠运动向量把新旧对齐，再混合成一张更干净的画——代价是历史用错了地方就变成拖影</text>
</svg>
"""


def fig_01_pipeline_svg() -> None:
    """Figure 26-1：后处理流水线次序（手绘 SVG）。"""
    save_svg(SVG_01_PIPELINE, "fig-26-01-post-pipeline.svg")


def fig_18_aliasing_svg() -> None:
    """Figure 26-18：锯齿病理 vs MSAA 药理（手绘 SVG）。"""
    save_svg(SVG_18_ALIASING, "fig-26-18-aliasing-anatomy.svg")


def fig_22_taa_svg() -> None:
    """Figure 26-22：TAA 三件事示意（手绘 SVG）。"""
    save_svg(SVG_22_TAA, "fig-26-22-taa-anatomy.svg")


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_01_pipeline_svg,
    fig_02_tonemapping_grid,
    fig_03_hdr_vs_ldr,
    fig_04_bloom_intensity,
    fig_05_emissive_vs_intensity,
    fig_06_bloom_presets,
    fig_07_silent_bloom_death,
    fig_08_focus_rack,
    fig_09_aperture_ladder,
    fig_10_bokeh_vs_gaussian,
    fig_11_shutter_angle,
    fig_12_carousel_strobe,
    fig_13_blur_samples,
    fig_14_camera_pan_blur,
    fig_15_vignette_distortion,
    fig_16_chromatic_aberration,
    fig_17_auto_exposure_ramp,
    fig_18_aliasing_svg,
    fig_19_msaa_ladder,
    fig_20_fxaa,
    fig_21_smaa_vs_fxaa,
    fig_22_taa_svg,
    fig_23_taa_melts_noise,
    fig_24_cas_sharpen,
    fig_25_final_shot,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    cargo("build", "-p", "ch26-quality", "--bins", "--examples")
    only = sys.argv[1] if len(sys.argv) > 1 else None
    for fig in ALL:
        if only and only not in fig.__name__:
            continue
        fig()
        time.sleep(0.5)


if __name__ == "__main__":
    main()

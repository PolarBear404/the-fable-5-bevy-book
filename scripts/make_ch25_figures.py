# -*- coding: utf-8 -*-
"""一键重建第 25 章全部 14 张插图（11 张运行截图 + 3 张手绘 SVG）。

    py -3.11 scripts/make_ch25_figures.py [图名筛选]

运行图全部由真实键鼠驱动（SendInput：绝对落点点击/拖动喂拾取管线，
相对位移喂 FreeCamera 的 raw 摇臂，滚轮喂 Scroll/缩放），窗口截取走
capture.Example（PrintWindow，物理像素，统一归一到 1280×720 逻辑像素）。

工单 §3 的现场坐标（客户区比例）直接沿用：三件货 (0.29,0.44)/(0.50,0.345)/
(0.71,0.47)，25-11 重叠区 (0.53125,0.4167)，25-12 牌面 (0.29,0.435)。
两联图的排版是 hstack + 每联下方中文标注条（ch17 惯例的下置变体）。

要点（血泪见 scripts/figures-ops.md 与 workorders/ch25.md §3）：
- BEVY_ASSET_ROOT 必须指到 crate 目录，否则 sprite 图与字体静默加载失败；
- 启动进程前先把光标挪到屏幕顶边，杜绝开场 hover 杂音；
- 首个输入安排在窗口出现 ≥2.5s 后（管线要热身）；
- PrintWindow 截不到系统光标——alt 文本里提到光标的图，按注入时的真实
  坐标补画一枚光标箭头（标注，不是伪造：截图瞬间光标真在那里）。
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
CRATE = CODE / "ch25-picking"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch25"

# 直接跑 exe 时没有 CARGO_MANIFEST_DIR，Bevy 靠它找 assets/（缺了会静默失败）
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


class MOUSEINPUT(ctypes.Structure):
    _fields_ = [
        ("dx", ctypes.c_long),
        ("dy", ctypes.c_long),
        ("mouseData", ctypes.c_ulong),
        ("dwFlags", ctypes.c_ulong),
        ("time", ctypes.c_ulong),
        ("dwExtraInfo", ULONG_PTR),
    ]


class _INPUTunion(ctypes.Union):
    _fields_ = [("ki", KEYBDINPUT), ("mi", MOUSEINPUT)]


class INPUT(ctypes.Structure):
    _fields_ = [("type", ctypes.c_ulong), ("union", _INPUTunion)]


INPUT_MOUSE, INPUT_KEYBOARD = 0, 1
KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE = 0x2, 0x8
MOUSEEVENTF_MOVE = 0x0001
# 连续注入的相对位移会被系统合并（coalesce）而丢量，加此标志逐条保真
MOUSEEVENTF_MOVE_NOCOALESCE = 0x2000
MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP = 0x0002, 0x0004
MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP = 0x0008, 0x0010
MOUSEEVENTF_WHEEL = 0x0800

SCAN = {
    "1": 0x02, "2": 0x03, "3": 0x04, "4": 0x05,
    "R": 0x13, "P": 0x19, "U": 0x16, "M": 0x32, "D": 0x20, "V": 0x2F,
    "SPACE": 0x39,
}


def _send(*inputs: INPUT) -> None:
    array = (INPUT * len(inputs))(*inputs)
    if user32.SendInput(len(inputs), array, ctypes.sizeof(INPUT)) != len(inputs):
        raise RuntimeError("SendInput 未全部送达")


def _key(scan: int, up: bool) -> INPUT:
    inp = INPUT()
    inp.type = INPUT_KEYBOARD
    inp.union.ki = KEYBDINPUT(
        0, scan, KEYEVENTF_SCANCODE | (KEYEVENTF_KEYUP if up else 0), 0, None
    )
    return inp


def key_down(name: str) -> None:
    _send(_key(SCAN[name], False))


def key_up(name: str) -> None:
    _send(_key(SCAN[name], True))


def tap(name: str) -> None:
    key_down(name)
    time.sleep(0.06)
    key_up(name)


def _mouse(dx=0, dy=0, data=0, flags=0) -> INPUT:
    inp = INPUT()
    inp.type = INPUT_MOUSE
    inp.union.mi = MOUSEINPUT(dx, dy, data, flags, 0, None)
    return inp


def client_size(hwnd: int) -> tuple[int, int]:
    rect = wintypes.RECT()
    user32.GetClientRect(hwnd, ctypes.byref(rect))
    return rect.right, rect.bottom


def force_foreground(hwnd: int, tries: int = 8) -> None:
    """确保示例窗口在前台拿焦点——SendInput 的键击只进焦点窗口。"""
    for _ in range(tries):
        fg = user32.GetForegroundWindow()
        if fg == hwnd:
            return
        tid_fg = user32.GetWindowThreadProcessId(fg, None)
        tid_us = kernel32.GetCurrentThreadId()
        user32.AttachThreadInput(tid_us, tid_fg, True)
        user32.BringWindowToTop(hwnd)
        user32.SetForegroundWindow(hwnd)
        user32.AttachThreadInput(tid_us, tid_fg, False)
        time.sleep(0.15)
    if user32.GetForegroundWindow() != hwnd:
        raise RuntimeError("示例窗口拿不到前台焦点，输入会落空——关掉抢焦点的程序再试")


def park(hwnd: int, fx: float, fy: float) -> None:
    """把光标挪到窗口客户区的比例坐标 (fx, fy)——喂 hover。"""
    w, h = client_size(hwnd)
    point = wintypes.POINT(int(w * fx), int(h * fy))
    user32.ClientToScreen(hwnd, ctypes.byref(point))
    user32.SetCursorPos(point.x, point.y)


def click_at(hwnd: int, fx: float, fy: float) -> None:
    """点客户区比例坐标：落点 → 按 → 抬（间隔留足，DOWN/UP 各自成帧）。"""
    park(hwnd, fx, fy)
    time.sleep(0.15)
    _send(_mouse(flags=MOUSEEVENTF_LEFTDOWN))
    time.sleep(0.06)
    _send(_mouse(flags=MOUSEEVENTF_LEFTUP))
    time.sleep(0.15)


def drag_abs(hwnd: int, f0: tuple, f1: tuple, steps: int = 18, settle: float = 0.4,
             release: bool = True) -> None:
    """按住左键从比例坐标 f0 拖到 f1（逐步 SetCursorPos，喂 Pointer<Drag>）。

    release=False 时不抬键（拖拽中途截图用），调用方负责随后 mouse_up()。
    """
    w, h = client_size(hwnd)
    p0 = wintypes.POINT(int(w * f0[0]), int(h * f0[1]))
    p1 = wintypes.POINT(int(w * f1[0]), int(h * f1[1]))
    user32.ClientToScreen(hwnd, ctypes.byref(p0))
    user32.ClientToScreen(hwnd, ctypes.byref(p1))
    user32.SetCursorPos(p0.x, p0.y)
    time.sleep(0.25)
    _send(_mouse(flags=MOUSEEVENTF_LEFTDOWN))
    time.sleep(settle)
    for i in range(1, steps + 1):
        user32.SetCursorPos(
            p0.x + (p1.x - p0.x) * i // steps, p0.y + (p1.y - p0.y) * i // steps
        )
        time.sleep(0.04)
    time.sleep(settle)
    if release:
        _send(_mouse(flags=MOUSEEVENTF_LEFTUP))
        time.sleep(0.2)


def mouse_up() -> None:
    _send(_mouse(flags=MOUSEEVENTF_LEFTUP))
    time.sleep(0.15)


def drag_right_relative(dx: int, dy: int, steps: int = 14) -> None:
    """按住右键做相对位移（喂 FreeCamera 的 AccumulatedMouseMotion）。

    首尾留足帧余量：位移只在「右键按住」的帧里生效（ch17 的教训）。
    """
    _send(_mouse(flags=MOUSEEVENTF_RIGHTDOWN))
    time.sleep(0.5)
    for _ in range(steps):
        _send(_mouse(dx=dx // steps, dy=dy // steps,
                     flags=MOUSEEVENTF_MOVE | MOUSEEVENTF_MOVE_NOCOALESCE))
        time.sleep(0.05)
    time.sleep(0.5)
    _send(_mouse(flags=MOUSEEVENTF_RIGHTUP))


def wheel(notches: int) -> None:
    _send(_mouse(data=notches * 120, flags=MOUSEEVENTF_WHEEL))
    time.sleep(0.12)


def park_top() -> None:
    """启动进程前把光标挪到屏幕顶边（标题栏之外），防开场 hover 杂音。"""
    user32.SetCursorPos(user32.GetSystemMetrics(0) // 2, 2)


def launch(name: str) -> Example:
    """光标预归位 → 启动示例 → 拿前台焦点。"""
    park_top()
    ex = Example(exe(name), workdir=CODE)
    force_foreground(ex.hwnd)
    return ex


# ---------------------------------------------------------------- 通用排版

def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch25-picking.exe"
    return EXAMPLES / f"{name}.exe"


def cargo(*args: str) -> None:
    subprocess.run(["cargo", *args], cwd=CODE, check=True)


def label_bar(width: int, text: str) -> Image.Image:
    bar = Image.new("RGB", (width, LABEL_H), LABEL_BG)
    draw = ImageDraw.Draw(bar)
    w = draw.textlength(text, font=FONT)
    draw.text(((width - w) / 2, 6), text, font=FONT, fill=LABEL_FG)
    return bar


def hstack_below(images: list[Image.Image], labels: list[str]) -> Image.Image:
    """两联横排，每联下方一条中文标注条（ch17 hstack 的下置标注变体）。"""
    h = max(im.height for im in images)
    w = sum(im.width for im in images) + GAP * (len(images) - 1)
    canvas = Image.new("RGB", (w, h + LABEL_H), GAP_COLOR)
    x = 0
    for im, text in zip(images, labels):
        canvas.paste(im, (x, 0))
        canvas.paste(label_bar(im.width, text), (x, h))
        x += im.width + GAP
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


HALF = 0.5  # 联图单帧 640×360


def shrink(img: Image.Image) -> Image.Image:
    return img.resize((int(img.width * HALF), int(img.height * HALF)), Image.LANCZOS)


# ---------------------------------------------------------------- 标注小件
# PrintWindow 截不到系统光标；alt 里写明「光标在哪」的图，按注入坐标补画。
# 描边/十字用于标出「谁应声」——listing 本身不换色的场合（25-6、25-10）。

CURSOR_SHAPE = [(0, 0), (0, 16.5), (3.6, 13.2), (6.6, 20.2), (9.2, 19.1),
                (6.2, 12.4), (11.2, 12.4)]


def draw_cursor(img: Image.Image, x: float, y: float, k: float = 1.5) -> None:
    """在 (x, y)（逻辑像素，箭头尖）画一枚标准光标箭头：白身黑边。"""
    draw = ImageDraw.Draw(img)
    pts = [(x + px * k, y + py * k) for px, py in CURSOR_SHAPE]
    draw.polygon(pts, fill=(250, 250, 250), outline=(20, 20, 20), width=2)


def draw_cross(img: Image.Image, x: float, y: float, r: int = 16) -> None:
    """点位十字：黑底黄芯，明暗画面上都读得出。"""
    draw = ImageDraw.Draw(img)
    draw.line((x - r, y, x + r, y), fill=(15, 15, 15), width=7)
    draw.line((x, y - r, x, y + r), fill=(15, 15, 15), width=7)
    draw.line((x - r, y, x + r, y), fill=(255, 214, 64), width=3)
    draw.line((x, y - r, x, y + r), fill=(255, 214, 64), width=3)


def outline_rect(img: Image.Image, box: tuple, color=(255, 214, 64)) -> None:
    """矩形描边（先深后亮双描，保证浅色画面上也清楚）。"""
    draw = ImageDraw.Draw(img)
    draw.rectangle(box, outline=(15, 15, 15), width=6)
    draw.rectangle(box, outline=color, width=3)


def outline_ellipse(img: Image.Image, box: tuple, color=(255, 214, 64)) -> None:
    draw = ImageDraw.Draw(img)
    draw.ellipse(box, outline=(15, 15, 15), width=6)
    draw.ellipse(box, outline=color, width=3)


def outline_quad(img: Image.Image, pts: list, color=(255, 214, 64)) -> None:
    draw = ImageDraw.Draw(img)
    draw.polygon(pts, outline=(15, 15, 15), width=6)
    draw.polygon(pts, outline=color, width=3)


# ---------------------------------------------------------------- 运行截图
# 客户区比例坐标沿用工单 §3 的实测点位。

GONG = (0.50, 0.345)     # 鎏金锣顶部环带（listing-25-01 布景，多章共用）
CUP = (0.285, 0.405)     # 琉璃盏球面偏左上（法线明显朝斜上方）
BOX = (0.71, 0.47)       # 剔红漆盒正面
SKY = (0.50, 0.10)       # 空处（天上）


def fig_02_three_wares() -> None:
    """Figure 25-2：验货台开张——三件货一字排开（listing-25-01）。"""
    with launch("listing-25-01") as ex:
        shot = logical(ex.shot(3.5))
    save_png(shot, "fig-25-02-three-wares.png")


def fig_03_hover_highlight() -> None:
    """Figure 25-3：悬停锣高亮 vs 移开还原（listing-25-03，两联）。"""
    with launch("listing-25-03") as ex:
        ex.wait_until(2.6)
        park(ex.hwnd, *GONG)          # Over：高亮漆上身
        hover = logical(ex.shot(3.6))
        park(ex.hwnd, *SKY)           # Out：原漆还原
        away = logical(ex.shot(4.8))
    draw_cursor(hover, GONG[0] * 1280, GONG[1] * 720)
    draw_cursor(away, SKY[0] * 1280, SKY[1] * 720)
    save_png(
        hstack_below([shrink(hover), shrink(away)],
                     ["光标进锣的地界——Over 换上高亮漆",
                      "光标移开——Out 还原金铜原漆"]),
        "fig-25-03-hover-highlight.png",
    )


def fig_04_hit_gizmo() -> None:
    """Figure 25-4：命中点红球 + 法线粉箭，贴着琉璃盏球面（listing-25-04）。"""
    with launch("listing-25-04") as ex:
        ex.wait_until(2.6)
        park(ex.hwnd, *CUP)
        shot = logical(ex.shot(3.8))
    # 裁到琉璃盏近景：整幅里 8px 的小红球看不清，裁片放大到能读
    piece = shot.crop((150, 130, 630, 490)).resize((960, 720), Image.LANCZOS)
    save_png(piece, "fig-25-04-hit-gizmo.png")


# 纱幕四象限（listing-25-07）：纱幕在镜头与锣之间，点位打锣顶环带（射线
# 先撞纱幕）。画面本身不换色，谁应声用描边标注（同 fig-25-10 的手法）。
VEIL_QUAD = [(174, 149), (1106, 149), (1041, 562), (239, 562)]  # 纱幕投影四角
GONG_BOX = (545, 231, 733, 419)                                  # 锣的外接圆（外半径 0.72）


def fig_06_veil_modes() -> None:
    """Figure 25-6：档 1 点纱幕应声 vs 档 2 穿幕点锣（listing-25-07，两联）。"""
    with launch("listing-25-07") as ex:
        ex.wait_until(2.8)
        click_at(ex.hwnd, *GONG)      # 第 1 档守门：纱幕收下这一点
        guard = logical(ex.shot(3.6))
        ex.wait_until(4.2)
        tap("2")                      # 换第 2 档：隐身（IGNORE）
        ex.wait_until(4.7)
        click_at(ex.hwnd, *GONG)      # 同一点：穿幕，锣应声
        ghost = logical(ex.shot(5.5))
    draw_cursor(guard, GONG[0] * 1280, GONG[1] * 720)
    draw_cursor(ghost, GONG[0] * 1280, GONG[1] * 720)
    outline_quad(guard, VEIL_QUAD)    # 应声者描边：纱幕
    outline_ellipse(ghost, GONG_BOX)  # 应声者描边：鎏金锣
    save_png(
        hstack_below([shrink(guard), shrink(ghost)],
                     ["第 1 档守门：点纱幕，纱幕应声",
                      "第 2 档隐身（IGNORE）：同一点穿幕，锣应声"]),
        "fig-25-06-veil-modes.png",
    )


def fig_07_drag_feel() -> None:
    """Figure 25-7：跟手档往左上拖 vs 生搬档往下坠（listing-25-08，两联）。"""
    # 左联：默认跟手档，漆盒拖向画面左上
    with launch("listing-25-08") as ex:
        ex.wait_until(2.8)
        drag_abs(ex.hwnd, BOX, (0.40, 0.22))
        follow = logical(ex.shot(5.2))
    draw_cursor(follow, 0.40 * 1280, 0.22 * 720)
    # 右联：R 切生搬档，同方向往左上拖——货反而往下坠（工单实测手势 -141,-122）
    with launch("listing-25-08") as ex:
        ex.wait_until(2.6)
        tap("R")
        ex.wait_until(3.0)
        drag_abs(ex.hwnd, BOX, (0.60, 0.30))
        raw = logical(ex.shot(5.2))
    draw_cursor(raw, 0.60 * 1280, 0.30 * 720)
    save_png(
        hstack_below([shrink(follow), shrink(raw)],
                     ["跟手档：往左上拖，漆盒随手走",
                      "生搬档：同样往左上拖，漆盒往下坠"]),
        "fig-25-07-drag-feel.png",
    )


def fig_08_drop_done() -> None:
    """Figure 25-8：琉璃盏拖进托盘成交——吸附正中（listing-25-09）。"""
    with launch("listing-25-09") as ex:
        ex.wait_until(2.8)
        # 盏在 (-2.4, 1, 0)，托盘在画面右侧——拖到托盘上空撒手
        drag_abs(ex.hwnd, (0.255, 0.465), (0.784, 0.576), steps=24)
        shot = logical(ex.shot(6.0))
    save_png(shot, "fig-25-08-drop-done.png")


# sprite 场（listing-25-11）：阿燕 32×40×10 倍立 (-80,0)，灯笼 16×16×10 倍吊
# (40,60)。重叠区 = 阿燕包围盒右上透明角、恰是灯笼灯体（工单实测点位）。
OVERLAP = (0.53125, 0.4167)          # 逻辑 (680, 300)
AYAN_BOX = (400, 160, 720, 560)      # 阿燕包围盒（世界几何换算）
LANTERN_BOX = (600, 220, 760, 380)   # 灯笼包围盒


def fig_10_alpha_vs_bbox() -> None:
    """Figure 25-10：同一点，认像素报灯笼 vs 认框报阿燕（listing-25-11，两联）。"""
    with launch("listing-25-11") as ex:
        ex.wait_until(2.6)
        tap("P")                      # 先挂牌：sprite 不挂 Pickable 不参赛
        ex.wait_until(3.0)
        click_at(ex.hwnd, *OVERLAP)   # 认像素：阿燕透明角让路，灯笼应声
        alpha = logical(ex.shot(3.8))
        ex.wait_until(4.2)
        tap("M")                      # 换认框
        ex.wait_until(4.6)
        click_at(ex.hwnd, *OVERLAP)   # 认框：前排包围盒说了算，阿燕应声
        bbox = logical(ex.shot(5.4))
    for img in (alpha, bbox):
        draw_cross(img, OVERLAP[0] * 1280, OVERLAP[1] * 720)
    outline_rect(alpha, LANTERN_BOX)  # 应声者描边：灯笼
    outline_rect(bbox, AYAN_BOX)      # 应声者描边：阿燕
    save_png(
        hstack_below([shrink(alpha), shrink(bbox)],
                     ["认像素（AlphaThreshold 0.1）：透明角让路，灯笼应声",
                      "认框（BoundingBox）：包围盒说了算，阿燕应声"]),
        "fig-25-10-alpha-vs-bbox.png",
    )


def fig_11_ui_sign() -> None:
    """Figure 25-11：金色悬停态的「上手验货」字牌压着琉璃盏（listing-25-12）。"""
    with launch("listing-25-12") as ex:
        ex.wait_until(2.6)
        park(ex.hwnd, 0.29, 0.435)    # 悬停牌面：白字换金黄（工单点位）
        shot = logical(ex.shot(3.8))
    save_png(shot, "fig-25-11-ui-sign.png")


def fig_12_free_look() -> None:
    """Figure 25-12：FreeCamera 开场机位 vs 右键拖后右转（listing-25-13，两联）。"""
    with launch("listing-25-13") as ex:
        park(ex.hwnd, 0.5, 0.5)
        before = logical(ex.shot(3.0))
        ex.wait_until(3.5)
        drag_right_relative(280, 0)   # 右键抓取 + raw 位移右拖：镜头右转 ~20°
        after = logical(ex.shot(5.8))
    save_png(
        hstack_below([shrink(before), shrink(after)],
                     ["开场机位：正对三件货",
                      "右键按住向右拖：镜头右转，货移向画面左侧"]),
        "fig-25-12-free-look.png",
    )


def fig_13_pan_street() -> None:
    """Figure 25-13：PanCamera 长街开场 vs 右移拉近（listing-25-14，两联）。"""
    with launch("listing-25-14") as ex:
        before = logical(ex.shot(3.0))
        ex.wait_until(3.5)
        park(ex.hwnd, 0.5, 0.5)       # 滚轮要落在窗口内才进拾取管线
        key_down("D")                 # 键盘右移半秒：机位 x ≈ 350
        ex.wait_until(4.0)
        key_up("D")
        ex.wait_until(4.3)
        wheel(3)                      # 滚轮三格：zoom_factor 1.0 → 0.70（拉近）
        after = logical(ex.shot(5.2))
    save_png(
        hstack_below([shrink(before), shrink(after)],
                     ["开场机位：五盏灯笼横排，阿燕居中",
                      "D 右移＋滚轮拉近：街景左移放大，剩三盏灯笼"]),
        "fig-25-13-pan-street.png",
    )


def find_gong_grip(img: Image.Image) -> tuple[float, float]:
    """按金铜色实测鎏金锣顶部环带的抓点（逻辑坐标）。

    转台一转，预置坐标全部作废；按颜色实测才稳。质心的 y 会被上半环的
    高光带偏，不能直接当环心用——取质心 x 那一列附近的金色上沿，往下
    进 28px 正落在顶部环带实体上（环带屏幕厚度约 55px）。
    """
    small = img.resize((320, 180))
    px = small.load()
    golden = [
        (x, y)
        for y in range(180)
        for x in range(320)
        if (lambda c: c[0] > 140 and c[1] > 105 and c[2] < 150
            and c[0] - c[2] > 45 and c[1] - c[2] > 20)(px[x, y])
    ]
    if len(golden) < 25:
        raise RuntimeError("帧里找不到鎏金锣的金色像素")
    cx = sum(x for x, _ in golden) / len(golden)
    top = min(y for x, y in golden if abs(x - cx) <= 4)
    return cx * 4, top * 4 + 28


def fig_14_grand_inspection() -> None:
    """Figure 25-14：《上手验货》——转台视角 + 拖着鎏金锣移向托盘（main）。

    剧本：拖空处把转台转 ~20°，按色抓帧实测锣的新位置，悬停顶部环带
    （高亮），按住往托盘方向拖，中途按住不放截一帧，再松手收尾。
    """
    with launch("main") as ex:
        ex.wait_until(2.8)
        # 拖空处（天上）往左 45px：窗口 Drag 观察者收下，yaw += 0.36（~20°）
        drag_abs(ex.hwnd, (0.560, 0.14), (0.525, 0.14), steps=10, settle=0.3)
        ex.wait_until(4.4)
        # 转台后按色实测锣顶环带的抓点（中孔点不中，环心也不可靠）
        gx, gy = find_gong_grip(logical(ex.grab()))
        grip = (gx / 1280, gy / 720)
        park(ex.hwnd, *grip)          # 悬停：高亮漆上身
        ex.wait_until(5.4)
        # 按住往托盘（右下方）拖到半路，按住不放截一帧——拖拽进行时
        dest = (grip[0] + 0.13, grip[1] + 0.09)
        drag_abs(ex.hwnd, grip, dest, steps=10, settle=0.35, release=False)
        mid = logical(ex.grab())
        mouse_up()
    draw_cursor(mid, dest[0] * 1280, dest[1] * 720)
    save_png(mid, "fig-25-14-grand-inspection.png")


# ---------------------------------------------------------------- 手绘 SVG
# 内容即代码：落盘即重建（插图规范）。浅色卡片底，明暗主题均可读。

SVG_01_PIPELINE = """<svg viewBox="0 0 1000 500" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr25a" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#7a7468"/>
    </marker>
    <marker id="arr25b" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto">
      <path d="M0,0 L9,5 L0,10 z" fill="#b8862e"/>
    </marker>
    <marker id="arr25c" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#274a91"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="1000" height="500" rx="10" fill="#f7f5f0"/>
  <text x="500" y="36" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">拾取流水线：四段一条龙</text>

  <!-- ============ ① 指针 ============ -->
  <rect x="30" y="66" width="205" height="356" rx="10" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
  <text x="132" y="96" text-anchor="middle" font-size="14.5" fill="#4a463f" font-weight="bold">① 指针 pointer</text>
  <text x="132" y="115" text-anchor="middle" font-size="11" fill="#7a7468">鼠标、触摸、手写笔归一</text>

  <!-- 鼠标 -->
  <rect x="62" y="140" width="38" height="56" rx="18" fill="#f0ece2" stroke="#4a463f" stroke-width="2"/>
  <line x1="81" y1="140" x2="81" y2="166" stroke="#4a463f" stroke-width="1.6"/>
  <rect x="77.5" y="148" width="7" height="12" rx="3.5" fill="#4a463f"/>
  <!-- 触摸手指：指尖圆 + 指身，指尖两圈涟漪 -->
  <path d="M162,196 L162,158 q0,-9 9,-9 q9,0 9,9 L180,196" fill="#f0ece2" stroke="#4a463f" stroke-width="2"/>
  <path d="M156,150 a15,15 0 0 1 30,0" fill="none" stroke="#7a7468" stroke-width="1.6"/>
  <path d="M148,148 a23,23 0 0 1 46,0" fill="none" stroke="#c9c2b2" stroke-width="1.6"/>

  <path d="M81,206 L120,238" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr25a)"/>
  <path d="M171,206 L144,238" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr25a)"/>
  <rect x="52" y="246" width="160" height="30" rx="8" fill="#efe9db" stroke="#b8862e" stroke-width="1.6"/>
  <text x="132" y="266" text-anchor="middle" font-size="12.5" fill="#8a6a1e" font-family="Consolas, monospace">PointerLocation</text>
  <text x="132" y="310" text-anchor="middle" font-size="11" fill="#7a7468">每帧更新：这枚指针</text>
  <text x="132" y="327" text-anchor="middle" font-size="11" fill="#7a7468">落在屏幕的哪一点</text>
  <text x="132" y="396" text-anchor="middle" font-size="10.5" fill="#9a9280">手柄也能自造一枚虚拟指针</text>

  <path d="M239,244 L266,244" stroke="#b8862e" stroke-width="4" fill="none" marker-end="url(#arr25b)"/>

  <!-- ============ ② 后端 ============ -->
  <rect x="270" y="66" width="205" height="356" rx="10" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
  <text x="372" y="96" text-anchor="middle" font-size="14.5" fill="#4a463f" font-weight="bold">② 后端 backend</text>
  <text x="372" y="115" text-anchor="middle" font-size="11" fill="#7a7468">三家车间同时开工，各自报账</text>

  <!-- mesh 车间：射线打中球 -->
  <rect x="284" y="136" width="104" height="48" rx="8" fill="#f0ece2" stroke="#7a7468" stroke-width="1.2"/>
  <line x1="292" y1="176" x2="318" y2="150" stroke="#274a91" stroke-width="1.8"/>
  <circle cx="322" cy="147" r="7" fill="none" stroke="#274a91" stroke-width="1.8"/>
  <text x="340" y="156" font-size="11.5" fill="#4a463f">mesh</text>
  <text x="340" y="172" font-size="10.5" fill="#7a7468">放射线</text>
  <path d="M392,160 L402,160" stroke="#7a7468" stroke-width="1.8" fill="none" marker-end="url(#arr25a)"/>
  <rect x="405" y="146" width="62" height="28" rx="6" fill="#efe9db" stroke="#b8862e" stroke-width="1.2"/>
  <text x="436" y="164" text-anchor="middle" font-size="8" fill="#8a6a1e" font-family="Consolas, monospace">PointerHits</text>

  <!-- sprite 车间：像素格 -->
  <rect x="284" y="206" width="104" height="48" rx="8" fill="#f0ece2" stroke="#7a7468" stroke-width="1.2"/>
  <g stroke="#7a7468" stroke-width="1">
    <rect x="296" y="216" width="27" height="27" fill="none"/>
    <line x1="305" y1="216" x2="305" y2="243"/><line x1="314" y1="216" x2="314" y2="243"/>
    <line x1="296" y1="225" x2="323" y2="225"/><line x1="296" y1="234" x2="323" y2="234"/>
  </g>
  <rect x="305" y="225" width="9" height="9" fill="#274a91"/>
  <text x="340" y="226" font-size="11.5" fill="#4a463f">sprite</text>
  <text x="340" y="242" font-size="10.5" fill="#7a7468">查像素</text>
  <path d="M392,230 L402,230" stroke="#7a7468" stroke-width="1.8" fill="none" marker-end="url(#arr25a)"/>
  <rect x="405" y="216" width="62" height="28" rx="6" fill="#efe9db" stroke="#b8862e" stroke-width="1.2"/>
  <text x="436" y="234" text-anchor="middle" font-size="8" fill="#8a6a1e" font-family="Consolas, monospace">PointerHits</text>

  <!-- UI 车间：嵌套节点 -->
  <rect x="284" y="276" width="104" height="48" rx="8" fill="#f0ece2" stroke="#7a7468" stroke-width="1.2"/>
  <rect x="294" y="286" width="30" height="28" rx="3" fill="none" stroke="#7a7468" stroke-width="1.4"/>
  <rect x="300" y="292" width="18" height="10" rx="2" fill="none" stroke="#274a91" stroke-width="1.4"/>
  <text x="340" y="296" font-size="11.5" fill="#4a463f">UI</text>
  <text x="340" y="312" font-size="10.5" fill="#7a7468">查节点</text>
  <path d="M392,300 L402,300" stroke="#7a7468" stroke-width="1.8" fill="none" marker-end="url(#arr25a)"/>
  <rect x="405" y="286" width="62" height="28" rx="6" fill="#efe9db" stroke="#b8862e" stroke-width="1.2"/>
  <text x="436" y="304" text-anchor="middle" font-size="8" fill="#8a6a1e" font-family="Consolas, monospace">PointerHits</text>

  <text x="372" y="366" text-anchor="middle" font-size="11" fill="#7a7468">报的都是 PointerHits：</text>
  <text x="372" y="383" text-anchor="middle" font-size="11" fill="#7a7468">打到了谁、离相机多深</text>

  <path d="M479,244 L506,244" stroke="#b8862e" stroke-width="4" fill="none" marker-end="url(#arr25b)"/>

  <!-- ============ ③ 悬停 ============ -->
  <rect x="510" y="66" width="205" height="356" rx="10" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
  <text x="612" y="96" text-anchor="middle" font-size="14.5" fill="#4a463f" font-weight="bold">③ 悬停 hover</text>
  <text x="612" y="115" text-anchor="middle" font-size="11" fill="#7a7468">各家命中并到一处裁决</text>

  <!-- 三枚命中片落进漏斗 -->
  <rect x="540" y="128" width="30" height="14" rx="4" fill="#efe9db" stroke="#b8862e" stroke-width="1"/>
  <rect x="597" y="124" width="30" height="14" rx="4" fill="#efe9db" stroke="#b8862e" stroke-width="1"/>
  <rect x="654" y="128" width="30" height="14" rx="4" fill="#efe9db" stroke="#b8862e" stroke-width="1"/>
  <polygon points="530,154 694,154 640,268 584,268" fill="#e9e3d6" stroke="#7a7468" stroke-width="1.6"/>
  <text x="612" y="192" text-anchor="middle" font-size="11.5" fill="#4a463f">按深度排序</text>
  <text x="612" y="212" text-anchor="middle" font-size="11.5" fill="#4a463f">被挡的筛掉</text>
  <text x="612" y="232" text-anchor="middle" font-size="10" fill="#7a7468">（Pickable 的规矩）</text>
  <path d="M612,268 L612,290" stroke="#7a7468" stroke-width="2" fill="none" marker-end="url(#arr25a)"/>
  <rect x="566" y="296" width="92" height="30" rx="8" fill="#efe9db" stroke="#b8862e" stroke-width="1.6"/>
  <text x="612" y="316" text-anchor="middle" font-size="12.5" fill="#8a6a1e" font-family="Consolas, monospace">HoverMap</text>
  <text x="612" y="360" text-anchor="middle" font-size="11" fill="#7a7468">「此刻指针真正悬停在</text>
  <text x="612" y="377" text-anchor="middle" font-size="11" fill="#7a7468">谁身上」的权威名单</text>

  <path d="M719,244 L746,244" stroke="#b8862e" stroke-width="4" fill="none" marker-end="url(#arr25b)"/>

  <!-- ============ ④ 事件 ============ -->
  <rect x="750" y="66" width="220" height="356" rx="10" fill="#ffffff" stroke="#c9c2b2" stroke-width="1.4"/>
  <text x="860" y="96" text-anchor="middle" font-size="14.5" fill="#4a463f" font-weight="bold">④ 事件 events</text>
  <text x="860" y="115" text-anchor="middle" font-size="11" fill="#7a7468">名单变化＋按键 → 高层事件</text>

  <!-- 父子树：根在上 -->
  <circle cx="912" cy="150" r="12" fill="#f0ece2" stroke="#4a463f" stroke-width="1.8"/>
  <circle cx="878" cy="222" r="12" fill="#f0ece2" stroke="#4a463f" stroke-width="1.8"/>
  <circle cx="944" cy="222" r="12" fill="#f0ece2" stroke="#4a463f" stroke-width="1.8"/>
  <circle cx="878" cy="294" r="12" fill="#ffe9b0" stroke="#b8862e" stroke-width="2.2"/>
  <line x1="905" y1="161" x2="884" y2="212" stroke="#7a7468" stroke-width="1.6"/>
  <line x1="919" y1="161" x2="938" y2="212" stroke="#7a7468" stroke-width="1.6"/>
  <line x1="878" y1="234" x2="878" y2="282" stroke="#7a7468" stroke-width="1.6"/>
  <text x="878" y="322" text-anchor="middle" font-size="10" fill="#8a6a1e">目标实体</text>
  <!-- 信封沿树向上飘 -->
  <path d="M872,286 C 858,262 862,240 872,230 M878,222 C 868,200 884,170 900,156"
        stroke="#b8862e" stroke-width="2" fill="none" stroke-dasharray="5 4" marker-end="url(#arr25b)"/>
  <g>
    <rect x="774" y="270" width="52" height="32" rx="4" fill="#ffffff" stroke="#b8862e" stroke-width="1.6" transform="rotate(-8 800 286)"/>
    <path d="M776,274 L800,290 L824,274" stroke="#b8862e" stroke-width="1.3" fill="none" transform="rotate(-8 800 286)"/>
    <text x="800" y="298" text-anchor="middle" font-size="9.5" fill="#8a6a1e" font-family="Consolas, monospace" transform="rotate(-8 800 286)">Drag</text>
  </g>
  <g>
    <rect x="768" y="216" width="52" height="32" rx="4" fill="#ffffff" stroke="#b8862e" stroke-width="1.6" transform="rotate(6 794 232)"/>
    <path d="M770,220 L794,236 L818,220" stroke="#b8862e" stroke-width="1.3" fill="none" transform="rotate(6 794 232)"/>
    <text x="794" y="244" text-anchor="middle" font-size="9.5" fill="#8a6a1e" font-family="Consolas, monospace" transform="rotate(6 794 232)">Click</text>
  </g>
  <g>
    <rect x="774" y="162" width="52" height="32" rx="4" fill="#ffffff" stroke="#b8862e" stroke-width="1.6" transform="rotate(-6 800 178)"/>
    <path d="M776,166 L800,182 L824,166" stroke="#b8862e" stroke-width="1.3" fill="none" transform="rotate(-6 800 178)"/>
    <text x="800" y="190" text-anchor="middle" font-size="9.5" fill="#8a6a1e" font-family="Consolas, monospace" transform="rotate(-6 800 178)">Over</text>
  </g>
  <text x="860" y="348" text-anchor="middle" font-size="11" fill="#7a7468">派发到目标实体门上，</text>
  <text x="860" y="365" text-anchor="middle" font-size="11" fill="#7a7468">再沿父子树一路向上冒泡</text>
  <text x="860" y="396" text-anchor="middle" font-size="10.5" fill="#9a9280">Over / Out / Click / Drag / …</text>

  <text x="500" y="464" text-anchor="middle" font-size="11.5" fill="#7a7468">四段全部跑在 PreUpdate——你的 Update 系统看到的，永远是「本帧拾取已处理完」的世界</text>
</svg>
"""

SVG_05_BUBBLING = """<svg viewBox="0 0 900 540" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr25g" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto">
      <path d="M0,0 L9,5 L0,10 z" fill="#b8862e"/>
    </marker>
    <marker id="arr25r" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto">
      <path d="M0,0 L9,5 L0,10 z" fill="#b3402e"/>
    </marker>
    <marker id="arr25d" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#7a7468"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="900" height="540" rx="10" fill="#f7f5f0"/>
  <text x="450" y="36" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">账单的走向：冒泡、拦截与空处直达</text>

  <!-- ============ 实体树 ============ -->
  <!-- 台口（窗口实体，终点站） -->
  <rect x="290" y="76" width="220" height="50" rx="10" fill="#ffffff" stroke="#4a463f" stroke-width="2"/>
  <text x="400" y="102" text-anchor="middle" font-size="14.5" fill="#4a463f" font-weight="bold">台口</text>
  <text x="400" y="118" text-anchor="middle" font-size="10.5" fill="#7a7468">窗口实体——冒泡终点站</text>

  <!-- 台口—货架：不是 ChildOf，是传播规则的「跳窗口」一步 -->
  <line x1="400" y1="126" x2="400" y2="216" stroke="#7a7468" stroke-width="1.8" stroke-dasharray="6 4"/>
  <text x="413" y="160" font-size="10.5" fill="#7a7468">父链到头，</text>
  <text x="413" y="176" font-size="10.5" fill="#7a7468">跳到窗口实体</text>

  <!-- 货架 -->
  <rect x="290" y="216" width="220" height="50" rx="10" fill="#ffffff" stroke="#4a463f" stroke-width="2"/>
  <text x="400" y="242" text-anchor="middle" font-size="14.5" fill="#4a463f" font-weight="bold">货架</text>
  <text x="400" y="258" text-anchor="middle" font-size="10.5" fill="#7a7468">父实体——总账挂这儿</text>

  <!-- ChildOf 两支 -->
  <line x1="352" y1="266" x2="268" y2="366" stroke="#7a7468" stroke-width="1.8"/>
  <line x1="448" y1="266" x2="532" y2="366" stroke="#7a7468" stroke-width="1.8"/>
  <text x="252" y="322" font-size="10" fill="#9a9280" font-family="Consolas, monospace">ChildOf</text>
  <text x="516" y="352" font-size="10" fill="#9a9280" font-family="Consolas, monospace">ChildOf</text>

  <!-- 鎏金锣 / 琉璃盏 -->
  <rect x="160" y="366" width="180" height="50" rx="10" fill="#ffffff" stroke="#4a463f" stroke-width="2"/>
  <text x="250" y="392" text-anchor="middle" font-size="14.5" fill="#4a463f" font-weight="bold">鎏金锣</text>
  <text x="250" y="408" text-anchor="middle" font-size="10.5" fill="#7a7468">照单全记，账单继续上行</text>
  <rect x="460" y="366" width="180" height="50" rx="10" fill="#ffffff" stroke="#4a463f" stroke-width="2"/>
  <text x="550" y="392" text-anchor="middle" font-size="14.5" fill="#4a463f" font-weight="bold">琉璃盏</text>
  <text x="550" y="408" text-anchor="middle" font-size="10.5" fill="#7a7468">易碎——这笔账到我为止</text>

  <!-- ============ 走向一：点锣，冒泡三连 ============ -->
  <path d="M212,478 L232,424" stroke="#b8862e" stroke-width="3" fill="none" marker-end="url(#arr25g)"/>
  <text x="204" y="500" text-anchor="middle" font-size="12" fill="#8a6a1e" font-weight="bold">点鎏金锣</text>
  <path d="M282,362 C 300,330 322,296 344,272" stroke="#b8862e" stroke-width="3" fill="none" marker-end="url(#arr25g)"/>
  <path d="M372,212 C 366,186 366,160 376,132" stroke="#b8862e" stroke-width="3" fill="none" marker-end="url(#arr25g)"/>
  <!-- 三站的「敲响」标记 + 序号 -->
  <g stroke="#b8862e" stroke-width="2" fill="none">
    <path d="M168,362 L160,352"/><path d="M178,358 L174,346"/><path d="M160,374 L148,370"/>
    <path d="M298,212 L290,202"/><path d="M308,208 L304,196"/><path d="M290,224 L278,220"/>
    <path d="M298,72 L290,62"/><path d="M308,68 L304,56"/><path d="M290,84 L278,80"/>
  </g>
  <circle cx="148" cy="391" r="12" fill="#b8862e"/>
  <text x="148" y="396" text-anchor="middle" font-size="12" fill="#ffffff" font-weight="bold">1</text>
  <circle cx="278" cy="241" r="12" fill="#b8862e"/>
  <text x="278" y="246" text-anchor="middle" font-size="12" fill="#ffffff" font-weight="bold">2</text>
  <circle cx="278" cy="101" r="12" fill="#b8862e"/>
  <text x="278" y="106" text-anchor="middle" font-size="12" fill="#ffffff" font-weight="bold">3</text>

  <!-- ============ 走向二：点盏，propagate(false) 拦截 ============ -->
  <path d="M588,478 L568,424" stroke="#b3402e" stroke-width="3" fill="none" marker-end="url(#arr25r)"/>
  <text x="598" y="500" text-anchor="middle" font-size="12" fill="#b3402e" font-weight="bold">点琉璃盏</text>
  <path d="M518,362 C 508,346 500,334 492,324" stroke="#b3402e" stroke-width="3" fill="none" marker-end="url(#arr25r)"/>
  <!-- 红色停止牌（八角） -->
  <polygon points="473,276 491,276 504,289 504,307 491,320 473,320 460,307 460,289"
           fill="#b3402e" stroke="#ffffff" stroke-width="2.5"/>
  <text x="482" y="304" text-anchor="middle" font-size="13" fill="#ffffff" font-weight="bold">停</text>
  <text x="524" y="290" font-size="11.5" fill="#b3402e" font-family="Consolas, monospace">propagate(false)</text>
  <text x="524" y="307" font-size="10.5" fill="#7a7468">账单止步，货架与台口不知情</text>

  <!-- ============ 走向三：点空处，直达台口 ============ -->
  <circle cx="760" cy="390" r="17" fill="none" stroke="#7a7468" stroke-width="1.8" stroke-dasharray="4 3"/>
  <path d="M756,382 L756,396 L760,392 L763,398 L766,396 L763,391 L768,391 z" fill="#4a463f"/>
  <text x="760" y="428" text-anchor="middle" font-size="12" fill="#4a463f" font-weight="bold">点空处</text>
  <path d="M760,372 C 764,240 660,120 516,98" stroke="#7a7468" stroke-width="2.4" fill="none" stroke-dasharray="7 5" marker-end="url(#arr25d)"/>
  <text x="756" y="212" font-size="10.5" fill="#7a7468">窗口后端的直接命中</text>
  <text x="756" y="228" font-size="10.5" fill="#7a7468">——不是冒泡</text>

  <text x="450" y="524" text-anchor="middle" font-size="11.5" fill="#7a7468">冒泡途中 entity 随站更新（锣→货架→台口），original_event_target() 一路不变（起头都是鎏金锣）</text>
</svg>
"""

SVG_09_SKEWER = """<svg viewBox="0 0 980 470" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr25s" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto">
      <path d="M0,0 L9,5 L0,10 z" fill="#274a91"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="980" height="470" rx="10" fill="#f7f5f0"/>
  <text x="490" y="36" text-anchor="middle" font-size="17" fill="#4a463f" font-weight="bold">一线串糖葫芦（侧视剖面）：按距离排序的完整命中清单</text>

  <!-- 台面：粗线 + 下方浅影 -->
  <rect x="200" y="332" width="740" height="14" fill="#e6e0d2"/>
  <line x1="200" y1="332" x2="940" y2="332" stroke="#4a463f" stroke-width="3"/>
  <text x="912" y="366" text-anchor="end" font-size="12" fill="#4a463f">台面</text>

  <!-- 相机 -->
  <rect x="96" y="86" width="54" height="42" rx="7" fill="#ffffff" stroke="#4a463f" stroke-width="2"/>
  <circle cx="112" cy="100" r="6" fill="none" stroke="#4a463f" stroke-width="1.6"/>
  <polygon points="150,96 178,104 178,118 150,122" fill="#ffffff" stroke="#4a463f" stroke-width="2"/>
  <text x="123" y="74" text-anchor="middle" font-size="12" fill="#4a463f">相机</text>
  <text x="123" y="150" text-anchor="middle" font-size="10.5" fill="#7a7468">viewport_to_world</text>
  <text x="123" y="166" text-anchor="middle" font-size="10.5" fill="#7a7468">反算出 Ray3d</text>

  <!-- 射线：虚线，扎进台面 -->
  <line x1="176" y1="117" x2="833" y2="323" stroke="#274a91" stroke-width="2.4" stroke-dasharray="9 6" marker-end="url(#arr25s)"/>

  <!-- 琉璃盏：半透明球 -->
  <circle cx="532" cy="228" r="44" fill="#bfe0dd" fill-opacity="0.55" stroke="#5a8a86" stroke-width="2"/>
  <text x="532" y="170" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">琉璃盏</text>

  <!-- 鎏金锣：立环側影（下缘半埋进台面，与实测场景一致） -->
  <rect x="639" y="230" width="42" height="120" rx="21" fill="#f3e2b8" stroke="#8a6a1e" stroke-width="2"/>
  <rect x="651" y="262" width="18" height="56" rx="9" fill="#f7f5f0" stroke="#8a6a1e" stroke-width="1.4"/>
  <rect x="637" y="334" width="46" height="18" fill="#e6e0d2"/>
  <text x="660" y="212" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">鎏金锣</text>

  <!-- 备用漆盒：Hidden，虚线轮廓 -->
  <rect x="766" y="256" width="76" height="76" fill="none" stroke="#8a4a42" stroke-width="2" stroke-dasharray="7 5"/>
  <text x="826" y="240" text-anchor="middle" font-size="12.5" fill="#8a4a42" font-weight="bold">备用漆盒</text>
  <text x="856" y="382" text-anchor="middle" font-size="10.5" fill="#8a4a42">Visibility::Hidden</text>
  <text x="856" y="398" text-anchor="middle" font-size="10.5" fill="#8a4a42">——Any 档才串上</text>

  <!-- 四枚串珠 + 距离 -->
  <g fill="#c05a2e" stroke="#f7f5f0" stroke-width="2">
    <circle cx="490" cy="215" r="7"/>
    <circle cx="644" cy="263" r="7"/>
    <circle cx="766" cy="301" r="7"/>
    <circle cx="850" cy="328" r="7"/>
  </g>
  <g font-size="12.5" fill="#c05a2e" font-weight="bold" text-anchor="middle" stroke="#f7f5f0" stroke-width="4" paint-order="stroke">
    <text x="474" y="196">4.38 m</text>
    <text x="628" y="244">6.39 m</text>
    <text x="752" y="282">7.99 m</text>
    <text x="874" y="310">9.29 m</text>
  </g>

  <text x="490" y="446" text-anchor="middle" font-size="11.5" fill="#7a7468">cast_ray 不早退（early_exit_test 恒 false）时给出整串命中，从近到远排好序；VisibleInView 档会跳过隐身的漆盒</text>
</svg>
"""


def fig_01_pipeline_svg() -> None:
    """Figure 25-1：拾取流水线四段（手绘 SVG）。"""
    save_svg(SVG_01_PIPELINE, "fig-25-01-picking-pipeline.svg")


def fig_05_bubbling_svg() -> None:
    """Figure 25-5：实体树与四种账单走向（手绘 SVG）。"""
    save_svg(SVG_05_BUBBLING, "fig-25-05-bubbling.svg")


def fig_09_skewer_svg() -> None:
    """Figure 25-9：一线串糖葫芦侧视剖面（手绘 SVG）。"""
    save_svg(SVG_09_SKEWER, "fig-25-09-skewer.svg")


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_01_pipeline_svg,
    fig_02_three_wares,
    fig_03_hover_highlight,
    fig_04_hit_gizmo,
    fig_05_bubbling_svg,
    fig_06_veil_modes,
    fig_07_drag_feel,
    fig_08_drop_done,
    fig_09_skewer_svg,
    fig_10_alpha_vs_bbox,
    fig_11_ui_sign,
    fig_12_free_look,
    fig_13_pan_street,
    fig_14_grand_inspection,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    cargo("build", "-p", "ch25-picking", "--bins", "--examples")
    only = sys.argv[1] if len(sys.argv) > 1 else None
    for fig in ALL:
        if only and only not in fig.__name__:
            continue
        fig()
        time.sleep(0.5)


if __name__ == "__main__":
    main()

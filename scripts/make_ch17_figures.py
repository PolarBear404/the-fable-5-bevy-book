# -*- coding: utf-8 -*-
"""一键重建第 17 章全部运行截图与动图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch17_figures.py [图名筛选]

本章画面由实时输入驱动：脚本用 SendInput 发真实键鼠事件（扫描码键击、
相对位移、滚轮、绝对落点点击），窗口由 capture.Example 启动时置前。
前置：scripts/make_ch17_assets.py 已就位资产。产物输出到 book/src/images/ch17/。
"""

import ctypes
import os
import subprocess
import sys
import time
from ctypes import wintypes
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch17-input"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch17"

# 子进程（Bevy 示例）靠它定位 assets/
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
MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP = 0x0002, 0x0004
MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP = 0x0008, 0x0010
MOUSEEVENTF_WHEEL = 0x0800

SCAN = {"A": 0x1E, "D": 0x20, "G": 0x22, "SPACE": 0x39, "ESC": 0x01}


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
    time.sleep(0.05)
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
    """确保示例窗口在前台拿焦点——SendInput 的键击只进焦点窗口。

    SetForegroundWindow 有抢焦点限制：把自己挂到当前前台线程的输入队列上
    再请求（AttachThreadInput 技巧），不行就重试。
    """
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


def click_at(hwnd: int, fx: float, fy: float) -> None:
    """点窗口客户区的比例坐标 (fx, fy)。"""
    w, h = client_size(hwnd)
    point = wintypes.POINT(int(w * fx), int(h * fy))
    user32.ClientToScreen(hwnd, ctypes.byref(point))
    user32.SetCursorPos(point.x, point.y)
    time.sleep(0.05)
    _send(_mouse(flags=MOUSEEVENTF_LEFTDOWN))
    time.sleep(0.04)
    _send(_mouse(flags=MOUSEEVENTF_LEFTUP))


def park_cursor(hwnd: int, fx: float, fy: float) -> None:
    w, h = client_size(hwnd)
    point = wintypes.POINT(int(w * fx), int(h * fy))
    user32.ClientToScreen(hwnd, ctypes.byref(point))
    user32.SetCursorPos(point.x, point.y)


def drag_right_button(dx: int, dy: int, steps: int = 12) -> None:
    """按住右键做相对位移拖动（喂 MouseMotion）。"""
    _send(_mouse(flags=MOUSEEVENTF_RIGHTDOWN))
    time.sleep(0.05)
    for _ in range(steps):
        _send(_mouse(dx=dx // steps, dy=dy // steps, flags=MOUSEEVENTF_MOVE))
        time.sleep(0.02)
    time.sleep(0.05)
    _send(_mouse(flags=MOUSEEVENTF_RIGHTUP))


def wheel(notches: int) -> None:
    _send(_mouse(data=notches * 120, flags=MOUSEEVENTF_WHEEL))


# ---------------------------------------------------------------- 通用排版

def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch17-input.exe"
    return EXAMPLES / f"{name}.exe"


def label_bar(width: int, texts: list[str]) -> Image.Image:
    bar = Image.new("RGB", (width, LABEL_H), LABEL_BG)
    draw = ImageDraw.Draw(bar)
    cell = width / len(texts)
    for i, text in enumerate(texts):
        w = draw.textlength(text, font=FONT)
        draw.text((cell * i + (cell - w) / 2, 6), text, font=FONT, fill=LABEL_FG)
    return bar


def hstack(images: list[Image.Image], labels: list[str] | None = None) -> Image.Image:
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


def logical(img: Image.Image) -> Image.Image:
    """物理像素 → 1280×720 逻辑像素（125% 缩放下物理是 1600×900）。"""
    return img.resize((1280, 720), Image.LANCZOS)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def save_webp(frames: list[Image.Image], filename: str, fps: int, quality: int = 65) -> None:
    path = OUT / filename
    frames[0].save(
        path,
        save_all=True,
        append_images=frames[1:],
        duration=int(1000 / fps),
        loop=0,
        quality=quality,
        method=4,
    )
    kb = path.stat().st_size // 1024
    print(f"{filename}：{len(frames)} 帧，{kb} KB")
    if kb > 2000:
        print("  警告：超过 2 MB 上限，考虑降帧率/质量/裁切")


def record_scripted(
    ex: Example, schedule: list, start: float, dur: float, fps: int, size
) -> list[Image.Image]:
    """边录边演：schedule 为 [(t, 动作), ...]，t 以窗口出现为零点。"""
    frames = []
    queue = sorted(schedule, key=lambda item: item[0])
    qi = 0
    total = int(dur * fps)
    for i in range(total):
        t = start + i / fps
        while qi < len(queue) and queue[qi][0] <= t:
            queue[qi][1]()
            qi += 1
        ex.wait_until(t)
        frames.append(ex.grab().resize(size, Image.LANCZOS))
    return frames


# ---------------------------------------------------------------- 各图

def fig_01_keyboard_walk() -> None:
    """Figure 17-1：按住 D 一秒前后（Listing 17-1）。"""
    with Example(exe("listing-17-01"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        before = logical(ex.shot(2.0))
        key_down("D")
        ex.wait_until(3.0)
        key_up("D")
        after = logical(ex.shot(3.3))
    crop = (160, 250, 1120, 700)
    save_png(
        hstack(
            [before.crop(crop).resize((624, 293)), after.crop(crop).resize((624, 293))],
            ["开场：原地候命", "按住 KeyD 一秒后：右移 240 单位，面朝右"],
        ),
        "fig-17-01-keyboard-walk.png",
    )


def fig_04_click_to_dash() -> None:
    """Figure 17-4：左键插旗 → 阿燕跑到旗下（Listing 17-4）。"""
    with Example(exe("listing-17-04"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.0)
        click_at(ex.hwnd, 0.85, 0.50)
        planted = logical(ex.shot(2.45))
        arrived = logical(ex.shot(4.3))
    crop = (160, 250, 1180, 700)
    save_png(
        hstack(
            [planted.crop(crop).resize((624, 275)), arrived.crop(crop).resize((624, 275))],
            ["点击右侧台面：令旗插下，阿燕起步", "一秒多后：到位，旗收"],
        ),
        "fig-17-04-click-to-dash.png",
    )


def fig_05_crane() -> None:
    """Figure 17-5：摇臂三连——开机、滚轮拉远、右键拖向东头（Listing 17-5）。"""
    with Example(exe("listing-17-05"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        park_cursor(ex.hwnd, 0.5, 0.5)
        first = logical(ex.shot(2.0))
        wheel(-3)
        zoomed = logical(ex.shot(2.6))
        drag_right_button(-760, 0)
        dragged = logical(ex.shot(3.9))
    crop = (0, 180, 1280, 700)
    panels = [im.crop(crop).resize((416, 169)) for im in (first, zoomed, dragged)]
    save_png(
        hstack(panels, ["开机机位", "滚轮向后：拉远", "按住右键左拖：台面跟手往西"]),
        "fig-17-05-crane.png",
    )


def fig_08_booth() -> None:
    """Figure 17-8：体验场动图——键盘走位出剑，鼠标插旗，HUD 换听差（main）。"""
    with Example(exe("main"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        park_cursor(ex.hwnd, 0.5, 0.3)
        schedule = [
            (2.4, lambda: key_down("D")),
            (4.15, lambda: key_up("D")),
            (4.5, lambda: tap("SPACE")),
            (5.4, lambda: tap("SPACE")),
            (6.2, lambda: click_at(ex.hwnd, 0.15, 0.55)),
        ]
        frames = record_scripted(ex, schedule, start=2.0, dur=8.8, fps=8, size=(800, 450))
    save_webp(frames, "fig-17-08-booth.webp", fps=8, quality=68)


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_01_keyboard_walk,
    fig_04_click_to_dash,
    fig_05_crane,
    fig_08_booth,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch17-input", "--bins", "--examples"],
        cwd=CODE,
        check=True,
    )
    only = sys.argv[1] if len(sys.argv) > 1 else None
    for fig in ALL:
        if only and only not in fig.__name__:
            continue
        fig()
        time.sleep(0.5)


if __name__ == "__main__":
    main()

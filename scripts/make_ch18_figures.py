# -*- coding: utf-8 -*-
"""一键重建第 18 章全部运行截图与动图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch18_figures.py [图名筛选]

本章三张截图与一张动图：两面钟（中场对照）、袖箭冷却条、丢拍记分牌、
《赶月》直写 vs 插值动图。键盘事件用 SendInput 发真实扫描码键击（抄 ch17）。
前置：scripts/make_ch18_assets.py 已就位资产。产物输出到 book/src/images/ch18/。
"""

import ctypes
import os
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch18-time"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch18"

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


INPUT_KEYBOARD = 1
KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYEVENTF_EXTENDEDKEY = 0x2, 0x8, 0x1

SCAN = {"SPACE": 0x39, "LSHIFT": 0x2A, "1": 0x02, "2": 0x03, "UP": 0x48, "DOWN": 0x50}
EXTENDED = {"UP", "DOWN"}


def _send(*inputs: INPUT) -> None:
    array = (INPUT * len(inputs))(*inputs)
    if user32.SendInput(len(inputs), array, ctypes.sizeof(INPUT)) != len(inputs):
        raise RuntimeError("SendInput 未全部送达")


def _key(name: str, up: bool) -> INPUT:
    inp = INPUT()
    inp.type = INPUT_KEYBOARD
    flags = KEYEVENTF_SCANCODE | (KEYEVENTF_KEYUP if up else 0)
    if name in EXTENDED:
        flags |= KEYEVENTF_EXTENDEDKEY
    inp.union.ki = KEYBDINPUT(0, SCAN[name], flags, 0, None)
    return inp


def key_down(name: str) -> None:
    _send(_key(name, False))


def key_up(name: str) -> None:
    _send(_key(name, True))


def tap(name: str, hold: float = 0.05) -> None:
    key_down(name)
    time.sleep(hold)
    key_up(name)


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


# ---------------------------------------------------------------- 通用排版

def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch18-time.exe"
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
    """物理像素 → 1280×720 逻辑像素（DPI 缩放下物理分辨率会更大）。"""
    if img.size == (1280, 720):
        return img
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


# ---------------------------------------------------------------- 各图

def fig_02_two_clocks() -> None:
    """Figure 18-2：两面钟——开戏对表 vs 中场只停一面（Listing 18-3）。"""
    with Example(exe("listing-18-03"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        running = logical(ex.shot(3.0))
        ex.wait_until(3.2)
        tap("SPACE")  # 中场：戏台钟与阿燕定格，怀表照走
        paused = logical(ex.shot(6.4))
    crop = (0, 0, 1280, 680)
    save_png(
        hstack(
            [running.crop(crop).resize((624, 332)), paused.crop(crop).resize((624, 332))],
            ["开戏：两面钟几乎同步", "中场 3 秒后：怀表照走，戏台钟和阿燕定格"],
        ),
        "fig-18-02-two-clocks.png",
    )


def fig_04_cooldown() -> None:
    """Figure 18-4：冷却条——出手瞬间归零，0.8 秒长回满格（Listing 18-4）。"""
    with Example(exe("listing-18-04"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.0)
        tap("SPACE")
        fresh = logical(ex.shot(2.12))
        recovering = logical(ex.shot(2.62))

    def panel(img: Image.Image) -> Image.Image:
        """掐头去尾：HUD 条 + 台面两条横带竖拼，挤掉中间的空场。"""
        hud = img.crop((0, 16, 1280, 96))
        stage = img.crop((0, 360, 1280, 656))
        out = Image.new("RGB", (1280, hud.height + GAP + stage.height), GAP_COLOR)
        out.paste(hud, (0, 0))
        out.paste(stage, (0, hud.height + GAP))
        return out.resize((624, out.height * 624 // 1280), Image.LANCZOS)

    save_png(
        hstack(
            [panel(fresh), panel(recovering)],
            ["出手一瞬：冷却条清空，匣剩两支", "半秒后：冷却条快满，袖箭已近木桩"],
        ),
        "fig-18-04-cooldown.png",
    )


def fig_09_tally() -> None:
    """Figure 18-9：两本账对不上——慢板丢拍 vs 拖戏重复（Listing 18-8）。"""
    with Example(exe("listing-18-08"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(2.5)
        for _ in range(8):  # 慢板下快速点按八下
            tap("SPACE", hold=0.04)
            time.sleep(0.14)
        slow = logical(ex.shot(5.0))
        tap("2")  # 拖戏
        time.sleep(0.8)
        tap("SPACE")
        time.sleep(1.0)
        tap("SPACE")
        time.sleep(1.2)
        dragged = logical(ex.grab())
    crop = (330, 230, 1010, 392)
    save_png(
        hstack(
            [slow.crop(crop), dragged.crop(crop)],
            ["慢板点八下：鼓师几乎全漏", "拖戏再点两下：每招被收近十遍"],
        ),
        "fig-18-09-tally.png",
    )


def fig_11_ghost_vs_glide() -> None:
    """Figure 18-11：《赶月》——替身一顿一顿，阿燕每帧都在走（main）。"""
    with Example(exe("main"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        frames = ex.record(start=2.5, dur=5.0, fps=12)
    crop = (0, 150, 1280, 700)
    frames = [logical(f).crop(crop).resize((768, 330), Image.LANCZOS) for f in frames]
    save_webp(frames, "fig-18-11-ghost-vs-glide.webp", fps=12, quality=68)


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_02_two_clocks,
    fig_04_cooldown,
    fig_09_tally,
    fig_11_ghost_vs_glide,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch18-time", "--bins", "--examples"],
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

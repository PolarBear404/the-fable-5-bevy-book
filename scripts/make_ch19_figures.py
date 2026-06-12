# -*- coding: utf-8 -*-
"""一键重建第 19 章全部 PNG 插图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch19_figures.py [图名筛选]

四张图：锣的波形解剖（直接读合成 WAV 的采样画出来）、中场两停对照、
巡夜远近对照、《长风渡》首演全景。键盘事件用 SendInput 发真实扫描码（抄 ch18）。
前置：scripts/make_ch19_assets.py 已就位资产；产物输出到 book/src/images/ch19/。
"""

import ctypes
import os
import struct
import subprocess
import sys
import time
import wave
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch19-audio"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch19"

os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402

user32 = ctypes.windll.user32
kernel32 = ctypes.windll.kernel32

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
FONT_SMALL = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 16)
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36

# 波形图配色（与手绘 SVG 同一卡片风）
CARD_BG = (247, 245, 240)
CARD_INK = (74, 70, 63)
CARD_MUTED = (122, 116, 104)
CARD_WAVE = (192, 90, 46)
CARD_ENV = (29, 107, 64)

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
KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE = 0x2, 0x8

SCAN = {"SPACE": 0x39, "P": 0x19, "ALT": 0x38}


def _key(name: str, up: bool) -> INPUT:
    inp = INPUT()
    inp.type = INPUT_KEYBOARD
    flags = KEYEVENTF_SCANCODE | (KEYEVENTF_KEYUP if up else 0)
    inp.union.ki = KEYBDINPUT(0, SCAN[name], flags, 0, None)
    return inp


def _send(*inputs: INPUT) -> None:
    array = (INPUT * len(inputs))(*inputs)
    if user32.SendInput(len(inputs), array, ctypes.sizeof(INPUT)) != len(inputs):
        raise RuntimeError("SendInput 未全部送达")


def force_foreground(hwnd: int, tries: int = 8) -> None:
    """确保示例窗口在前台拿焦点——SendInput 的键击只进焦点窗口。"""
    for _ in range(tries):
        if user32.GetForegroundWindow() == hwnd:
            return
        _send(_key("ALT", False), _key("ALT", True))  # ALT 轻击解除前台锁
        tid_fg = user32.GetWindowThreadProcessId(user32.GetForegroundWindow(), None)
        tid_us = kernel32.GetCurrentThreadId()
        user32.AttachThreadInput(tid_us, tid_fg, True)
        user32.BringWindowToTop(hwnd)
        user32.SetForegroundWindow(hwnd)
        user32.AttachThreadInput(tid_us, tid_fg, False)
        time.sleep(0.15)
    raise RuntimeError("示例窗口拿不到前台焦点，输入会落空——关掉抢焦点的程序再试")


def tap(hwnd: int, name: str, hold: float = 0.06) -> None:
    force_foreground(hwnd)
    _send(_key(name, False))
    time.sleep(hold)
    _send(_key(name, True))


# ---------------------------------------------------------------- 通用排版

def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch19-audio.exe"
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


# ---------------------------------------------------------------- 各图

def fig_01_gong_waveform() -> None:
    """Figure 19-1：锣的波形——直接读 make_ch19_assets.py 合成的 WAV 采样。"""
    with wave.open(str(CRATE / "assets" / "sfx" / "gong.wav")) as w:
        rate = w.getframerate()
        n = w.getnframes()
        samples = struct.unpack(f"<{n}h", w.readframes(n))

    def card(width: int, height: int, title: str) -> tuple[Image.Image, ImageDraw.ImageDraw]:
        img = Image.new("RGB", (width, height), CARD_BG)
        draw = ImageDraw.Draw(img)
        draw.text((width // 2 - draw.textlength(title, font=FONT) // 2, 12), title,
                  font=FONT, fill=CARD_INK)
        return img, draw

    # 左幅：全长 2.2 秒的包络视图（每列画 min~max 振幅带）
    lw, lh, pad_l, pad_t = 760, 420, 56, 64
    plot_w, plot_h = lw - pad_l - 24, lh - pad_t - 56
    mid = pad_t + plot_h // 2
    left, draw = card(lw, lh, "锣（gong.wav）全长 2.2 秒")
    draw.line([(pad_l, mid), (pad_l + plot_w, mid)], fill=(184, 177, 164), width=1)
    cols = plot_w
    per = max(1, n // cols)
    env_pts = []
    for x in range(cols):
        chunk = samples[x * per:(x + 1) * per]
        if not chunk:
            break
        lo = min(chunk) / 32767 * (plot_h / 2)
        hi = max(chunk) / 32767 * (plot_h / 2)
        draw.line([(pad_l + x, mid - hi), (pad_l + x, mid - lo)], fill=CARD_WAVE, width=1)
        env_pts.append((pad_l + x, mid - max(abs(lo), abs(hi))))
    draw.line(env_pts, fill=CARD_ENV, width=3)
    for sec in (0.0, 0.5, 1.0, 1.5, 2.0):
        x = pad_l + int(sec * rate / per)
        draw.line([(x, mid + plot_h // 2 + 4), (x, mid + plot_h // 2 + 10)],
                  fill=CARD_MUTED, width=1)
        draw.text((x - 14, mid + plot_h // 2 + 14), f"{sec:.1f}s",
                  font=FONT_SMALL, fill=CARD_MUTED)
    draw.text((pad_l + 130, pad_t + 6), "包络：起音陡、衰减长", font=FONT_SMALL, fill=CARD_ENV)

    # 右幅：放大 0.30s 附近的 25 毫秒——看见一根根正弦摆动
    rw = 480
    right, draw = card(rw, lh, "放大 25 毫秒")
    z0, zn = int(0.30 * rate), int(0.025 * rate)
    zoom = samples[z0:z0 + zn]
    zpad_l, zplot_w, zplot_h = 36, rw - 36 - 24, lh - pad_t - 56
    zmid = pad_t + zplot_h // 2
    draw.line([(zpad_l, zmid), (zpad_l + zplot_w, zmid)], fill=(184, 177, 164), width=1)
    peak = max(max(zoom), -min(zoom))
    pts = [
        (zpad_l + i * zplot_w / (zn - 1), zmid - s / peak * (zplot_h / 2 - 8))
        for i, s in enumerate(zoom)
    ]
    draw.line(pts, fill=CARD_WAVE, width=2)
    draw.text((zpad_l + 6, lh - 44), "每个点是一个采样：22050 个/秒",
              font=FONT_SMALL, fill=CARD_MUTED)

    save_png(hstack([left, right]), "fig-19-01-gong-waveform.png")


def fig_03_two_pauses() -> None:
    """Figure 19-3：中场两停对照（Listing 19-6）——空格停人不停曲，P 才停曲。"""
    with Example(exe("listing-19-06"), workdir=CODE) as ex:
        running = logical(ex.shot(3.0))           # 开戏：两个读数一起走
        tap(ex.hwnd, "SPACE")                     # 中场：戏台钟停
        paused_stage = logical(ex.shot(6.4))      # 人定格，曲子读数照涨
        tap(ex.hwnd, "P")                         # 琴师压弦
        paused_both = logical(ex.shot(9.2))       # 曲子读数也停了

    def panel(img: Image.Image) -> Image.Image:
        hud = img.crop((140, 40, 1140, 110))      # 读数牌
        stage = img.crop((140, 380, 1140, 660))   # 台面与阿燕
        out = Image.new("RGB", (1000, hud.height + GAP + stage.height), GAP_COLOR)
        out.paste(hud, (0, 0))
        out.paste(stage, (0, hud.height + GAP))
        return out.resize((416, out.height * 416 // 1000), Image.LANCZOS)

    save_png(
        hstack(
            [panel(running), panel(paused_stage), panel(paused_both)],
            ["开戏：两个读数一起走", "空格中场：人定格，曲照奏", "再按 P：曲子才真停"],
        ),
        "fig-19-03-two-pauses.png",
    )


def fig_06_night_patrol() -> None:
    """Figure 19-6：巡夜远近（Listing 19-8）——贴着左耳 vs 走到远端。"""
    with Example(exe("listing-19-08"), workdir=CODE) as ex:
        near_left = logical(ex.shot(1.9))   # x ≈ -192，贴着左耳
        far_right = logical(ex.shot(6.8))   # x ≈ +396，远在右舷尽头
    crop = (0, 124, 1280, 620)
    save_png(
        hstack(
            [near_left.crop(crop).resize((624, 242)), far_right.crop(crop).resize((624, 242))],
            ["贴着左耳：更声全偏左", "右舷尽头：两耳都只剩零头"],
        ),
        "fig-19-06-night-patrol.png",
    )


def fig_07_premiere() -> None:
    """Figure 19-7：《长风渡》首演（main）——文武场齐备的全景。"""
    with Example(exe("main"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot.crop((0, 30, 1280, 700)), "fig-19-07-premiere.png")


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_01_gong_waveform,
    fig_03_two_pauses,
    fig_06_night_patrol,
    fig_07_premiere,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch19-audio", "--bins", "--examples"],
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

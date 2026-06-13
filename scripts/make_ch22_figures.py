# -*- coding: utf-8 -*-
"""一键重建第 22 章全部 PNG 插图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch22_figures.py [图名筛选]

九张 PNG：平行光下的园子、影子开关对照、shadow acne 对照、点光灯笼、
聚光追光、环境光补光对照、金属黑/照世界对照、晨雾、昼夜四档。
前置：scripts/make_ch22_assets.py 已生成 skybox.png；产物输出到
book/src/images/ch22/。昼夜四档要给 main 发真实空格键切换档位。
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
CRATE = CODE / "ch22-lighting"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch22"

os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36

# ---- 给 main 发真实空格键（keybd_event，无 cbSize 之累，最省心） ----------
_keybd_event = ctypes.windll.user32.keybd_event
VK_SPACE = 0x20
KEYEVENTF_KEYUP = 0x0002


VK_MENU = 0x12  # ALT


def ensure_foreground(hwnd: int) -> None:
    # Windows 前台锁会让外部进程的首次 SetForegroundWindow 失手；
    # 轻点一下 ALT 可解锁，之后 SetForegroundWindow 才真正生效
    u = ctypes.windll.user32
    for _ in range(15):
        _keybd_event(VK_MENU, 0, 0, 0)
        _keybd_event(VK_MENU, 0, KEYEVENTF_KEYUP, 0)
        u.SetForegroundWindow(hwnd)
        time.sleep(0.15)
        if u.GetForegroundWindow() == hwnd:
            return


def press_space(hwnd: int) -> None:
    ensure_foreground(hwnd)
    _keybd_event(VK_SPACE, 0, 0, 0)
    time.sleep(0.05)
    _keybd_event(VK_SPACE, 0, KEYEVENTF_KEYUP, 0)


# ---- 拼图工具（同第 21 章） ------------------------------------------------

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


def grid2x2(images: list[Image.Image], labels: list[str]) -> Image.Image:
    cell_w = min(im.width for im in images)
    cell_h = min(im.height for im in images)
    cells = []
    for im, text in zip(images, labels):
        c = Image.new("RGB", (cell_w, cell_h + LABEL_H), GAP_COLOR)
        c.paste(label_bar(cell_w, [text]), (0, 0))
        c.paste(im.resize((cell_w, cell_h)), (0, LABEL_H))
        cells.append(c)
    w = cell_w * 2 + GAP
    h = (cell_h + LABEL_H) * 2 + GAP
    canvas = Image.new("RGB", (w, h), GAP_COLOR)
    canvas.paste(cells[0], (0, 0))
    canvas.paste(cells[1], (cell_w + GAP, 0))
    canvas.paste(cells[2], (0, cell_h + LABEL_H + GAP))
    canvas.paste(cells[3], (cell_w + GAP, cell_h + LABEL_H + GAP))
    return canvas


def logical(img: Image.Image) -> Image.Image:
    if img.size == (1280, 720):
        return img
    return img.resize((1280, 720), Image.LANCZOS)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch22-lighting.exe"
    return EXAMPLES / f"{name}.exe"


# ---------------------------------------------------------------- 各图

def fig_01_the_sun() -> None:
    with Example(exe("listing-22-01"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop((90, 70, 1190, 700)), "fig-22-01-the-sun.png")


def fig_02_shadows_on_off() -> None:
    with Example(exe("listing-22-01"), workdir=CRATE) as ex:
        off = logical(ex.shot(2.0))
    with Example(exe("listing-22-02"), workdir=CRATE) as ex:
        on = logical(ex.shot(2.0))
    crop = (120, 90, 1160, 700)
    save_png(
        hstack([off.crop(crop), on.crop(crop)], ["没影子：东西像浮着", "开影子：踩在地上"]),
        "fig-22-02-shadows-on-off.png",
    )


def fig_03_shadow_acne() -> None:
    with Example(exe("listing-22-03"), workdir=CRATE) as ex:
        acne = logical(ex.shot(2.5))
    with Example(exe("listing-22-02"), workdir=CRATE) as ex:
        clean = logical(ex.shot(2.5))
    crop = (440, 300, 840, 640)  # 台中绣球
    save_png(
        hstack([acne.crop(crop), clean.crop(crop)], ["bias 归零：自阴影噪点", "默认 bias：光洁"]),
        "fig-22-04-shadow-acne.png",
    )


def fig_05_point_light() -> None:
    with Example(exe("listing-22-05"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop((150, 70, 1130, 700)), "fig-22-08-point-light.png")


def fig_06_spot_light() -> None:
    with Example(exe("listing-22-06"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop((250, 70, 1030, 700)), "fig-22-09-spot-light.png")


def fig_07_ambient() -> None:
    with Example(exe("listing-22-05"), workdir=CRATE) as ex:
        low = logical(ex.shot(2.0))
    with Example(exe("listing-22-07"), workdir=CRATE) as ex:
        high = logical(ex.shot(2.0))
    crop = (150, 90, 1130, 700)
    save_png(
        hstack([low.crop(crop), high.crop(crop)], ["默认环境光：暗部死黑", "补一层月色：暗部托起"]),
        "fig-22-10-ambient.png",
    )


def fig_08_metal_world() -> None:
    # 「没有世界」的黑球：把立方体贴图暂时移走，环境光照无图可挂，金属保持漆黑
    asset = CRATE / "assets" / "textures" / "skybox.png"
    bak = asset.with_suffix(".png.bak")
    asset.rename(bak)
    try:
        with Example(exe("listing-22-09"), workdir=CRATE) as ex:
            black = logical(ex.shot(2.0))
    finally:
        bak.rename(asset)  # 无论如何都还原
    # 「照出世界」的球：贴图就位，环境光照接管
    with Example(exe("listing-22-09"), workdir=CRATE) as ex:
        lit = logical(ex.shot(4.0))
    crop = (390, 150, 890, 650)
    save_png(
        hstack([black.crop(crop), lit.crop(crop)], ["没有环境光照：漆黑", "环境光照接管：照出暖阁"]),
        "fig-22-12-metal-world.png",
    )


def fig_09_fog() -> None:
    with Example(exe("listing-22-10"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop((40, 80, 1240, 690)), "fig-22-13-fog.png")


def fig_11_day_night() -> None:
    labels = ["黎明", "正午", "黄昏", "入夜"]
    frames = []
    with Example(exe("main"), workdir=CRATE) as ex:
        ex.wait_until(2.0)
        ensure_foreground(ex.hwnd)  # 先把窗口稳稳抢到前台，免得首次按键失手
        frames.append(logical(ex.shot(2.5)))  # 黎明（起始档）
        for _ in range(3):
            press_space(ex.hwnd)
            time.sleep(0.7)
            frames.append(logical(ex.grab()))
    crop = (60, 60, 1220, 700)
    cells = [f.crop(crop) for f in frames]
    save_png(grid2x2(cells, labels), "fig-22-15-day-night.png")


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_01_the_sun,
    fig_02_shadows_on_off,
    fig_03_shadow_acne,
    fig_05_point_light,
    fig_06_spot_light,
    fig_07_ambient,
    fig_08_metal_world,
    fig_09_fog,
    fig_11_day_night,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch22-lighting", "--bins", "--examples"],
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

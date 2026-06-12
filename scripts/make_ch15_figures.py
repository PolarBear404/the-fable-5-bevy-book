# -*- coding: utf-8 -*-
"""一键重建第 15 章全部运行截图与动图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch15_figures.py

前置：scripts/make_ch15_assets.py 已生成资产。脚本会先 cargo build 本章 crate，
再逐图截取。产物输出到 book/src/images/ch15/。
"""

import os
import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch15-sprites"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch15"

# 子进程（Bevy 示例）靠它定位 assets/——脚本不在 cargo 下启动 exe，必须显式给
os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
FONT_S = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 17)
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36


def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch15-sprites.exe"
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
    h = images[0].height
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


def shot(name: str, t: float) -> Image.Image:
    """截一帧并归一化到 1280×720 逻辑像素（DPI 缩放下物理分辨率可能是 1600×900）。"""
    with Example(exe(name), workdir=CODE) as ex:
        return ex.shot(t).resize((1280, 720), Image.LANCZOS)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


# ---------------------------------------------------------------- 各图

def fig_01_blurry_vs_nearest() -> None:
    """Figure 15-1：线性采样的糊 vs Nearest 的锐（Listing 15-1/15-2 各截一块）。"""
    blurry = shot("listing-15-01", 2.5).crop((480, 170, 800, 550))      # ×8 居中
    crisp = shot("listing-15-02", 2.5).crop((90, 225, 310, 495))        # 一号位 ×6
    crisp = crisp.resize((int(crisp.width * 380 / 270), 380), Image.LANCZOS)
    save_png(
        hstack([blurry, crisp], ["Listing 15-1：线性采样（默认）", "Listing 15-2：Nearest 采样"]),
        "fig-15-01-blurry-vs-nearest.png",
    )


def fig_02_fitting_room() -> None:
    """Figure 15-2：试装台五个位（Listing 15-2，静止场景）。"""
    save_png(shot("listing-15-02", 2.5).resize((880, 495)), "fig-15-02-fitting-room.png")


def fig_04_sheet_and_frame() -> None:
    """Figure 15-4：原稿 + 取景框高亮 + 单格放大（Listing 15-3，t=2.0 高亮在第 3 格附近）。"""
    save_png(shot("listing-15-03", 2.0).resize((880, 495)), "fig-15-04-sheet-and-frame.png")


def fig_05_zoetrope() -> None:
    """Figure 15-5：走马灯动图（Listing 15-4，录 11 秒含两次转身）。"""
    with Example(exe("listing-15-04"), workdir=CODE) as ex:
        frames = ex.record(start=1.5, dur=11.0, fps=10, size=(1280, 720))
    frames = [f.crop((40, 290, 1240, 540)).resize((840, 175)) for f in frames]
    path = OUT / "fig-15-05-zoetrope.webp"
    frames[0].save(
        path,
        save_all=True,
        append_images=frames[1:],
        duration=100,
        loop=0,
        quality=70,
        method=4,
    )
    print(f"fig-15-05-zoetrope.webp：{len(frames)} 帧，{path.stat().st_size // 1024} KB")


def fig_06_off_by_one() -> None:
    """Figure 15-6：正常帧 vs 整张原稿闪现（Listing 15-5，闪现窗口 4.8–5.6s）。"""
    with Example(exe("listing-15-05"), workdir=CODE) as ex:
        normal = ex.shot(2.0).resize((1280, 720), Image.LANCZOS).crop((440, 200, 840, 560))
        flash = ex.shot(5.1).resize((1280, 720), Image.LANCZOS).crop((180, 160, 1100, 560))
    flash = flash.resize((int(flash.width * 360 / 400), 360), Image.LANCZOS)
    save_png(
        hstack([normal, flash], ["平时：一次一格", "第 12 格那 0.8 秒：整张原稿"]),
        "fig-15-06-off-by-one.png",
    )


def fig_08_anchor_grid() -> None:
    """Figure 15-8：锚点九宫（Listing 15-6），叠上九个锚点常量名。"""
    img = shot("listing-15-06", 2.5)
    draw = ImageDraw.Draw(img)
    names = [
        "TOP_LEFT", "TOP_CENTER", "TOP_RIGHT",
        "CENTER_LEFT", "CENTER", "CENTER_RIGHT",
        "BOTTOM_LEFT", "BOTTOM_CENTER", "BOTTOM_RIGHT",
    ]
    # 每行把标签放进空当：TOP 行画挂在钉子下方，标签放钉子上方；
    # CENTER 行画盖住钉子，标签放上一行画与本行画之间；BOTTOM 行画立在钉子上方，标签放两行画之间
    row_label_dy = [-40, -90, -150]
    for i, name in enumerate(names):
        sx = 640 + (i % 3) * 330 - 330
        sy = 360 - (110 - (i // 3) * 220)
        ly = sy + row_label_dy[i // 3]
        w = draw.textlength(name, font=FONT_S)
        draw.text((sx - w / 2, ly), name, font=FONT_S, fill=(168, 172, 182))
    save_png(img.resize((960, 540)), "fig-15-08-anchor-grid.png")


def fig_09_feet_on_floor() -> None:
    """Figure 15-9：推拉镜头两个极值时刻（Listing 15-7，sin 谷 3.93s / 峰 6.55s）。"""
    with Example(exe("listing-15-07"), workdir=CODE) as ex:
        far = ex.shot(3.93).resize((640, 360), Image.LANCZOS)
        near = ex.shot(6.55).resize((640, 360), Image.LANCZOS)
    save_png(
        hstack([far, near], ["拉远（zoom ≈ 1.6）", "推近（zoom ≈ 4.4）"]),
        "fig-15-09-feet-on-floor.png",
    )


def fig_11_mounting() -> None:
    """Figure 15-11：装裱三连 + 平铺水面桥板（Listing 15-8，静止场景）。"""
    img = shot("listing-15-08", 2.5)
    bar = label_bar(1280, ["Auto：整张硬拉", "Sliced：角保形但只有 12px", "Sliced + max_corner_scale 4.0"])
    canvas = Image.new("RGB", (1280, 720 + LABEL_H), LABEL_BG)
    canvas.paste(bar, (0, 0))
    canvas.paste(img, (0, LABEL_H))
    save_png(canvas.resize((960, 567)), "fig-15-11-mounting.png")


def fig_12_color_wall() -> None:
    """Figure 15-12：色卡墙（Listing 15-10），左侧加行标签栏。"""
    img = shot("listing-15-10", 2.5)
    gutter = 170
    canvas = Image.new("RGB", (1280 + gutter, 720), (16, 17, 21))
    canvas.paste(img, (gutter, 0))
    draw = ImageDraw.Draw(canvas)
    rows = [
        (250, "sRGB 直线"),
        (160, "Oklch 路线"),
        (60, "rotate_hue"),
        (-40, "darker / lighter"),
        (-150, "现成色票"),
        (-260, "with_alpha"),
    ]
    for wy, text in rows:
        sy = 360 - wy
        w = draw.textlength(text, font=FONT_S)
        draw.text((gutter - 14 - w, sy - 12), text, font=FONT_S, fill=(190, 193, 202))
    save_png(canvas.resize((1015, 504)), "fig-15-12-color-wall.png")


def fig_13_moon_and_magenta() -> None:
    """Figure 15-13：Mesh2d 的月夜，外加忘上料的空位与洋红催料单（Listing 15-11）。"""
    save_png(shot("listing-15-11", 2.5).resize((880, 495)), "fig-15-13-moon-and-magenta.png")


def fig_14_dress_rehearsal() -> None:
    """Figure 15-14：《渡口夜话》带妆彩排全景（main，t=8 阿燕在东头亮相）。"""
    save_png(shot("main", 8.0).resize((960, 540)), "fig-15-14-dress-rehearsal.png")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        ["cargo", "build", "-p", "ch15-sprites", "--examples", "--bins"],
        cwd=CODE,
        check=True,
    )
    figs = {
        "01": fig_01_blurry_vs_nearest,
        "02": fig_02_fitting_room,
        "04": fig_04_sheet_and_frame,
        "05": fig_05_zoetrope,
        "06": fig_06_off_by_one,
        "08": fig_08_anchor_grid,
        "09": fig_09_feet_on_floor,
        "11": fig_11_mounting,
        "12": fig_12_color_wall,
        "13": fig_13_moon_and_magenta,
        "14": fig_14_dress_rehearsal,
    }
    only = sys.argv[1:]
    for key, fn in figs.items():
        if not only or key in only:
            fn()


if __name__ == "__main__":
    main()

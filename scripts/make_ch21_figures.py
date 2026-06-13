# -*- coding: utf-8 -*-
"""一键重建第 21 章全部 PNG 插图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch21_figures.py [图名筛选]

九张图：无灯/点灯对照、几何体全家福、细分四件、材质墙、班旗贴三件套、
失聪的旗、旗的正侧背三联、亭盖平滑/平面对照、得月楼开张全景。
本章示例不吃键盘（main 的空格换漆在正文里让读者亲手按），全部按时刻截帧。
前置：scripts/make_ch21_assets.py 已生成班旗贴图；产物输出到 book/src/images/ch21/。
"""

import os
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch21-meshes"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch21"

os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36


def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch21-meshes.exe"
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

def fig_01_unlit_vs_lit() -> None:
    """Figure 21-1：同一只箱笼——没点灯的一团昏红 vs 一盏堂灯下的三副面孔。"""
    with Example(exe("listing-21-01"), workdir=CODE) as ex:
        unlit = logical(ex.shot(2.0))
    with Example(exe("listing-21-02"), workdir=CODE) as ex:
        lit = logical(ex.shot(2.0))
    crop = (320, 130, 1010, 650)  # 箱笼居中的区域
    save_png(
        hstack(
            [unlit.crop(crop), lit.crop(crop)],
            ["Listing 21-1：没点灯", "Listing 21-2：一盏堂灯"],
        ),
        "fig-21-01-unlit-vs-lit.png",
    )


def fig_03_lineup() -> None:
    """Figure 21-3：内置几何体全家福（Listing 21-3 后排）。"""
    with Example(exe("listing-21-03"), workdir=CODE) as ex:
        shot = logical(ex.shot(2.0))
    # 只取后排九件——前排的细分实验是 Figure 21-4 的事
    save_png(shot.crop((0, 95, 1280, 372)), "fig-21-03-lineup.png")


def fig_04_resolution() -> None:
    """Figure 21-4：前排细分四件（Listing 21-3 前排），带标签。"""
    with Example(exe("listing-21-03"), workdir=CODE) as ex:
        shot = logical(ex.shot(2.0))
    # 前排四件在画面下半部，均匀分布在四个等宽列里
    band = shot.crop((220, 380, 1060, 700))
    panels = []
    w = band.width // 4
    for i in range(4):
        panels.append(band.crop((i * w, 0, (i + 1) * w, band.height)))
    save_png(
        hstack(panels, ["ico(0)", "uv(8, 5)", "默认（ico 5 档）", "resolution(6)"]),
        "fig-21-04-resolution.png",
    )


def fig_05_material_wall() -> None:
    """Figure 21-5：材质墙（Listing 21-5）——粗糙度 × 金属感，带坐标轴标注。"""
    with Example(exe("listing-21-05"), workdir=CODE) as ex:
        shot = logical(ex.shot(2.0))
    grid = shot.crop((230, 60, 1060, 700))

    pad_l, pad_b = 46, 42
    canvas = Image.new("RGB", (grid.width + pad_l, grid.height + pad_b), LABEL_BG)
    canvas.paste(grid, (pad_l, 0))
    draw = ImageDraw.Draw(canvas)
    bottom = "粗糙度 perceptual_roughness：0.1 → 1.0"
    w = draw.textlength(bottom, font=FONT)
    draw.text((pad_l + (grid.width - w) / 2, grid.height + 8), bottom,
              font=FONT, fill=LABEL_FG)
    # 纵轴标注：先画横排再转 90 度
    side_text = "金属感 metallic：0.0 → 1.0"
    sw = int(ImageDraw.Draw(Image.new("RGB", (8, 8))).textlength(side_text, font=FONT))
    side = Image.new("RGB", (sw + 16, 30), LABEL_BG)
    ImageDraw.Draw(side).text((8, 2), side_text, font=FONT, fill=LABEL_FG)
    side = side.rotate(90, expand=True)
    canvas.paste(side, (6, (grid.height - side.height) // 2))
    save_png(canvas, "fig-21-05-material-wall.png")


def fig_06_banner_on_primitives() -> None:
    """Figure 21-6：班旗贴三件套（Listing 21-6）——倒挂的箱笼面与挤皱的球。"""
    with Example(exe("listing-21-06"), workdir=CODE) as ex:
        shot = logical(ex.shot(1.6))
    save_png(shot.crop((120, 100, 1160, 620)), "fig-21-06-banner-on-primitives.png")


def fig_08_deaf_flag() -> None:
    """Figure 21-8：失聪的旗（Listing 21-7）——灯就在跟前，旗面一片死色。"""
    with Example(exe("listing-21-07"), workdir=CODE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop((330, 100, 950, 620)), "fig-21-08-deaf-flag.png")


def fig_09_flag_front_and_back() -> None:
    """Figure 21-9：旗的正、侧、背（Listing 21-8）——背面整张消失。"""
    with Example(exe("listing-21-08"), workdir=CODE) as ex:
        front = logical(ex.shot(4.35))   # 转满一圈，正面朝镜头
        edge = logical(ex.shot(5.2))     # ≈ 90°，侧身一条线
        back = logical(ex.shot(6.3))     # ≈ 180°，背面——什么都不画
    crop = (350, 90, 930, 640)
    save_png(
        hstack(
            [front.crop(crop), edge.crop(crop), back.crop(crop)],
            ["正面", "侧身", "背面：消失"],
        ),
        "fig-21-09-flag-front-and-back.png",
    )


def fig_11_roof_smooth_vs_flat() -> None:
    """Figure 21-11：亭盖两种做法（Listing 21-9）——馒头与利落。"""
    with Example(exe("listing-21-09"), workdir=CODE) as ex:
        shot = logical(ex.shot(2.0))
    left = shot.crop((90, 130, 640, 620))
    right = shot.crop((640, 130, 1190, 620))
    save_png(
        hstack(
            [left, right],
            ["共用顶点 + 平滑法线", "拆开顶点 + 按面法线"],
        ),
        "fig-21-11-roof-smooth-vs-flat.png",
    )


def fig_12_grand_opening() -> None:
    """Figure 21-12：得月楼开张（main）——立体布景合龙的全景。"""
    with Example(exe("main"), workdir=CODE) as ex:
        shot = logical(ex.shot(2.5))
    save_png(shot.crop((0, 20, 1280, 700)), "fig-21-12-grand-opening.png")


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_01_unlit_vs_lit,
    fig_03_lineup,
    fig_04_resolution,
    fig_05_material_wall,
    fig_06_banner_on_primitives,
    fig_08_deaf_flag,
    fig_09_flag_front_and_back,
    fig_11_roof_smooth_vs_flat,
    fig_12_grand_opening,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch21-meshes", "--bins", "--examples"],
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

# -*- coding: utf-8 -*-
"""一键重建第 24 章的截图与动图（SVG 概念图为手绘，不在此列）。

    py -3.11 scripts/make_ch24_figures.py [图名筛选]

六张 PNG + 一张 WebP：
  fig-24-02 自发光 / unlit / 素球    （listing-24-01）
  fig-24-03 法线贴图：没切线 vs 有切线（listing-24-03）
  fig-24-05 透明五调                 （listing-24-04）
  fig-24-07 清漆与镜面两排          （listing-24-05）
  fig-24-09 双面旗自转（动图）       （listing-24-06）
  fig-24-10 深度偏移：打架 vs 压住   （listing-24-07）
  fig-24-11 材质球画廊（兼 demo 占位图）（main）

前置：scripts/make_ch24_assets.py 已生成贴图；产物输出到 book/src/images/ch24/。
"""

import os
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch24-pbr-materials"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch24"

os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36


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
    if img.size == (1280, 720):
        return img
    return img.resize((1280, 720), Image.LANCZOS)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def save_webp(frames: list[Image.Image], filename: str, duration: int = 90) -> None:
    path = OUT / filename
    frames[0].save(
        path,
        save_all=True,
        append_images=frames[1:],
        duration=duration,
        loop=0,
        quality=72,
        method=4,
    )
    print(f"{filename}：{len(frames)} 帧，{path.stat().st_size // 1024} KB")


def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch24-pbr-materials.exe"
    return EXAMPLES / f"{name}.exe"


# ---------------------------------------------------------------- 各图

def fig_02_emissive() -> None:
    with Example(exe("listing-24-01"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop((150, 120, 1130, 660)), "fig-24-02-emissive.png")


def fig_03_normal_map() -> None:
    with Example(exe("listing-24-03"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop((180, 80, 1100, 640)), "fig-24-03-normal-map.png")


def fig_05_alpha() -> None:
    with Example(exe("listing-24-04"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop((90, 90, 1190, 660)), "fig-24-05-alpha-panes.png")


def fig_07_clearcoat() -> None:
    with Example(exe("listing-24-05"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop((230, 70, 1050, 700)), "fig-24-07-clearcoat.png")


def fig_09_double_sided() -> None:
    # 两面旗自转：红的（单面）转到背朝相机就消失，蓝的（双面）正反都在
    with Example(exe("listing-24-06"), workdir=CRATE) as ex:
        frames = ex.record(start=1.2, dur=6.0, fps=12, size=(1280, 720))
    frames = [f.crop((230, 110, 1050, 660)).resize((720, 484)) for f in frames]
    save_webp(frames, "fig-24-09-double-sided.webp", duration=90)


def fig_10_depth_bias() -> None:
    with Example(exe("listing-24-07"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    content = shot.crop((150, 150, 1130, 610))  # 两块板
    w = content.width // 2
    left = content.crop((0, 0, w, content.height))
    right = content.crop((w, 0, content.width, content.height))
    save_png(
        hstack([left, right], ["bias 0：标签被同面底板吞了", "bias 1.0：稳稳压在上面"]),
        "fig-24-10-depth-bias.png",
    )


def fig_11_gallery() -> None:
    # 画廊：等环境光照装配完（skybox 加载 + 装配）再截，金属/清漆/玻璃才照出周遭
    with Example(exe("main"), workdir=CRATE) as ex:
        shot = logical(ex.shot(4.5))
    save_png(shot.crop((40, 150, 1240, 690)), "fig-24-11-gallery.png")


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_02_emissive,
    fig_03_normal_map,
    fig_05_alpha,
    fig_07_clearcoat,
    fig_09_double_sided,
    fig_10_depth_bias,
    fig_11_gallery,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch24-pbr-materials", "--bins", "--examples"],
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

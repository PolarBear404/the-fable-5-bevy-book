# -*- coding: utf-8 -*-
"""一键重建第 23 章的 PNG 插图（SVG 概念图为手绘，不在此列）。

    py -3.11 scripts/make_ch23_figures.py [图名筛选]

四张 PNG：阿福上台、小旗挂右臂、行进三连帧、角儿登场（capstone）。
前置：scripts/make_ch23_assets.py 已生成 puppet.gltf；产物输出到
book/src/images/ch23/。
"""

import os
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch23-gltf"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch23"

# Bevy 按这个根找 assets/——直接跑 exe（非 cargo run）时必须显式给
os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402

GAP_COLOR = (58, 61, 68)
GAP = 4


def logical(img: Image.Image) -> Image.Image:
    if img.size == (1280, 720):
        return img
    return img.resize((1280, 720), Image.LANCZOS)


def hstack(images: list[Image.Image]) -> Image.Image:
    h = max(im.height for im in images)
    w = sum(im.width for im in images) + GAP * (len(images) - 1)
    canvas = Image.new("RGB", (w, h), GAP_COLOR)
    x = 0
    for im in images:
        canvas.paste(im, (x, 0))
        x += im.width + GAP
    return canvas


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch23-gltf.exe"
    return EXAMPLES / f"{name}.exe"


CROP = (330, 24, 950, 706)  # 居中框住木偶，留头顶与脚下


def fig_01_puppet() -> None:
    with Example(exe("listing-23-01"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop(CROP), "fig-23-01-puppet-on-stage.png")


def fig_04_flag() -> None:
    with Example(exe("listing-23-04"), workdir=CRATE) as ex:
        shot = logical(ex.shot(2.0))
    save_png(shot.crop(CROP), "fig-23-04-flag-on-arm.png")


def fig_05_strip() -> None:
    # 一段 1 秒循环里取三帧，跨半个循环 → 三个明显不同的姿势
    with Example(exe("listing-23-05"), workdir=CRATE) as ex:
        a = logical(ex.shot(1.6))
        b = logical(ex.shot(1.85))
        c = logical(ex.shot(2.1))
    crop = (380, 60, 900, 700)
    save_png(hstack([a.crop(crop), b.crop(crop), c.crop(crop)]), "fig-23-05-marching-strip.png")


def fig_07_capstone() -> None:
    with Example(exe("main"), workdir=CRATE) as ex:
        shot = logical(ex.shot(1.7))
    save_png(shot.crop(CROP), "fig-23-07-actor-on-stage.png")


ALL = [fig_01_puppet, fig_04_flag, fig_05_strip, fig_07_capstone]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch23-gltf", "--bins", "--examples"],
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

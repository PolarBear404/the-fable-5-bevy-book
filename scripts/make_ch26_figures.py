# -*- coding: utf-8 -*-
"""一键重建第 26 章的运行截图（SVG 概念图为手绘，不在此列）。

    py -3.11 scripts/make_ch26_figures.py [图名筛选]

三张 PNG：
  fig-26-02 原始画面 vs 成品画质       （main，CH26_PRESET=raw / default）
  fig-26-03 运动模糊与锐化组合          （main，CH26_PRESET=motion）
  fig-26-04 画质开关面板               （main，default）

产物输出到 book/src/images/ch26/。
"""

import os
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch26-post-processing-aa"
OUT = ROOT / "book" / "src" / "images" / "ch26"

os.environ["BEVY_ASSET_ROOT"] = str(CRATE / "assets")

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
LABEL_BG = (24, 27, 32)
LABEL_FG = (232, 236, 242)
GAP = 4
LABEL_H = 38


def logical(img: Image.Image) -> Image.Image:
    if img.size == (1280, 720):
        return img
    return img.resize((1280, 720), Image.LANCZOS)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def exe() -> Path:
    return CODE / "target" / "debug" / "ch26-post-processing-aa.exe"


def shot_with_preset(preset: str | None, at: float = 2.0) -> Image.Image:
    old = os.environ.get("CH26_PRESET")
    if preset is None:
        os.environ.pop("CH26_PRESET", None)
    else:
        os.environ["CH26_PRESET"] = preset
    try:
        with Example(exe(), workdir=CRATE) as ex:
            return logical(ex.shot(at))
    finally:
        if old is None:
            os.environ.pop("CH26_PRESET", None)
        else:
            os.environ["CH26_PRESET"] = old


def label_bar(width: int, labels: list[str]) -> Image.Image:
    bar = Image.new("RGB", (width, LABEL_H), LABEL_BG)
    draw = ImageDraw.Draw(bar)
    cell = width / len(labels)
    for i, text in enumerate(labels):
        w = draw.textlength(text, font=FONT)
        draw.text((cell * i + (cell - w) / 2, 7), text, font=FONT, fill=LABEL_FG)
    return bar


def hstack(images: list[Image.Image], labels: list[str]) -> Image.Image:
    height = max(img.height for img in images)
    width = sum(img.width for img in images) + GAP * (len(images) - 1)
    canvas = Image.new("RGB", (width, height + LABEL_H), (48, 50, 56))
    canvas.paste(label_bar(width, labels), (0, 0))
    x = 0
    for img in images:
        canvas.paste(img, (x, LABEL_H))
        x += img.width + GAP
    return canvas


def fig_02_raw_vs_grade() -> None:
    raw = shot_with_preset("raw", 2.0).crop((70, 105, 1210, 690)).resize((760, 390))
    grade = shot_with_preset(None, 2.0).crop((70, 105, 1210, 690)).resize((760, 390))
    save_png(
        hstack([raw, grade], ["Raw：无 HDR / 无 Bloom / 无 AA", "Grade：HDR + Tonemapping + Bloom + TAA"]),
        "fig-26-02-raw-vs-grade.png",
    )


def fig_03_motion_blur() -> None:
    shot = shot_with_preset("motion", 2.6)
    save_png(shot, "fig-26-03-motion-blur.png")


def fig_04_quality_panel() -> None:
    shot = shot_with_preset(None, 2.0)
    save_png(shot, "fig-26-04-quality-panel.png")


ALL = [
    fig_02_raw_vs_grade,
    fig_03_motion_blur,
    fig_04_quality_panel,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    subprocess.run([sys.executable, str(ROOT / "scripts" / "make_ch26_assets.py")], check=True)
    print("构建第 26 章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch26-post-processing-aa", "--bins", "--examples"],
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

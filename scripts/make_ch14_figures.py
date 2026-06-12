"""一键重建第 14 章全部运行截图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch14_figures.py

脚本会先 cargo build 本章 crate，再逐图截取；fig-14-04 在运行中改写资产文件，
结束后自动跑 make_ch14_assets.py 还原。产物输出到 book/src/images/ch14/。
"""

import os
import shutil
import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch14-assets"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch14"

# 子进程（Bevy 示例）靠它定位 assets/——脚本不在 cargo 下启动 exe，必须显式给
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
        return CODE / "target" / "debug" / "ch14-assets.exe"
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


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def fig_01_first_prop() -> None:
    """Figure 14-1：第一件道具——青霜剑挂上片场（Listing 14-1，静止场景）。"""
    with Example(exe("listing-14-01"), workdir=CODE) as ex:
        img = ex.shot(2.5)
    save_png(img.resize((800, 450)), "fig-14-01-first-prop.png")


def fig_04_hot_swap() -> None:
    """Figure 14-4：贴图热替换前后（Listing 14-8，运行中把灯笼盖到剑的文件上）。

    青霜剑位于窗口正中；前后各截一帧，裁出中央 400×400（物理像素，DPI 125%）。
    本图动过资产文件，必须当场还原，否则污染后续截图。
    """
    props = CRATE / "assets" / "props"
    try:
        with Example(exe("listing-14-08"), workdir=CODE) as ex:
            before = ex.shot(2.0).crop((600, 250, 1000, 650))
            shutil.copyfile(props / "lantern.png", props / "qingshuang-sword.png")
            after = ex.shot(4.5).crop((600, 250, 1000, 650))
    finally:
        restore_assets()
    save_png(
        hstack([before, after], ["改文件前：青霜剑", "改文件后：同一实体"]),
        "fig-14-04-hot-swap.png",
    )


def fig_05_samplers() -> None:
    """Figure 14-5：像素稿三种采样（Listing 14-9，静止场景）。

    三把剑的逻辑位置 x = -300/0/+300、尺寸 256；按 1.25 倍 DPI 折成物理像素裁带状区。
    """
    with Example(exe("listing-14-09"), workdir=CODE) as ex:
        band = ex.shot(2.5).crop((225, 250, 1375, 650))
    save_png(
        hstack([band], ["默认：线性采样", ".meta 档案：Nearest", "load_with_settings：Nearest"]),
        "fig-14-05-samplers.png",
    )


def fig_06_07_rolling() -> None:
    """Figure 14-6/14-7：开机日两幕（main）——加载画面（最短亮相闸内）与开机后的台面。"""
    with Example(exe("main"), workdir=CODE) as ex:
        loading = ex.shot(1.5)
        rolling = ex.shot(4.5)
    save_png(loading.resize((800, 450)), "fig-14-06-loading-screen.png")
    save_png(rolling.resize((800, 450)), "fig-14-07-rolling.png")


def restore_assets() -> None:
    subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "make_ch14_assets.py")],
        check=True,
        stdout=subprocess.DEVNULL,
    )
    print("资产已还原（make_ch14_assets.py）。")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建 ch14-assets …")
    subprocess.run(
        ["cargo", "build", "-p", "ch14-assets", "--examples", "--bins"],
        cwd=CODE,
        check=True,
    )
    fig_01_first_prop()
    fig_04_hot_swap()
    fig_05_samplers()
    fig_06_07_rolling()


if __name__ == "__main__":
    main()

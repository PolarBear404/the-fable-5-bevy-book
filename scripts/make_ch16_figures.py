# -*- coding: utf-8 -*-
"""一键重建第 16 章全部运行截图与动图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch16_figures.py

前置：scripts/make_ch16_assets.py 已生成资产（字体 + 美术）。脚本会先 cargo build
本章 crate（listing-16-05 需要 system_font_discovery feature，单独再 build 一次），
再逐图截取。产物输出到 book/src/images/ch16/。

时间轴注意：Bevy 的 Time 从首帧起算，而首帧前有时长不定的渲染管线编译；
对时刻敏感的图一律"录一段、按画面内容选帧"，保证任何机器上可复现。
"""

import os
import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch16-text"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch16"

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
        return CODE / "target" / "debug" / "ch16-text.exe"
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


def shot(name: str, t: float) -> Image.Image:
    """截一帧并归一化到 1280×720 逻辑像素（DPI 缩放下物理分辨率可能是 1600×900）。"""
    with Example(exe(name), workdir=CODE) as ex:
        return ex.shot(t).resize((1280, 720), Image.LANCZOS)


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
        print(f"  警告：超过 2 MB 上限，考虑降帧率/质量/裁切")


def region_diff(a: Image.Image, b: Image.Image, box: tuple[int, int, int, int]) -> float:
    """两帧在指定区域的平均像素差（粗糙但够用的"画面变了吗"判据）。"""
    ra, rb = a.crop(box).convert("L"), b.crop(box).convert("L")
    pa, pb = ra.tobytes(), rb.tobytes()
    return sum(abs(x - y) for x, y in zip(pa, pb)) / len(pa)


# ---------------------------------------------------------------- 各图

def fig_01_first_line() -> None:
    """Figure 16-1：第一行 Text2d（Listing 16-1，静止场景）。"""
    save_png(shot("listing-16-01", 2.5).crop((320, 280, 960, 440)), "fig-16-01-first-line.png")


def fig_02_tofu() -> None:
    """Figure 16-2：英文行正常、中文行十块豆腐（Listing 16-2，静止场景）。"""
    save_png(shot("listing-16-02", 2.5).crop((340, 240, 940, 460)), "fig-16-02-tofu.png")


def fig_03_zh_font() -> None:
    """Figure 16-3：加载字体后中文上屏（Listing 16-3，静止但需等字体到货）。"""
    save_png(shot("listing-16-03", 3.0).crop((280, 280, 1000, 480)), "fig-16-03-zh-font.png")


def fig_04_font_sources() -> None:
    """Figure 16-4：一副字模的三种叫法——两行成功、两行消失（Listing 16-4）。"""
    save_png(shot("listing-16-04", 3.0).crop((60, 80, 1060, 660)), "fig-16-04-font-sources.png")


def fig_05_system_fonts() -> None:
    """Figure 16-5：向系统借字模——等宽、仿宋与不再豆腐的默认行（Listing 16-5）。"""
    save_png(shot("listing-16-05", 3.0).crop((280, 150, 1000, 580)), "fig-16-05-system-fonts.png")


def fig_06_variable_weights() -> None:
    """Figure 16-6：可变字体——字重阶梯、字宽与拨轴（Listing 16-6）。"""
    save_png(shot("listing-16-06", 3.0).crop((100, 60, 1180, 700)), "fig-16-06-variable-weights.png")


def fig_08_size_ladder() -> None:
    """Figure 16-8：字号阶梯 + 64 号字模 vs 16 号放大四倍（Listing 16-7）。"""
    save_png(shot("listing-16-07", 3.0).crop((300, 60, 980, 680)), "fig-16-08-size-ladder.png")


def fig_09_line_height() -> None:
    """Figure 16-9：三种行高 + 字距对比 + 磨边对照（Listing 16-8）。"""
    save_png(shot("listing-16-08", 3.0).crop((60, 100, 1220, 700)), "fig-16-09-line-height-spacing-smoothing.png")


def fig_10_responsive() -> None:
    """Figure 16-10：会自己变的字号——原始窗、收窄后、基准尺拨大后（Listing 16-9）。

    检场系统在 3s/6s 动手；为躲开首帧前编译期的时间漂移，取 2.0/4.5/7.5 三个时刻。
    """
    with Example(exe("listing-16-09"), workdir=CODE) as ex:
        states = []
        for t in (2.0, 4.5, 7.5):
            img = ex.shot(t)
            # 统一到逻辑像素高 720，保持各自宽高比（收窄后窗口变瘦）
            img = img.resize((round(img.width * 720 / img.height), 720), Image.LANCZOS)
            # 居中裁出 640 宽的竖条（文字都在窗口中央）
            x0 = (img.width - 640) // 2
            states.append(img.crop((x0, 60, x0 + 640, 700)).resize((480, 480), Image.LANCZOS))
    save_png(
        hstack(states, ["开场：窗宽 1280", "窗户收窄到 880", "基准尺 20 → 30"]),
        "fig-16-10-responsive-font-size.png",
    )


def fig_11_bounds_boxes() -> None:
    """Figure 16-11：三只字幕框——换行、出框、溢出（Listing 16-10）。"""
    save_png(shot("listing-16-10", 3.0).crop((30, 60, 1250, 660)), "fig-16-11-bounds-boxes.png")


def fig_12_justify_anchor() -> None:
    """Figure 16-12：Justify 三态 + Anchor 三态（Listing 16-11）。"""
    save_png(shot("listing-16-11", 3.0).crop((120, 130, 1160, 610)), "fig-16-12-justify-anchor.png")


def fig_13_typewriter() -> None:
    """Figure 16-13：提词器动图（Listing 16-12）。

    Time 起点不定：录足 14 秒，再从"字幕框里第一次出现变化"起截 6.5 秒。
    """
    box = (260, 530, 1020, 670)  # 字幕框区域（逻辑像素）
    with Example(exe("listing-16-12"), workdir=CODE) as ex:
        frames = ex.record(start=0.5, dur=14.0, fps=8, size=(1280, 720))
    start = 0
    for i in range(1, len(frames)):
        if region_diff(frames[i], frames[0], box) > 1.0:
            start = max(0, i - 4)  # 留半秒空框做起拍
            break
    frames = [f.crop(box) for f in frames[start : start + 52]]
    save_webp(frames, "fig-16-13-typewriter.webp", fps=8)


def fig_14_rich_text() -> None:
    """Figure 16-14：秋白的改词手稿——spans 全套妆容（Listing 16-13，静止场景）。"""
    save_png(shot("listing-16-13", 3.0).crop((360, 260, 920, 460)), "fig-16-14-rich-text.png")


def fig_15_stage_vs_glass() -> None:
    """Figure 16-15：镜头摇到两头的两帧对比（Listing 16-14）。

    世界文字亮、背景暗：按中央条带亮像素的质心横坐标挑左右极值帧。
    """
    with Example(exe("listing-16-14"), workdir=CODE) as ex:
        frames = ex.record(start=1.0, dur=10.0, fps=5, size=(1280, 720))

    def centroid_x(img: Image.Image) -> float:
        band = img.crop((0, 300, 1280, 420)).convert("L")
        px = band.tobytes()
        xs = [i % 1280 for i, v in enumerate(px) if v > 150]
        return sum(xs) / len(xs) if xs else 640.0

    cs = [centroid_x(f) for f in frames]
    left = frames[cs.index(min(cs))].crop((0, 0, 1280, 560)).resize((624, 273))
    right = frames[cs.index(max(cs))].crop((0, 0, 1280, 560)).resize((624, 273))
    save_png(
        hstack([left, right], ["镜头摇到一头……", "……再摇到另一头：左上角的 UI 文字钉着不动"]),
        "fig-16-15-stage-vs-glass.png",
    )


def fig_16_night_drill() -> None:
    """Figure 16-16：《夜战》伤害飘字动图（main，含至少一记会心与一次歇手）。"""
    with Example(exe("main"), workdir=CODE) as ex:
        frames = ex.record(start=0.8, dur=11.0, fps=8, size=(1280, 720))
    # 从阿燕第一次起手（画面变化）起截，覆盖六剑 + 会心 + 歇手归零
    box = (700, 350, 1200, 700)
    start = 0
    for i in range(1, len(frames)):
        if region_diff(frames[i], frames[0], box) > 1.0:
            start = max(0, i - 2)
            break
    frames = [f.resize((800, 450), Image.LANCZOS) for f in frames[start : start + 76]]
    save_webp(frames, "fig-16-16-night-drill.webp", fps=8, quality=60)


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_01_first_line,
    fig_02_tofu,
    fig_03_zh_font,
    fig_04_font_sources,
    fig_05_system_fonts,
    fig_06_variable_weights,
    fig_08_size_ladder,
    fig_09_line_height,
    fig_10_responsive,
    fig_11_bounds_boxes,
    fig_12_justify_anchor,
    fig_13_typewriter,
    fig_14_rich_text,
    fig_15_stage_vs_glass,
    fig_16_night_drill,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch16-text", "--bins", "--examples"],
        cwd=CODE,
        check=True,
    )
    # listing-16-05（向系统借字模）在门后，单独带 feature 构建
    subprocess.run(
        [
            "cargo", "build", "-p", "ch16-text",
            "--example", "listing-16-05",
            "--features", "system_font_discovery",
        ],
        cwd=CODE,
        check=True,
    )
    only = sys.argv[1] if len(sys.argv) > 1 else None
    for fig in ALL:
        if only and only not in fig.__name__:
            continue
        fig()


if __name__ == "__main__":
    main()

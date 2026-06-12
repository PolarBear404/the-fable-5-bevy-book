"""一键重建第 13 章全部运行截图与动图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch13_figures.py

前置：cargo build -p ch13-cameras --examples --bins 已通过（脚本会自动执行）。
产物输出到 book/src/images/ch13/。
"""

import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch13"

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
        return CODE / "target" / "debug" / "ch13-cameras.exe"
    return EXAMPLES / f"{name}.exe"


def label_bar(width: int, texts: list[str]) -> Image.Image:
    """一条标签：texts 均分宽度，居中绘制。"""
    bar = Image.new("RGB", (width, LABEL_H), LABEL_BG)
    draw = ImageDraw.Draw(bar)
    cell = width / len(texts)
    for i, text in enumerate(texts):
        w = draw.textlength(text, font=FONT)
        draw.text((cell * i + (cell - w) / 2, 6), text, font=FONT, fill=LABEL_FG)
    return bar


def hstack(images: list[Image.Image], labels: list[str] | None = None) -> Image.Image:
    """横向拼接（带分隔线），可选顶部标签条。"""
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
    with Example(exe(name), workdir=CODE) as ex:
        return ex.shot(t)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def fig_01_clear_color() -> None:
    """Figure 13-1：默认灰底 vs 夜幕（Listing 13-1/13-2，静止场景）。"""
    gray = shot("listing-13-01", 2.5).resize((800, 450))
    night = shot("listing-13-02", 2.5).resize((800, 450))
    save_png(
        hstack([gray, night], ["Listing 13-1：默认清屏色", "Listing 13-2：换上夜幕"]),
        "fig-13-01-clear-color.png",
    )


def fig_02_follow_compare() -> None:
    """Figure 13-2：硬跟随 vs smooth_nudge 并排动图（各录 12 秒）。

    两个示例的走位都由 elapsed_secs 决定，分别录制后并排即可对比运镜质感。
    """
    size = (640, 360)
    with Example(exe("listing-13-03"), workdir=CODE) as ex:
        hard = ex.record(start=1.5, dur=12.0, fps=10, size=size)
    with Example(exe("listing-13-04"), workdir=CODE) as ex:
        soft = ex.record(start=1.5, dur=12.0, fps=10, size=size)
    labels = ["Listing 13-3：硬跟随", "Listing 13-4：smooth_nudge"]
    frames = [hstack([a, b], labels) for a, b in zip(hard, soft)]
    path = OUT / "fig-13-02-follow-compare.webp"
    frames[0].save(
        path,
        save_all=True,
        append_images=frames[1:],
        duration=100,
        loop=0,
        quality=70,
        method=4,
    )
    print(f"fig-13-02-follow-compare.webp：{len(frames)} 帧，{path.stat().st_size // 1024} KB")


def fig_04_shot_scales() -> None:
    """Figure 13-4：远景/中景/特写三联（Listing 13-7，三秒一切）。

    切镜发生在 Time 的 3/6/9 秒，取 1.5/4.5/7.5 秒采样，落在各档位的中段。
    """
    with Example(exe("listing-13-07"), workdir=CODE) as ex:
        frames = [ex.shot(t).resize((533, 300)) for t in (1.5, 4.5, 7.5)]
    save_png(
        hstack(frames, ["远景 scale = 1.5", "中景 scale = 1.0", "特写 scale = 0.6"]),
        "fig-13-04-shot-scales.png",
    )


def fig_05_split_screen() -> None:
    """Figure 13-5：双相机分屏（Listing 13-9）。"""
    save_png(shot("listing-13-09", 6.0), "fig-13-05-split-screen.png")


def fig_06_minimap() -> None:
    """Figure 13-6：沙盘与防穿帮（Listing 13-10）。"""
    save_png(shot("listing-13-10", 7.0), "fig-13-06-minimap.png")


def fig_08_ortho_vs_persp() -> None:
    """Figure 13-8：正交 vs 透视分屏（Listing 13-11，静止场景）。"""
    save_png(shot("listing-13-11", 4.0), "fig-13-08-ortho-vs-persp.png")


def fig_09_wrap() -> None:
    """Figure 13-9：全机位联动总装（src/main.rs）。"""
    save_png(shot("main", 7.0), "fig-13-09-wrap.png")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建示例……")
    subprocess.run(
        ["cargo", "build", "-p", "ch13-cameras", "--examples", "--bins"],
        cwd=CODE,
        check=True,
    )
    fig_01_clear_color()
    fig_02_follow_compare()
    fig_04_shot_scales()
    fig_05_split_screen()
    fig_06_minimap()
    fig_08_ortho_vs_persp()
    fig_09_wrap()
    print("完成。")


if __name__ == "__main__":
    main()

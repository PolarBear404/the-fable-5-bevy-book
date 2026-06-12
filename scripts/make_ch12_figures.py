"""一键重建第 12 章全部运行截图与动图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch12_figures.py

前置：cargo build -p ch12-transforms --examples --bins 已通过（脚本会自动执行）。
产物输出到 book/src/images/ch12/。
"""

import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch12"

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
        return CODE / "target" / "debug" / "ch12-transforms.exe"
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


def crop_center(img: Image.Image, frac_w: float, frac_h: float) -> Image.Image:
    """按比例裁中心区域：与 DPI 缩放无关，任何机器上取景一致。"""
    w, h = img.size
    cw, ch = int(w * frac_w), int(h * frac_h)
    x, y = (w - cw) // 2, (h - ch) // 2
    return img.crop((x, y, x + cw, y + ch))


def shot(name: str, t: float) -> Image.Image:
    with Example(exe(name), workdir=CODE) as ex:
        return ex.shot(t)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def fig_01_compass() -> None:
    """Figure 12-1：罗盘四方块（Listing 12-1，静止场景）。

    整窗不裁：原点在窗口正中央本身就是要看的结论。
    """
    save_png(shot("listing-12-01", 2.5).resize((1200, 675)), "fig-12-01-compass.png")


def fig_02_z_order() -> None:
    """Figure 12-2：z 决定遮挡（Listing 12-2，静止场景）。三方块都在中央，裁近些。"""
    save_png(
        crop_center(shot("listing-12-02", 2.5), 0.5, 0.62),
        "fig-12-02-z-order.png",
    )


def fig_05_tidal_lock() -> None:
    """Figure 12-5：rotate_around 的副产品——公转多少度，自转也同步多少度。

    地球角速度 0.5 rad/s：取公转 45°（t=1.57s，斜成菱形最醒目）与 115°（t=4.01s）
    两个时刻。采样不能早于 ~1.5s——渲染管线初始化期间 Sprite 还没画出来。
    """
    with Example(exe("listing-12-05"), workdir=CODE) as ex:
        at45 = ex.shot(1.57)
        at115 = ex.shot(4.01)
    panels = [crop_center(im, 0.55, 0.7).resize((760, 544)) for im in (at45, at115)]
    save_png(
        hstack(panels, ["公转 45°：自转跟着 45°", "公转 115°：自转跟着 115°"]),
        "fig-12-05-tidal-lock.png",
    )


def fig_06_comet_heading() -> None:
    """Figure 12-6：彗星机头追运动方向（Listing 12-6）。

    椭圆相位 = 0.8t：取 t=1.96（顶点，正向左飞）与 t=3.93（左端，正向下扎），
    两帧的机头一横一竖，朝向跟着运动方向走看得最清。
    """
    with Example(exe("listing-12-06"), workdir=CODE) as ex:
        top = ex.shot(1.96)
        left = ex.shot(3.93)
    panels = [crop_center(im, 0.66, 0.72).resize((760, 467)) for im in (top, left)]
    save_png(
        hstack(panels, ["飞过顶点：正向左，长条横了过来", "掠过左端：正向下，长条竖了回去"]),
        "fig-12-06-comet-heading.png",
    )


def fig_08_moon_compare() -> None:
    """Figure 12-8：松紧带月亮 vs 转盘月亮，并排动图（各录 12 秒）。

    两例的地球同速同起点（走位由时间决定），分别录制后逐帧并排即可对比。
    12-8 的轨道半径以 ~4.2s 为周期胀缩，12 秒看满约三个来回。
    """
    size = (640, 360)
    with Example(exe("listing-12-08"), workdir=CODE) as ex:
        chase = ex.record(start=1.5, dur=12.0, fps=10, size=size)
    with Example(exe("listing-12-09"), workdir=CODE) as ex:
        geared = ex.record(start=1.5, dur=12.0, fps=10, size=size)
    labels = ["Listing 12-8：逐帧追赶", "Listing 12-9：轨道盘层级"]
    frames = [hstack([a, b], labels) for a, b in zip(chase, geared)]
    path = OUT / "fig-12-08-moon-compare.webp"
    frames[0].save(
        path,
        save_all=True,
        append_images=frames[1:],
        duration=100,
        loop=0,
        quality=70,
        method=4,
    )
    print(f"fig-12-08-moon-compare.webp：{len(frames)} 帧，{path.stat().st_size // 1024} KB")


def fig_10_b0004() -> None:
    """Figure 12-10：B0004 现场（Listing 12-11，静止场景）。

    行星甲写着 (150, 0) 却瘫在正中央；行星乙在 (-150, 0) 一切正常。
    """
    save_png(
        crop_center(shot("listing-12-11", 2.5), 0.55, 0.5),
        "fig-12-10-b0004.png",
    )


def fig_12_wrap() -> None:
    """Figure 12-12：天文馆总装（src/main.rs）。

    t=6.8：彗星在右下斜着爬升、月亮与地球拉开、太阳正胀到 1.12 倍。
    """
    save_png(shot("main", 6.8), "fig-12-12-wrap.png")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建示例……")
    subprocess.run(
        ["cargo", "build", "-p", "ch12-transforms", "--examples", "--bins"],
        cwd=CODE,
        check=True,
    )
    fig_01_compass()
    fig_02_z_order()
    fig_05_tidal_lock()
    fig_06_comet_heading()
    fig_08_moon_compare()
    fig_10_b0004()
    fig_12_wrap()
    print("完成。")


if __name__ == "__main__":
    main()

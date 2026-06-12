# -*- coding: utf-8 -*-
"""ch14 美术资产一键重建：像素道具 + 大幅幕布。

用法：py -3.11 scripts/make_ch14_assets.py
产物全部写入 code/ch14-assets/，确定性生成（固定随机种子），可随时重建。
.script 与 .meta 是手写内容文件，不由本脚本生成。
"""

import random
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parent.parent
CRATE = ROOT / "code" / "ch14-assets"
ASSETS = CRATE / "assets"

# ---------------------------------------------------------------- 像素道具

PALETTE = {
    "W": (242, 248, 255, 255),  # 冰白高光
    "B": (159, 208, 232, 255),  # 青霜剑刃
    "D": (91, 143, 174, 255),   # 剑刃暗部
    "G": (232, 184, 75, 255),   # 金（护手/灯笼箍）
    "H": (138, 106, 31, 255),   # 暗金
    "R": (200, 58, 58, 255),    # 正红（灯笼/旗面）
    "M": (140, 27, 39, 255),    # 深红（灯笼骨）
    "S": (160, 72, 72, 255),    # 剑柄缠绳亮
    "r": (110, 44, 44, 255),    # 剑柄缠绳暗
    "Y": (240, 199, 94, 255),   # 黄（灯穗）
    "P": (107, 74, 47, 255),    # 旗杆棕
    "L": (232, 217, 176, 255),  # 旗面布白条
    "K": (35, 37, 46, 255),     # 场记板黑
    "C": (232, 232, 238, 255),  # 粉笔白
    ".": (0, 0, 0, 0),          # 透明
}

SWORD = [
    "................",
    ".......W........",
    "......WB........",
    "......WBD.......",
    "......WBD.......",
    "......WBD.......",
    "......WBD.......",
    "......WBD.......",
    "......WBD.......",
    "......WBD.......",
    "....GGGHGGG.....",
    ".......S........",
    ".......r........",
    ".......S........",
    "......GHG.......",
    "................",
]

LANTERN = [
    "................",
    ".......G........",
    "......GGG.......",
    "....RRRRRRR.....",
    "...RRMRRMRR.....",
    "..RRMRRMRRMR....",
    "..RRMRRMRRMR....",
    "..RRMRRMRRMR....",
    "..RRMRRMRRMR....",
    "...RRMRRMRR.....",
    "....RRRRRRR.....",
    "......GGG.......",
    ".......Y........",
    ".......Y........",
    "......YYY.......",
    "................",
]

BANNER = [
    "................",
    "...P............",
    "...PRRRRRRRRR...",
    "...PRRRRRRRRRR..",
    "...PRLLLLLLRR...",
    "...PRRRRRRRR....",
    "...PRRRRRRR.....",
    "...PRRRRRRRR....",
    "...PRRRR........",
    "...P............",
    "...P............",
    "...P............",
    "...P............",
    "...P............",
    "...P............",
    "................",
]

CLAPPER = [
    "................",
    ".KCKKCKKCKKCKK..",
    ".KKCKKCKKCKKCK..",
    ".KKKKKKKKKKKKK..",
    ".KKKKKKKKKKKKK..",
    ".KKCCCCCCCKKKK..",
    ".KKKKKKKKKKKKK..",
    ".KKCCCCCKKKKKK..",
    ".KKKKKKKKKKKKK..",
    ".KKKKKKKKKKKKK..",
    ".KKKKKKKKKKKKK..",
    "................",
    "................",
    "................",
    "................",
    "................",
]


def draw_pixels(rows: list[str]) -> Image.Image:
    img = Image.new("RGBA", (16, 16), (0, 0, 0, 0))
    for y, row in enumerate(rows):
        for x, ch in enumerate(row):
            img.putpixel((x, y), PALETTE[ch])
    return img


def save_prop(rows: list[str], path: Path, scale: int = 8) -> None:
    img = draw_pixels(rows)
    big = img.resize((16 * scale, 16 * scale), Image.NEAREST)
    path.parent.mkdir(parents=True, exist_ok=True)
    big.save(path, optimize=True)
    print(f"  {path.relative_to(ROOT)}  {big.size[0]}x{big.size[1]}")


def save_raw16(rows: list[str], path: Path) -> None:
    img = draw_pixels(rows)
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path, optimize=True)
    print(f"  {path.relative_to(ROOT)}  16x16")


# ---------------------------------------------------------------- 幕布

def vertical_gradient(size: int, stops: list[tuple[float, tuple[int, int, int]]]) -> Image.Image:
    """竖向渐变：stops 为 (位置 0..1, RGB)。先画 1 像素宽的窄条再拉伸，快且省内存。"""
    strip = Image.new("RGB", (1, size))
    for y in range(size):
        t = y / (size - 1)
        for (p0, c0), (p1, c1) in zip(stops, stops[1:]):
            if p0 <= t <= p1:
                k = (t - p0) / (p1 - p0) if p1 > p0 else 0.0
                col = tuple(round(a + (b - a) * k) for a, b in zip(c0, c1))
                strip.putpixel((0, y), col)
                break
    return strip.resize((size, size), Image.NEAREST)


def make_night_crossing(size: int = 8192) -> Image.Image:
    """夜渡江湾全景幕布：星空、满月、远山、水面。"""
    rng = random.Random(1401)
    img = vertical_gradient(size, [
        (0.0, (10, 14, 42)),
        (0.55, (20, 50, 74)),
        (0.66, (13, 31, 48)),
        (1.0, (9, 22, 34)),
    ])
    d = ImageDraw.Draw(img, "RGBA")
    # 星
    for _ in range(2400):
        x = rng.randrange(size)
        y = rng.randrange(int(size * 0.52))
        r = rng.choice([1, 1, 1, 2])
        a = rng.randint(90, 220)
        d.ellipse([x - r, y - r, x + r, y + r], fill=(235, 238, 248, a))
    # 月亮（带一圈淡晕）
    mx, my, mr = int(size * 0.72), int(size * 0.2), int(size * 0.055)
    for k, alpha in [(2.1, 26), (1.6, 44), (1.0, 255)]:
        rr = int(mr * k)
        d.ellipse([mx - rr, my - rr, mx + rr, my + rr], fill=(232, 228, 208, alpha))
    # 远山剪影（两层）
    horizon = int(size * 0.62)
    for layer, (amp, col) in enumerate([(0.06, (8, 16, 26, 255)), (0.045, (12, 24, 38, 255))]):
        pts = [(0, horizon)]
        x = 0
        rngl = random.Random(77 + layer)
        while x < size:
            x += rngl.randint(size // 22, size // 9)
            pts.append((min(x, size), horizon - rngl.randint(0, int(size * amp))))
        pts += [(size, horizon + 10), (size, size), (0, size)]
        d.polygon(pts, fill=col)
        horizon = int(size * 0.66)
    # 水面波光：短横线
    for _ in range(2600):
        y = rng.randrange(int(size * 0.68), size)
        x = rng.randrange(size)
        w = rng.randint(size // 160, size // 50)
        a = rng.randint(14, 52)
        d.line([x, y, x + w, y], fill=(120, 190, 220, a), width=max(2, size // 1400))
    # 月亮的倒影柱
    for _ in range(700):
        y = rng.randrange(int(size * 0.68), size)
        x = mx + rng.randint(-mr, mr) + rng.randint(-mr // 2, mr // 2)
        w = rng.randint(size // 200, size // 80)
        d.line([x, y, x + w, y], fill=(220, 214, 180, rng.randint(20, 70)), width=max(2, size // 1400))
    return img


def make_bamboo_sea(size: int = 2048) -> Image.Image:
    """竹海幕布：绿色渐变 + 竹竿竖纹。"""
    rng = random.Random(1402)
    img = vertical_gradient(size, [
        (0.0, (22, 48, 30)),
        (0.6, (44, 90, 50)),
        (1.0, (16, 36, 24)),
    ])
    d = ImageDraw.Draw(img, "RGBA")
    for _ in range(64):
        x = rng.randrange(size)
        w = rng.randint(size // 200, size // 70)
        shade = rng.choice([(12, 28, 18, 150), (60, 116, 66, 110), (30, 66, 40, 130)])
        d.rectangle([x, 0, x + w, size], fill=shade)
        # 竹节
        for y in range(rng.randint(0, size // 8), size, rng.randint(size // 10, size // 6)):
            d.line([x, y, x + w, y], fill=(14, 30, 20, 160), width=max(2, size // 500))
    # 斜射的光
    for i in range(5):
        x0 = rng.randrange(-size // 3, size)
        d.polygon([(x0, 0), (x0 + size // 14, 0), (x0 + size // 3, size), (x0 + size // 5, size)],
                  fill=(210, 230, 170, 14))
    return img


def make_old_road(size: int = 2048) -> Image.Image:
    """官道幕布：黄昏天色 + 土路远山。"""
    rng = random.Random(1403)
    img = vertical_gradient(size, [
        (0.0, (88, 52, 78)),
        (0.4, (201, 122, 74)),
        (0.55, (120, 70, 56)),
        (1.0, (44, 34, 24)),
    ])
    d = ImageDraw.Draw(img, "RGBA")
    # 落日
    sx, sy, sr = int(size * 0.3), int(size * 0.42), int(size * 0.07)
    for k, a in [(1.8, 30), (1.0, 235)]:
        rr = int(sr * k)
        d.ellipse([sx - rr, sy - rr, sx + rr, sy + rr], fill=(244, 176, 96, a))
    # 远山
    pts = [(0, int(size * 0.52))]
    x = 0
    while x < size:
        x += rng.randint(size // 14, size // 7)
        pts.append((min(x, size), int(size * 0.52) - rng.randint(0, int(size * 0.07))))
    pts += [(size, int(size * 0.55)), (size, size), (0, size)]
    d.polygon(pts, fill=(56, 40, 46, 255))
    # 地面与官道
    d.rectangle([0, int(size * 0.56), size, size], fill=(74, 56, 40, 255))
    d.polygon([(int(size * 0.46), int(size * 0.56)), (int(size * 0.54), int(size * 0.56)),
               (int(size * 0.78), size), (int(size * 0.2), size)], fill=(106, 86, 56, 255))
    # 路上的车辙
    d.polygon([(int(size * 0.49), int(size * 0.56)), (int(size * 0.505), int(size * 0.56)),
               (int(size * 0.52), size), (int(size * 0.45), size)], fill=(88, 70, 46, 255))
    return img


def make_ferry_dock(size: int = 2048) -> Image.Image:
    """渡口栈桥幕布：青灰水色 + 木板桥面。"""
    rng = random.Random(1404)
    img = vertical_gradient(size, [
        (0.0, (35, 50, 66)),
        (0.5, (58, 82, 102)),
        (1.0, (28, 40, 52)),
    ])
    d = ImageDraw.Draw(img, "RGBA")
    # 水面碎波
    for _ in range(1400):
        y = rng.randrange(0, int(size * 0.55))
        x = rng.randrange(size)
        w = rng.randint(size // 120, size // 40)
        d.line([x, y, x + w, y], fill=(140, 180, 205, rng.randint(12, 40)), width=2)
    # 栈桥木板（下半）
    top = int(size * 0.58)
    plank = (94, 74, 50)
    d.rectangle([0, top, size, size], fill=plank)
    for i, y in enumerate(range(top, size, size // 18)):
        d.line([0, y, size, y], fill=(60, 46, 30, 255), width=max(3, size // 400))
    for x in range(0, size, size // 9):
        off = rng.randint(-size // 40, size // 40)
        d.line([x + off, top, x + off, size], fill=(70, 54, 36, 200), width=max(2, size // 600))
    # 系缆桩
    for fx in (0.18, 0.55, 0.86):
        x = int(size * fx)
        w = size // 36
        d.rectangle([x, top - size // 9, x + w, top + size // 60], fill=(52, 40, 28, 255))
        d.rectangle([x - w // 3, top - size // 9, x + w + w // 3, top - size // 9 + size // 70],
                    fill=(44, 34, 24, 255))
    return img


def main() -> None:
    print("生成 ch14 道具（128×128，16 格像素稿 ×8）：")
    save_prop(SWORD, ASSETS / "props" / "qingshuang-sword.png")
    save_prop(LANTERN, ASSETS / "props" / "lantern.png")
    save_prop(BANNER, ASSETS / "props" / "changfeng-banner.png")

    print("生成像素原稿（16×16，海报采样实验用；三份副本对应三种载入设置）：")
    save_raw16(SWORD, ASSETS / "props" / "sword-16.png")
    save_raw16(SWORD, ASSETS / "props" / "sword-16-meta.png")
    save_raw16(SWORD, ASSETS / "props" / "sword-16-settings.png")

    print("生成嵌入资产（场记板，两份：examples 与 src 各一）：")
    save_prop(CLAPPER, CRATE / "examples" / "embedded" / "clapper.png")
    save_prop(CLAPPER, CRATE / "src" / "embedded" / "clapper.png")

    print("生成幕布（大文件，撑出可见的加载过程）：")
    for name, fn in [
        ("night-crossing.png", lambda: make_night_crossing(8192)),
        ("bamboo-sea.png", lambda: make_bamboo_sea(2048)),
        ("old-road.png", lambda: make_old_road(2048)),
        ("ferry-dock.png", lambda: make_ferry_dock(2048)),
    ]:
        path = ASSETS / "backdrops" / name
        path.parent.mkdir(parents=True, exist_ok=True)
        img = fn()
        img.save(path, optimize=True)
        kb = path.stat().st_size / 1024
        print(f"  {path.relative_to(ROOT)}  {img.size[0]}x{img.size[1]}  {kb:.0f} KB")


if __name__ == "__main__":
    main()

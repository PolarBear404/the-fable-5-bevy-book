# -*- coding: utf-8 -*-
"""一键重建第 22 章的全部图片资产（纯 PIL 合成，确定性随机种子，可反复重建）。

    py -3.11 scripts/make_ch22_assets.py

产物（输出到 code/ch22-lighting/assets/textures/）：
  night_cubemap.png       夜空天幕：竖条 cubemap（512×3072，六面 +X -X +Y -Y +Z -Z），
                          星野 + 月亮（+Z 面）+ 一条淡银河，22.9 节 Skybox 用
  night_cubemap_odd.png   同一夜空的 500×3000 版——边长不是 2 的幂，
                          22.9 节喂给 GeneratedEnvironmentMapLight 亲眼看 panic 用
  hall_warm_cubemap.png   暖阁六面（128×768）：朱漆墙 + 木格窗，22.11 节反射探针用
  hall_cool_cubemap.png   冰厅六面（128×768）：青蓝墙 + 冰裂纹窗，同上
"""

import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter

sys.stdout.reconfigure(encoding="utf-8")

OUT = Path(__file__).resolve().parent.parent / "code" / "ch22-lighting" / "assets" / "textures"

# 竖条 cubemap 的六面顺序即数组层顺序：+X -X +Y -Y +Z -Z（wgpu 约定）
FACES = ["+x", "-x", "+y", "-y", "+z", "-z"]


# ---------------------------------------------------------------- 夜空

def night_face(size: int, face: str, rng: random.Random) -> Image.Image:
    """一面夜空：越靠地平线越亮的深蓝渐变 + 星星；月亮只在 -Z 面。"""
    img = Image.new("RGB", (size, size))
    px = img.load()
    for y in range(size):
        if face == "+y":        # 天顶：最暗
            t = 0.0
        elif face == "-y":      # 脚下：近地辉光
            t = 1.0
        else:                   # 四面侧墙：上暗下亮（y 向下增大 = 靠近地平线）
            t = y / (size - 1)
        r = int(5 + 14 * t)
        g = int(8 + 20 * t)
        b = int(18 + 38 * t)
        for x in range(size):
            px[x, y] = (r, g, b)

    draw = ImageDraw.Draw(img)

    # 一条斜过侧面的淡银河（+x/-x/-z 三面，避开月亮那面）
    if face in ("+x", "-x", "-z"):
        band = Image.new("L", (size, size), 0)
        bd = ImageDraw.Draw(band)
        for i in range(220):
            fx = rng.random() * size
            fy = size * 0.28 + (fx / size - 0.5) * size * 0.30 + rng.gauss(0, size * 0.055)
            rr = rng.uniform(0.5, 1.8)
            bd.ellipse([fx - rr, fy - rr, fx + rr, fy + rr], fill=rng.randint(10, 32))
        band = band.filter(ImageFilter.GaussianBlur(size * 0.02))
        img.paste(Image.new("RGB", (size, size), (120, 130, 165)), (0, 0), band)

    # 星星：三档亮度，确定性撒点
    for _ in range(int(size * size / 210)):
        x, y = rng.uniform(0, size - 1), rng.uniform(0, size - 1)
        mag = rng.random()
        if mag > 0.97:      # 亮星带一点十字光芒
            c = (235, 238, 255)
            draw.ellipse([x - 1.2, y - 1.2, x + 1.2, y + 1.2], fill=c)
            draw.line([x - 3, y, x + 3, y], fill=(150, 155, 190))
            draw.line([x, y - 3, x, y + 3], fill=(150, 155, 190))
        elif mag > 0.80:
            draw.ellipse([x - 0.8, y - 0.8, x + 0.8, y + 0.8], fill=(190, 196, 228))
        else:
            draw.point((x, y), fill=(96 + int(60 * mag), 100 + int(60 * mag), 138))

    # 月亮挂在戏台正对的天上（+Z 面——wgpu 的 cubemap 是左手系，
    # 相机朝世界 -Z 看，采到的是 +Z 那面）
    if face == "+z":
        cx, cy, r = size * 0.62, size * 0.40, size * 0.055
        halo = Image.new("L", (size, size), 0)
        ImageDraw.Draw(halo).ellipse(
            [cx - r * 3.2, cy - r * 3.2, cx + r * 3.2, cy + r * 3.2], fill=70
        )
        halo = halo.filter(ImageFilter.GaussianBlur(size * 0.05))
        img.paste(Image.new("RGB", (size, size), (168, 178, 205)), (0, 0), halo)
        draw = ImageDraw.Draw(img)
        draw.ellipse([cx - r, cy - r, cx + r, cy + r], fill=(228, 232, 240))
        # 几块环形山的阴影，别让月亮太像圆片
        for dx, dy, rr, shade in [(-0.35, -0.1, 0.30, 12), (0.25, 0.3, 0.22, 10),
                                  (0.1, -0.4, 0.16, 8)]:
            draw.ellipse(
                [cx + dx * r - rr * r, cy + dy * r - rr * r,
                 cx + dx * r + rr * r, cy + dy * r + rr * r],
                fill=(228 - shade * 3, 232 - shade * 3, 240 - shade * 2),
            )
    return img


def make_night_strip(size: int, path: Path) -> None:
    rng = random.Random(2260)   # 固定种子：每次重建逐像素一致
    strip = Image.new("RGB", (size, size * 6))
    for i, face in enumerate(FACES):
        strip.paste(night_face(size, face, rng), (0, i * size))
    strip.save(path, optimize=True)
    print(f"{path.name}：{strip.size[0]}x{strip.size[1]}，{path.stat().st_size // 1024} KB")


# ---------------------------------------------------------------- 镜厅

def hall_face(size: int, face: str, wall: tuple, trim: tuple, pattern: str) -> Image.Image:
    """镜厅一面：墙色 + 腰线 + 各面不同的窗格图案，反射里一眼能认出方向。"""
    floor = (86, 92, 88)     # 青砖
    ceiling = (52, 40, 30)   # 木顶
    if face == "-y":
        img = Image.new("RGB", (size, size), floor)
        d = ImageDraw.Draw(img)
        step = size // 8
        for i in range(0, size + 1, step):   # 砖缝
            d.line([(i, 0), (i, size)], fill=(70, 76, 72), width=2)
            d.line([(0, i), (size, i)], fill=(70, 76, 72), width=2)
        return img
    if face == "+y":
        img = Image.new("RGB", (size, size), ceiling)
        d = ImageDraw.Draw(img)
        for i in range(1, 6):                # 顶棚椽条
            d.line([(0, i * size // 6), (size, i * size // 6)], fill=(40, 30, 22), width=3)
        return img

    img = Image.new("RGB", (size, size), wall)
    d = ImageDraw.Draw(img)
    d.rectangle([0, int(size * 0.78), size, size], fill=trim)          # 墙裙
    d.line([(0, int(size * 0.78)), (size, int(size * 0.78))], fill=(30, 24, 20), width=3)

    cx0, cy0 = int(size * 0.25), int(size * 0.18)
    cx1, cy1 = int(size * 0.75), int(size * 0.62)
    d.rectangle([cx0, cy0, cx1, cy1], fill=(20, 16, 14))               # 窗洞
    if pattern == "lattice":     # 井字木格
        for i in range(1, 3):
            x = cx0 + (cx1 - cx0) * i // 3
            y = cy0 + (cy1 - cy0) * i // 3
            d.line([(x, cy0), (x, cy1)], fill=trim, width=4)
            d.line([(cx0, y), (cx1, y)], fill=trim, width=4)
    elif pattern == "diamond":   # 斜方格
        d.line([(cx0, cy0), (cx1, cy1)], fill=trim, width=4)
        d.line([(cx0, cy1), (cx1, cy0)], fill=trim, width=4)
    elif pattern == "bars":      # 竖棂条
        for i in range(1, 4):
            x = cx0 + (cx1 - cx0) * i // 4
            d.line([(x, cy0), (x, cy1)], fill=trim, width=4)
    elif pattern == "moon":      # 月洞门形
        d.ellipse([cx0 + 8, cy0 + 8, cx1 - 8, cy1 - 8], outline=trim, width=5)
    d.rectangle([cx0, cy0, cx1, cy1], outline=trim, width=5)
    return img


def make_hall_strip(size: int, path: Path, wall: tuple, trim: tuple) -> None:
    patterns = {"+x": "bars", "-x": "diamond", "+z": "moon", "-z": "lattice"}
    strip = Image.new("RGB", (size, size * 6))
    for i, face in enumerate(FACES):
        strip.paste(hall_face(size, face, wall, trim, patterns.get(face, "")), (0, i * size))
    strip.save(path, optimize=True)
    print(f"{path.name}：{strip.size[0]}x{strip.size[1]}，{path.stat().st_size // 1024} KB")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    make_night_strip(512, OUT / "night_cubemap.png")
    make_night_strip(500, OUT / "night_cubemap_odd.png")   # 故意不是 2 的幂
    make_hall_strip(128, OUT / "hall_warm_cubemap.png",
                    wall=(150, 52, 40), trim=(196, 148, 82))
    make_hall_strip(128, OUT / "hall_cool_cubemap.png",
                    wall=(46, 84, 110), trim=(150, 190, 205))


if __name__ == "__main__":
    main()

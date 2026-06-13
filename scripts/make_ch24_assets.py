# -*- coding: utf-8 -*-
"""一键生成第 24 章的美术资产，全部用纯 Python 标准库 + Pillow 合成：

    py -3.11 scripts/make_ch24_assets.py

三件产物，落到 code/ch24-pbr-materials/assets/textures/：
  - studs-normal.png  法线贴图：一排圆铆钉的切线空间法线（平处是经典的紫蓝 128,128,255）。
                      存的是「方向」不是颜色，加载时务必 is_srgb = false。
  - lattice.png       带 alpha 的镂空贴图：暖色薄板上钻一格格圆孔，给 AlphaMode::Mask 用。
  - skybox.png        竖摞六面的立方体贴图（同第 22 章的装配口味），一座中性的「影棚」：
                      顶上柔光、四壁带亮条、地面压暗——给金属 / 清漆 / 玻璃一个干净可映的世界。
"""

import math
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "code" / "ch24-pbr-materials" / "assets" / "textures"
SIZE = 256  # 单图边长，2 的幂


def lerp(a, b, t):
    return tuple(round(a[i] + (b[i] - a[i]) * t) for i in range(3))


# ---- 法线贴图：一排圆铆钉 -------------------------------------------------

def make_studs_normal():
    """一格格半球形铆钉的切线空间法线图。

    平处法线朝正前方 (0,0,1) → 编码成 (128,128,255)，就是法线图常见的紫蓝底色；
    铆钉处取半球面法线，沿斜面偏出去，受光时鼓成立体。Y 用 OpenGL 约定（向上为正），
    Bevy 默认就吃这一套（DirectX 风格才需 flip_normal_map_y）。
    """
    img = Image.new("RGB", (SIZE, SIZE))
    px = img.load()
    spacing = 64       # 铆钉网格间距
    radius = 26.0      # 铆钉半径
    for y in range(SIZE):
        for x in range(SIZE):
            # 到最近一个铆钉中心的偏移
            cx = (x // spacing) * spacing + spacing / 2
            cy = (y // spacing) * spacing + spacing / 2
            dx = (x - cx) / radius
            # 图像 y 向下，法线 y 向上，取反对齐 OpenGL 约定
            dy = -(y - cy) / radius
            d2 = dx * dx + dy * dy
            if d2 < 1.0:
                nx, ny = dx, dy
                nz = math.sqrt(1.0 - d2)
            else:
                nx, ny, nz = 0.0, 0.0, 1.0
            px[x, y] = (
                round((nx * 0.5 + 0.5) * 255),
                round((ny * 0.5 + 0.5) * 255),
                round((nz * 0.5 + 0.5) * 255),
            )
    path = OUT / "studs-normal.png"
    img.save(path, optimize=True)
    print(f"studs-normal.png：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


# ---- 镂空贴图：钻孔薄板（带 alpha） --------------------------------------

def make_lattice():
    """暖色薄板上钻一格格圆孔：孔内 alpha=0，板上 alpha=255。供 AlphaMode::Mask 切镂空。"""
    img = Image.new("RGBA", (SIZE, SIZE), (206, 172, 120, 255))
    px = img.load()
    spacing = 48
    radius = 15.0
    for y in range(SIZE):
        for x in range(SIZE):
            cx = (x // spacing) * spacing + spacing / 2
            cy = (y // spacing) * spacing + spacing / 2
            if (x - cx) ** 2 + (y - cy) ** 2 < radius * radius:
                px[x, y] = (206, 172, 120, 0)  # 孔：透明
    path = OUT / "lattice.png"
    img.save(path, optimize=True)
    print(f"lattice.png：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


# ---- 立方体贴图：中性影棚 -------------------------------------------------

def wall(top, bottom, strip=None):
    """一面墙：上下竖直渐变，可选在中间竖一条亮光带（影棚柔光灯条），供金属映出。"""
    img = Image.new("RGB", (SIZE, SIZE))
    px = img.load()
    for y in range(SIZE):
        row = lerp(top, bottom, y / (SIZE - 1))
        for x in range(SIZE):
            px[x, y] = row
    if strip:
        x0, x1, color = strip
        for y in range(SIZE):
            for x in range(x0, x1):
                t = 1.0 - abs((x - (x0 + x1) / 2) / ((x1 - x0) / 2))
                px[x, y] = lerp(px[x, y], color, max(t, 0.0))
    return img


def flat(color):
    return Image.new("RGB", (SIZE, SIZE), color)


def make_skybox():
    wall_top = (150, 156, 166)
    wall_bottom = (96, 100, 110)
    softbox = (245, 246, 250)
    half = SIZE // 2
    faces = [
        wall(wall_top, wall_bottom, (half - 26, half + 26, softbox)),  # +X：带柔光灯条
        wall(wall_top, wall_bottom, (40, 70, softbox)),                # -X
        flat((236, 238, 244)),                                         # +Y 顶：柔光
        flat((52, 54, 60)),                                            # -Y 底：压暗
        wall(wall_top, wall_bottom, (half - 30, half + 30, softbox)),  # +Z：带柔光灯条
        wall(wall_top, wall_bottom),                                   # -Z
    ]
    stacked = Image.new("RGB", (SIZE, SIZE * 6))
    for i, face in enumerate(faces):
        stacked.paste(face, (0, i * SIZE))
    path = OUT / "skybox.png"
    stacked.save(path, optimize=True)
    print(f"skybox.png：{stacked.size[0]}x{stacked.size[1]}，{path.stat().st_size // 1024} KB")


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    make_studs_normal()
    make_lattice()
    make_skybox()


if __name__ == "__main__":
    main()

# -*- coding: utf-8 -*-
"""一键生成第 22 章的美术资产：一张竖摞六面的立方体贴图 PNG。

    py -3.11 scripts/make_ch22_assets.py

立方体贴图的六张面按 Bevy/wgpu 约定竖着摞成一张图，顺序为
+X, -X, +Y, -Y, +Z, -Z。这里画一座程式化的「暖阁」环境：
顶面是暖白天光，底面是暗木地板，四壁是带灯笼暖光的渐变——
专门给那颗镜面金属球一个值得照的世界。产物输出到
code/ch22-lighting/assets/textures/skybox.png。
"""

from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "code" / "ch22-lighting" / "assets" / "textures"
FACE = 256  # 每张面边长，2 的幂


def lerp(a, b, t):
    return tuple(round(a[i] + (b[i] - a[i]) * t) for i in range(3))


def wall(top, bottom, glow=None):
    """一面墙：上下竖直渐变，可选在下方中央点一团灯笼暖光。"""
    img = Image.new("RGB", (FACE, FACE))
    px = img.load()
    for y in range(FACE):
        row = lerp(top, bottom, y / (FACE - 1))
        for x in range(FACE):
            px[x, y] = row
    if glow:
        cx, cy, radius, color = glow
        for y in range(FACE):
            for x in range(FACE):
                d = ((x - cx) ** 2 + (y - cy) ** 2) ** 0.5
                if d < radius:
                    t = (1.0 - d / radius) ** 2
                    base = px[x, y]
                    px[x, y] = lerp(base, color, t)
    return img


def flat(top, bottom):
    return wall(top, bottom)


def main():
    OUT.mkdir(parents=True, exist_ok=True)

    sky_top = (236, 224, 198)
    sky_bottom = (150, 138, 120)
    wall_top = (120, 96, 78)
    wall_bottom = (54, 40, 34)
    lantern = (255, 196, 120)

    faces = [
        wall(wall_top, wall_bottom, (FACE // 2, FACE - 70, 90, lantern)),  # +X
        wall(wall_top, wall_bottom, (FACE // 2, FACE - 70, 90, lantern)),  # -X
        flat(sky_top, sky_top),                                            # +Y 顶：天光
        flat((40, 30, 26), (24, 18, 16)),                                  # -Y 底：暗木
        wall(wall_top, wall_bottom, (FACE // 2, FACE - 80, 110, lantern)), # +Z
        wall(sky_bottom, wall_bottom),                                     # -Z
    ]

    stacked = Image.new("RGB", (FACE, FACE * 6))
    for i, face in enumerate(faces):
        stacked.paste(face, (0, i * FACE))
    path = OUT / "skybox.png"
    stacked.save(path, optimize=True)
    print(f"skybox.png：{stacked.size[0]}x{stacked.size[1]}，{path.stat().st_size // 1024} KB")


if __name__ == "__main__":
    main()

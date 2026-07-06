# -*- coding: utf-8 -*-
"""一键重建第 24 章的全部图片资产（纯 PIL 合成，确定性随机种子，可反复重建）。

    py -3.11 scripts/make_ch24_assets.py

产物（输出到 code/ch24-materials/assets/textures/）：
  studio_cubemap.png   影棚六面（256×1536，+X -X +Y -Y +Z -Z）：中性灰墙 + 三块柔光箱 +
                       天窗亮板——给金属、清漆和玻璃一个值得照的世界（24.1 起全章地基）
  gong_base.png        铜锣底色（512²，sRGB）：铜黄 + 锈斑，与 ORM 图逐像素对位
  gong_orm.png         铜锣 ORM 图（512²，线性数据）：R=环境光遮蔽、G=粗糙度、B=金属度
                       ——glTF 同款打包惯例（24.4）
  carve_height.png     云纹浮雕深度图（512²，灰度）：按 depth_map 约定白=深、黑=凸（24.6 视差）
  carve_normal.png     同一高度场差分出的切线空间法线图（OpenGL 约定，24.5）
  bamboo_alpha.png     竹枝叶影（512²，RGBA）：alpha 通道即形状（24.9 透明七款）
  lantern_sign.png     《琉璃记》戏牌（512×256）：黑底亮纹，emissive_texture 素材（24.3）
  banner.png           复制 ch21 的雷字旗（24.12 uv_transform 修倒旗）
"""

import math
import random
import shutil
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter

sys.stdout.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "code" / "ch24-materials" / "assets" / "textures"

# 竖条 cubemap 的六面顺序即数组层顺序：+X -X +Y -Y +Z -Z（wgpu 约定，ch22 同款）
FACES = ["+x", "-x", "+y", "-y", "+z", "-z"]


def save(img: Image.Image, name: str) -> None:
    path = OUT / name
    img.save(path, optimize=True)
    print(f"{name}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


# ---------------------------------------------------------------- 影棚 cubemap

def softbox(draw: ImageDraw.ImageDraw, size: int, box: tuple, color: tuple) -> None:
    """一块柔光箱：亮面板 + 四周一圈渐变衬边（模拟灯箱柔边）。"""
    x0, y0, x1, y1 = (int(v * size) for v in box)
    for i, a in enumerate((40, 90, 160)):
        pad = (3 - i) * size // 48
        c = tuple(int(ch * a / 255) for ch in color)
        draw.rectangle([x0 - pad, y0 - pad, x1 + pad, y1 + pad], fill=c)
    draw.rectangle([x0, y0, x1, y1], fill=color)
    # 面板上两根格条，反射里能读出"这是灯箱"
    w = max(2, size // 128)
    draw.line([(x0, (y0 + y1) // 2), (x1, (y0 + y1) // 2)], fill=tuple(int(c * 0.82) for c in color), width=w)
    draw.line([((x0 + x1) // 2, y0), ((x0 + x1) // 2, y1)], fill=tuple(int(c * 0.82) for c in color), width=w)


def studio_face(size: int, face: str) -> Image.Image:
    """影棚一面：灰墙渐变打底，各面镶不同的光源，方向感靠色温区分。"""
    img = Image.new("RGB", (size, size))
    px = img.load()
    if face == "-y":                       # 地面：深灰，中心略亮
        for y in range(size):
            for x in range(size):
                d = math.hypot(x - size / 2, y - size / 2) / (size / 2)
                v = int(46 - 14 * min(d, 1.0))
                px[x, y] = (v, v, v + 2)
        return img
    if face == "+y":                       # 天顶：暗顶棚 + 一大块天窗亮板
        for y in range(size):
            for x in range(size):
                px[x, y] = (30, 30, 33)
        draw = ImageDraw.Draw(img)
        softbox(draw, size, (0.22, 0.22, 0.78, 0.78), (255, 253, 246))
        return img
    # 四面墙：上亮下暗的中性灰（摄影棚无缝背景纸的感觉）
    for y in range(size):
        v = int(120 - 52 * (y / (size - 1)))
        for x in range(size):
            px[x, y] = (v, v, v + 2)
    draw = ImageDraw.Draw(img)
    if face == "+x":                       # 右墙：大暖箱
        softbox(draw, size, (0.18, 0.20, 0.82, 0.62), (255, 214, 158))
    elif face == "-x":                     # 左墙：竖长冷箱
        softbox(draw, size, (0.34, 0.10, 0.66, 0.72), (168, 205, 255))
    elif face == "+z":                     # 相机初始背对的墙：中性宽箱
        softbox(draw, size, (0.14, 0.26, 0.86, 0.56), (240, 240, 238))
    else:                                  # -z：素墙，只挂一条暗踢脚
        draw.rectangle([0, int(size * 0.86), size, size], fill=(58, 58, 60))
    return img


def make_studio_strip(size: int) -> None:
    strip = Image.new("RGB", (size, size * 6))
    for i, face in enumerate(FACES):
        strip.paste(studio_face(size, face), (0, i * size))
    save(strip, "studio_cubemap.png")


# ---------------------------------------------------------------- 铜锣 ORM

def rust_mask(size: int) -> Image.Image:
    """锈斑蒙版（L 模式，255=锈）：确定性斑块 + 模糊 + 阈值出硬边缘。"""
    rng = random.Random(2404)
    m = Image.new("L", (size, size), 0)
    d = ImageDraw.Draw(m)
    for _ in range(46):
        cx, cy = rng.uniform(0, size), rng.uniform(0, size)
        r = rng.uniform(size * 0.02, size * 0.09)
        # 每个斑块由一串小圆堆出不规则外形
        for _ in range(14):
            ox, oy = rng.gauss(0, r * 0.55), rng.gauss(0, r * 0.55)
            rr = rng.uniform(r * 0.25, r * 0.6)
            d.ellipse([cx + ox - rr, cy + oy - rr, cx + ox + rr, cy + oy + rr], fill=255)
    m = m.filter(ImageFilter.GaussianBlur(size * 0.008))
    return m.point(lambda v: 255 if v > 110 else 0)


def make_gong(size: int) -> None:
    mask = rust_mask(size)
    mpx = mask.load()
    rng = random.Random(2405)

    base = Image.new("RGB", (size, size))
    orm = Image.new("RGB", (size, size))
    bpx, opx = base.load(), orm.load()
    for y in range(size):
        for x in range(size):
            rusty = mpx[x, y] > 0
            jitter = rng.randint(-8, 8)
            if rusty:
                # 锈：红褐带绿锈点；AO 压到 0.2、粗糙 0.9、非金属
                g_extra = 18 if rng.random() < 0.18 else 0
                bpx[x, y] = (118 + jitter, 66 + jitter // 2 + g_extra, 40)
                opx[x, y] = (50, 230, 0)
            else:
                # 铜：暖黄；AO 全亮、较光滑、纯金属
                bpx[x, y] = (196 + jitter, 148 + jitter // 2, 72)
                opx[x, y] = (255, 64, 255)
    # 底色上加几圈旋纹提示这是打出来的锣（纯装饰，不进 ORM）
    d = ImageDraw.Draw(base)
    for r in range(size // 10, size // 2, size // 10):
        d.ellipse([size / 2 - r, size / 2 - r, size / 2 + r, size / 2 + r],
                  outline=(178, 132, 60), width=1)
    save(base, "gong_base.png")
    save(orm, "gong_orm.png")


# ---------------------------------------------------------------- 云纹浮雕

def cloud_height(size: int) -> list:
    """云纹高度场（0..1，1=凸起）：几组螺旋卷云 + 回纹边框，float 网格。"""
    h = [[0.0] * size for _ in range(size)]

    def stamp(cx: float, cy: float, r0: float, turns: float, width: float) -> None:
        steps = int(turns * 120)
        for i in range(steps):
            t = i / steps
            ang = t * turns * 2 * math.pi
            r = r0 * (1 - 0.78 * t)
            x, y = cx + r * math.cos(ang), cy + r * math.sin(ang)
            w = width * (1 - 0.4 * t)
            x0, x1 = max(0, int(x - w - 2)), min(size - 1, int(x + w + 2))
            y0, y1 = max(0, int(y - w - 2)), min(size - 1, int(y + w + 2))
            for yy in range(y0, y1 + 1):
                for xx in range(x0, x1 + 1):
                    d = math.hypot(xx - x, yy - y)
                    if d < w + 2:
                        v = max(0.0, min(1.0, (w + 2 - d) / 2.5))
                        if v > h[yy][xx]:
                            h[yy][xx] = v

    s = size
    # 四组卷云，两大两小，错落
    stamp(s * 0.32, s * 0.34, s * 0.17, 2.4, s * 0.018)
    stamp(s * 0.70, s * 0.62, s * 0.19, 2.6, s * 0.018)
    stamp(s * 0.72, s * 0.22, s * 0.10, 2.0, s * 0.014)
    stamp(s * 0.26, s * 0.74, s * 0.11, 2.0, s * 0.014)
    # 回纹边框：一圈方折线
    m, w = int(s * 0.055), int(s * 0.012)
    seg = (s - 2 * m) // 8
    for k in range(8):
        x = m + k * seg
        pts = [(x, m), (x + seg // 2, m), (x + seg // 2, m + seg // 3), (x + seg, m + seg // 3)]
        pts_b = [(p[0], s - p[1]) for p in pts]
        for pp in (pts, pts_b):
            for (xa, ya), (xb, yb) in zip(pp, pp[1:]):
                x0, x1 = sorted((int(xa), int(xb)))
                y0, y1 = sorted((int(ya), int(yb)))
                for yy in range(max(0, y0 - w), min(s, y1 + w + 1)):
                    for xx in range(max(0, x0 - w), min(s, x1 + w + 1)):
                        h[yy][xx] = max(h[yy][xx], 0.85)
    return h


def make_carve(size: int) -> None:
    h = cloud_height(size)
    # 轻度平滑两遍，免得法线差分出楼梯
    for _ in range(2):
        nh = [[0.0] * size for _ in range(size)]
        for y in range(size):
            for x in range(size):
                acc = cnt = 0.0
                for dy in (-1, 0, 1):
                    for dx in (-1, 0, 1):
                        xx, yy = x + dx, y + dy
                        if 0 <= xx < size and 0 <= yy < size:
                            acc += h[yy][xx]
                            cnt += 1
                nh[y][x] = acc / cnt
        h = nh

    # 深度图：Bevy 的 depth_map 约定白=深、黑=凸——高度取反
    depth = Image.new("L", (size, size))
    dpx = depth.load()
    for y in range(size):
        for x in range(size):
            dpx[x, y] = int(255 * (1.0 - h[y][x]))
    save(depth, "carve_height.png")

    # 法线图：高度场中心差分，OpenGL 约定（绿通道朝图上方）
    strength = 6.0
    normal = Image.new("RGB", (size, size))
    npx = normal.load()
    for y in range(size):
        for x in range(size):
            xm, xp = max(0, x - 1), min(size - 1, x + 1)
            ym, yp = max(0, y - 1), min(size - 1, y + 1)
            dx = (h[y][xp] - h[y][xm]) * 0.5 * strength
            dy = (h[yp][x] - h[ym][x]) * 0.5 * strength
            inv = 1.0 / math.sqrt(dx * dx + dy * dy + 1.0)
            n = (-dx * inv, dy * inv, inv)
            npx[x, y] = (int((n[0] * 0.5 + 0.5) * 255),
                         int((n[1] * 0.5 + 0.5) * 255),
                         int((n[2] * 0.5 + 0.5) * 255))
    save(normal, "carve_normal.png")


# ---------------------------------------------------------------- 竹影纱

def make_bamboo(size: int) -> None:
    rng = random.Random(2406)
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)

    def leaf(cx: float, cy: float, ang: float, ln: float, color: tuple) -> None:
        pts = []
        for t in (0.0, 0.18, 0.5, 0.82, 1.0):
            w = math.sin(t * math.pi) * ln * 0.16
            x = cx + math.cos(ang) * ln * (t - 0.5)
            y = cy + math.sin(ang) * ln * (t - 0.5)
            pts.append((x + math.cos(ang + math.pi / 2) * w, y + math.sin(ang + math.pi / 2) * w))
        for t in (0.82, 0.5, 0.18):
            w = math.sin(t * math.pi) * ln * 0.16
            x = cx + math.cos(ang) * ln * (t - 0.5)
            y = cy + math.sin(ang) * ln * (t - 0.5)
            pts.append((x - math.cos(ang + math.pi / 2) * w, y - math.sin(ang + math.pi / 2) * w))
        d.polygon(pts, fill=color)

    # 两根斜竹竿
    for x0, tilt in ((size * 0.30, -0.06), (size * 0.62, 0.05)):
        seg = size // 5
        for i in range(6):
            y0, y1 = i * seg, (i + 1) * seg
            xa, xb = x0 + tilt * y0, x0 + tilt * y1
            d.line([(xa, y0), (xb, y1)], fill=(52, 88, 48, 255), width=max(4, size // 52))
            d.ellipse([xb - size // 90, y1 - size // 90, xb + size // 90, y1 + size // 90],
                      fill=(38, 66, 36, 255))
    # 叶子成簇
    for _ in range(34):
        cx, cy = rng.uniform(size * 0.06, size * 0.94), rng.uniform(size * 0.05, size * 0.95)
        ang = rng.uniform(-0.9, 0.9) + (0 if rng.random() < 0.5 else math.pi)
        ln = rng.uniform(size * 0.10, size * 0.20)
        g = rng.randint(84, 126)
        leaf(cx, cy, ang, ln, (30, g, 34, 255))
    save(img, "bamboo_alpha.png")


# ---------------------------------------------------------------- 戏牌

def make_lantern_sign(w: int, h: int) -> None:
    img = Image.new("RGB", (w, h), (6, 5, 8))
    d = ImageDraw.Draw(img)
    # 边框双线
    for pad, wd in ((h // 18, 4), (h // 9, 2)):
        d.rectangle([pad, pad, w - pad, h - pad], outline=(255, 236, 170), width=wd)
    # 一弯月牙（两圆相减）
    r = h * 0.30
    cx, cy = w * 0.26, h * 0.48
    d.ellipse([cx - r, cy - r, cx + r, cy + r], fill=(255, 244, 196))
    d.ellipse([cx - r + r * 0.5, cy - r - r * 0.18, cx + r + r * 0.5, cy + r - r * 0.18],
              fill=(6, 5, 8))
    # 三盏琉璃盏：圆肚 + 高脚
    for i, lx in enumerate((0.55, 0.71, 0.87)):
        gx, gy, gr = w * lx, h * 0.52, h * 0.13
        d.ellipse([gx - gr, gy - gr, gx + gr, gy + gr], outline=(180, 235, 255), width=4)
        d.ellipse([gx - gr * 0.45, gy - gr * 0.45, gx + gr * 0.1, gy + gr * 0.1],
                  fill=(180, 235, 255))
        d.line([(gx, gy + gr), (gx, gy + gr * 1.8)], fill=(180, 235, 255), width=4)
        d.line([(gx - gr * 0.5, gy + gr * 1.8), (gx + gr * 0.5, gy + gr * 1.8)],
               fill=(180, 235, 255), width=4)
    save(img, "lantern_sign.png")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    make_studio_strip(256)
    make_gong(512)
    make_carve(512)
    make_bamboo(512)
    make_lantern_sign(512, 256)
    # 雷字旗从 ch21 原样复用（那边的资产脚本是它的出处）。
    # banner_tile.png 是同一张图的第二份拷贝：24.11 的瓦墙要配 Repeat 采样器装载，
    # 而一条路径只认一套 loader settings（23.5 的规矩）——两种开法，两个路径
    src = ROOT / "code" / "ch21-meshes" / "assets" / "textures" / "banner.png"
    shutil.copyfile(src, OUT / "banner.png")
    shutil.copyfile(src, OUT / "banner_tile.png")
    print(f"banner.png / banner_tile.png：复制自 ch21，各 {(OUT / 'banner.png').stat().st_size // 1024} KB")


if __name__ == "__main__":
    main()

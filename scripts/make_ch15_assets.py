# -*- coding: utf-8 -*-
"""ch15 美术资产一键重建：阿燕/梢公连环画帧图 + 装裱画框 + 平铺贴片 + 小道具。

用法：py -3.11 scripts/make_ch15_assets.py [--preview]
产物全部写入 code/ch15-sprites/assets/，确定性生成，可随时重建。
所有图都是原始像素尺寸（不预放大）——本章在引擎里用 Nearest 采样放大，这正是教学点。
--preview 额外输出一张放大的拼合预览图到 target/ch15-preview.png，便于人工目检。
"""

import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parent.parent
CRATE = ROOT / "code" / "ch15-sprites"
ASSETS = CRATE / "assets"

# ---------------------------------------------------------------- 调色板（沿用 ch14 戏装用色）

PALETTE = {
    ".": (0, 0, 0, 0),          # 透明
    "K": (35, 37, 46, 255),     # 发黑
    "k": (70, 74, 92, 255),     # 发光泽
    "F": (244, 222, 190, 255),  # 肤色
    "f": (214, 178, 140, 255),  # 肤色暗部
    "E": (35, 37, 46, 255),     # 眼睛
    "R": (200, 58, 58, 255),    # 正红（戏服，与 ch14 色块同源）
    "M": (140, 27, 39, 255),    # 深红（衣褶）
    "m": (104, 20, 30, 255),    # 红最暗（下摆/衣缘）
    "G": (232, 184, 75, 255),   # 金（腰带/剑柄/发绳）
    "H": (138, 106, 31, 255),   # 暗金
    "W": (242, 248, 255, 255),  # 冰白高光
    "B": (159, 208, 232, 255),  # 青霜剑刃
    "D": (91, 143, 174, 255),   # 剑刃暗部/鞘
    "T": (84, 66, 84, 255),     # 裤
    "N": (50, 42, 52, 255),     # 靴
    "U": (112, 126, 142, 255),  # 梢公布衣
    "u": (76, 88, 102, 255),    # 布衣暗部
    "Y": (226, 192, 110, 255),  # 草帽黄
    "y": (170, 138, 70, 255),   # 草帽暗
    "P": (122, 86, 50, 255),    # 橹杆/木
    "p": (86, 60, 36, 255),     # 木暗部
}


def blit(img: Image.Image, rows: list[str], ox: int = 0, oy: int = 0) -> None:
    """把 ASCII 像素稿盖到 img 上（透明字符跳过）。"""
    for dy, row in enumerate(rows):
        for dx, ch in enumerate(row):
            if ch != ".":
                img.putpixel((ox + dx, oy + dy), PALETTE[ch])


def from_ascii(rows: list[str]) -> Image.Image:
    img = Image.new("RGBA", (len(rows[0]), len(rows)), (0, 0, 0, 0))
    blit(img, rows)
    return img


# ---------------------------------------------------------------- 阿燕（32×40 / 帧）
# 第一行 6 帧是正面原地（呼吸/眨眼/剑上寒光），第二行 6 帧是侧身走路（朝右）。
# 正面与侧身各自分层拼装：上身 28 行（y0..27），腿 12 行（y28..39），靴底齐 y39。

AYAN_FRONT_TOP = [
    "................................",
    "..............KKK...............",
    ".............KkKKK..............",
    "..............KGK...............",
    "...........KKKKKKKK.............",
    "..........KKKKKKKKKK............",
    "..........KKKKKKKKKK............",
    "........G.KKKKKKKKKK............",
    "........GGKKFFFFFFKK............",
    "........D.KFFFFFFFFK............",
    "........D.KFFFFFFFFK............",
    "........D.KFFEFFEFFK............",
    "........D..FFFFFFFF.............",
    "........D..fFFFFFFf.............",
    "........D...ffffff..............",
    "........DRRRRRRRRRRR............",
    ".......RRRRRRRRRRRRRR...........",
    "......RRRRRRRRRRRRRRRR..........",
    "......RRRMRRRRRRRRMRRR..........",
    "......RRRMRRRRRRRRMRRR..........",
    "......fRRMRRRRRRRRMRRf..........",
    "......fRRGGGGGGGGGGRRf..........",
    ".......RRMRRRRRRRRMRR...........",
    ".......RRMRRRRRRRRMRR...........",
    ".......RMRRRRRRRRRRMR...........",
    ".......RMRRRRRRRRRRMR...........",
    ".......MmRRRRRRRRRRmM...........",
    ".......mmmmmmmmmmmmmm...........",
]

# 站立腿（正面 idle 用）
AYAN_LEGS_IDLE = [
    "........TTT....TTT..............",
    "........TTT....TTT..............",
    "........TTT....TTT..............",
    "........TTT....TTT..............",
    "........TTT....TTT..............",
    "........TTT....TTT..............",
    "........TTT....TTT..............",
    "........TTT....TTT..............",
    "........TTT....TTT..............",
    ".......NNNN....NNNN.............",
    ".......NNNN....NNNN.............",
    "......NNNNN....NNNNN............",
]

# 侧身上身（朝右）：发髻偏后，背上斜挎青霜剑（剑柄在肩后上方）
AYAN_SIDE_TOP = [
    "................................",
    "............KKK.................",
    "...........KkKKK................",
    "............KGK.................",
    "..........KKKKKKKK..............",
    ".........KKKKKKKKKK.............",
    ".........KKKKKKKKKKK............",
    ".........KKKKKFFFFFF............",
    ".........KKKKFFFFFFF............",
    ".........KKKKFFEFFFF............",
    ".........KKKKFFFFFFf............",
    "........G.KKKFFFFFf.............",
    "........GG.KFFFFFf..............",
    ".........DD.ffff................",
    "..........DDRRRRR...............",
    ".........RRDDRRRRR..............",
    ".........RRRDDRRRRR.............",
    "........RRRRRDDRRRR.............",
    "........RRRRRRDDRRR.............",
    "........RRRMRRRDDRR.............",
    "........RRRMRRRRDD..............",
    "........RRGGGGGGGGG.............",
    "........RRRRRRRRRRR.............",
    "........RRRRRRRRRRR.............",
    "........RRMRRRRRRMR.............",
    "........RRMRRRRRRMR.............",
    "........RMmRRRRRRmM.............",
    "........mmmmmmmmmmm.............",
]

# 侧身走路六帧腿姿（接触→下沉→经过→另一侧接触→下沉→经过），朝右，靴底齐 y39
AYAN_SIDE_LEGS = [
    [  # 0 接触：前腿伸出、后腿蹬直
        "..........TTTTTT................",
        ".........TTT..TTT...............",
        ".........TTT...TTT..............",
        "........TTT.....TTT.............",
        "........TTT......TTT............",
        ".......TTT........TTT...........",
        ".......TTT.........TTT..........",
        "......TTT...........TTT.........",
        "......TTT............TTT........",
        ".....NNN..............NNNN......",
        ".....NNN..............NNNNN.....",
        "....NNNN..............NNNNN.....",
    ],
    [  # 1 下沉：步幅收小、重心略降（配 bob+1）
        "..........TTTTTT................",
        ".........TTT..TTT...............",
        ".........TTT...TTT..............",
        ".........TTT....TTT.............",
        "........TTT......TTT............",
        "........TTT......TTT............",
        ".......TTT........TTT...........",
        ".......TTT........TTT...........",
        "......TTT..........TTT..........",
        "......NNN..........NNNN.........",
        ".....NNNN..........NNNNN........",
        ".....NNNN..........NNNNN........",
    ],
    [  # 2 经过：后腿摆到身下，两腿几乎并拢
        "..........TTTTTT................",
        "..........TTTTTT................",
        "..........TTTTT.................",
        "..........TTTTT.................",
        "..........TTTT..................",
        "..........TTTT..................",
        ".........TTTTT..................",
        ".........TT.TTT.................",
        ".........TT..TTT................",
        "........NNN...NNN...............",
        "........NNN...NNNN..............",
        ".......NNNN...NNNN..............",
    ],
    [  # 3 接触（另一侧）
        "..........TTTTTT................",
        ".........TTT..TTT...............",
        ".........TTT...TTT..............",
        "........TTT.....TTT.............",
        "........TTT......TTT............",
        ".......TTT........TTT...........",
        ".......TTT.........TTT..........",
        "......TTT...........TTT.........",
        "......TTT............TTT........",
        ".....NNNN.............NNN.......",
        ".....NNNNN............NNN.......",
        ".....NNNNN...........NNNN.......",
    ],
    [  # 4 下沉
        "..........TTTTTT................",
        ".........TTT..TTT...............",
        ".........TTT...TTT..............",
        ".........TTT....TTT.............",
        "........TTT......TTT............",
        "........TTT......TTT............",
        ".......TTT........TTT...........",
        ".......TTT........TTT...........",
        "......TTT..........TTT..........",
        "......NNNN.........NNN..........",
        ".....NNNNN.........NNN..........",
        ".....NNNNN........NNNN..........",
    ],
    [  # 5 经过
        "..........TTTTTT................",
        "..........TTTTTT................",
        "..........TTTTT.................",
        "..........TTTTT.................",
        "..........TTTT..................",
        "..........TTTT..................",
        ".........TTTTT..................",
        ".........TTT.TT.................",
        ".........TTT..TT................",
        "........NNNN..NNN...............",
        "........NNNN..NNN...............",
        ".......NNNNN..NNN...............",
    ],
]

# 飘带（腰侧），盖在上身之上；正面是belt 旁垂落的绦带，侧身拖在身后
RIBBON_FRONT = {
    "A": [(21, 22, "G"), (21, 23, "G"), (22, 24, "G"), (22, 25, "H"), (21, 26, "H")],
    "B": [(21, 22, "G"), (22, 23, "G"), (22, 24, "G"), (23, 25, "H"), (22, 26, "H")],
}
RIBBON_SIDE = {
    "A": [(7, 21, "G"), (6, 22, "G"), (5, 23, "G"), (4, 23, "H"), (3, 24, "H")],
    "B": [(7, 21, "G"), (6, 21, "G"), (5, 20, "G"), (4, 20, "H"), (3, 19, "H")],
}


def ayan_front_frame(bob: int, blink: bool, glint_y: int, phase: str) -> Image.Image:
    """正面一帧：上身随呼吸浮动 bob 像素，眼睛可眨，寒光沿剑刃游走。"""
    img = Image.new("RGBA", (32, 40), (0, 0, 0, 0))
    blit(img, AYAN_FRONT_TOP, 0, bob)
    blit(img, AYAN_LEGS_IDLE, 0, 28)
    for x, y, ch in RIBBON_FRONT[phase]:
        img.putpixel((x, y + bob), PALETTE[ch])
    if blink:
        # 眼睛那一行（相对上身 y=11）改画成闭眼线
        for x in (13, 16):
            img.putpixel((x, 11 + bob), PALETTE["f"])
    img.putpixel((8, glint_y + bob), PALETTE["W"])
    return img


def ayan_side_frame(legs: list[str], bob: int, phase: str) -> Image.Image:
    img = Image.new("RGBA", (32, 40), (0, 0, 0, 0))
    blit(img, AYAN_SIDE_TOP, 0, bob)
    blit(img, legs, 0, 28)
    for x, y, ch in RIBBON_SIDE[phase]:
        img.putpixel((x, y + bob), PALETTE[ch])
    return img


def make_ayan_frames() -> list[Image.Image]:
    """12 帧：0..5 正面原地（呼吸/眨眼/寒光游走），6..11 侧身走路（朝右）。"""
    frames = []
    # idle：呼吸 bob 0,0,1,1,0,0；第 3、4 帧眨眼；寒光沿剑刃下移
    idle_spec = [
        (0, False, 9), (0, False, 10), (1, False, 11),
        (1, True, 12), (0, True, 13), (0, False, 14),
    ]
    for i, (bob, blink, gy) in enumerate(idle_spec):
        frames.append(ayan_front_frame(bob, blink, gy, "A" if i % 2 == 0 else "B"))
    # walk：六帧腿姿；下沉帧（1、4）身体降 1px；飘带两相位交替
    walk_bob = [0, 1, 0, 0, 1, 0]
    for i in range(6):
        frames.append(ayan_side_frame(AYAN_SIDE_LEGS[i], walk_bob[i],
                                      "B" if i % 2 == 0 else "A"))
    return frames


# ---------------------------------------------------------------- 梢公（32×40 / 帧，摇橹四帧）

def shaogong_frame(lean: int, oar_dx: int) -> Image.Image:
    """梢公一帧：lean 为躯干前倾像素，oar_dx 为橹末端的横向摆幅。"""
    img = Image.new("RGBA", (32, 40), (0, 0, 0, 0))
    body = [
        "................................",
        "..........YYYYYYYY..............",
        ".........YYYYYYYYYY.............",
        "........yyyyyyyyyyyy............",
        "...........yYYYy...............",
        "...........FFFFF................",
        "...........FEFEF................",
        "...........fFFFf................",
        "............fff.................",
        "..........UUUUUUU...............",
        ".........UUUUUUUUU..............",
        "........UUUUUUUUUUU.............",
        "........UUUuUUUuUUU.............",
        "........UUUuUUUuUUU.............",
        "........fUUuUUUuUUf.............",
        ".........UUuUUUuUU..............",
        ".........UUUUUUUUU..............",
        ".........UUUUUUUUU..............",
        ".........uUUUUUUUu..............",
        ".........uuuuuuuuu..............",
        "..........TTT.TTT...............",
        "..........TTT.TTT...............",
        "..........TTT.TTT...............",
        ".........NNNN.NNNN..............",
        "........NNNNN.NNNNN.............",
    ]
    blit(img, body, lean, 6)
    d = ImageDraw.Draw(img)
    # 橹：从双手（约 9+lean+9, 6+14）斜向右下水面
    hx, hy = 13 + lean, 19
    ex, ey = 24 + oar_dx, 38
    d.line([hx, hy, ex, ey], fill=PALETTE["P"], width=2)
    d.line([ex - 1, ey - 4, ex + 1, ey], fill=PALETTE["p"], width=3)  # 橹板
    # 双手握杆
    img.putpixel((hx, hy), PALETTE["f"])
    img.putpixel((hx + 1, hy + 1), PALETTE["F"])
    return img


def make_shaogong_frames() -> list[Image.Image]:
    # 推橹→压橹→回橹→提橹
    spec = [(0, 0), (1, 3), (1, 6), (0, 3)]
    return [shaogong_frame(lean, dx) for lean, dx in spec]


# ---------------------------------------------------------------- 图集组装

def sheet(frames: list[Image.Image], columns: int) -> Image.Image:
    fw, fh = frames[0].size
    rows = (len(frames) + columns - 1) // columns
    img = Image.new("RGBA", (fw * columns, fh * rows), (0, 0, 0, 0))
    for i, fr in enumerate(frames):
        img.paste(fr, ((i % columns) * fw, (i // columns) * fh))
    return img


# ---------------------------------------------------------------- 装裱画框（48×48，供九宫格切片）

CORNER = [  # 8×8 回纹角花（左上方向）
    "GGGGGGGG",
    "G......G",
    "G.GGGG.G",
    "G.G..G.G",
    "G.G.GG.G",
    "G.G....G",
    "G.GGGGGG",
    "G.......",
]


def make_scroll_panel(size: int = 48, border: int = 12) -> Image.Image:
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    # 外缘深木 → 木框 → 金线 → 帛心
    d.rectangle([0, 0, size - 1, size - 1], fill=(62, 44, 30, 255))
    d.rectangle([2, 2, size - 3, size - 3], fill=(107, 74, 47, 255))
    d.rectangle([border - 2, border - 2, size - border + 1, size - border + 1],
                fill=PALETTE["H"])
    d.rectangle([border - 1, border - 1, size - border, size - border],
                fill=(139, 98, 62, 255))
    d.rectangle([border, border, size - border - 1, size - border - 1],
                fill=(232, 217, 176, 255))
    # 帛心四角淡纹
    for x, y in [(border + 2, border + 2), (size - border - 4, border + 2),
                 (border + 2, size - border - 4), (size - border - 4, size - border - 4)]:
        d.rectangle([x, y, x + 1, y + 1], fill=(204, 186, 142, 255))
    # 四角回纹（旋转盖章）
    corner = from_ascii(CORNER)
    img.alpha_composite(corner, (1, 1))
    img.alpha_composite(corner.transpose(Image.FLIP_LEFT_RIGHT), (size - 9, 1))
    img.alpha_composite(corner.transpose(Image.FLIP_TOP_BOTTOM), (1, size - 9))
    img.alpha_composite(corner.transpose(Image.ROTATE_180), (size - 9, size - 9))
    return img


# ---------------------------------------------------------------- 平铺贴片（16×16）

WATER_TILE = [
    "................",
    "................",
    "................",
    ".WDD............",
    "DD..DD..........",
    "......D.........",
    "................",
    "................",
    "..........WDD...",
    ".........DD..DD.",
    "...............D",
    "................",
    "....DDD.........",
    "...D...D........",
    "................",
    "................",
]


def make_water_tile() -> Image.Image:
    img = Image.new("RGBA", (16, 16), (24, 52, 78, 255))
    blit(img, WATER_TILE)
    return img


def make_plank_tile() -> Image.Image:
    img = Image.new("RGBA", (16, 16), (94, 74, 50, 255))
    d = ImageDraw.Draw(img)
    d.line([0, 7, 15, 7], fill=(70, 54, 36, 255))
    d.line([0, 15, 15, 15], fill=(60, 46, 30, 255))
    d.line([4, 0, 4, 7], fill=(78, 60, 40, 255))
    d.line([11, 8, 11, 15], fill=(78, 60, 40, 255))
    for x, y in [(2, 3), (9, 4), (7, 11), (13, 12)]:
        img.putpixel((x, y), (104, 82, 56, 255))
    return img


# ---------------------------------------------------------------- 小道具

LANTERN = [  # 与 ch14 同款灯笼（16×16）
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


def make_boat() -> Image.Image:
    """乌篷小船（64×24）。"""
    img = Image.new("RGBA", (64, 24), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    # 船身：两头微翘
    d.polygon([(2, 12), (61, 12), (54, 22), (9, 22)], fill=(86, 60, 36, 255))
    d.polygon([(2, 12), (8, 12), (6, 16)], fill=(122, 86, 50, 255))
    d.polygon([(55, 12), (61, 12), (58, 18)], fill=(122, 86, 50, 255))
    d.line([2, 12, 61, 12], fill=(122, 86, 50, 255), width=2)
    d.line([9, 21, 54, 21], fill=(56, 40, 24, 255))
    # 船头乌篷：半椭圆拱顶 + 竹篾纹
    d.pieslice([8, 2, 32, 22], 180, 360, fill=(58, 50, 42, 255))
    d.pieslice([11, 5, 29, 22], 180, 360, fill=(88, 78, 62, 255))
    d.pieslice([14, 8, 26, 22], 180, 360, fill=(58, 50, 42, 255))
    return img


# ---------------------------------------------------------------- 主流程

def save(img: Image.Image, rel: str) -> None:
    path = ASSETS / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path, optimize=True)
    print(f"  assets/{rel}  {img.size[0]}x{img.size[1]}")


def main() -> None:
    ayan = make_ayan_frames()
    shaogong = make_shaogong_frames()

    print("生成 ch15 资产：")
    save(ayan[0], "actors/ayan-still.png")
    save(sheet(ayan, 6), "actors/ayan-sheet.png")
    save(sheet(shaogong, 4), "actors/shaogong-sheet.png")
    save(make_scroll_panel(), "props/scroll-panel.png")
    save(make_water_tile(), "props/water-tile.png")
    save(make_plank_tile(), "props/dock-plank.png")
    save(from_ascii(LANTERN), "props/lantern.png")
    save(make_boat(), "props/ferry-boat.png")

    if "--preview" in sys.argv:
        # 拼合放大预览：阿燕 12 帧 + 梢公 4 帧 + 道具
        tiles: list[Image.Image] = ayan + shaogong + [
            make_scroll_panel(), make_water_tile(), make_plank_tile(),
            from_ascii(LANTERN), make_boat(),
        ]
        scale = 6
        pad = 8
        w = sum(t.width * scale + pad for t in tiles) + pad
        h = max(t.height for t in tiles) * scale + 2 * pad
        canvas = Image.new("RGB", (w, h), (40, 42, 50))
        x = pad
        for t in tiles:
            big = t.resize((t.width * scale, t.height * scale), Image.NEAREST)
            canvas.paste(big, (x, pad), big.convert("RGBA"))
            x += big.width + pad
        out = ROOT / "target"
        out.mkdir(exist_ok=True)
        canvas.save(out / "ch15-preview.png")
        print(f"预览图：target/ch15-preview.png  {canvas.size[0]}x{canvas.size[1]}")


if __name__ == "__main__":
    main()

# -*- coding: utf-8 -*-
"""ch16 资产一键重建：中文字体子集 + 练功房美术（木人桩/阿燕练剑帧）+ 复用 ch15 道具。

用法：py -3.11 scripts/make_ch16_assets.py [--preview]
产物全部写入 code/ch16-text/assets/，确定性生成，可随时重建。

字体部分：
  下载 Noto Sans SC（SIL OFL 1.1）Regular 与 Bold 原始字体（缓存在 target/font-cache/，
  约 8 MB/个，只下一次），用 fontTools 子集化到 GB2312 全集 + ASCII + 本章用到的生僻字，
  并按 OFL 的保留字体名（Reserved Font Name）条款改名为 "Book Sans SC"。
  产物约 1–2 MB/个，随仓库分发，配套 OFL.txt 许可证。
  网络走系统代理（HTTPS_PROXY 等环境变量）；GitHub 时通时断，脚本自带重试。

美术部分沿用 ch15 的调色板与画法（像素原始尺寸，引擎里 Nearest 放大）。
"""

import os
import shutil
import sys
import urllib.error
import urllib.request
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parent.parent
CRATE = ROOT / "code" / "ch16-text"
ASSETS = CRATE / "assets"
CH15_ASSETS = ROOT / "code" / "ch15-sprites" / "assets"
FONT_CACHE = ROOT / "target" / "font-cache"

# ---------------------------------------------------------------- 字体
# Noto Sans SC：SIL OFL 1.1，保留字体名 "Noto"。子集化属于修改，
# 按 OFL 必须改名再分发——改为 "Book Sans SC"。

NOTO_BASE = "https://raw.githubusercontent.com/notofonts/noto-cjk/main/Sans/SubsetOTF/SC"
FONTS = [
    ("NotoSansSC-Regular.otf", "book-sans-sc-regular.otf", "Book Sans SC"),
    ("NotoSansSC-Bold.otf", "book-sans-sc-bold.otf", "Book Sans SC Bold"),
]

OFL_NOTICE = """Book Sans SC（本书配套字体子集）
========================================

本字体是 Noto Sans SC 的子集化修改版（GB2312 字符集 + ASCII），
仅为《The Bevy Book》示例代码体积考虑而裁剪，按 OFL 条款更名。

原字体：Noto Sans SC
Copyright 2014-2021 Adobe (http://www.adobe.com/), with Reserved Font
Name 'Source'. Copyright 2021 Google LLC, with Reserved Font Name
'Noto'. All Rights Reserved.

本子集与原字体均以 SIL Open Font License, Version 1.1 发布。
完整许可证文本：https://openfontlicense.org
原始字体仓库：https://github.com/notofonts/noto-cjk
子集化脚本：scripts/make_ch16_assets.py
"""


def gb2312_charset() -> set[str]:
    """GB2312 全集（含一、二级汉字与符号区），逐码位解码生成，确定性。"""
    chars = set()
    for hi in range(0xA1, 0xF8):
        for lo in range(0xA1, 0xFF):
            try:
                chars.add(bytes((hi, lo)).decode("gb2312"))
            except UnicodeDecodeError:
                continue
    return chars


# 本章正文/示例里用到、但不在 GB2312 的字符兜底（随写随加）
EXTRA_TEXT = "·—…“”‘’、。！？；：《》〇　"


def chars_used_in_sources() -> set[str]:
    """扫出本章 crate 源码里实际用到的全部非 ASCII 字符。

    GB2312 之外的字（如"欸乃"的欸，GBK 才收）若漏在子集外，
    上屏就是一个无声的空档——所以把源码用字并进子集兜底。
    """
    used = set()
    for rs in CRATE.rglob("*.rs"):
        used.update(c for c in rs.read_text(encoding="utf-8") if ord(c) > 0x7F)
    return used


def subset_charset() -> str:
    chars = gb2312_charset()
    chars.update(chr(c) for c in range(0x20, 0x7F))  # ASCII 可打印区
    chars.update(EXTRA_TEXT)
    chars.update(chars_used_in_sources())
    return "".join(sorted(chars))


def download(url: str, dest: Path, attempts: int = 5) -> None:
    """带重试的下载（走环境变量里的系统代理）。"""
    if dest.exists() and dest.stat().st_size > 1_000_000:
        print(f"  缓存命中：{dest.name}（{dest.stat().st_size // 1024} KB）")
        return
    dest.parent.mkdir(parents=True, exist_ok=True)
    last_error: Exception | None = None
    for attempt in range(1, attempts + 1):
        try:
            print(f"  下载 {url}（第 {attempt} 次）…")
            with urllib.request.urlopen(url, timeout=60) as resp:
                data = resp.read()
            dest.write_bytes(data)
            print(f"  完成：{dest.name}（{len(data) // 1024} KB）")
            return
        except (urllib.error.URLError, TimeoutError, ConnectionError) as e:
            last_error = e
            print(f"  失败：{e}")
    raise RuntimeError(f"下载 {url} 失败（已试 {attempts} 次）") from last_error


def rename_family(font, old_new: list[tuple[str, str]]) -> None:
    """改写 name 表里的家族名，满足 OFL 保留字体名条款。"""
    for record in font["name"].names:
        try:
            text = record.toUnicode()
        except UnicodeDecodeError:
            continue
        for old, new in old_new:
            if old in text:
                text = text.replace(old, new)
        record.string = text


def make_fonts() -> None:
    from fontTools import subset

    chars = subset_charset()
    print(f"字体子集字符数：{len(chars)}")
    for src_name, out_name, family in FONTS:
        cache = FONT_CACHE / src_name
        download(f"{NOTO_BASE}/{src_name}", cache)

        options = subset.Options()
        options.name_IDs = ["*"]
        options.name_languages = ["*"]
        options.layout_features = ["*"]  # 保留 OpenType 特性（本章会演示）
        font = subset.load_font(str(cache), options)
        subsetter = subset.Subsetter(options=options)
        subsetter.populate(text=chars)
        subsetter.subset(font)
        rename_family(font, [("Noto Sans SC", family.replace(" Bold", "")),
                             ("Noto Sans", family.replace(" Bold", ""))])
        out = ASSETS / "fonts" / out_name
        out.parent.mkdir(parents=True, exist_ok=True)
        subset.save_font(font, str(out), options)
        print(f"  assets/fonts/{out_name}  {out.stat().st_size // 1024} KB")

    (ASSETS / "fonts" / "OFL.txt").write_text(OFL_NOTICE, encoding="utf-8")
    print("  assets/fonts/OFL.txt")


# ---------------------------------------------------------------- 调色板（沿用 ch15）

PALETTE = {
    ".": (0, 0, 0, 0),          # 透明
    "K": (35, 37, 46, 255),     # 发黑
    "k": (70, 74, 92, 255),     # 发光泽
    "F": (244, 222, 190, 255),  # 肤色
    "f": (214, 178, 140, 255),  # 肤色暗部
    "E": (35, 37, 46, 255),     # 眼睛
    "R": (200, 58, 58, 255),    # 正红（戏服）
    "M": (140, 27, 39, 255),    # 深红（衣褶）
    "m": (104, 20, 30, 255),    # 红最暗
    "G": (232, 184, 75, 255),   # 金
    "H": (138, 106, 31, 255),   # 暗金
    "W": (242, 248, 255, 255),  # 冰白高光
    "D": (91, 143, 174, 255),   # 剑鞘青
    "T": (84, 66, 84, 255),     # 裤
    "N": (50, 42, 52, 255),     # 靴
    "P": (122, 86, 50, 255),    # 木
    "p": (86, 60, 36, 255),     # 木暗部
    "q": (62, 44, 30, 255),     # 木最暗
}


def blit(img: Image.Image, rows: list[str], ox: int = 0, oy: int = 0) -> None:
    for dy, row in enumerate(rows):
        for dx, ch in enumerate(row):
            if ch != ".":
                img.putpixel((ox + dx, oy + dy), PALETTE[ch])


def from_ascii(rows: list[str]) -> Image.Image:
    img = Image.new("RGBA", (len(rows[0]), len(rows)), (0, 0, 0, 0))
    blit(img, rows)
    return img


# ---------------------------------------------------------------- 阿燕练剑（32×40 / 帧，朝右）
# 四帧：0/1 持剑预备（呼吸），2 抬剑，3 劈落。
# 上身沿用 ch15 的侧身画法（背上剑鞘还在——练功用的是木剑）。

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

# 并拢站姿腿（预备/抬剑用）
LEGS_STAND = [
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
]

# 弓步腿（劈落那一帧用）：前腿弓、后腿绷
LEGS_LUNGE = [
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
]


def ayan_drill_frame(pose: str) -> Image.Image:
    """阿燕练剑一帧。pose ∈ {ready_a, ready_b, raise, slash}。"""
    img = Image.new("RGBA", (32, 40), (0, 0, 0, 0))
    bob = 1 if pose == "ready_b" else 0
    lean = 2 if pose == "slash" else 0
    blit(img, AYAN_SIDE_TOP, lean, bob)
    blit(img, LEGS_LUNGE if pose == "slash" else LEGS_STAND, lean if pose == "slash" else 0, 28)

    d = ImageDraw.Draw(img)
    wood, dark = PALETTE["P"], PALETTE["p"]
    if pose in ("ready_a", "ready_b"):
        # 木剑斜垂在身前：剑尖指向右下
        hx, hy = 19, 20 + bob
        d.line([hx, hy, hx + 8, hy + 8], fill=wood, width=2)
        d.line([hx + 6, hy + 6, hx + 8, hy + 8], fill=dark, width=2)
        img.putpixel((hx, hy), PALETTE["f"])
    elif pose == "raise":
        # 抬剑过顶：剑指右上
        hx, hy = 19, 18
        d.line([hx, hy, hx + 7, hy - 12], fill=wood, width=2)
        d.line([hx + 6, hy - 10, hx + 7, hy - 12], fill=dark, width=2)
        img.putpixel((hx, hy), PALETTE["f"])
    else:  # slash
        # 劈落：剑平指向前
        hx, hy = 21, 21
        d.line([hx, hy, hx + 10, hy + 2], fill=wood, width=2)
        d.line([hx + 8, hy + 1, hx + 10, hy + 2], fill=dark, width=2)
        img.putpixel((hx, hy), PALETTE["f"])
    return img


def make_ayan_drill_frames() -> list[Image.Image]:
    return [ayan_drill_frame(p) for p in ("ready_a", "ready_b", "raise", "slash")]


# ---------------------------------------------------------------- 木人桩（24×48）

DUMMY = [
    "........PPPPPPP.........",
    ".......PPPPPPPPP........",
    ".......PPpPPPPpP........",
    ".......PPPPPPPPP........",
    "........PpPPPPp.........",
    ".........PPPPP..........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "PPPPPPPPP.PPP.PPPPPPPPP.",
    "qppppppPP.PpP.PPppppppq.",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "....PPPPP.PpP...........",
    "....qpppP.PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PpP...........",
    "..........PPP...........",
    "..........PPP...........",
    ".........PPPPP..........",
    "........PPPPPPP.........",
    ".....qqqPPPPPPPqqq......",
    "....qppppppppppppppq....",
    "...qppppppppppppppppq...",
    "...qqqqqqqqqqqqqqqqqq...",
    "........................",
    "........................",
]


def make_dummy() -> Image.Image:
    return from_ascii(DUMMY)


# ---------------------------------------------------------------- 图集组装 / 复用 ch15 道具

def sheet(frames: list[Image.Image], columns: int) -> Image.Image:
    fw, fh = frames[0].size
    rows = (len(frames) + columns - 1) // columns
    img = Image.new("RGBA", (fw * columns, fh * rows), (0, 0, 0, 0))
    for i, fr in enumerate(frames):
        img.paste(fr, ((i % columns) * fw, (i // columns) * fh))
    return img


def save(img: Image.Image, rel: str) -> None:
    path = ASSETS / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path, optimize=True)
    print(f"  assets/{rel}  {img.size[0]}x{img.size[1]}")


def copy_ch15(rel: str) -> None:
    src = CH15_ASSETS / rel
    dst = ASSETS / rel
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(src, dst)
    print(f"  assets/{rel}（复用 ch15）")


def main() -> None:
    print("生成 ch16 资产：")
    drill = make_ayan_drill_frames()
    save(sheet(drill, 4), "actors/ayan-drill-sheet.png")
    save(make_dummy(), "props/wooden-dummy.png")
    copy_ch15("props/scroll-panel.png")
    copy_ch15("props/dock-plank.png")
    make_fonts()

    if "--preview" in sys.argv:
        tiles: list[Image.Image] = drill + [make_dummy()]
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
        canvas.save(out / "ch16-preview.png")
        print(f"预览图：target/ch16-preview.png  {canvas.size[0]}x{canvas.size[1]}")


if __name__ == "__main__":
    main()

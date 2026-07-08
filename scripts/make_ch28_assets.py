# -*- coding: utf-8 -*-
"""ch28 资产一键重建：字体复用 ch16，看板皮与小图标用 PIL 现画。

用法：py -3.11 scripts/make_ch28_assets.py

产出（code/ch28-ui-layout/assets/）：
  - fonts/book-sans-sc-{regular,bold}.otf + OFL.txt —— 复用 ch16 的 GB2312 子集字体
  - ui/panel-board.png —— 96×96 木框看板皮（四角铆钉 + 12px 金线木框 + 纸芯），
    专为九宫格切片设计：border=28，四角有明显图案，Stretch 拉伸时一眼看出变形
  - ui/icons-sheet.png —— 192×48 图集条，4 帧 48×48：
    帧 0 彩球（打瓦的球）/ 帧 1 上釉瓦（亮青）/ 帧 2 素瓦（灰白）/ 帧 3 金瓦（战利品）
"""

import shutil
import sys
from pathlib import Path

from PIL import Image, ImageDraw

# Windows 控制台默认 GBK，中文输出先转 UTF-8
sys.stdout.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
DEST = CODE / "ch28-ui-layout" / "assets"

FONT_FILES = [
    ("ch16-text/assets/fonts/book-sans-sc-regular.otf", "fonts/book-sans-sc-regular.otf"),
    ("ch16-text/assets/fonts/book-sans-sc-bold.otf", "fonts/book-sans-sc-bold.otf"),
    ("ch16-text/assets/fonts/OFL.txt", "fonts/OFL.txt"),
]

# ---- 调色盘（与前几章戏班配色一脉相承）----
WOOD_DARK = (74, 50, 34, 255)  # 深胡桃木框
WOOD_MID = (104, 72, 48, 255)  # 木框亮面
GOLD = (212, 175, 92, 255)  # 描金线
PAPER = (247, 240, 224, 255)  # 纸芯
RIVET = (58, 42, 30, 255)  # 铆钉
RIVET_HI = (232, 198, 120, 255)  # 铆钉高光


def make_panel_board() -> None:
    """96×96 看板皮：28px 木框（含描金线与四角铆钉）+ 纸芯。"""
    size = 96
    b = 28  # 与代码里 TextureSlicer 的 border 对齐
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)

    # 外框：两层木色 + 一圈描金线
    d.rounded_rectangle([0, 0, size - 1, size - 1], radius=10, fill=WOOD_DARK)
    d.rounded_rectangle([4, 4, size - 5, size - 5], radius=8, fill=WOOD_MID)
    d.rounded_rectangle([9, 9, size - 10, size - 10], radius=6, outline=GOLD, width=3)
    # 纸芯：正好从 border 往里
    d.rectangle([b, b, size - 1 - b, size - 1 - b], fill=PAPER)
    # 纸芯淡竖纹（平铺时能看出重复节奏）
    for x in range(b + 6, size - b, 12):
        d.line([x, b + 2, x, size - b - 3], fill=(235, 226, 206, 255), width=1)
    # 四角铆钉：切片后钉在角上不变形，是「角永远不缩放」的活教材
    for cx, cy in [(14, 14), (size - 15, 14), (14, size - 15), (size - 15, size - 15)]:
        d.ellipse([cx - 5, cy - 5, cx + 5, cy + 5], fill=RIVET)
        d.ellipse([cx - 5, cy - 5, cx + 1, cy + 1], fill=RIVET_HI)

    out = DEST / "ui" / "panel-board.png"
    out.parent.mkdir(parents=True, exist_ok=True)
    img.save(out)
    print(f"  assets/ui/panel-board.png  96×96，切片 border=28")


def _draw_tile(d: ImageDraw.ImageDraw, ox: int, body, edge, sheen=None) -> None:
    """一片带弧度的瓦：圆角矩形 + 上缘弧光。"""
    d.rounded_rectangle([ox + 6, 4, ox + 41, 43], radius=8, fill=body, outline=edge, width=2)
    if sheen:
        d.arc([ox + 9, 7, ox + 38, 30], start=200, end=340, fill=sheen, width=3)


def make_icons_sheet() -> None:
    """192×48 图集条：球 / 上釉瓦 / 素瓦 / 金瓦，各 48×48 一帧。"""
    img = Image.new("RGBA", (192, 48), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)

    # 帧 0：彩球（朱红底白高光，和第 20 章的球一个气质）
    d.ellipse([8, 8, 39, 39], fill=(196, 78, 60, 255), outline=(120, 40, 30, 255), width=2)
    d.ellipse([14, 12, 24, 22], fill=(255, 214, 190, 220))

    # 帧 1：上釉瓦（亮青）
    _draw_tile(d, 48, body=(96, 176, 178, 255), edge=(52, 110, 112, 255), sheen=(214, 240, 240, 255))

    # 帧 2：素瓦（灰白）
    _draw_tile(d, 96, body=(196, 188, 172, 255), edge=(128, 120, 104, 255))

    # 帧 3：金瓦（战利品成色）
    _draw_tile(d, 144, body=(222, 178, 92, 255), edge=(150, 110, 44, 255), sheen=(255, 236, 180, 255))

    out = DEST / "ui" / "icons-sheet.png"
    out.parent.mkdir(parents=True, exist_ok=True)
    img.save(out)
    print(f"  assets/ui/icons-sheet.png  192×48，4 帧 48×48")


def main() -> None:
    missing = [src for src, _ in FONT_FILES if not (CODE / src).exists()]
    if missing:
        for src in missing:
            print(f"缺上游资产：code/{src}")
        print("先运行 make_ch16_assets.py 再来。")
        sys.exit(1)

    print("就位 ch28 资产：")
    for src, dst in FONT_FILES:
        target = DEST / dst
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(CODE / src, target)
        print(f"  assets/{dst}  <- code/{src}")

    make_panel_board()
    make_icons_sheet()


if __name__ == "__main__":
    main()

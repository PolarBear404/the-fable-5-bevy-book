# -*- coding: utf-8 -*-
"""一键就位第 25 章的美术资产（全部复用前章，无新合成）。

    py -3.11 scripts/make_ch25_assets.py

产物（输出到 code/ch25-picking/assets/）：
  sprites/ayan-still.png   阿燕立像（ch15 原图，32×40 像素画）——四周大片透明，
                           是 SpritePickingMode 两档对比实验的主角
  sprites/lantern.png      灯笼（ch15 原图，16×16）——叠在阿燕包围盒后当「下家」
  fonts/book-sans-sc-regular.otf  中文字模（ch16 的 GB2312 子集）——UI 牌子用
"""

import shutil
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
SRC15 = ROOT / "code" / "ch15-sprites" / "assets"
SRC16 = ROOT / "code" / "ch16-text" / "assets"
OUT = ROOT / "code" / "ch25-picking" / "assets"

COPIES = [
    (SRC15 / "actors" / "ayan-still.png", OUT / "sprites" / "ayan-still.png"),
    (SRC15 / "props" / "lantern.png", OUT / "sprites" / "lantern.png"),
    (SRC16 / "fonts" / "book-sans-sc-regular.otf", OUT / "fonts" / "book-sans-sc-regular.otf"),
]


def main() -> None:
    for src, dst in COPIES:
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(src, dst)
        print(f"就位 {dst.relative_to(ROOT)}（{dst.stat().st_size} 字节）")


if __name__ == "__main__":
    main()

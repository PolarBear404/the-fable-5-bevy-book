# -*- coding: utf-8 -*-
"""ch17 资产一键就位：全部复用前两章的脚本化资产，本章不新画一笔。

用法：py -3.11 scripts/make_ch17_assets.py

来源（均为仓库内由脚本生成、入了 git 的文件）：
  - code/ch15-sprites/assets/ —— 阿燕十二格连环画、桥板贴片（make_ch15_assets.py 产物）
  - code/ch16-text/assets/   —— 中文字体子集、木人桩（make_ch16_assets.py 产物）
若上游缺失，先运行对应章节的资产脚本再来。
"""

import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
DEST = CODE / "ch17-input" / "assets"

# (来源 crate 相对路径, 本章 assets 相对路径)
FILES = [
    ("ch15-sprites/assets/actors/ayan-sheet.png", "actors/ayan-sheet.png"),
    ("ch15-sprites/assets/props/dock-plank.png", "props/dock-plank.png"),
    ("ch16-text/assets/props/wooden-dummy.png", "props/wooden-dummy.png"),
    ("ch16-text/assets/fonts/book-sans-sc-regular.otf", "fonts/book-sans-sc-regular.otf"),
    ("ch16-text/assets/fonts/book-sans-sc-bold.otf", "fonts/book-sans-sc-bold.otf"),
    ("ch16-text/assets/fonts/OFL.txt", "fonts/OFL.txt"),
]


def main() -> None:
    missing = [src for src, _ in FILES if not (CODE / src).exists()]
    if missing:
        for src in missing:
            print(f"缺上游资产：code/{src}")
        print("先运行 make_ch15_assets.py / make_ch16_assets.py 再来。")
        sys.exit(1)

    print("就位 ch17 资产（全部复用前章）：")
    for src, dst in FILES:
        target = DEST / dst
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(CODE / src, target)
        print(f"  assets/{dst}  <- code/{src}")


if __name__ == "__main__":
    main()

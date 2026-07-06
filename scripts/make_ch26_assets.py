# -*- coding: utf-8 -*-
"""ch26 资产一键就位：只要一套中文字体（总成《定妆照》的屏上状态牌用），复用 ch16 子集。

用法：py -3.11 scripts/make_ch26_assets.py

来源（仓库内由脚本生成、入了 git 的文件）：
  - code/ch16-text/assets/fonts/ —— 中文字体子集（make_ch16_assets.py 产物）
若上游缺失，先运行 make_ch16_assets.py 再来。
"""

import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
DEST = CODE / "ch26-quality" / "assets"

FILES = [
    ("ch16-text/assets/fonts/book-sans-sc-regular.otf", "fonts/book-sans-sc-regular.otf"),
    ("ch16-text/assets/fonts/OFL.txt", "fonts/OFL.txt"),
]


def main() -> None:
    missing = [src for src, _ in FILES if not (CODE / src).exists()]
    if missing:
        for src in missing:
            print(f"缺上游资产：code/{src}")
        print("先运行 make_ch16_assets.py 再来。")
        sys.exit(1)

    print("就位 ch26 资产（复用 ch16 字体子集）：")
    for src, dst in FILES:
        target = DEST / dst
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(CODE / src, target)
        print(f"  assets/{dst}  <- code/{src}")


if __name__ == "__main__":
    main()

# -*- coding: utf-8 -*-
"""准备第 26 章资产。

第 26 章只需要中文 UI 字体，复用第 16 章已经子集化并按 OFL 改名的
Noto Sans SC。

运行：
    py -3.11 scripts/make_ch26_assets.py
"""

import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "code" / "ch16-text" / "assets" / "fonts"
DST = ROOT / "code" / "ch26-post-processing-aa" / "assets" / "fonts"
FILES = [
    "book-sans-sc-regular.otf",
    "OFL.txt",
]


def main() -> None:
    DST.mkdir(parents=True, exist_ok=True)
    for name in FILES:
        src = SRC / name
        if not src.exists():
            raise FileNotFoundError(f"缺少源字体资产：{src}")
        shutil.copyfile(src, DST / name)
        print(f"{name} -> {DST.relative_to(ROOT)}")


if __name__ == "__main__":
    main()

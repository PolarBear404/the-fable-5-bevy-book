# -*- coding: utf-8 -*-
"""ch21 资产一键就位：合成「雷」字班旗贴图（banner.png）。

用法：py -3.11 scripts/make_ch21_assets.py

本章唯一的图片资产由本脚本用 PIL 现场绘制：256×256，
暗朱底、双道金边、正中一个金「雷」字（微软雅黑加粗，系统自带字体）。
方旗取材自戏曲的门旗/帅字旗，特意画成上下不对称——贴图坐标
一旦写反，旗面立刻穿帮，是 21.4 节的教学需要。
"""

from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
DEST = ROOT / "code" / "ch21-meshes" / "assets" / "textures"

SIZE = 256
GROUND = (122, 31, 26)      # 暗朱
GOLD = (212, 168, 84)       # 金
GOLD_DIM = (164, 124, 58)   # 暗金（内边）


def main() -> None:
    img = Image.new("RGB", (SIZE, SIZE), GROUND)
    draw = ImageDraw.Draw(img)

    # 双道金边：外粗内细
    draw.rectangle([4, 4, SIZE - 5, SIZE - 5], outline=GOLD, width=6)
    draw.rectangle([18, 18, SIZE - 19, SIZE - 19], outline=GOLD_DIM, width=2)

    # 旗顶一排三颗铆钉——上下不对称的记号，UV 写反一眼即知
    for x in (64, 128, 192):
        draw.ellipse([x - 7, 30, x + 7, 44], fill=GOLD)

    # 正中金「雷」
    font = ImageFont.truetype("C:/Windows/Fonts/msyhbd.ttc", 150)
    box = draw.textbbox((0, 0), "雷", font=font)
    w, h = box[2] - box[0], box[3] - box[1]
    draw.text(((SIZE - w) / 2 - box[0], (SIZE - h) / 2 - box[1] + 12), "雷",
              font=font, fill=GOLD)

    DEST.mkdir(parents=True, exist_ok=True)
    out = DEST / "banner.png"
    img.save(out, optimize=True)
    print(f"已生成 {out.relative_to(ROOT)}（{SIZE}x{SIZE}，{out.stat().st_size // 1024} KB）")


if __name__ == "__main__":
    main()

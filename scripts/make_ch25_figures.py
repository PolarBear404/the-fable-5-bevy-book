# -*- coding: utf-8 -*-
"""一键重建第 25 章的截图（SVG 概念图为手绘，不在此列）。

    py -3.11 scripts/make_ch25_figures.py [图名筛选]

三张 PNG：
  fig-25-02 mesh 观察者示例        （listing-25-01）
  fig-25-04 sprite + UI 后端示例   （listing-25-04）
  fig-25-06 点选拖拽最终舞台       （main）

产物输出到 book/src/images/ch25/。
"""

import os
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch25-picking-camera-control"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch25"

os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402


def logical(img: Image.Image) -> Image.Image:
    if img.size == (1280, 720):
        return img
    return img.resize((1280, 720), Image.LANCZOS)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch25-picking-camera-control.exe"
    return EXAMPLES / f"{name}.exe"


def fig_02_mesh_observers() -> None:
    with Example(exe("listing-25-01"), workdir=CRATE) as ex:
        shot = logical(ex.shot(1.6))
    save_png(shot, "fig-25-02-mesh-observers.png")


def fig_04_sprite_ui() -> None:
    with Example(exe("listing-25-04"), workdir=CRATE) as ex:
        shot = logical(ex.shot(1.2))
    save_png(shot, "fig-25-04-sprite-ui.png")


def fig_06_final_stage() -> None:
    with Example(exe("main"), workdir=CRATE) as ex:
        shot = logical(ex.shot(1.8))
    save_png(shot, "fig-25-06-final-stage.png")


ALL = [
    fig_02_mesh_observers,
    fig_04_sprite_ui,
    fig_06_final_stage,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    subprocess.run([sys.executable, str(ROOT / "scripts" / "make_ch25_assets.py")], check=True)
    print("构建第 25 章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch25-picking-camera-control", "--bins", "--examples"],
        cwd=CODE,
        check=True,
    )
    only = sys.argv[1] if len(sys.argv) > 1 else None
    for fig in ALL:
        if only and only not in fig.__name__:
            continue
        fig()
        time.sleep(0.5)


if __name__ == "__main__":
    main()

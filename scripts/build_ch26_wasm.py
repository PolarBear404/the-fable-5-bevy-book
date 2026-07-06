# -*- coding: utf-8 -*-
"""ch26 内嵌 demo 一键重建：把《定妆照》画质开关面板编成 WebAssembly。

用法：py -3.11 scripts/build_ch26_wasm.py

本章一个 demo（规格见 workorders/ch26.md「demo 判定」）：
  主演示 = src/main.rs（bin 目标，Listing 26-13）——1~5 换磨边方案
  （素颜/MSAA4x/FXAA/SMAA/TAA），B 辉光、F 景深、M 运动模糊、V 老镜头套餐、
  T 换冲印配方，屏上中文状态牌实时显示当前档位。canvas 挂 #bevy-canvas
  （main.rs 里定死，26.13 正文交代过这两个 web-only 字段）。

流程（三步，全部可重入）：
  1. cargo build --profile wasm-release --target wasm32-unknown-unknown
     -p ch26-quality --features bevy/web
     （wasm-release 见 code/Cargo.toml：opt-level="z" + fat LTO + strip 压体积；
       bevy/web 开浏览器 API——requestAnimationFrame 主循环、panic 信息进 console。
       本 crate 的 debug feature 也在默认集合里，--features 是追加而非替换，
       不加 --no-default-features 它就在，千万别加）
  2. wasm-bindgen --target web 生成 JS 胶水 + 精简后的 .wasm，
     输出到 book/src/demos/ch26/（--out-name 必须是合法标识符，不能带连字符）
  3. 拷贝运行资产 → book/src/demos/ch26/assets/
     （main.rs 状态牌 load 了 fonts/book-sans-sc-regular.otf，没有这个文件
       wasm 侧 AssetServer fetch 404、Text 用不上字体渲不出中文；
       OFL.txt 是许可证文本，运行不需要，不拷）

产物一律 gitignore（见 /.gitignore 的 ch26 段），唯一入库的源是手写的宿主页
book/src/demos/ch26/index.html——启动层在那里。

前置：rustup target add wasm32-unknown-unknown；
wasm-bindgen-cli 版本必须与 code/Cargo.lock 锁定的 wasm-bindgen 精确一致，
不一致时本脚本直接报错退出（产物会在浏览器里以离奇方式坏掉，不能将就）。
"""

import re
import shutil
import subprocess
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
sys.stderr.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = "ch26-quality"
OUT = ROOT / "book" / "src" / "demos" / "ch26"
PROFILE_DIR = CODE / "target" / "wasm32-unknown-unknown" / "wasm-release"

OUT_NAME = "ch26_quality"
ARTIFACT = PROFILE_DIR / f"{CRATE}.wasm"

# main.rs 实际 load 的运行资产（crate assets/ 下的相对路径，拷到 OUT/assets/ 同路径）
ASSETS = [
    "fonts/book-sans-sc-regular.otf",  # 屏上状态牌中文字体（ch16 字体子集）
]


def locked_wasm_bindgen_version() -> str:
    """从 Cargo.lock 读出 wasm-bindgen 的锁定版本。"""
    lock = (CODE / "Cargo.lock").read_text(encoding="utf-8")
    m = re.search(r'name = "wasm-bindgen"\nversion = "([^"]+)"', lock)
    if not m:
        sys.exit("Cargo.lock 里找不到 wasm-bindgen——先在 wasm target 下 cargo check 一次")
    return m.group(1)


def installed_cli_version() -> str:
    out = subprocess.run(
        ["wasm-bindgen", "--version"], capture_output=True, text=True, check=True
    ).stdout
    return out.strip().split()[-1]


def main() -> None:
    locked = locked_wasm_bindgen_version()
    cli = installed_cli_version()
    if cli != locked:
        sys.exit(
            f"wasm-bindgen-cli 是 {cli}，Cargo.lock 锁的是 {locked}——版本必须精确一致。\n"
            f"修法：cargo install wasm-bindgen-cli --version {locked} --locked"
        )
    print(f"wasm-bindgen {cli}（与 Cargo.lock 一致）")

    OUT.mkdir(parents=True, exist_ok=True)

    print("== 1/3 cargo build（wasm-release，fat LTO 链接慢，耐心）==")
    subprocess.run(
        [
            "cargo", "build",
            "--profile", "wasm-release",
            "--target", "wasm32-unknown-unknown",
            "-p", CRATE,
            "--features", "bevy/web",
        ],
        cwd=CODE,
        check=True,
    )

    print("== 2/3 wasm-bindgen 生成 JS 胶水 ==")
    subprocess.run(
        [
            "wasm-bindgen",
            "--target", "web",
            "--no-typescript",
            "--out-dir", str(OUT),
            "--out-name", OUT_NAME,
            str(ARTIFACT),
        ],
        check=True,
    )

    print("== 3/3 拷贝运行资产 ==")
    dst_assets = OUT / "assets"
    if dst_assets.exists():
        shutil.rmtree(dst_assets)
    for rel in ASSETS:
        src = CODE / CRATE / "assets" / rel
        dst = dst_assets / rel
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst)
        print(f"  assets/{rel}")

    total = sum(f.stat().st_size for f in OUT.rglob("*") if f.is_file())
    wasm = OUT / f"{OUT_NAME}_bg.wasm"
    print(f"完成：{OUT}")
    print(f"  {wasm.name}  {wasm.stat().st_size / 1024 / 1024:.1f} MB")
    print(f"  目录合计 {total / 1024 / 1024:.1f} MB")
    if not (OUT / "index.html").exists():
        print("  提醒：index.html（手写、入库）还不在——嵌入页缺它跑不起来")


if __name__ == "__main__":
    main()

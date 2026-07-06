# -*- coding: utf-8 -*-
"""ch24 内嵌 demo 一键重建：把《琉璃记·验货》材质画廊编成 WebAssembly。

用法：py -3.11 scripts/build_ch24_wasm.py

本章一个 demo（规格见 workorders/ch24.md §6）：
  主演示 = src/main.rs（bin 目标）——八件展品沿浅弧排开，数字键 1~8 逐件看、
  0 退回全景、左键拖动转台（cursor_position 差分，web 可用），展品缓缓自转。
  嵌在 ch24-14《画廊开张》，canvas #bevy-ch24。

流程（三步，全部可重入）：
  1. cargo build --profile wasm-release --target wasm32-unknown-unknown
     -p ch24-materials --features bevy/web
     （wasm-release 见 code/Cargo.toml：opt-level="z" + fat LTO + strip 压体积；
       bevy/web 开浏览器 API——requestAnimationFrame 主循环、panic 信息进 console。
       本 crate 的默认 feature aniso=bevy/pbr_anisotropy_texture 也必须在——
       7 号台《拉丝金》的各向异性着色路径靠它；--features 是追加而非替换，
       不加 --no-default-features 它就在，千万别加）
  2. wasm-bindgen --target web 生成 JS 胶水 + 精简后的 .wasm，
     输出到 book/src/demos/ch24/（--out-name 必须是合法标识符，不能带连字符）
  3. 拷贝运行资产 → book/src/demos/ch24/assets/
     （只拷 main.rs 实际 load 的六张纹理；lantern_sign/banner/banner_tile 是
       其它 listing 的道具，画廊不用。wasm 侧 AssetServer 用 fetch 按相对路径
       取资产，目录结构原样即可）

产物一律 gitignore（见 /.gitignore 的 ch24 段），唯一入库的源是手写的宿主页
book/src/demos/ch24/index.html——启动层与键盘焦点都在那里。

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
CRATE = "ch24-materials"
OUT = ROOT / "book" / "src" / "demos" / "ch24"
PROFILE_DIR = CODE / "target" / "wasm32-unknown-unknown" / "wasm-release"

OUT_NAME = "ch24_gallery"
ARTIFACT = PROFILE_DIR / f"{CRATE}.wasm"

# main.rs 实际 load 的运行资产（crate assets/ 下的相对路径，拷到 OUT/assets/ 同路径）
ASSETS = [
    "textures/studio_cubemap.png",   # 影棚墙 cubemap（Skybox + 环境光）
    "textures/bamboo_alpha.png",     # 竹影纱背景
    "textures/gong_base.png",        # 锈锣 base color
    "textures/gong_orm.png",         # 锈锣 ORM 三通道
    "textures/carve_normal.png",     # 雕花漆盖法线
    "textures/carve_height.png",     # 雕花漆盖深度（视差）
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

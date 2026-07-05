# -*- coding: utf-8 -*-
"""ch23 内嵌 demo 一键重建：把《阿福亮相》与「哑巴坑」编成 WebAssembly。

用法：py -3.11 scripts/build_ch23_wasm.py

本章两个 demo（DEMOS 列表驱动，规格见 workorders/ch23.md §6）：
  demo① 主演示 = src/main.rs（bin 目标）——拖动转台 + 空格歇锣，
        嵌在 ch23-11《开演》，canvas #bevy-ch23，运行资产只要单件 afu.glb
  demo② 哑巴坑 = examples/listing-23-13.rs（example 目标）——点击抽谱/还谱，
        嵌在 ch23-08《动画》，canvas #bevy-ch23-anim，运行资产是 afu/ 三件套

流程（对每个 demo 三步，全部可重入）：
  1. cargo build --profile wasm-release --target wasm32-unknown-unknown
     -p ch23-gltf [--example …] --features bevy/web
     （wasm-release 见 code/Cargo.toml：opt-level="z" + fat LTO + strip 压体积；
       bevy/web 开浏览器 API——requestAnimationFrame 主循环、panic 信息进 console。
       注意产物路径按目标分：bin 在 <profile>/<crate>.wasm，example 在
       <profile>/examples/<名>.wasm）
  2. wasm-bindgen --target web 生成 JS 胶水 + 精简后的 .wasm，
     输出到 book/src/demos/ch23/（--out-name 必须是合法标识符，不能带连字符）
  3. 拷贝各自的运行资产 → book/src/demos/ch23/assets/
     （wasm 侧 AssetServer 用 fetch 按相对路径取资产，目录结构原样即可）

产物一律 gitignore（见 /.gitignore），唯一入库的源是手写的两张宿主页
book/src/demos/ch23/{index,anim}.html——启动层与键盘焦点都在那里。

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
CRATE = "ch23-gltf"
OUT = ROOT / "book" / "src" / "demos" / "ch23"
PROFILE_DIR = CODE / "target" / "wasm32-unknown-unknown" / "wasm-release"

# 每个 demo 一条：cargo 目标参数、wasm 产物路径、胶水名、运行资产
# （assets 是 crate assets/ 下的相对路径，文件或整目录皆可，拷到 OUT/assets/ 同路径）
DEMOS = [
    {
        "label": "demo① 主演示《阿福亮相》（src/main.rs，bin）",
        "cargo_target": [],
        "artifact": PROFILE_DIR / f"{CRATE}.wasm",
        "out_name": "ch23_afu",
        "assets": ["models/afu.glb"],
    },
    {
        "label": "demo② 哑巴坑（examples/listing-23-13.rs，example）",
        "cargo_target": ["--example", "listing-23-13"],
        "artifact": PROFILE_DIR / "examples" / "listing-23-13.wasm",
        "out_name": "ch23_anim",
        "assets": ["models/afu"],
    },
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
    dst_assets = OUT / "assets"
    if dst_assets.exists():
        shutil.rmtree(dst_assets)

    for i, demo in enumerate(DEMOS, 1):
        print(f"==== [{i}/{len(DEMOS)}] {demo['label']} ====")

        print("== 1/3 cargo build（wasm-release，fat LTO 链接慢，耐心）==")
        subprocess.run(
            [
                "cargo", "build",
                "--profile", "wasm-release",
                "--target", "wasm32-unknown-unknown",
                "-p", CRATE,
                *demo["cargo_target"],
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
                "--out-name", demo["out_name"],
                str(demo["artifact"]),
            ],
            check=True,
        )

        print("== 3/3 拷贝运行资产 ==")
        for rel in demo["assets"]:
            src = CODE / CRATE / "assets" / rel
            dst = dst_assets / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            if src.is_dir():
                shutil.copytree(src, dst, dirs_exist_ok=True)
            else:
                shutil.copy2(src, dst)
            print(f"  assets/{rel}")

    total = sum(f.stat().st_size for f in OUT.rglob("*") if f.is_file())
    print(f"完成：{OUT}")
    for demo in DEMOS:
        wasm = OUT / f"{demo['out_name']}_bg.wasm"
        print(f"  {wasm.name}  {wasm.stat().st_size / 1024 / 1024:.1f} MB")
    print(f"  目录合计 {total / 1024 / 1024:.1f} MB")
    for page in ["index.html", "anim.html"]:
        if not (OUT / page).exists():
            print(f"  提醒：{page}（手写、入库）还不在——嵌入页缺它跑不起来")


if __name__ == "__main__":
    main()

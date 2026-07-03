# -*- coding: utf-8 -*-
"""ch20 内嵌 demo 一键重建：把《打瓦》编成 WebAssembly，供 20.8 节网页内游玩。

用法：py -3.11 scripts/build_ch20_wasm.py

流程（三步，全部可重入）：
  1. cargo build --profile wasm-release --target wasm32-unknown-unknown
     -p ch20-breakout --features bevy/web
     （wasm-release 见 code/Cargo.toml：opt-level="z" + fat LTO + strip 压体积；
       bevy/web 开浏览器 API——requestAnimationFrame 主循环、panic 信息进 console）
  2. wasm-bindgen --target web 生成 JS 胶水 + 精简后的 .wasm，
     输出到 book/src/demos/ch20/（--out-name ch20-breakout）
  3. 拷贝 code/ch20-breakout/assets/ → book/src/demos/ch20/assets/
     （wasm 侧 AssetServer 用 fetch 按相对路径取资产，目录结构原样即可）

产物一律 gitignore（见 /.gitignore），唯一入库的源是手写的
book/src/demos/ch20/index.html——启动层、画布缩放与音频恢复都在那里。

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
CRATE = "ch20-breakout"
OUT = ROOT / "book" / "src" / "demos" / "ch20"
WASM_ARTIFACT = CODE / "target" / "wasm32-unknown-unknown" / "wasm-release" / f"{CRATE}.wasm"


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

    print("== 1/3 cargo build（wasm-release，首次要全量编 Bevy，耐心）==")
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
    OUT.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        [
            "wasm-bindgen",
            "--target", "web",
            "--no-typescript",
            "--out-dir", str(OUT),
            "--out-name", CRATE,
            str(WASM_ARTIFACT),
        ],
        check=True,
    )

    print("== 3/3 拷贝资产 ==")
    src_assets = CODE / CRATE / "assets"
    dst_assets = OUT / "assets"
    if dst_assets.exists():
        shutil.rmtree(dst_assets)
    shutil.copytree(src_assets, dst_assets)

    total = sum(f.stat().st_size for f in OUT.rglob("*") if f.is_file())
    wasm = OUT / f"{CRATE}_bg.wasm"
    print(f"完成：{OUT}")
    print(f"  wasm 主体 {wasm.stat().st_size / 1024 / 1024:.1f} MB，目录合计 {total / 1024 / 1024:.1f} MB")
    if not (OUT / "index.html").exists():
        print("  提醒：index.html（手写、入库）还不在——嵌入页缺它跑不起来")


if __name__ == "__main__":
    main()

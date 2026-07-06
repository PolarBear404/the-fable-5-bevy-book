# -*- coding: utf-8 -*-
"""ch25 内嵌 demo 一键重建：把《上手验货》拾取画廊编成 WebAssembly。

用法：py -3.11 scripts/build_ch25_wasm.py

本章一个 demo（规格见 workorders/ch25.md §6）：
  主演示 = src/main.rs（bin 目标）——三件货 + 朱漆托盘的 3D 验货台，
  悬停高亮、单击点名、双击归位、按住拖挪、拖进托盘装箱、
  拖空处/台面转台、滚轮推拉。全部交互由指针事件驱动，无键盘、无音频、
  无 AccumulatedMouseMotion（web 输入坑天然绕开）。
  嵌在 ch25-14《上手验货》，canvas 挂 #bevy-canvas（main.rs 里定死，
  25.14 正文交代过这两个 web-only 字段）。

流程（两步，全部可重入；main.rs 全程序化网格，零资产加载，
assets/ 下的字体与 sprites 是本章其它 listing 的道具，demo 不拷）：
  1. cargo build --profile wasm-release --target wasm32-unknown-unknown
     -p ch25-picking --features bevy/web
     （wasm-release 见 code/Cargo.toml：opt-level="z" + fat LTO + strip 压体积；
       bevy/web 开浏览器 API——requestAnimationFrame 主循环、panic 信息进 console。
       本 crate 的默认 feature camera-controllers=bevy/free_camera+bevy/pan_camera
       也必须在——25.10~25.13 的现成控制器靠它；--features 是追加而非替换，
       不加 --no-default-features 它就在，千万别加）
  2. wasm-bindgen --target web 生成 JS 胶水 + 精简后的 .wasm，
     输出到 book/src/demos/ch25/（--out-name 必须是合法标识符，不能带连字符）

产物一律 gitignore（见 /.gitignore 的 ch25 段），唯一入库的源是手写的宿主页
book/src/demos/ch25/index.html——启动层在那里。

前置：rustup target add wasm32-unknown-unknown；
wasm-bindgen-cli 版本必须与 code/Cargo.lock 锁定的 wasm-bindgen 精确一致，
不一致时本脚本直接报错退出（产物会在浏览器里以离奇方式坏掉，不能将就）。
"""

import re
import subprocess
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
sys.stderr.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = "ch25-picking"
OUT = ROOT / "book" / "src" / "demos" / "ch25"
PROFILE_DIR = CODE / "target" / "wasm32-unknown-unknown" / "wasm-release"

OUT_NAME = "ch25_inspection"
ARTIFACT = PROFILE_DIR / f"{CRATE}.wasm"


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

    print("== 1/2 cargo build（wasm-release，fat LTO 链接慢，耐心）==")
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

    print("== 2/2 wasm-bindgen 生成 JS 胶水 ==")
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

    total = sum(f.stat().st_size for f in OUT.rglob("*") if f.is_file())
    wasm = OUT / f"{OUT_NAME}_bg.wasm"
    print(f"完成：{OUT}")
    print(f"  {wasm.name}  {wasm.stat().st_size / 1024 / 1024:.1f} MB")
    print(f"  目录合计 {total / 1024 / 1024:.1f} MB")
    if not (OUT / "index.html").exists():
        print("  提醒：index.html（手写、入库）还不在——嵌入页缺它跑不起来")


if __name__ == "__main__":
    main()

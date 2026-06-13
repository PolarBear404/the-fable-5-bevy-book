#!/usr/bin/env python3
"""把第 25 章的点选/拖拽舞台（code/ch25-picking-camera-control 的默认 bin）
编译成 WebAssembly，产物落进 mdBook 工程，供 25.6 节的网页内嵌演示加载。

前置（一次性，仅首次需要联网）：
    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli --version 0.2.123

运行：
    py -3.11 scripts/build_ch25_wasm.py

输出：book/src/demos/ch25/ch25_stage.{js,wasm}。这些是生成物，已在 .gitignore 里忽略；
手写的 index.html 不在其列。
"""

import pathlib
import shutil
import subprocess
import sys

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")
    sys.stderr.reconfigure(encoding="utf-8")

ROOT = pathlib.Path(__file__).resolve().parent.parent
CODE_DIR = ROOT / "code"
CRATE = "ch25-picking-camera-control"
OUT_DIR = ROOT / "book" / "src" / "demos" / "ch25"
OUT_NAME = "ch25_stage"
WASM_BINDGEN_VERSION = "0.2.123"
PROFILE = "wasm-release"
TARGET = "wasm32-unknown-unknown"
ASSET_FILES = ["book-sans-sc-regular.otf"]
ASSET_SRC_DIR = CODE_DIR / CRATE / "assets" / "fonts"
ASSET_DST_DIR = OUT_DIR / "assets" / "fonts"


def fail(msg):
    print(f"\n[build_ch25_wasm] 出错：{msg}", file=sys.stderr)
    sys.exit(1)


def check_tools():
    exe = shutil.which("wasm-bindgen")
    if not exe:
        fail(
            "找不到 wasm-bindgen。先安装：\n"
            f"    cargo install wasm-bindgen-cli --version {WASM_BINDGEN_VERSION}"
        )
    out = subprocess.run([exe, "--version"], capture_output=True, text=True)
    version = out.stdout.strip().split()[-1] if out.stdout else "?"
    if version != WASM_BINDGEN_VERSION:
        fail(
            f"wasm-bindgen 版本是 {version}，本书锁定 {WASM_BINDGEN_VERSION}。请重装：\n"
            f"    cargo install wasm-bindgen-cli --version {WASM_BINDGEN_VERSION} --force"
        )
    installed = subprocess.run(
        ["rustup", "target", "list", "--installed"], capture_output=True, text=True
    )
    if TARGET not in installed.stdout:
        fail(f"未安装编译目标 {TARGET}。先执行：\n    rustup target add {TARGET}")
    print(f"[build_ch25_wasm] wasm-bindgen {version} ✓  target {TARGET} ✓")


def build_wasm():
    print(f"[build_ch25_wasm] cargo build -p {CRATE}（默认 bin）…")
    subprocess.run(
        ["cargo", "build", "--profile", PROFILE, "--target", TARGET, "-p", CRATE],
        cwd=CODE_DIR,
        check=True,
    )
    wasm = CODE_DIR / "target" / TARGET / PROFILE / f"{CRATE}.wasm"
    if not wasm.exists():
        fail(f"没找到预期的 wasm 产物：{wasm}")
    return wasm


def run_bindgen(wasm):
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    print(f"[build_ch25_wasm] wasm-bindgen {OUT_NAME} → {OUT_DIR.relative_to(ROOT)}")
    subprocess.run(
        [
            "wasm-bindgen",
            "--no-typescript",
            "--target",
            "web",
            "--out-name",
            OUT_NAME,
            "--out-dir",
            str(OUT_DIR),
            str(wasm),
        ],
        check=True,
    )


def copy_assets():
    subprocess.run([sys.executable, str(ROOT / "scripts" / "make_ch25_assets.py")], check=True)
    ASSET_DST_DIR.mkdir(parents=True, exist_ok=True)
    for name in ASSET_FILES:
        src = ASSET_SRC_DIR / name
        if not src.exists():
            fail(f"资产不存在：{src}")
        shutil.copyfile(src, ASSET_DST_DIR / name)
    print(f"[build_ch25_wasm] 资产 → {ASSET_DST_DIR.relative_to(ROOT)}（{', '.join(ASSET_FILES)}）")


def main():
    check_tools()
    wasm = build_wasm()
    run_bindgen(wasm)
    copy_assets()
    bg = OUT_DIR / f"{OUT_NAME}_bg.wasm"
    size_mb = bg.stat().st_size / 1024 / 1024 if bg.exists() else 0
    print(
        f"\n[build_ch25_wasm] 完成。{OUT_NAME}_bg.wasm ≈ {size_mb:.1f} MB\n"
        "  预览：mdbook serve book；在 25.6 节点击占位图即可运行 demo。"
    )


if __name__ == "__main__":
    main()

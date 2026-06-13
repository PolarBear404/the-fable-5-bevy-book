#!/usr/bin/env python3
"""把第 23 章的 demo（code/ch23-gltf）编译成 WebAssembly，产物落进 mdBook 工程，
供 ch23-07 的网页内嵌演示加载。

与全书「插图/资产一条命令重建」的铁律一致：版本迁移或改了 demo 代码后，重跑本脚本
即可重新生成网页产物，无需任何手工操作。

前置（一次性，仅首次需要联网）：
    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli --version 0.2.123

为什么锁 0.2.123：wasm-bindgen-cli 的版本必须与依赖树里锁定的 wasm-bindgen 完全一致
（见 code/Cargo.lock），否则生成的 JS 胶水与 .wasm 对不上、一加载就报错。

运行：
    py -3.11 scripts/build_ch23_wasm.py

输出：book/src/demos/ch23/{ch23_gltf.js, ch23_gltf_bg.wasm} 以及 assets/ 资产副本。
这些是生成物，已在 .gitignore 里忽略——手写的 index.html 不在其列。
"""

import pathlib
import shutil
import subprocess
import sys

# Windows 控制台默认 GBK，输出 ✓ / → 之类符号会抛 UnicodeEncodeError；强制 UTF-8
if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")
    sys.stderr.reconfigure(encoding="utf-8")

ROOT = pathlib.Path(__file__).resolve().parent.parent
CODE_DIR = ROOT / "code"
CRATE = "ch23-gltf"
OUT_NAME = "ch23_gltf"  # JS/wasm 文件名前缀，须是合法标识符（不能带连字符）
OUT_DIR = ROOT / "book" / "src" / "demos" / "ch23"

# 必须与 code/Cargo.lock 里的 wasm-bindgen 版本字字相同
WASM_BINDGEN_VERSION = "0.2.123"

# 自定义 wasm-release：在 release 上加 opt-level="z" + 全程 LTO + strip 去符号，
# 把体积从裸 release 的几十 MB 压下来（定义见 code/Cargo.toml）。不依赖外部 wasm-opt。
PROFILE = "wasm-release"
TARGET = "wasm32-unknown-unknown"

ASSET_SRC = CODE_DIR / CRATE / "assets" / "models" / "puppet.gltf"
ASSET_DST = OUT_DIR / "assets" / "models" / "puppet.gltf"


def fail(msg):
    print(f"\n[build_ch23_wasm] 出错：{msg}", file=sys.stderr)
    sys.exit(1)


def check_tools():
    """确认 wasm-bindgen 在 PATH、版本与锁定一致，且编译目标已安装。"""
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
            f"wasm-bindgen 版本是 {version}，本书锁定 {WASM_BINDGEN_VERSION}；"
            "版本错配会让 JS 胶水与 .wasm 对不上。请重装：\n"
            f"    cargo install wasm-bindgen-cli --version {WASM_BINDGEN_VERSION} --force"
        )
    installed = subprocess.run(
        ["rustup", "target", "list", "--installed"], capture_output=True, text=True
    )
    if TARGET not in installed.stdout:
        fail(f"未安装编译目标 {TARGET}。先执行：\n    rustup target add {TARGET}")
    print(f"[build_ch23_wasm] wasm-bindgen {version} ✓  target {TARGET} ✓")


def build_wasm():
    """把 demo crate 编译成 wasm 二进制。首次会连同 Bevy 一起编，耗时较长。"""
    print(f"[build_ch23_wasm] cargo build --profile {PROFILE} --target {TARGET} -p {CRATE} …")
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
    """生成浏览器可加载的 ES module 胶水 + 处理过的 _bg.wasm。"""
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    print(f"[build_ch23_wasm] wasm-bindgen → {OUT_DIR.relative_to(ROOT)}")
    subprocess.run(
        [
            "wasm-bindgen",
            "--no-typescript",
            "--target", "web",
            "--out-name", OUT_NAME,
            "--out-dir", str(OUT_DIR),
            str(wasm),
        ],
        check=True,
    )


def copy_assets():
    """资产要和网页同源可取：把木偶 glTF 拷进 demo 目录。"""
    ASSET_DST.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(ASSET_SRC, ASSET_DST)
    print(f"[build_ch23_wasm] 资产 → {ASSET_DST.relative_to(ROOT)}")


def main():
    if not ASSET_SRC.exists():
        fail(f"资产不存在：{ASSET_SRC}\n先跑 scripts/make_ch23_assets.py 生成木偶。")
    check_tools()
    wasm = build_wasm()
    run_bindgen(wasm)
    copy_assets()
    bg = OUT_DIR / f"{OUT_NAME}_bg.wasm"
    size_mb = bg.stat().st_size / 1024 / 1024 if bg.exists() else 0
    print(
        f"\n[build_ch23_wasm] 完成。{OUT_NAME}_bg.wasm ≈ {size_mb:.1f} MB\n"
        "  预览：mdbook serve book，开 ch23-07，点占位图运行 demo。"
    )


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""把第 24 章的材质球画廊（code/ch24-pbr-materials 的默认 bin）编译成 WebAssembly，
产物落进 mdBook 工程，供 24.7 节的网页内嵌演示加载：

    ch24-07「材质球画廊」 ← crate 默认 bin（src/main.rs） → ch24_gallery.{js,wasm}

与全书「插图/资产一条命令重建」的铁律一致：版本迁移或改了 demo 代码后，重跑本脚本
即可重新生成全部网页产物，无需任何手工操作。

前置（一次性，仅首次需要联网）：
    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli --version 0.2.123

为什么锁 0.2.123：wasm-bindgen-cli 的版本必须与依赖树里锁定的 wasm-bindgen 完全一致
（见 code/Cargo.lock），否则生成的 JS 胶水与 .wasm 对不上、一加载就报错。

运行：
    py -3.11 scripts/build_ch24_wasm.py

输出：book/src/demos/ch24/ch24_gallery.{js,wasm} 以及 assets/textures/ 资产副本。
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
CRATE = "ch24-pbr-materials"
OUT_DIR = ROOT / "book" / "src" / "demos" / "ch24"

# 本章只有一个 demo：crate 默认 binary（src/main.rs）→ ch24_gallery。
# 输出名须是合法标识符（不能带连字符），它决定 <名>.js / <名>_bg.wasm，
# 也是 index.html 里 `import init from "./<名>.js"` 引的名字。
OUT_NAME = "ch24_gallery"

# 必须与 code/Cargo.lock 里的 wasm-bindgen 版本字字相同
WASM_BINDGEN_VERSION = "0.2.123"

# 自定义 wasm-release：在 release 上加 opt-level="z" + 全程 LTO + strip 去符号，
# 把体积从裸 release 的几十 MB 压下来（定义见 code/Cargo.toml）。不依赖外部 wasm-opt。
PROFILE = "wasm-release"
TARGET = "wasm32-unknown-unknown"

# 画廊只用到这两张贴图（法线图 + 环境 skybox）；镂空贴图 lattice.png 只在示例里用，不随 demo 走
ASSET_FILES = ["studs-normal.png", "skybox.png"]
ASSET_SRC_DIR = CODE_DIR / CRATE / "assets" / "textures"
ASSET_DST_DIR = OUT_DIR / "assets" / "textures"


def fail(msg):
    print(f"\n[build_ch24_wasm] 出错：{msg}", file=sys.stderr)
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
    print(f"[build_ch24_wasm] wasm-bindgen {version} ✓  target {TARGET} ✓")


def build_wasm():
    """把默认 bin 编译成 wasm 二进制，返回产物路径。首次会连同 Bevy 一起编，耗时较长。"""
    cmd = ["cargo", "build", "--profile", PROFILE, "--target", TARGET, "-p", CRATE]
    print(f"[build_ch24_wasm] cargo build -p {CRATE}（默认 bin）…")
    subprocess.run(cmd, cwd=CODE_DIR, check=True)
    wasm = CODE_DIR / "target" / TARGET / PROFILE / f"{CRATE}.wasm"
    if not wasm.exists():
        fail(f"没找到预期的 wasm 产物：{wasm}")
    return wasm


def run_bindgen(wasm):
    """生成浏览器可加载的 ES module 胶水 + 处理过的 _bg.wasm。"""
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    print(f"[build_ch24_wasm] wasm-bindgen {OUT_NAME} → {OUT_DIR.relative_to(ROOT)}")
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
    """资产要和网页同源可取：把画廊用到的贴图拷进 demo 目录。"""
    ASSET_DST_DIR.mkdir(parents=True, exist_ok=True)
    for name in ASSET_FILES:
        src = ASSET_SRC_DIR / name
        if not src.exists():
            fail(f"资产不存在：{src}\n先跑 scripts/make_ch24_assets.py 生成贴图。")
        shutil.copyfile(src, ASSET_DST_DIR / name)
    print(f"[build_ch24_wasm] 资产 → {ASSET_DST_DIR.relative_to(ROOT)}（{', '.join(ASSET_FILES)}）")


def main():
    check_tools()
    wasm = build_wasm()
    run_bindgen(wasm)
    copy_assets()
    bg = OUT_DIR / f"{OUT_NAME}_bg.wasm"
    size_mb = bg.stat().st_size / 1024 / 1024 if bg.exists() else 0
    print(
        f"\n[build_ch24_wasm] 完成。{OUT_NAME}_bg.wasm ≈ {size_mb:.1f} MB\n"
        "  预览：mdbook serve book；在 24.7 节点占位图即可运行画廊 demo。"
    )


if __name__ == "__main__":
    main()

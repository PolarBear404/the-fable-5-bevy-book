# -*- coding: utf-8 -*-
"""一键重建第 24 章全部 26 张插图（23 张运行截图 + 3 张手绘 SVG）。

    py -3.11 scripts/make_ch24_figures.py [图名筛选]

运行图：样品间三球、反光度五连（裁前排）、高光染色（裁后排）、自发光暗房、
真灯对照两联、曝光权重两联、ORM 接管两联、AO 对比裁片两联、法线双盖、
视差双盖、视差层数两联、清漆四球、各向异性炸白（--no-default-features）、
各向异性四连、alpha 七款、琉璃盏、底片步数两联（临时补丁 steps=0）、
双面旗三联、depth_bias 裁片两联、晨雾全景、unlit 裁片两联、UV 手脚、画廊全景。
SVG（内容手绘、由本脚本落盘，保证一条命令全量重建）：光路账单、TBN 切线、视差原理。

键盘注入走 PostMessage 客户区消息（WM_KEYDOWN/WM_KEYUP）——本机 SendInput
经常拿不到前台焦点，投递窗口消息不依赖前台，实测可靠（ch23 工单的结论）。

两处特殊构建（函数内自建自恢复，脚本收尾再统一 --all-targets 兜底）：
- fig-24-16 用 `--no-default-features` 的 listing-24-09（各向异性 feature 坑）；
- fig-24-20 下联临时给 listing-24-11.rs 的相机补 `ScreenSpaceTransmission { steps: 0 }`
  （运行时拨到 0 会残留旧底片，必须启动即 0——工单 §3 台账），拍完恢复原文件并重建。
"""

import ctypes
import os
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

sys.stdout.reconfigure(encoding="utf-8")
sys.stderr.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch24-materials"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch24"

os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402

user32 = ctypes.windll.user32

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36

# ------------------------------------------------- 键盘注入（PostMessage）

WM_KEYDOWN, WM_KEYUP = 0x0100, 0x0101
# 名字 -> (virtual-key, scancode)。winit 按扫描码解析物理键（KeyCode），两者都要对
KEYS = {
    "SPACE": (0x20, 0x39),
    "E": (0x45, 0x12),
    "L": (0x4C, 0x26),
    "F": (0x46, 0x21),
    "B": (0x42, 0x30),
    "U": (0x55, 0x16),
    "N": (0x4E, 0x31),
    "]": (0xDD, 0x1B),  # VK_OEM_6 / BracketRight
}


def post_tap(ex: Example, name: str, hold: float = 0.06) -> None:
    """向示例窗口投递一次键击。

    直接给窗口线程的消息队列塞 WM_KEYDOWN/WM_KEYUP：不动真键盘、
    不要求窗口在前台（PrintWindow 同理），比 SendInput 稳。
    lparam 低 16 位是重复计数，16..23 位是扫描码；keyup 还要置
    「先前按下」(bit 30) 与「正在释放」(bit 31) 两个标志位。
    """
    vk, scan = KEYS[name]
    down = 1 | (scan << 16)
    up = down | (1 << 30) | (1 << 31)
    user32.PostMessageW(ex.hwnd, WM_KEYDOWN, vk, down)
    time.sleep(hold)
    user32.PostMessageW(ex.hwnd, WM_KEYUP, vk, up)


# ------------------------------------------------- 通用排版（沿 ch23 惯例）

def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch24-materials.exe"
    return EXAMPLES / f"{name}.exe"


def cargo(*args: str) -> None:
    subprocess.run(["cargo", *args], cwd=CODE, check=True)


def label_bar(width: int, text: str) -> Image.Image:
    bar = Image.new("RGB", (width, LABEL_H), LABEL_BG)
    draw = ImageDraw.Draw(bar)
    w = draw.textlength(text, font=FONT)
    draw.text(((width - w) / 2, 6), text, font=FONT, fill=LABEL_FG)
    return bar


def vstack(images: list[Image.Image], labels: list[str]) -> Image.Image:
    """竖排联图：每联上方一条标签，联间细缝——正文「上联/下联」的排法。"""
    w = max(im.width for im in images)
    h = sum(im.height + LABEL_H for im in images) + GAP * (len(images) - 1)
    canvas = Image.new("RGB", (w, h), GAP_COLOR)
    y = 0
    for im, text in zip(images, labels):
        canvas.paste(label_bar(w, text), (0, y))
        canvas.paste(im, ((w - im.width) // 2, y + LABEL_H))
        y += im.height + LABEL_H + GAP
    return canvas


def logical(img: Image.Image) -> Image.Image:
    """物理像素 → 1280×720 逻辑像素（DPI 缩放下物理分辨率会更大）。"""
    if img.size == (1280, 720):
        return img
    return img.resize((1280, 720), Image.LANCZOS)


def save_png(img: Image.Image, filename: str) -> None:
    path = OUT / filename
    img.save(path, optimize=True)
    print(f"{filename}：{img.size[0]}x{img.size[1]}，{path.stat().st_size // 1024} KB")


def save_svg(text: str, filename: str) -> None:
    path = OUT / filename
    path.write_text(text, encoding="utf-8")
    print(f"{filename}：{path.stat().st_size // 1024} KB")


HALF = 0.5  # 联图里单帧缩到 640×360


def shrink(img: Image.Image) -> Image.Image:
    return img.resize((int(img.width * HALF), int(img.height * HALF)), Image.LANCZOS)


# 裁片框（逻辑 1280×720 坐标，实拍目检后校准）
# 注：listing-24-02 前排球顶（y≈232）与后排球底（y≈260）纵向重叠，
# 横条裁片两头只能取其轻——保本排球体完整，容邻排 ≤15px 残边
CROP_03_FRONT_ROW = (40, 245, 1240, 462)     # listing-24-02 前排五颗白瓷
CROP_04_BACK_ROW = (225, 106, 1052, 252)     # listing-24-02 后排 2×2 染色对照
CROP_09_GONG_SE = (520, 100, 810, 340)       # listing-24-04 锣球（带受光/背光两片锈斑）
CROP_22_POSTER = (945, 150, 1255, 460)       # listing-24-12 告示板（避开右旗）
CROP_24_BOARD = (860, 300, 1230, 580)        # listing-24-13 提词板（完整含板）


def crop_zoom(img: Image.Image, box: tuple[int, int, int, int], k: float) -> Image.Image:
    """裁片再放大 k 倍——小裁片直接联排太窄（标签都装不下），放大保可读。"""
    piece = img.crop(box)
    return piece.resize((int(piece.width * k), int(piece.height * k)), Image.LANCZOS)


# ------------------------------------------------- 运行截图

def fig_02_sample_room() -> None:
    """Figure 24-2：样品间开张——素坯/亮瓷/镜面金三球，金球映柔光箱。"""
    with Example(exe("listing-24-01"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-02-sample-room.png")


def fig_03_reflectance_row() -> None:
    """Figure 24-3：反光度五连——0 档石膏相，右端扎眼（裁前排）。"""
    with Example(exe("listing-24-02"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot.crop(CROP_03_FRONT_ROW), "fig-24-03-reflectance-row.png")


def fig_04_specular_tint() -> None:
    """Figure 24-4：石绿染高光——黑釉变墨绿琉璃，银器双胞胎纹丝不动（裁后排）。"""
    with Example(exe("listing-24-02"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot.crop(CROP_04_BACK_ROW), "fig-24-04-specular-tint.png")


def fig_05_darkroom() -> None:
    """Figure 24-5：自发光暗房——灯箱 + 1/10/100 尼特阶梯，素坯位置纯黑。"""
    with Example(exe("listing-24-03"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-05-darkroom.png")


def fig_06_real_lamp() -> None:
    """Figure 24-6：自发光不是灯（两联竖排）——L 键换真点光，地板洇出光池。"""
    with Example(exe("listing-24-03"), workdir=CODE) as ex:
        dark = logical(ex.shot(4.0))
        ex.wait_until(8.0)
        post_tap(ex, "L")
        lamp = logical(ex.shot(9.0))
    save_png(
        vstack([shrink(dark), shrink(lamp)],
               ["只有自发光——地板与素坯全黑",
                "同位置换一盏真点光——光池洇开，素坯现身"]),
        "fig-24-06-real-lamp.png",
    )


def fig_07_exposure_weight() -> None:
    """Figure 24-7：曝光的秤（两联竖排）——E 拨 EV100，唯 weight=1 青球变暗。"""
    with Example(exe("listing-24-03"), workdir=CODE) as ex:
        ev_default = logical(ex.shot(4.0))
        ex.wait_until(5.0)
        post_tap(ex, "E")
        ev_overcast = logical(ex.shot(6.5))
    save_png(
        vstack([shrink(ev_default), shrink(ev_overcast)],
               ["EV100 = 9.7（默认）",
                "E 拨到 EV100 = 12——全场不动，唯 weight=1 的右青球掉了亮"]),
        "fig-24-07-exposure-weight.png",
    )


def fig_08_orm_takeover() -> None:
    """Figure 24-8：贴图接管（两联竖排）——阶段甲塑料相坑，阶段乙铜锈分明。"""
    with Example(exe("listing-24-04"), workdir=CODE) as ex:
        naive = logical(ex.shot(4.0))
        ex.wait_until(5.0)
        post_tap(ex, "SPACE")
        takeover = logical(ex.shot(6.0))
    save_png(
        vstack([shrink(naive), shrink(takeover)],
               ["阶段甲：标量全默认——金属度 0 乘什么都是 0，贴图白贴",
                "阶段乙：标量拨到 1——贴图接管，铜是铜，锈是锈"]),
        "fig-24-08-orm-takeover.png",
    )


def fig_09_ao_compare() -> None:
    """Figure 24-9：AO 只压环境光（两联裁片竖排）——锣球右下背光面对比。"""
    with Example(exe("listing-24-04"), workdir=CODE) as ex:
        ex.wait_until(5.0)
        post_tap(ex, "SPACE")  # 甲 → 乙
        stage_b = logical(ex.shot(6.0))
        ex.wait_until(7.0)
        post_tap(ex, "SPACE")  # 乙 → 丙
        stage_c = logical(ex.shot(8.0))
    save_png(
        vstack([crop_zoom(stage_b, CROP_09_GONG_SE, 2.0),
                crop_zoom(stage_c, CROP_09_GONG_SE, 2.0)],
               ["无 AO（阶段乙）——背光面锈斑均匀暗褐",
                "有 AO（阶段丙）——背光锈斑压黑，受光锈斑没动"]),
        "fig-24-09-ao-compare.png",
    )


def fig_11_carved_lids() -> None:
    """Figure 24-11：法线贴图哑坑——左盖死平，右盖云纹浮雕（掠射光）。"""
    with Example(exe("listing-24-05"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-11-carved-lids.png")


def fig_13_parallax_vs_normal() -> None:
    """Figure 24-13：法线 vs 视差——平躺双盖斜视角，左画上去右陷进去。"""
    with Example(exe("listing-24-07"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-13-parallax-vs-normal.png")


def fig_14_parallax_layers() -> None:
    """Figure 24-14：层数是精度（两联竖排）——0.16 深度下 16 层阶梯，64 层平滑。"""
    with Example(exe("listing-24-07"), workdir=CODE) as ex:
        ex.wait_until(5.0)
        post_tap(ex, "]")  # 0.08 → 0.12
        ex.wait_until(5.3)
        post_tap(ex, "]")  # 0.12 → 0.16
        stepped = logical(ex.shot(6.0))
        ex.wait_until(7.0)
        post_tap(ex, "N")  # 16 → 64 层
        smooth = logical(ex.shot(8.0))
    save_png(
        vstack([shrink(stepped), shrink(smooth)],
               ["depth_scale 0.16 + 默认 16 层——侧壁爬满阶梯条带",
                "N 拨到 64 层——同一侧壁平滑连续"]),
        "fig-24-14-parallax-layers.png",
    )


def fig_15_clearcoat() -> None:
    """Figure 24-15：清漆四球——素身/亮清漆/哑清漆/膜下雕花。"""
    with Example(exe("listing-24-08"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-15-clearcoat.png")


def fig_16_anisotropy_blowout() -> None:
    """Figure 24-16：feature 坑——不开 pbr_anisotropy_texture 拨旋钮，受光半球炸白。

    本图专用 --no-default-features 的产物（重编 bevy_pbr，约 2 分钟）；
    拍完立即用默认 feature 重建 listing-24-09，不给后续图留坑。
    """
    print("  构建 --no-default-features 的 listing-24-09（要重编 bevy_pbr，等它几分钟）……")
    cargo("build", "-p", "ch24-materials", "--example", "listing-24-09",
          "--no-default-features")
    try:
        with Example(exe("listing-24-09"), workdir=CODE) as ex:
            shot = logical(ex.shot(4.0))
        save_png(shot, "fig-24-16-anisotropy-blowout.png")
    finally:
        print("  恢复默认 feature 的 listing-24-09……")
        cargo("build", "-p", "ch24-materials", "--example", "listing-24-09")


def fig_17_anisotropy_lineup() -> None:
    """Figure 24-17：拉丝四连（默认 feature）——圆斑→拉环→亮带→拧 90°。"""
    with Example(exe("listing-24-09"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-17-anisotropy-lineup.png")


def fig_18_alpha_modes() -> None:
    """Figure 24-18：alpha 全家福——四幕 + 幽灵 + 茶镜 + 双胞胎。"""
    with Example(exe("listing-24-10"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-18-alpha-modes.png")


def fig_19_glass() -> None:
    """Figure 24-19：琉璃盏——倒像 + 竹纱镜内隐形 + 灯笼对照双端。"""
    with Example(exe("listing-24-11"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-19-glass.png")


STEPS0_MARK = "Camera3d::default(),"
STEPS0_PATCH = (
    "Camera3d::default(),\n"
    "        ScreenSpaceTransmission { steps: 0, ..default() },"
)


def fig_20_transmission_steps() -> None:
    """Figure 24-20：底片步数（两联竖排）——steps=1 倒像清晰，steps=0 黑琉璃。

    运行时拨到 0 会残留最后一次抄的底片（工单台账），必须启动即 0：
    临时给 listing-24-11.rs 的相机补 ScreenSpaceTransmission { steps: 0 }，
    拍完恢复原文件并重建产物。
    """
    with Example(exe("listing-24-11"), workdir=CODE) as ex:
        steps1 = logical(ex.shot(4.0))

    src = CRATE / "examples" / "listing-24-11.rs"
    original = src.read_text(encoding="utf-8")
    assert original.count(STEPS0_MARK) == 1, "listing-24-11.rs 结构变了，先对补丁"
    assert "ScreenSpaceTransmission" in original, "补丁依赖已有的 use bevy::pbr::ScreenSpaceTransmission"
    try:
        src.write_text(original.replace(STEPS0_MARK, STEPS0_PATCH), encoding="utf-8")
        print("  临时补丁 steps=0，重建 listing-24-11……")
        cargo("build", "-p", "ch24-materials", "--example", "listing-24-11")
        with Example(exe("listing-24-11"), workdir=CODE) as ex:
            steps0 = logical(ex.shot(4.0))
    finally:
        src.write_text(original, encoding="utf-8")
        print("  恢复 listing-24-11.rs 原文件并重建……")
        cargo("build", "-p", "ch24-materials", "--example", "listing-24-11")

    save_png(
        vstack([shrink(steps1), shrink(steps0)],
               ["steps = 1（默认）——玻璃里铜锣倒像清晰",
                "steps = 0——黑琉璃：不抄画面，只剩环境反光的微光"]),
        "fig-24-20-transmission-steps.png",
    )


def fig_21_flag_stages() -> None:
    """Figure 24-21：左旗三档（三联竖排）——隐形 → 黑脸 → 受光。"""
    with Example(exe("listing-24-12"), workdir=CODE) as ex:
        culled = logical(ex.shot(4.0))
        ex.wait_until(5.0)
        post_tap(ex, "F")  # 一档 → 二档
        dark_face = logical(ex.shot(6.0))
        ex.wait_until(7.0)
        post_tap(ex, "F")  # 二档 → 三档
        lit_both = logical(ex.shot(8.0))
    save_png(
        vstack([shrink(culled), shrink(dark_face), shrink(lit_both)],
               ["一档：出厂默认（cull Back）——左旗整面隐形",
                "二档：cull_mode = None——背面现身，借正面法线黑着脸",
                "三档：再加 double_sided——两面同亮，雷字隔布反写"]),
        "fig-24-21-flag-stages.png",
    )


def fig_22_depth_bias() -> None:
    """Figure 24-22：共面打架与垫纸（两联裁片竖排）——戏报被吃 vs bias=2 浮出。"""
    with Example(exe("listing-24-12"), workdir=CODE) as ex:
        fighting = logical(ex.shot(4.0))
        ex.wait_until(5.0)
        post_tap(ex, "B")
        floated = logical(ex.shot(6.0))
    save_png(
        vstack([crop_zoom(fighting, CROP_22_POSTER, 1.6),
                crop_zoom(floated, CROP_22_POSTER, 1.6)],
               ["depth_bias = 0——共面打架，戏报整块被吃",
                "depth_bias = 2——垫了纸，戏报稳稳浮出"]),
        "fig-24-22-depth-bias.png",
    )


def fig_23_morning_fog() -> None:
    """Figure 24-23：晨雾——金球列队没入，右灯箱扎眼左灯箱只剩淡影。"""
    with Example(exe("listing-24-13"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-23-morning-fog.png")


def fig_24_unlit() -> None:
    """Figure 24-24：unlit 不吃光、照样吃雾（两联裁片竖排）——提词板前后对比。"""
    with Example(exe("listing-24-13"), workdir=CODE) as ex:
        lit = logical(ex.shot(4.0))
        ex.wait_until(7.0)
        post_tap(ex, "U")
        unlit = logical(ex.shot(8.0))
    save_png(
        vstack([crop_zoom(lit, CROP_24_BOARD, 1.5),
                crop_zoom(unlit, CROP_24_BOARD, 1.5)],
               ["unlit = false——受光的明暗渐变",
                "unlit = true——整面平色，但照样蒙着雾"]),
        "fig-24-24-unlit.png",
    )


def fig_25_uv_transform() -> None:
    """Figure 24-25：UV 手脚——左倒旗右正旗 + 后墙 3×2 瓦。"""
    with Example(exe("listing-24-14"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-25-uv-transform.png")


def fig_26_gallery() -> None:
    """Figure 24-26：画廊全景——八件展品沿浅弧排开（兼 WASM demo 占位图）。"""
    with Example(exe("main"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-24-26-gallery.png")


# ------------------------------------------------- 手绘 SVG（内容即代码，落盘即重建）

SVG_01_LIGHT_LEDGER = """<svg viewBox="0 0 860 560" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr-in24" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#b8862e"/>
    </marker>
    <marker id="arr-spec24" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#c05a2e"/>
    </marker>
    <marker id="arr-diff24" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#274a91"/>
    </marker>
    <marker id="arr-trans24" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#3c7a4e"/>
    </marker>
    <marker id="arr-emis24" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#b3402e"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="860" height="560" rx="10" fill="#f7f5f0"/>

  <text x="430" y="34" text-anchor="middle" font-size="16" fill="#4a463f" font-weight="bold">一束光的四路账单——每个字段管一行</text>

  <!-- 表面板：上表面一条粗线，板体有厚度（透射要穿过它） -->
  <rect x="90" y="300" width="680" height="66" fill="#e9e3d6" stroke="#7a7468" stroke-width="1.2"/>
  <line x1="90" y1="300" x2="770" y2="300" stroke="#4a463f" stroke-width="3"/>
  <text x="108" y="352" font-size="12" fill="#7a7468">表面</text>

  <!-- 入射光：左上斜射到入射点 (365,300) -->
  <path d="M175,112 L355,290" stroke="#b8862e" stroke-width="4" fill="none" marker-end="url(#arr-in24)"/>
  <text x="150" y="96" font-size="13" fill="#8a6a1e" font-weight="bold">入射光</text>

  <!-- ① 镜面反射：入射点对称弹向右上 -->
  <path d="M372,293 L548,118" stroke="#c05a2e" stroke-width="3.4" fill="none" marker-end="url(#arr-spec24)"/>
  <text x="562" y="96" font-size="13.5" fill="#c05a2e" font-weight="bold">镜面反射：在表面弹走</text>
  <text x="562" y="115" font-size="11" fill="#7a7468" font-family="Consolas, monospace">metallic · perceptual_roughness</text>
  <text x="562" y="131" font-size="11" fill="#7a7468" font-family="Consolas, monospace">reflectance · specular_tint</text>
  <text x="562" y="147" font-size="11" fill="#7a7468" font-family="Consolas, monospace">clearcoat · anisotropy_*</text>

  <!-- ② 漫反射：钻进表面染色，再从旁边向上散出一把小箭头 -->
  <path d="M365,300 q-12,26 -38,20" stroke="#274a91" stroke-width="2" fill="none" stroke-dasharray="4 3"/>
  <g stroke="#274a91" stroke-width="2.4" fill="none">
    <path d="M300,296 L232,246" marker-end="url(#arr-diff24)"/>
    <path d="M300,296 L262,228" marker-end="url(#arr-diff24)"/>
    <path d="M300,296 L300,220" marker-end="url(#arr-diff24)"/>
    <path d="M300,296 L338,232" marker-end="url(#arr-diff24)"/>
  </g>
  <text x="238" y="196" text-anchor="middle" font-size="13.5" fill="#274a91" font-weight="bold">漫反射：吃进去染色再散出</text>
  <text x="238" y="215" text-anchor="middle" font-size="11" fill="#7a7468" font-family="Consolas, monospace">base_color(_texture)</text>
  <text x="238" y="231" text-anchor="middle" font-size="10.5" fill="#7a7468">金属没有这一路</text>

  <!-- ③ 透射：穿过板体到背面去——一支直的（成像）、两支散的（糊光） -->
  <path d="M365,300 L412,364" stroke="#3c7a4e" stroke-width="2.4" fill="none" stroke-dasharray="5 4"/>
  <path d="M412,366 L458,452" stroke="#3c7a4e" stroke-width="2.8" fill="none" marker-end="url(#arr-trans24)"/>
  <g stroke="#3c7a4e" stroke-width="1.8" fill="none" opacity="0.75">
    <path d="M412,366 L386,442" marker-end="url(#arr-trans24)"/>
    <path d="M412,366 L424,448" marker-end="url(#arr-trans24)"/>
  </g>
  <text x="470" y="428" font-size="13.5" fill="#3c7a4e" font-weight="bold">透射：穿到背后去</text>
  <text x="470" y="447" font-size="11" fill="#7a7468" font-family="Consolas, monospace">diffuse_transmission（糊光，便宜）</text>
  <text x="470" y="463" font-size="11" fill="#7a7468" font-family="Consolas, monospace">specular_transmission（成像，贵）</text>

  <!-- ④ 自发光：表面右端自己冒光，没有入射来源 -->
  <circle cx="672" cy="300" r="26" fill="#f3c9a6" opacity="0.5"/>
  <g stroke="#b3402e" stroke-width="2.4" fill="none">
    <path d="M672,294 L672,226" marker-end="url(#arr-emis24)"/>
    <path d="M660,296 L618,240" marker-end="url(#arr-emis24)"/>
    <path d="M684,296 L726,240" marker-end="url(#arr-emis24)"/>
  </g>
  <text x="694" y="196" text-anchor="middle" font-size="13.5" fill="#b3402e" font-weight="bold">自发光：不靠外来光</text>
  <text x="694" y="215" text-anchor="middle" font-size="11" fill="#7a7468" font-family="Consolas, monospace">emissive(_texture)</text>
  <text x="694" y="231" text-anchor="middle" font-size="10.5" fill="#7a7468">尼特计价，不照亮邻居</text>

  <text x="430" y="530" text-anchor="middle" font-size="11" fill="#7a7468">同一束光的能量在四行账里分配——本章每一节拆一行</text>
</svg>
"""

SVG_10_TBN = """<svg viewBox="0 0 840 470" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr-n24" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#274a91"/>
    </marker>
    <marker id="arr-t24" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#c05a2e"/>
    </marker>
    <marker id="arr-b24" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#3c7a4e"/>
    </marker>
    <marker id="arr-px24" markerWidth="8" markerHeight="8" refX="6" refY="4" orient="auto">
      <path d="M0,0 L7,4 L0,8 z" fill="#2b2b46"/>
    </marker>
    <marker id="arr-link24" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#7a7468"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="840" height="470" rx="10" fill="#f7f5f0"/>

  <text x="420" y="34" text-anchor="middle" font-size="16" fill="#4a463f" font-weight="bold">法线贴图的向量活在 TBN 坐标系里</text>

  <!-- ============ 左：微弯的表面 + 三根轴 ============ -->
  <!-- 曲面片（微弯，带 UV 网格线） -->
  <path d="M60,310 C 150,258 300,246 420,280 L 396,392 C 290,352 160,362 78,408 z"
        fill="#ffffff" stroke="#7a7468" stroke-width="1.6"/>
  <!-- u 向网格线（顺 T） -->
  <g stroke="#c9c2b2" stroke-width="1" fill="none">
    <path d="M66,342 C 154,292 298,281 412,316"/>
    <path d="M72,375 C 158,326 294,316 404,353"/>
  </g>
  <!-- v 向网格线（顺 B） -->
  <g stroke="#c9c2b2" stroke-width="1" fill="none">
    <path d="M150,262 L 132,384"/>
    <path d="M240,250 L 228,368"/>
    <path d="M330,254 L 322,372"/>
  </g>
  <text x="92" y="436" font-size="11" fill="#7a7468">一块微弯的表面（贴着法线贴图）</text>

  <!-- 原点：表面中央一点 -->
  <circle cx="245" cy="308" r="4.5" fill="#4a463f"/>

  <!-- N：沿表面法线朝外（微弯处的垂直方向） -->
  <path d="M245,308 L233,180" stroke="#274a91" stroke-width="3.2" fill="none" marker-end="url(#arr-n24)"/>
  <text x="206" y="166" font-size="14" fill="#274a91" font-weight="bold">N</text>
  <text x="228" y="166" font-size="11.5" fill="#274a91">法线：朝表面外</text>

  <!-- T：顺贴图 u 方向 -->
  <path d="M245,308 L392,290" stroke="#c05a2e" stroke-width="3.2" fill="none" marker-end="url(#arr-t24)"/>
  <text x="360" y="268" font-size="14" fill="#c05a2e" font-weight="bold">T</text>
  <text x="378" y="268" font-size="11.5" fill="#c05a2e">切线：顺贴图 u 方向</text>

  <!-- B：与 N、T 都垂直 -->
  <path d="M245,308 L152,368" stroke="#3c7a4e" stroke-width="3.2" fill="none" marker-end="url(#arr-b24)"/>
  <text x="120" y="392" font-size="14" fill="#3c7a4e" font-weight="bold">B</text>
  <text x="138" y="392" font-size="11.5" fill="#3c7a4e">副切线：与两者垂直</text>

  <!-- 贴图 v 方向小注（u 方向即 T，标签里已说） -->
  <text x="318" y="372" font-size="10.5" fill="#9a9280">v ↓</text>

  <!-- ============ 右：法线贴图像素放大 ============ -->
  <rect x="520" y="96" width="240" height="240" rx="8" fill="#8f8fe8" stroke="#7a7468" stroke-width="1.6"/>
  <g stroke="#a7a7ef" stroke-width="1.2">
    <line x1="600" y1="96" x2="600" y2="336"/>
    <line x1="680" y1="96" x2="680" y2="336"/>
    <line x1="520" y1="176" x2="760" y2="176"/>
    <line x1="520" y1="256" x2="760" y2="256"/>
  </g>
  <!-- 九格像素向量：中央出屏（点圈），边缘向外倾 -->
  <g stroke="#2b2b46" stroke-width="2.4" fill="none">
    <path d="M566,142 L544,120" marker-end="url(#arr-px24)"/>
    <path d="M640,146 L640,116" marker-end="url(#arr-px24)"/>
    <path d="M714,142 L736,120" marker-end="url(#arr-px24)"/>
    <path d="M556,216 L528,216" marker-end="url(#arr-px24)"/>
    <path d="M724,216 L752,216" marker-end="url(#arr-px24)"/>
    <path d="M566,290 L544,312" marker-end="url(#arr-px24)"/>
    <path d="M640,286 L640,316" marker-end="url(#arr-px24)"/>
    <path d="M714,290 L736,312" marker-end="url(#arr-px24)"/>
  </g>
  <!-- 中央像素：向量正对观众（出屏符号 ⊙） -->
  <circle cx="640" cy="216" r="11" fill="none" stroke="#2b2b46" stroke-width="2.4"/>
  <circle cx="640" cy="216" r="3" fill="#2b2b46"/>

  <text x="640" y="362" text-anchor="middle" font-size="12" fill="#4a463f" font-weight="bold">法线贴图的像素放大</text>
  <text x="640" y="381" text-anchor="middle" font-size="10.5" fill="#7a7468">每个像素的 RGB 编码一根法线：r→x（沿 T）、g→y（沿 B）、b→z（沿 N）</text>
  <text x="640" y="398" text-anchor="middle" font-size="10.5" fill="#7a7468">(128, 128, 255) = 不偏不倚 = 原法线——标志性的浅紫蓝底</text>

  <!-- 两侧连接：像素向量在 TBN 里摆动（低位近水平，穿过 T 标签下方的空档） -->
  <path d="M516,300 C 470,318 436,326 392,326" stroke="#7a7468" stroke-width="1.6" fill="none" stroke-dasharray="6 4" marker-end="url(#arr-link24)"/>
  <text x="508" y="350" text-anchor="end" font-size="10.5" fill="#7a7468">向量在 TBN 里摆动</text>

  <text x="420" y="452" text-anchor="middle" font-size="11" fill="#7a7468">N、UV 坯子里都有；T 不是标配——缺了 T，整套坐标系立不起来（本节的坑）</text>
</svg>
"""

SVG_12_PARALLAX = """<svg viewBox="0 0 860 470" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr-view24" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#274a91"/>
    </marker>
    <marker id="arr-shift24" markerWidth="8" markerHeight="8" refX="6" refY="4" orient="auto">
      <path d="M0,0 L7,4 L0,8 z" fill="#c05a2e"/>
    </marker>
    <linearGradient id="depth-strip24" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0" stop-color="#ffffff"/>
      <stop offset="1" stop-color="#1c1c1c"/>
    </linearGradient>
  </defs>
  <rect x="0" y="0" width="860" height="470" rx="10" fill="#f7f5f0"/>

  <text x="430" y="34" text-anchor="middle" font-size="16" fill="#4a463f" font-weight="bold">视差贴图（侧剖）：沿视线在深度场里找真交点</text>

  <!-- 眼睛 -->
  <g>
    <path d="M62,108 q28,-20 56,0 q-28,20 -56,0 z" fill="#ffffff" stroke="#4a463f" stroke-width="2"/>
    <circle cx="90" cy="108" r="7" fill="#4a463f"/>
  </g>
  <text x="60" y="82" font-size="12" fill="#4a463f">视线</text>

  <!-- 深度场轮廓：平面下方的起伏（云纹意象的波浪） -->
  <path d="M140,250 C 190,250 205,296 250,296 C 295,296 306,254 350,254 C 394,254 402,312 452,312 C 502,312 512,258 556,258 C 600,258 612,300 660,300 C 700,300 712,252 740,252 L 740,250 L 140,250 z"
        fill="#e8e2d2" stroke="#7a7468" stroke-width="1.6" stroke-dasharray="7 4"/>
  <text x="238" y="348" font-size="11.5" fill="#7a7468">深度场：depth_map 骗出来的起伏（几何本身没动）</text>

  <!-- 几何平面：一条水平粗实线 -->
  <line x1="140" y1="250" x2="740" y2="250" stroke="#4a463f" stroke-width="3"/>
  <text x="600" y="236" font-size="11.5" fill="#4a463f">几何平面（真实网格）</text>

  <!-- 视线：斜插，先交平面于 A，再撞深度场于 B -->
  <path d="M118,116 L446,254" stroke="#274a91" stroke-width="2.2" fill="none" stroke-dasharray="8 5"/>
  <path d="M446,254 L512,282" stroke="#274a91" stroke-width="2.2" fill="none" stroke-dasharray="8 5" marker-end="url(#arr-view24)"/>

  <!-- A：视线与平面的交点（空心） -->
  <circle cx="437" cy="250" r="6" fill="#f7f5f0" stroke="#274a91" stroke-width="2.4"/>
  <text x="404" y="200" font-size="12.5" fill="#274a91" font-weight="bold">A</text>
  <text x="330" y="200" font-size="11" fill="#274a91">与平面的交点</text>
  <path d="M432,206 L436,242" stroke="#274a91" stroke-width="1.2" fill="none"/>

  <!-- B：视线与深度场的真交点（实心） -->
  <circle cx="516" cy="284" r="6" fill="#c05a2e" stroke="#c05a2e"/>
  <text x="530" y="308" font-size="12.5" fill="#c05a2e" font-weight="bold">B</text>
  <text x="548" y="308" font-size="11" fill="#c05a2e">与起伏的真交点——采样用它</text>

  <!-- B 投回平面的 B' 与偏移标注 -->
  <line x1="516" y1="284" x2="516" y2="250" stroke="#c05a2e" stroke-width="1.4" stroke-dasharray="4 3"/>
  <circle cx="516" cy="250" r="4" fill="#c05a2e"/>
  <path d="M441,226 L510,226" stroke="#c05a2e" stroke-width="2.2" fill="none" marker-end="url(#arr-shift24)"/>
  <line x1="437" y1="244" x2="437" y2="220" stroke="#c05a2e" stroke-width="1.2" stroke-dasharray="3 3"/>
  <text x="476" y="172" text-anchor="middle" font-size="12.5" fill="#c05a2e" font-weight="bold">采样点往视线来向挪</text>
  <text x="476" y="190" text-anchor="middle" font-size="10.5" fill="#7a7468">越斜挪得越多，越深挪得越多</text>

  <!-- 右上：深度图灰度条与约定 -->
  <rect x="590" y="76" width="200" height="88" rx="8" fill="#ffffff" stroke="#7a7468" stroke-width="1.2"/>
  <rect x="606" y="92" width="168" height="22" fill="url(#depth-strip24)" stroke="#7a7468" stroke-width="0.8"/>
  <text x="606" y="134" font-size="10.5" fill="#7a7468">白 = 深（陷下去）</text>
  <text x="774" y="134" text-anchor="end" font-size="10.5" fill="#7a7468">黑 = 凸（顶到面）</text>
  <text x="690" y="154" text-anchor="middle" font-size="10.5" fill="#4a463f">depth_map 的约定，与直觉相反</text>

  <text x="430" y="440" text-anchor="middle" font-size="11" fill="#7a7468">着色器把深度切层、沿视线逐层试探（Occlusion）；层数不够、刻痕太深，脚印就露出来——Figure 24-14 的阶梯</text>
</svg>
"""


def fig_01_light_ledger_svg() -> None:
    """Figure 24-1：一束光的四路账单（手绘 SVG）。"""
    save_svg(SVG_01_LIGHT_LEDGER, "fig-24-01-light-ledger.svg")


def fig_10_tbn_svg() -> None:
    """Figure 24-10：TBN 切线坐标系（手绘 SVG）。"""
    save_svg(SVG_10_TBN, "fig-24-10-tbn.svg")


def fig_12_parallax_principle_svg() -> None:
    """Figure 24-12：视差贴图原理（手绘 SVG）。"""
    save_svg(SVG_12_PARALLAX, "fig-24-12-parallax-principle.svg")


# ------------------------------------------------- 主流程

ALL = [
    fig_01_light_ledger_svg,
    fig_02_sample_room,
    fig_03_reflectance_row,
    fig_04_specular_tint,
    fig_05_darkroom,
    fig_06_real_lamp,
    fig_07_exposure_weight,
    fig_08_orm_takeover,
    fig_09_ao_compare,
    fig_10_tbn_svg,
    fig_11_carved_lids,
    fig_12_parallax_principle_svg,
    fig_13_parallax_vs_normal,
    fig_14_parallax_layers,
    fig_15_clearcoat,
    fig_16_anisotropy_blowout,
    fig_17_anisotropy_lineup,
    fig_18_alpha_modes,
    fig_19_glass,
    fig_20_transmission_steps,
    fig_21_flag_stages,
    fig_22_depth_bias,
    fig_23_morning_fog,
    fig_24_unlit,
    fig_25_uv_transform,
    fig_26_gallery,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制（默认 feature，全部目标）……")
    cargo("build", "-p", "ch24-materials", "--all-targets")
    only = sys.argv[1] if len(sys.argv) > 1 else None
    try:
        for fig in ALL:
            if only and only not in fig.__name__:
                continue
            fig()
            time.sleep(0.5)
    finally:
        # 收尾兜底：fig_16/fig_20 各自已恢复；这里再统一恢复默认产物，
        # 保证跑到一半中断也不给后续流程留坑
        if only is None:
            print("收尾：恢复默认 feature 的全部产物……")
            cargo("build", "-p", "ch24-materials", "--all-targets")


if __name__ == "__main__":
    main()

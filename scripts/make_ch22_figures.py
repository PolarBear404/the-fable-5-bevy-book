# -*- coding: utf-8 -*-
"""一键重建第 22 章全部位图插图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch22_figures.py [图名筛选]

十六张位图 + 一张动图：灯的档位、测光表、追光（动图+软硬边）、太阳角度、
灯箱、三种影子、影子预算、偏置三病、接触阴影、环境光三连、星空天幕、
大气三连、镜厅、晨雾光柱、切换台四格。
键盘事件用 SendInput 发真实扫描码键击（抄 ch17/ch18）。
前置：scripts/make_ch22_assets.py 已生成天幕与镜厅贴图；产物输出到 book/src/images/ch22/。
"""

import ctypes
import os
import subprocess
import sys
import time
from ctypes import wintypes
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

sys.stdout.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch22-lighting"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch22"

os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example  # noqa: E402

user32 = ctypes.windll.user32
kernel32 = ctypes.windll.kernel32

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36

# ---------------------------------------------------------------- SendInput

ULONG_PTR = ctypes.POINTER(ctypes.c_ulong)


class KEYBDINPUT(ctypes.Structure):
    _fields_ = [
        ("wVk", ctypes.c_ushort),
        ("wScan", ctypes.c_ushort),
        ("dwFlags", ctypes.c_ulong),
        ("time", ctypes.c_ulong),
        ("dwExtraInfo", ULONG_PTR),
    ]


class MOUSEINPUT(ctypes.Structure):
    _fields_ = [
        ("dx", ctypes.c_long),
        ("dy", ctypes.c_long),
        ("mouseData", ctypes.c_ulong),
        ("dwFlags", ctypes.c_ulong),
        ("time", ctypes.c_ulong),
        ("dwExtraInfo", ULONG_PTR),
    ]


class _INPUTunion(ctypes.Union):
    _fields_ = [("ki", KEYBDINPUT), ("mi", MOUSEINPUT)]


class INPUT(ctypes.Structure):
    _fields_ = [("type", ctypes.c_ulong), ("union", _INPUTunion)]


INPUT_KEYBOARD = 1
KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYEVENTF_EXTENDEDKEY = 0x2, 0x8, 0x1

SCAN = {
    "SPACE": 0x39, "TAB": 0x0F,
    "LBRACKET": 0x1A, "RBRACKET": 0x1B,
    "1": 0x02, "2": 0x03, "3": 0x04, "4": 0x05,
    "A": 0x1E, "B": 0x30, "C": 0x2E, "E": 0x12, "F": 0x21, "I": 0x17,
    "P": 0x19, "R": 0x13, "T": 0x14,
    "UP": 0x48, "DOWN": 0x50, "LEFT": 0x4B, "RIGHT": 0x4D,
}
EXTENDED = {"UP", "DOWN", "LEFT", "RIGHT"}


def _send(*inputs: INPUT) -> None:
    array = (INPUT * len(inputs))(*inputs)
    if user32.SendInput(len(inputs), array, ctypes.sizeof(INPUT)) != len(inputs):
        raise RuntimeError("SendInput 未全部送达")


def _key(name: str, up: bool) -> INPUT:
    inp = INPUT()
    inp.type = INPUT_KEYBOARD
    flags = KEYEVENTF_SCANCODE | (KEYEVENTF_KEYUP if up else 0)
    if name in EXTENDED:
        flags |= KEYEVENTF_EXTENDEDKEY
    inp.union.ki = KEYBDINPUT(0, SCAN[name], flags, 0, None)
    return inp


def _key_vk(vk: int, up: bool) -> INPUT:
    inp = INPUT()
    inp.type = INPUT_KEYBOARD
    inp.union.ki = KEYBDINPUT(vk, 0, KEYEVENTF_KEYUP if up else 0, 0, None)
    return inp


def force_foreground(hwnd: int, tries: int = 10) -> None:
    """确保示例窗口在前台拿焦点——SendInput 的键击只进焦点窗口。"""
    for _ in range(tries):
        if user32.GetForegroundWindow() == hwnd:
            return
        # Alt 键小动作：解锁系统的前台切换限制
        _send(_key_vk(0x12, False))
        _send(_key_vk(0x12, True))
        tid_fg = user32.GetWindowThreadProcessId(user32.GetForegroundWindow(), None)
        tid_us = kernel32.GetCurrentThreadId()
        user32.AttachThreadInput(tid_us, tid_fg, True)
        user32.BringWindowToTop(hwnd)
        user32.SetForegroundWindow(hwnd)
        user32.AttachThreadInput(tid_us, tid_fg, False)
        time.sleep(0.2)
    raise RuntimeError("示例窗口拿不到前台焦点，键击会落空——关掉抢焦点的程序再试")


def tap(ex: Example, name: str, hold: float = 0.06) -> None:
    force_foreground(ex.hwnd)
    _send(_key(name, False))
    time.sleep(hold)
    _send(_key(name, True))


def hold(ex: Example, name: str, dur: float) -> None:
    force_foreground(ex.hwnd)
    _send(_key(name, False))
    time.sleep(dur)
    _send(_key(name, True))


# ---------------------------------------------------------------- 通用排版

def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch22-lighting.exe"
    return EXAMPLES / f"{name}.exe"


def label_bar(width: int, texts: list[str]) -> Image.Image:
    bar = Image.new("RGB", (width, LABEL_H), LABEL_BG)
    draw = ImageDraw.Draw(bar)
    cell = width / len(texts)
    for i, text in enumerate(texts):
        w = draw.textlength(text, font=FONT)
        draw.text((cell * i + (cell - w) / 2, 6), text, font=FONT, fill=LABEL_FG)
    return bar


def hstack(images: list[Image.Image], labels: list[str] | None = None) -> Image.Image:
    h = max(im.height for im in images)
    w = sum(im.width for im in images) + GAP * (len(images) - 1)
    top = LABEL_H if labels else 0
    canvas = Image.new("RGB", (w, h + top), GAP_COLOR)
    if labels:
        canvas.paste(label_bar(w, labels), (0, 0))
    x = 0
    for im in images:
        canvas.paste(im, (x, top))
        x += im.width + GAP
    return canvas


def vstack(rows: list[Image.Image]) -> Image.Image:
    w = max(im.width for im in rows)
    h = sum(im.height for im in rows) + GAP * (len(rows) - 1)
    canvas = Image.new("RGB", (w, h), GAP_COLOR)
    y = 0
    for im in rows:
        canvas.paste(im, (0, y))
        y += im.height + GAP
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


HALF = 0.5  # 半尺寸：多联图里单帧缩到 640×360


def shrink(img: Image.Image) -> Image.Image:
    return img.resize((int(img.width * HALF), int(img.height * HALF)), Image.LANCZOS)


# ---------------------------------------------------------------- 各图

def fig_01_lamp_grades() -> None:
    """Figure 22-1：拆堂灯——四档流明，一档一层亮。"""
    shots = []
    with Example(exe("listing-22-01"), workdir=CODE) as ex:
        shots.append(logical(ex.shot(2.6)))
        for t in (3.2, 4.6, 6.0):
            ex.wait_until(t)
            tap(ex, "SPACE")
            shots.append(logical(ex.shot(t + 1.0)))
    rows = [
        hstack([shrink(shots[0]), shrink(shots[1])], ["影院大灯 1,000,000 lm", "堂会汽灯 60,000 lm"]),
        hstack([shrink(shots[2]), shrink(shots[3])], ["大红灯笼 8,000 lm", "白炽灯泡 800 lm"]),
    ]
    save_png(vstack(rows), "fig-22-01-lamp-grades.png")


def fig_02_light_meter() -> None:
    """Figure 22-2：测光表——同一盏灯笼，四档曝光四种夜。"""
    shots = []
    with Example(exe("listing-22-02"), workdir=CODE) as ex:
        shots.append(logical(ex.shot(2.6)))
        for t in (3.2, 4.6, 6.0):
            ex.wait_until(t)
            tap(ex, "E")
            shots.append(logical(ex.shot(t + 1.0)))
    rows = [
        hstack([shrink(shots[0]), shrink(shots[1])], ["默认 EV 9.7", "室内 EV 7.0"]),
        hstack([shrink(shots[2]), shrink(shots[3])], ["阴天 EV 12.0", "烈日 EV 15.0"]),
    ]
    save_png(vstack(rows), "fig-22-02-light-meter.png")


def fig_03_follow_spot() -> None:
    """Figure 22-3：追光跟人（动图）——角儿走圆场，光寸步不离。"""
    with Example(exe("listing-22-03"), workdir=CODE) as ex:
        ex.wait_until(2.4)
        force_foreground(ex.hwnd)
        frames: list[Image.Image] = []

        def grab_for(dur: float, key: str | None = None, fps: int = 10):
            """边抓帧边（可选）按住某键：每几帧重发一次按下，抓帧不至于吃掉键。"""
            n = int(dur * fps)
            for i in range(n):
                t0 = time.perf_counter()
                if key and i % 3 == 0:
                    _send(_key(key, False))
                frames.append(ex.grab().resize((640, 360), Image.LANCZOS))
                rest = 1 / fps - (time.perf_counter() - t0)
                if rest > 0:
                    time.sleep(rest)
            if key:
                _send(_key(key, True))
                time.sleep(0.05)
                _send(_key(key, True))   # 抬键补一枪：抬没抬干净决定下一段的成败

        grab_for(2.4, key="RIGHT")
        time.sleep(0.2)
        force_foreground(ex.hwnd)
        grab_for(4.0, key="LEFT")
        grab_for(1.0)
    path = OUT / "fig-22-03-follow-spot.webp"
    frames[0].save(path, save_all=True, append_images=frames[1:],
                   duration=100, loop=0, method=4, quality=80)
    print(f"fig-22-03-follow-spot.webp：{len(frames)} 帧，{path.stat().st_size // 1024} KB")


def fig_04_spot_edges() -> None:
    """Figure 22-4：软边与硬边——内沿收进去 vs 贴着外沿。"""
    with Example(exe("listing-22-03"), workdir=CODE) as ex:
        soft = logical(ex.shot(2.6))
        ex.wait_until(3.2)
        tap(ex, "I")
        hard = logical(ex.shot(4.2))
    crop = (280, 130, 1140, 700)
    save_png(
        hstack([soft.crop(crop), hard.crop(crop)],
               ["软边：inner 收进去，圈口羽化", "硬边：inner 贴着 outer，像刀裁"]),
        "fig-22-04-spot-edges.png",
    )


def fig_05_sun_angles() -> None:
    """Figure 22-5：太阳的高度角——高角立体、贴地拉丝。"""
    with Example(exe("listing-22-04"), workdir=CODE) as ex:
        high = logical(ex.shot(2.6))
        ex.wait_until(3.2)
        hold(ex, "LEFT", 1.6)
        low = logical(ex.shot(5.4))
    save_png(
        hstack([shrink(high), shrink(low)], ["高度角 0.9 rad", "压到 0.1 rad：立面亮、台面黑"]),
        "fig-22-05-sun-angles.png",
    )


def fig_06_rect_light() -> None:
    """Figure 22-6：灯箱三态——镜面倒影、改宽高、磨砂糊化。"""
    with Example(exe("listing-22-05"), workdir=CODE) as ex:
        base = logical(ex.shot(2.6))
        ex.wait_until(3.2)
        tap(ex, "RIGHT")
        time.sleep(0.2)
        tap(ex, "UP")
        wide = logical(ex.shot(4.4))
        ex.wait_until(5.0)
        tap(ex, "R")
        rough = logical(ex.shot(6.0))
    crop = (240, 130, 1360, 810)

    def cut(im):
        c = im.crop(crop)
        return c.resize((int(c.width * 0.5), int(c.height * 0.5)), Image.LANCZOS)

    save_png(
        hstack([cut(base), cut(wide), cut(rough)],
               ["3.0 × 1.2 米，光面", "加宽加高：倒影跟着变", "台面磨砂：倒影糊成光晕"]),
        "fig-22-06-rect-light.png",
    )


def fig_09_three_shadows() -> None:
    """Figure 22-9：三种灯的影子——平行、放射、锥形。"""
    shots = []
    with Example(exe("listing-22-06"), workdir=CODE) as ex:
        shots.append(logical(ex.shot(2.6)))
        for t in (3.2, 4.6):
            ex.wait_until(t)
            tap(ex, "TAB")
            shots.append(logical(ex.shot(t + 1.0)))
    crop = (120, 240, 1280, 720)

    def cut(im):
        c = im.crop(crop)
        return c.resize((int(c.width * 0.5), int(c.height * 0.5)), Image.LANCZOS)

    save_png(
        hstack([cut(s) for s in shots],
               ["平行光：齐刷刷一个向", "点光：从灯底散开", "聚光：锥里才有影"]),
        "fig-22-09-three-shadows.png",
    )


def fig_10_shadow_budget() -> None:
    """Figure 22-10：影子的预算——分辨率砍到 512，级联再把预算铺对地方。"""
    with Example(exe("listing-22-06"), workdir=CODE) as ex:
        base = logical(ex.shot(2.6))          # 2048 + 默认级联
        ex.wait_until(3.2)
        tap(ex, "RBRACKET")                    # 512
        low = logical(ex.shot(4.2))
        ex.wait_until(4.8)
        tap(ex, "T")                           # 512 + 紧级联
        tight = logical(ex.shot(5.8))
    crop = (60, 300, 900, 680)

    def cut(im):
        c = im.crop(crop)
        return c.resize((int(c.width * 0.62), int(c.height * 0.62)), Image.LANCZOS)

    save_png(
        hstack([cut(base), cut(low), cut(tight)],
               ["2048，默认级联", "512：边缘发虚", "512 + 收紧级联：又利落了"]),
        "fig-22-10-shadow-budget.png",
    )


def fig_12_bias_defects() -> None:
    """Figure 22-12：偏置三病——默认、全关长粉刺、加猛飞影子。"""
    shots = []
    with Example(exe("listing-22-07"), workdir=CODE) as ex:
        shots.append(logical(ex.shot(2.6)))
        for t in (3.2, 4.6):
            ex.wait_until(t)
            tap(ex, "B")
            shots.append(logical(ex.shot(t + 1.0)))
    # 裁近些：粉刺的细条纹缩图后容易糊掉
    crop = (430, 240, 1180, 690)

    def cut(im):
        c = im.crop(crop)
        return c.resize((int(c.width * 0.62), int(c.height * 0.62)), Image.LANCZOS)

    save_png(
        hstack([cut(s) for s in shots],
               ["默认（0.02 / 1.8）", "全关：一身粉刺", "加猛：影子飞了"]),
        "fig-22-12-bias-defects.png",
    )


def fig_13_contact_shadows() -> None:
    """Figure 22-13：接触阴影——彼得潘的影子被根须拽回来。"""
    with Example(exe("listing-22-07"), workdir=CODE) as ex:
        for t in (2.6, 3.2):
            ex.wait_until(t)
            tap(ex, "B")                       # 拨到“加猛”
        detached = logical(ex.shot(4.4))
        ex.wait_until(5.0)
        tap(ex, "C")
        fixed = logical(ex.shot(6.0))
    crop = (150, 220, 1230, 700)

    def cut(im):
        c = im.crop(crop)
        return c.resize((int(c.width * 0.5), int(c.height * 0.5)), Image.LANCZOS)

    save_png(
        hstack([cut(detached), cut(fixed)],
               ["加猛偏置：影子和脚脱开", "+ 接触阴影：根须回来了"]),
        "fig-22-13-contact-shadows.png",
    )


def fig_14_ambient_vs_env() -> None:
    """Figure 22-14：环境光三连——全黑、平光、三色天光。"""
    with Example(exe("listing-22-08"), workdir=CODE) as ex:
        dark = logical(ex.shot(2.6))
        ex.wait_until(3.2)
        tap(ex, "A")
        flat = logical(ex.shot(4.2))
        ex.wait_until(4.8)
        tap(ex, "A")                           # 关环境光
        time.sleep(0.2)
        tap(ex, "E")                           # 上天光
        env = logical(ex.shot(6.0))
    crop = (170, 250, 1110, 700)

    def cut(im):
        c = im.crop(crop)
        return c.resize((int(c.width * 0.56), int(c.height * 0.56)), Image.LANCZOS)

    save_png(
        hstack([cut(dark), cut(flat), cut(env)],
               ["没灯没兜底：全黑", "环境光：平光，镜球一团死灰", "三色天光：镜球照出天与地"]),
        "fig-22-14-ambient-vs-env.png",
    )


def fig_16_star_backdrop() -> None:
    """Figure 22-16：星空天幕——Skybox 画天，滤波过的天光洒在台上。"""
    with Example(exe("listing-22-09"), workdir=CODE) as ex:
        shot = logical(ex.shot(5.5))
    save_png(shot.crop((0, 0, 1280, 700)), "fig-22-16-star-backdrop.png")


def fig_17_atmosphere_trio() -> None:
    """Figure 22-17：大气三连——拂晓、清晨、正午，一颗太阳三种天。"""
    shots = []
    with Example(exe("listing-22-11"), workdir=CODE) as ex:
        ex.wait_until(2.6)
        for key, t in (("1", 3.0), ("2", 4.6), ("3", 6.2)):
            ex.wait_until(t)
            tap(ex, key)
            shots.append(logical(ex.shot(t + 1.0)))

    def cut(im):
        return im.resize((int(im.width * 0.5), int(im.height * 0.5)), Image.LANCZOS)

    save_png(
        hstack([cut(s) for s in shots],
               ["拂晓 0.02 rad", "清晨 0.35 rad", "正午 1.25 rad"]),
        "fig-22-17-atmosphere-trio.png",
    )


def fig_18_mirror_hall() -> None:
    """Figure 22-18：镜厅——球到哪半间，照见哪半间的墙。"""
    with Example(exe("listing-22-12"), workdir=CODE) as ex:
        ex.wait_until(2.6)
        hold(ex, "LEFT", 2.0)
        cool = logical(ex.shot(5.2))
        ex.wait_until(5.6)
        hold(ex, "RIGHT", 3.6)
        warm = logical(ex.shot(9.8))
    crop_l = (20, 200, 630, 640)
    crop_r = (650, 200, 1260, 640)

    def cut(im, c):
        p = im.crop(c)
        return p.resize((int(p.width * 0.62), int(p.height * 0.62)), Image.LANCZOS)

    save_png(
        hstack([cut(cool, crop_l), cut(warm, crop_r)],
               ["冰厅探针：照见冰裂窗", "暖阁探针：照见金棂窗"]),
        "fig-22-18-mirror-hall.png",
    )


def fig_19_god_rays() -> None:
    """Figure 22-19：晨雾光柱——柱缝漏下来的光，在雾里看得见。"""
    with Example(exe("listing-22-13"), workdir=CODE) as ex:
        shot = logical(ex.shot(3.5))
    save_png(shot.crop((0, 0, 1280, 700)), "fig-22-19-god-rays.png")


def fig_20_four_cues() -> None:
    """Figure 22-20：昼夜切换台四格——正午、黄昏、夜戏、晨雾。"""
    shots = []
    with Example(exe("main"), workdir=CODE) as ex:
        shots.append(logical(ex.shot(2.8)))    # cue 1 开场即正午
        for key, t in (("2", 3.4), ("3", 5.4), ("4", 7.4)):
            ex.wait_until(t)
            tap(ex, key)
            shots.append(logical(ex.shot(t + 1.4)))
    rows = [
        hstack([shrink(shots[0]), shrink(shots[1])], ["cue 1 正午", "cue 2 黄昏"]),
        hstack([shrink(shots[2]), shrink(shots[3])], ["cue 3 夜戏", "cue 4 晨雾"]),
    ]
    save_png(vstack(rows), "fig-22-20-four-cues.png")


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_01_lamp_grades,
    fig_02_light_meter,
    fig_03_follow_spot,
    fig_04_spot_edges,
    fig_05_sun_angles,
    fig_06_rect_light,
    fig_09_three_shadows,
    fig_10_shadow_budget,
    fig_12_bias_defects,
    fig_13_contact_shadows,
    fig_14_ambient_vs_env,
    fig_16_star_backdrop,
    fig_17_atmosphere_trio,
    fig_18_mirror_hall,
    fig_19_god_rays,
    fig_20_four_cues,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch22-lighting", "--bins", "--examples"],
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

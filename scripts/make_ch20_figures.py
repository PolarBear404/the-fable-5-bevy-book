# -*- coding: utf-8 -*-
"""一键重建第 20 章全部 PNG/WebP 插图（SVG 示意图为手绘，不在此列）。

    py -3.11 scripts/make_ch20_figures.py [图名筛选]

截图来自各阶段示例与最终成品；键盘事件用 SendInput 发真实扫描码（抄 ch18/19）。
胜利图与动图由一个“跟球 bot”亲手打出来：每 60ms 找一次绣球与条凳的质心，
按住 A/D 追球、贴凳即发球——正式参数、真实输入，不改一行游戏代码。
前置：scripts/make_ch20_assets.py 已就位资产；产物输出到 book/src/images/ch20/。
"""

import ctypes
import os
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch20-breakout"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch20"

os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
from capture import Example, grab_window  # noqa: E402

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
KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE = 0x2, 0x8

SCAN = {
    "SPACE": 0x39, "P": 0x19, "ESC": 0x01, "A": 0x1E, "D": 0x20, "ALT": 0x38,
}


def _key(name: str, up: bool) -> INPUT:
    inp = INPUT()
    inp.type = INPUT_KEYBOARD
    flags = KEYEVENTF_SCANCODE | (KEYEVENTF_KEYUP if up else 0)
    inp.union.ki = KEYBDINPUT(0, SCAN[name], flags, 0, None)
    return inp


def _send(*inputs: INPUT) -> None:
    array = (INPUT * len(inputs))(*inputs)
    if user32.SendInput(len(inputs), array, ctypes.sizeof(INPUT)) != len(inputs):
        raise RuntimeError("SendInput 未全部送达")


def force_foreground(hwnd: int, tries: int = 8) -> None:
    """确保示例窗口在前台拿焦点——SendInput 的键击只进焦点窗口。"""
    for _ in range(tries):
        if user32.GetForegroundWindow() == hwnd:
            return
        _send(_key("ALT", False), _key("ALT", True))
        tid_fg = user32.GetWindowThreadProcessId(user32.GetForegroundWindow(), None)
        tid_us = kernel32.GetCurrentThreadId()
        user32.AttachThreadInput(tid_us, tid_fg, True)
        user32.BringWindowToTop(hwnd)
        user32.SetForegroundWindow(hwnd)
        user32.AttachThreadInput(tid_us, tid_fg, False)
        time.sleep(0.15)
    raise RuntimeError("示例窗口拿不到前台焦点，输入会落空——关掉抢焦点的程序再试")


def tap(hwnd: int, name: str, hold: float = 0.06) -> None:
    force_foreground(hwnd)
    _send(_key(name, False))
    time.sleep(hold)
    _send(_key(name, True))


def hold_key(hwnd: int, name: str, secs: float) -> None:
    force_foreground(hwnd)
    _send(_key(name, False))
    time.sleep(secs)
    _send(_key(name, True))


# ---------------------------------------------------------------- 跟球 bot

def _centroid(px, w, h, y0, y1, test, step=4, x0=0):
    xs = ys = n = 0
    for y in range(y0, y1, step):
        for x in range(x0, w, step):
            r, g, b = px[x, y]
            if test(r, g, b):
                xs += x
                ys += y
                n += 1
    return (xs / n, ys / n, n) if n else None


def _is_ball(r, g, b):  # 绣球红
    return r > 195 and 55 < g < 135 and 45 < b < 125


def _is_paddle(r, g, b):  # 条凳木色
    return 190 < r < 235 and 140 < g < 190 and 65 < b < 125


def _is_wall(r, g, b):  # 戏台木框
    return 80 < r < 115 and 65 < g < 100 and 50 < b < 88


class Bot:
    """凭像素打球：找到球和凳，预测落点站好位；球趴凳上就按空格发球。"""

    # 物理像素几何（125% 缩放）：墙内沿 ±423 世界单位、球半径 11、凳面世界 y −271
    X_MIN = (640 - 423 + 11) * 1.25
    X_MAX = (640 + 423 - 11) * 1.25
    CATCH_Y = (360 + 271 - 11) * 1.25  # 球心到这条线就该被凳接住

    def __init__(self, hwnd: int):
        self.hwnd = hwnd
        self.held: str | None = None
        self.last_serve = 0.0
        self.prev: tuple[float, float, float] | None = None  # (t, x, y)
        self.descents = 0  # 球第几次回落——用来轮换接球偏移
        self.prev_vy = 0.0

    def _hold(self, key: str | None) -> None:
        if self.held == key:
            return
        if self.held:
            _send(_key(self.held, True))
        self.held = key
        if key:
            _send(_key(key, False))

    def _landing_x(self, bx: float, by: float, vx: float, vy: float) -> float:
        """按直线 + 侧墙反射，折算球落到凳面高度时的 x（物理像素）。"""
        if vy <= 1.0:
            return bx
        x = bx + vx * (self.CATCH_Y - by) / vy
        # 折叠进 [X_MIN, X_MAX]：撞墙一次相当于镜像一次
        span = self.X_MAX - self.X_MIN
        x = (x - self.X_MIN) % (2 * span)
        if x > span:
            x = 2 * span - x
        return self.X_MIN + x

    def step(self, img: Image.Image) -> bool:
        """走一步。返回 False 表示结算屏已出现（墙没了），该收手了。"""
        if user32.GetForegroundWindow() != self.hwnd:
            self._hold(None)  # 失焦时松手再夺回来，免得键卡死在别人窗口里
            try:
                force_foreground(self.hwnd)
            except RuntimeError:
                return True  # 夺不回来就这步歇着，下一步再试——别把整局崩掉
        w, h = img.size
        px = img.load()
        # 顶墙在物理 y≈129..146（世界 y 250±7 → 逻辑 103..117 → ×1.25）；
        # 只扫横向中段（避开两侧立柱），结算屏一出（墙没了）立刻知道
        if not _centroid(px, w * 2 // 3, h, 128, 148, _is_wall, step=6, x0=w // 3):
            self._hold(None)
            return False
        now = time.time()
        ball = _centroid(px, w, h, 120, h, _is_ball, step=8)
        # 条凳在物理 y≈789..811（世界 y −280±9）
        paddle = _centroid(px, w, h, 770, 835, _is_paddle)
        if not (ball and paddle):
            self._hold(None)
            return True
        bx, by = ball[0], ball[1]
        pad_x = paddle[0]

        # 球趴在凳上（粘球高度 ≈772）：发球
        if by > 760 and abs(bx - pad_x) < 90 and now - self.last_serve > 0.8:
            _send(_key("SPACE", False))
            time.sleep(0.04)
            _send(_key("SPACE", True))
            self.last_serve = now

        # 速度估计 → 落点预测；球上行或速度未知时回到球的正下方待命。
        # 接球点轮换三档偏移：球没有“击点改角”，全靠换接球位置扫开上行走廊
        target = bx
        if self.prev:
            t0, x0, y0 = self.prev
            dt = now - t0
            if 0.01 < dt < 0.5:
                vx, vy = (bx - x0) / dt, (by - y0) / dt
                if vy > 40.0 >= self.prev_vy:  # 由升转降：换一档接球偏移
                    self.descents += 1
                if vy > 40.0:  # 物理 y 向下为正：在掉
                    target = self._landing_x(bx, by, vx, vy)
                    if by < 500:  # 球还高：用三档接球偏移扫开走廊；进入低空就纯接球
                        target += (-26.0, 0.0, 26.0)[self.descents % 3]
                self.prev_vy = vy
        self.prev = (now, bx, by)

        if target - pad_x > 16:
            self._hold("D")
        elif target - pad_x < -16:
            self._hold("A")
        else:
            self._hold(None)
        return True

    def release(self) -> None:
        self._hold(None)


def bot_play(ex: Example, secs: float, on_frame=None) -> bool:
    """让 bot 打 secs 秒（或直到结算屏出现，返回 True）。on_frame 可顺手收帧。"""
    bot = Bot(ex.hwnd)
    force_foreground(ex.hwnd)
    start = time.time()
    deadline = start + secs
    curtain = False
    misses = 0  # “墙不见了”要连续坐实几帧才算结算屏（启动头几帧还没画东西）
    try:
        while time.time() < deadline:
            img = grab_window(ex.hwnd)
            if on_frame:
                on_frame(img)
            if bot.step(img):  # grab_window 给的已是 RGB
                misses = 0
            else:
                misses += 1
                if misses >= 3:
                    curtain = True
                    break
            # 球在低空时进入急速模式：不睡，全速追帧
            if not (bot.prev and bot.prev[2] > 560):
                time.sleep(0.03)
    finally:
        bot.release()
    print(f"  bot 打了 {time.time() - start:.1f}s，{'见到结算屏' if curtain else '到时收手'}")
    return curtain


# ---------------------------------------------------------------- 通用排版

def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch20-breakout.exe"
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


def vstack(images: list[Image.Image]) -> Image.Image:
    w = max(im.width for im in images)
    h = sum(im.height for im in images) + GAP * (len(images) - 1)
    canvas = Image.new("RGB", (w, h), GAP_COLOR)
    y = 0
    for im in images:
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


def save_webp(frames: list[Image.Image], filename: str, fps: int, quality: int = 65) -> None:
    path = OUT / filename
    frames[0].save(
        path,
        save_all=True,
        append_images=frames[1:],
        duration=int(1000 / fps),
        loop=0,
        quality=quality,
        method=4,
    )
    kb = path.stat().st_size // 1024
    print(f"{filename}：{len(frames)} 帧，{kb} KB")
    if kb > 2000:
        print("  警告：超过 2 MB 上限，考虑降帧率/质量/裁切")


def ball_world(img: Image.Image):
    """在逻辑帧（1280×720）里找球心的世界坐标；找不到返回 None。"""
    px = img.convert("RGB").load()
    w, h = img.size
    hit = _centroid(px, w, h, 0, h, _is_ball, step=3)
    if not hit:
        return None
    return hit[0] - w / 2, h / 2 - hit[1]


# 球场的常用裁切（逻辑像素）：场地连同记分牌一行
ARENA = (170, 20, 1110, 720)


# ---------------------------------------------------------------- 各图

def fig_02_court() -> None:
    """Figure 20-2：搭台——开局居中，按住 D 推到右墙根。"""
    with Example(exe("listing-20-01"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(1.2)
        centered = logical(ex.grab())
        hold_key(ex.hwnd, "D", 1.0)
        pushed = logical(ex.grab())
    crop = (170, 80, 1110, 660)
    save_png(
        hstack(
            [centered.crop(crop).resize((588, 363)), pushed.crop(crop).resize((588, 363))],
            ["开局：条凳居中", "按住 D：推到右墙根被夹住"],
        ),
        "fig-20-02-court.png",
    )


def fig_03_escape() -> None:
    """Figure 20-3：放球——直线飞行，对墙视而不见，穿出去一去不回。"""
    with Example(exe("listing-20-02"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        frames = [logical(f) for f in ex.record(start=0.5, dur=2.6, fps=20)]
    in_flight = piercing = gone = None
    for f in frames:
        pos = ball_world(f)
        if pos is None:
            if piercing is not None and gone is None:
                gone = f
            continue
        x, y = pos
        if in_flight is None and -60 < y < 140:
            in_flight = f
        # 顶墙带：中心 250、厚 14、球半径 11 → 球心在 232..268 即压在墙上
        if piercing is None and 228 < y < 272 and x < 420:
            piercing = f
    if piercing is None:  # 兜底：取最高的一帧
        piercing = max((f for f in frames if ball_world(f)), key=lambda f: ball_world(f)[1])
    if gone is None:
        gone = frames[-1]
    crop = (560, 30, 1200, 430)
    save_png(
        hstack(
            [im.crop(crop).resize((420, 263)) for im in (in_flight, piercing, gone)],
            ["上路：直线飞行", "撞上顶墙——直接穿了过去", "出了台口，一去不回"],
        ),
        "fig-20-03-escape.png",
    )


def fig_05_tile_wall() -> None:
    """Figure 20-5：瓦阵——刚铺好的 56 片 vs 打了一阵后的缺口。

    Listing 20-4 的球出生就在飞、半秒就破第一片，抢不到完好阵；
    “完好”那幅借 Listing 20-6 的开局（球粘在凳上，瓦阵原封不动），
    裁掉字牌与凳，台面与 20-4 完全相同。
    """
    with Example(exe("listing-20-06"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(1.2)
        tap(ex.hwnd, "SPACE")  # 开局——球粘在凳上，瓦阵完好
        time.sleep(0.8)
        intact = logical(ex.grab())
    with Example(exe("listing-20-04"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(0.3)  # 球出生就在飞，第一落点要从头追
        bot_play(ex, 25.0)  # bot 接球，让缺口开得更大
        battered = logical(ex.grab())
    crop = (170, 80, 1110, 560)
    save_png(
        hstack(
            [intact.crop(crop).resize((588, 300)), battered.crop(crop).resize((588, 300))],
            ["开局：8 列 × 7 行，顶上两行带釉", "打了一阵：瓦阵开了口"],
        ),
        "fig-20-05-tile-wall.png",
    )


def fig_06_scoreboard() -> None:
    """Figure 20-6：记分牌与瓦阵对账——缺了几片，牌上就记几片。

    bot 陪打 25 秒、顺手收帧；取“球还活着”的最后一帧（20-5 还没有补球，
    球一掉分数就冻结，那一帧正是战况最厚的一帧）。
    """
    frames: list[Image.Image] = []
    last = [0.0]

    def collect(img: Image.Image) -> None:
        now = time.perf_counter()
        if now - last[0] >= 0.5:
            frames.append(logical(img))
            last[0] = now

    with Example(exe("listing-20-05"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(0.3)  # 球出生就在飞，第一落点要从头追
        bot_play(ex, 25.0, on_frame=collect)
    alive = [f for f in frames if ball_world(f)]
    pick = alive[-1] if alive else frames[-1]
    save_png(pick.crop((170, 24, 1110, 500)), "fig-20-06-scoreboard.png")


def fig_08_menu_curtain() -> None:
    """Figure 20-8：后台招牌与结算屏（输的那种）——最终成品跑出来的。"""
    with Example(exe("main"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(1.5)
        menu = logical(ex.grab())
        tap(ex.hwnd, "SPACE")
        time.sleep(1.0)
        for _ in range(3):  # 发球后把凳挪开，三只绣球喂沟
            tap(ex.hwnd, "SPACE")
            time.sleep(0.4)
            hold_key(ex.hwnd, "A", 1.2)
            time.sleep(4.5)
        curtain = logical(ex.grab())
    crop = (200, 120, 1080, 640)
    save_png(
        hstack(
            [menu.crop(crop).resize((550, 325)), curtain.crop(crop).resize((550, 325))],
            ["OnEnter(Menu)：后台招牌", "OnEnter(GameOver)：绣球散尽"],
        ),
        "fig-20-08-menu-curtain.png",
    )


def fig_09_intermission() -> None:
    """Figure 20-9：中场——幕布压在定格的台面上。"""
    with Example(exe("main"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(1.5)
        tap(ex.hwnd, "SPACE")
        time.sleep(1.0)
        tap(ex.hwnd, "SPACE")
        time.sleep(2.2)
        tap(ex.hwnd, "P")
        time.sleep(0.8)
        paused = logical(ex.grab())
    save_png(paused.crop((170, 20, 1110, 700)), "fig-20-09-intermission.png")


def fig_11_playthrough() -> None:
    """Figure 20-11：动图——开局、发球、接球砸瓦的十来秒。"""
    frames: list[Image.Image] = []
    last = [0.0]

    def collect(img: Image.Image) -> None:
        now = time.perf_counter()
        if now - last[0] >= 0.1:  # 10 fps
            frames.append(logical(img))
            last[0] = now

    with Example(exe("main"), workdir=CODE) as ex:
        force_foreground(ex.hwnd)
        ex.wait_until(1.5)
        tap(ex.hwnd, "SPACE")
        time.sleep(0.8)
        bot_play(ex, 13.0, on_frame=collect)
    crop = (170, 20, 1110, 700)
    frames = [f.crop(crop).resize((700, 506), Image.LANCZOS) for f in frames]
    save_webp(frames, "fig-20-11-playthrough.webp", fps=10, quality=62)


# ---------------------------------------------------------------- 主流程

ALL = [
    fig_02_court,
    fig_03_escape,
    fig_05_tile_wall,
    fig_06_scoreboard,
    fig_08_menu_curtain,
    fig_09_intermission,
    fig_11_playthrough,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch20-breakout", "--bins", "--examples"],
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

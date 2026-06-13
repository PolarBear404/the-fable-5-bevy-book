# -*- coding: utf-8 -*-
"""ch20 资产一键重建：音效用 Python 标准库（wave + math）现场合成，
BGM 与落沟鼓声复用 ch19 的合成产物，字体复用 ch16 的子集字体。
本章不下载、不手做任何二进制。

用法：py -3.11 scripts/make_ch20_assets.py

合成产物（mono / 22050 Hz / 16-bit PCM WAV，Bevy 侧需开 `wav` feature，同 ch19）：
  - sfx/clack.wav   —— 梆子味的“哒”：球撞墙/凳的弹响，0.10 秒；
      凳与裂瓦在游戏里用 PlaybackSettings::with_speed 变调复用同一份素材（19.2 的招）
  - sfx/shatter.wav —— 瓦碎：陶片非谐分音 + 噪声簇快衰减，0.34 秒
  - sfx/win.wav     —— 满堂彩：D 宫五声上行琶音收锣，约 1.6 秒
  - sfx/lose.wav    —— 绣球散尽：宫音下行三音叹气，约 1.4 秒

复用产物（均为上游脚本生成、已入 git 的文件）：
  - code/ch19-audio/assets/music/changfeng-overture.wav —— BGM《长风渡》序曲
      （scripts/make_ch19_assets.py 合成；散场后的保留节目，曲子也沿用首演的）
  - code/ch19-audio/assets/sfx/drum.wav —— 堂鼓：绣球落沟的那一声闷响
  - code/ch16-text/assets/fonts/*.otf + OFL.txt —— 中文字体子集（OFL 改名版）
"""

import math
import shutil
import struct
import sys
import wave
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
DEST = CODE / "ch20-breakout" / "assets"

SAMPLE_RATE = 22050

# ---------------------------------------------------------------- 合成基件


def write_wav(path: Path, samples: list[float]) -> None:
    """浮点采样（-1.0..1.0）写成 16-bit mono WAV。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    peak = max(1e-9, max(abs(s) for s in samples))
    norm = 0.85 / peak if peak > 0.85 else 1.0  # 峰值压到 0.85，防削波
    with wave.open(str(path), "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(SAMPLE_RATE)
        w.writeframes(
            b"".join(
                struct.pack("<h", int(max(-1.0, min(1.0, s * norm)) * 32767))
                for s in samples
            )
        )
    secs = len(samples) / SAMPLE_RATE
    print(f"  {path.relative_to(CODE)}  {secs:.2f}s  {path.stat().st_size // 1024} KB")


def silence(seconds: float) -> list[float]:
    return [0.0] * int(SAMPLE_RATE * seconds)


def add_into(buf: list[float], start_sec: float, voice: list[float]) -> None:
    """把一段声音叠加进缓冲区的指定时刻（混音）。"""
    offset = int(start_sec * SAMPLE_RATE)
    for i, s in enumerate(voice):
        if 0 <= offset + i < len(buf):
            buf[offset + i] += s


def pluck(freq: float, dur: float, amp: float, decay: float) -> list[float]:
    """拨弦感音色（与 ch19 同款）：基波 + 两个谐波，短起音 + 指数衰减。"""
    n = int(SAMPLE_RATE * dur)
    out = []
    for i in range(n):
        t = i / SAMPLE_RATE
        attack = min(t / 0.008, 1.0)
        release = min((dur - t) / 0.012, 1.0)
        env = attack * release * math.exp(-decay * t)
        w = (
            math.sin(2 * math.pi * freq * t)
            + 0.30 * math.sin(2 * math.pi * freq * 2 * t)
            + 0.12 * math.sin(2 * math.pi * freq * 3 * t)
        )
        out.append(amp * env * w)
    return out


# ---------------------------------------------------------------- 各个文件


def make_clack(path: Path) -> None:
    """梆子味的“哒”：两支高频分音 + 一点起音噪声，0.10 秒收干净。"""
    dur = 0.10
    n = int(SAMPLE_RATE * dur)
    out = []
    noise = 0.0
    for i in range(n):
        t = i / SAMPLE_RATE
        env = min(t / 0.002, 1.0) * min((dur - t) / 0.012, 1.0) * math.exp(-32.0 * t)
        tone = math.sin(2 * math.pi * 860.0 * t) + 0.45 * math.sin(2 * math.pi * 1420.0 * t)
        noise = noise * 0.80 + (hash((i, 3)) % 1000 / 500.0 - 1.0) * 0.20
        click = noise * math.exp(-t / 0.006) * 0.30
        out.append(env * (tone + click))
    write_wav(path, out)


def make_shatter(path: Path) -> None:
    """瓦碎：几支不成整数比的陶片分音各自衰减 + 碎裂噪声簇，0.34 秒。"""
    dur = 0.34
    partials = [(1.00, 1.00, 11.0), (1.83, 0.62, 14.0), (2.51, 0.50, 17.0),
                (3.46, 0.34, 21.0), (4.33, 0.22, 26.0)]
    base = 620.0
    n = int(SAMPLE_RATE * dur)
    out = []
    noise = 0.0
    for i in range(n):
        t = i / SAMPLE_RATE
        attack = min(t / 0.002, 1.0)
        release = min((dur - t) / 0.02, 1.0)
        tone = sum(
            a * math.exp(-k * t) * math.sin(2 * math.pi * base * r * t)
            for r, a, k in partials
        )
        noise = noise * 0.62 + (hash((i, 9)) % 1000 / 500.0 - 1.0) * 0.38
        crackle = noise * math.exp(-t / 0.05) * 0.55
        out.append(attack * release * (tone + crackle))
    write_wav(path, out)


def make_win(path: Path) -> None:
    """满堂彩：D 宫五声音阶一路向上，末了高宫音长收，约 1.6 秒。"""
    d4, e4, fs4, a4, b4, d5 = 293.66, 329.63, 369.99, 440.00, 493.88, 587.33
    steps = [(0.00, d4, 0.18), (0.14, e4, 0.18), (0.28, fs4, 0.18),
             (0.42, a4, 0.18), (0.56, b4, 0.18), (0.72, d5, 0.85)]
    buf = silence(1.65)
    for start, freq, dur in steps:
        add_into(buf, start, pluck(freq, dur, 0.55, 2.0))
    # 高宫音上再叠一个五度，像收锣的余响
    add_into(buf, 0.72, pluck(d5 * 1.5, 0.85, 0.28, 2.4))
    write_wav(path, buf)


def make_lose(path: Path) -> None:
    """绣球散尽：宫—羽—徵 下行三音，越走越低、越走越轻，约 1.4 秒。"""
    d4, b3, a3 = 293.66, 246.94, 220.00
    steps = [(0.00, d4, 0.35, 0.50), (0.30, b3, 0.40, 0.42), (0.62, a3, 0.80, 0.36)]
    buf = silence(1.45)
    for start, freq, dur, amp in steps:
        add_into(buf, start, pluck(freq, dur, amp, 2.6))
    write_wav(path, buf)


# ---------------------------------------------------------------- 复用前章

REUSE = [
    # BGM 与堂鼓来自 ch19 的合成脚本（make_ch19_assets.py）
    ("ch19-audio/assets/music/changfeng-overture.wav", "music/changfeng-overture.wav"),
    ("ch19-audio/assets/sfx/drum.wav", "sfx/drum.wav"),
    # 字体来自 ch16 的子集化脚本（make_ch16_assets.py）
    ("ch16-text/assets/fonts/book-sans-sc-regular.otf", "fonts/book-sans-sc-regular.otf"),
    ("ch16-text/assets/fonts/book-sans-sc-bold.otf", "fonts/book-sans-sc-bold.otf"),
    ("ch16-text/assets/fonts/OFL.txt", "fonts/OFL.txt"),
]


def main() -> None:
    missing = [src for src, _ in REUSE if not (CODE / src).exists()]
    if missing:
        for src in missing:
            print(f"缺上游资产：code/{src}")
        print("先运行 make_ch16_assets.py / make_ch19_assets.py 再来。")
        sys.exit(1)

    print("合成 ch20 音效（标准库 wave + math）：")
    make_clack(DEST / "sfx" / "clack.wav")
    make_shatter(DEST / "sfx" / "shatter.wav")
    make_win(DEST / "sfx" / "win.wav")
    make_lose(DEST / "sfx" / "lose.wav")

    print("就位复用资产（前章脚本产物）：")
    for src, dst in REUSE:
        target = DEST / dst
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(CODE / src, target)
        print(f"  assets/{dst}  <- code/{src}")


if __name__ == "__main__":
    main()

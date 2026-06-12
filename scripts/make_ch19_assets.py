# -*- coding: utf-8 -*-
"""ch19 资产一键重建：音频全部用 Python 标准库（wave + math）现场合成，
字体与贴图复用前章脚本化资产，本章不下载、不手做任何二进制。

用法：py -3.11 scripts/make_ch19_assets.py

合成产物（mono / 22050 Hz / 16-bit PCM WAV——WAV 无压缩、标准库可直接写，
代价是 Bevy 需开 `wav` feature，正文 19.1 节正好拿这事讲格式与 feature）：
  - music/changfeng-overture.wav —— 《长风渡》序曲：D 宫五声调式双声部小循环，
      BPM 96、4/4 拍共 4 小节 = 10.0 秒整；所有音符包络在小节内收尾、
      尾音不跨循环点，结束和声回到宫音，首尾相接无缝循环
  - sfx/gong.wav   —— 锣：非谐泛音列 + 指数衰减 + 轻微音高下滑，2.2 秒
  - sfx/drum.wav   —— 堂鼓：90→55 Hz 扫频正弦 + 起音噪声，0.5 秒
  - sfx/bangzi-loop.wav —— 梆子巡更循环：1.6 秒内两响（哒、哒），
      击打音 60 毫秒即衰减归零，循环点处静音，可无缝循环

复用产物（均为上游脚本生成、已入 git 的文件）：
  - code/ch15-sprites/assets/ —— 阿燕十二格连环画、桥板贴片
  - code/ch16-text/assets/    —— 中文字体子集（OFL 改名版）
"""

import math
import shutil
import struct
import sys
import wave
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
DEST = CODE / "ch19-audio" / "assets"

SAMPLE_RATE = 22050

# ---------------------------------------------------------------- 合成基件


def write_wav(path: Path, samples: list[float]) -> None:
    """浮点采样（-1.0..1.0）写成 16-bit mono WAV。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    peak = max(1e-9, max(abs(s) for s in samples))
    norm = 0.85 / peak if peak > 0.85 else 1.0  # 峰值压到 0.85，留响度余量防削波
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


# ---------------------------------------------------------------- 乐器音色


def pluck(freq: float, dur: float, amp: float, decay: float) -> list[float]:
    """拨弦感音色：正弦基波 + 两个谐波，8ms 起音斜坡 + 指数衰减。"""
    n = int(SAMPLE_RATE * dur)
    out = []
    for i in range(n):
        t = i / SAMPLE_RATE
        attack = min(t / 0.008, 1.0)
        release = min((dur - t) / 0.012, 1.0)  # 末端 12ms 收音，杜绝爆音
        env = attack * release * math.exp(-decay * t)
        w = (
            math.sin(2 * math.pi * freq * t)
            + 0.30 * math.sin(2 * math.pi * freq * 2 * t)
            + 0.12 * math.sin(2 * math.pi * freq * 3 * t)
        )
        out.append(amp * env * w)
    return out


def gong(base: float = 165.0, dur: float = 2.2) -> list[float]:
    """锣：非谐泛音列（锣面震动模态不成整数比）各自衰减 + 整体音高微滑。"""
    partials = [(1.00, 1.00, 1.1), (1.48, 0.62, 1.5), (2.39, 0.45, 2.0),
                (3.04, 0.30, 2.6), (4.13, 0.18, 3.2)]
    n = int(SAMPLE_RATE * dur)
    out = []
    for i in range(n):
        t = i / SAMPLE_RATE
        attack = min(t / 0.003, 1.0)
        release = min((dur - t) / 0.05, 1.0)
        glide = 1.0 - 0.012 * t  # 敲响后锣面绷紧度松弛，音高缓缓下滑
        s = sum(
            a * math.exp(-k * t) * math.sin(2 * math.pi * base * r * glide * t)
            for r, a, k in partials
        )
        out.append(attack * release * s)
    return out


def drum(dur: float = 0.5) -> list[float]:
    """堂鼓：90→55 Hz 扫频正弦（鼓膜大振幅时音高更高）+ 8ms 起音噪声。"""
    n = int(SAMPLE_RATE * dur)
    out = []
    phase = 0.0
    noise = 1.0
    for i in range(n):
        t = i / SAMPLE_RATE
        freq = 55.0 + 35.0 * math.exp(-18.0 * t)
        phase += 2 * math.pi * freq / SAMPLE_RATE
        release = min((dur - t) / 0.04, 1.0)
        env = min(t / 0.004, 1.0) * release * math.exp(-9.0 * t)
        noise = noise * 0.82 + (hash((i, 7)) % 1000 / 500.0 - 1.0) * 0.18  # 伪随机起音杂色
        click = noise * math.exp(-t / 0.008) * 0.35
        out.append(env * (math.sin(phase) + click))
    return out


def bangzi_hit() -> list[float]:
    """梆子单击：高频短衰减双分音，60ms 内归零。"""
    dur = 0.06
    n = int(SAMPLE_RATE * dur)
    out = []
    for i in range(n):
        t = i / SAMPLE_RATE
        env = min(t / 0.002, 1.0) * min((dur - t) / 0.01, 1.0) * math.exp(-28.0 * t)
        s = math.sin(2 * math.pi * 1150.0 * t) + 0.4 * math.sin(2 * math.pi * 1900.0 * t)
        out.append(env * s)
    return out


# ---------------------------------------------------------------- 各个文件


def make_overture(path: Path) -> None:
    """《长风渡》序曲：D 宫五声调式，4 小节循环。

    BPM 96 → 1 拍 = 0.625s，16 拍 = 10.0s 整。
    旋律声部拨弦音色衰减快（decay 2.2），低音声部绵长（decay 0.8）；
    每个音符的时值都不越过 10s 边界，循环点处只剩低音余韵的静默尾，无缝。
    """
    beat = 60.0 / 96.0
    d4, e4, fs4, a4, b4, d5 = 293.66, 329.63, 369.99, 440.00, 493.88, 587.33
    d3, a2, b2 = 146.83, 110.00, 123.47

    # (起始拍, 频率, 时值拍数) —— 旋律：起、承、转、合，末音落回宫音 D
    melody = [
        (0.0, d4, 1.0), (1.0, fs4, 0.5), (1.5, e4, 0.5), (2.0, a4, 2.0),
        (4.0, b4, 1.0), (5.0, a4, 0.5), (5.5, fs4, 0.5), (6.0, e4, 2.0),
        (8.0, d4, 1.0), (9.0, e4, 0.5), (9.5, fs4, 0.5), (10.0, a4, 1.0), (11.0, d5, 1.0),
        (12.0, b4, 1.0), (13.0, a4, 0.5), (13.5, fs4, 0.5), (14.0, d4, 1.9),
    ]
    # 低音：每小节一个根音长音，4-1-4-5 进行收回主音
    bass = [(0.0, d3, 3.9), (4.0, b2, 3.9), (8.0, d3, 3.9), (12.0, a2, 3.9)]

    buf = silence(16 * beat)
    for start, freq, beats in melody:
        add_into(buf, start * beat, pluck(freq, beats * beat, 0.55, 2.2))
    for start, freq, beats in bass:
        add_into(buf, start * beat, pluck(freq, beats * beat, 0.40, 0.8))
    write_wav(path, buf)


def make_gong(path: Path) -> None:
    write_wav(path, gong())


def make_drum(path: Path) -> None:
    write_wav(path, drum())


def make_bangzi_loop(path: Path) -> None:
    """巡更节奏：1.6 秒内两响（0.0s、0.8s），其余静音，循环即连绵更声。"""
    buf = silence(1.6)
    hit = bangzi_hit()
    add_into(buf, 0.0, hit)
    add_into(buf, 0.8, [s * 0.85 for s in hit])  # 第二响稍轻，有人味
    write_wav(path, buf)


# ---------------------------------------------------------------- 复用前章

REUSE = [
    ("ch15-sprites/assets/actors/ayan-sheet.png", "actors/ayan-sheet.png"),
    ("ch15-sprites/assets/props/dock-plank.png", "props/dock-plank.png"),
    ("ch16-text/assets/fonts/book-sans-sc-regular.otf", "fonts/book-sans-sc-regular.otf"),
    ("ch16-text/assets/fonts/book-sans-sc-bold.otf", "fonts/book-sans-sc-bold.otf"),
    ("ch16-text/assets/fonts/OFL.txt", "fonts/OFL.txt"),
]


def main() -> None:
    missing = [src for src, _ in REUSE if not (CODE / src).exists()]
    if missing:
        for src in missing:
            print(f"缺上游资产：code/{src}")
        print("先运行 make_ch15_assets.py / make_ch16_assets.py 再来。")
        sys.exit(1)

    print("合成 ch19 音频（标准库 wave + math）：")
    make_overture(DEST / "music" / "changfeng-overture.wav")
    make_gong(DEST / "sfx" / "gong.wav")
    make_drum(DEST / "sfx" / "drum.wav")
    make_bangzi_loop(DEST / "sfx" / "bangzi-loop.wav")

    print("就位复用资产（前章脚本产物）：")
    for src, dst in REUSE:
        target = DEST / dst
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(CODE / src, target)
        print(f"  assets/{dst}  <- code/{src}")


if __name__ == "__main__":
    main()

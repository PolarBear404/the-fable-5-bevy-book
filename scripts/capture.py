"""可复现的示例截图工具：启动 Bevy 示例，按时刻截取窗口客户区。

供 make_chXX_figures.py 脚本 import 使用，也可单独运行：
    py -3.11 scripts/capture.py shot <exe路径> <输出.png> --at 6
    py -3.11 scripts/capture.py record <exe路径> <输出目录> --start 1.5 --dur 12 --fps 10

约定：截图为窗口客户区（无标题栏/边框），物理像素。
进程以 WM_CLOSE 优雅关闭（保证示例的 stdout 正常 flush，不影响行为）。
"""

import argparse
import ctypes
import subprocess
import sys
import time
from ctypes import wintypes
from pathlib import Path

from PIL import Image

user32 = ctypes.windll.user32
gdi32 = ctypes.windll.gdi32

WM_CLOSE = 0x0010
PW_RENDERFULLCONTENT = 2  # PrintWindow：含 GPU 合成内容（Win 8.1+）
BI_RGB = 0
DIB_RGB_COLORS = 0


class BITMAPINFOHEADER(ctypes.Structure):
    _fields_ = [
        ("biSize", wintypes.DWORD),
        ("biWidth", ctypes.c_long),
        ("biHeight", ctypes.c_long),
        ("biPlanes", wintypes.WORD),
        ("biBitCount", wintypes.WORD),
        ("biCompression", wintypes.DWORD),
        ("biSizeImage", wintypes.DWORD),
        ("biXPelsPerMeter", ctypes.c_long),
        ("biYPelsPerMeter", ctypes.c_long),
        ("biClrUsed", wintypes.DWORD),
        ("biClrImportant", wintypes.DWORD),
    ]


def _set_dpi_aware() -> None:
    """让本进程按物理像素看世界，避免 125% 缩放下截图错位。"""
    try:
        ctypes.windll.shcore.SetProcessDpiAwareness(2)
    except OSError:
        user32.SetProcessDPIAware()


def find_main_window(pid: int, timeout: float = 30.0) -> int:
    """轮询查找属于 pid 的可见、客户区非零的顶层窗口，返回 HWND。

    渲染器初始化可能耗时数秒，期间 winit 只有一个 0x0 的内部消息窗口
    （"Winit Thread Event Target"），必须靠客户区尺寸把它排除。
    """
    EnumWindowsProc = ctypes.WINFUNCTYPE(
        wintypes.BOOL, wintypes.HWND, wintypes.LPARAM
    )
    found: list[int] = []

    def callback(hwnd: int, _lparam: int) -> bool:
        if not user32.IsWindowVisible(hwnd):
            return True
        owner = wintypes.DWORD()
        user32.GetWindowThreadProcessId(hwnd, ctypes.byref(owner))
        if owner.value != pid:
            return True
        rect = wintypes.RECT()
        user32.GetClientRect(hwnd, ctypes.byref(rect))
        if rect.right > 0 and rect.bottom > 0:
            found.append(hwnd)
            return False
        return True

    deadline = time.perf_counter() + timeout
    while time.perf_counter() < deadline:
        found.clear()
        user32.EnumWindows(EnumWindowsProc(callback), 0)
        if found:
            return found[0]
        time.sleep(0.1)
    raise RuntimeError(f"进程 {pid} 在 {timeout}s 内没有出现可见的渲染窗口")


def grab_window(hwnd: int) -> Image.Image:
    """用 PrintWindow 截取窗口的客户区内容。

    直接从窗口取像素而非抄屏幕，因此不怕被其他窗口遮挡、不怕失去前台。
    """
    # 整窗与客户区的几何关系：PrintWindow 给整窗位图，最后裁出客户区
    win = wintypes.RECT()
    user32.GetWindowRect(hwnd, ctypes.byref(win))
    client = wintypes.RECT()
    user32.GetClientRect(hwnd, ctypes.byref(client))
    origin = wintypes.POINT(0, 0)
    user32.ClientToScreen(hwnd, ctypes.byref(origin))
    off_x, off_y = origin.x - win.left, origin.y - win.top
    win_w, win_h = win.right - win.left, win.bottom - win.top

    hdc_win = user32.GetWindowDC(hwnd)
    hdc_mem = gdi32.CreateCompatibleDC(hdc_win)
    bmp = gdi32.CreateCompatibleBitmap(hdc_win, win_w, win_h)
    gdi32.SelectObject(hdc_mem, bmp)
    try:
        if not user32.PrintWindow(hwnd, hdc_mem, PW_RENDERFULLCONTENT):
            raise RuntimeError("PrintWindow 失败")
        info = BITMAPINFOHEADER()
        info.biSize = ctypes.sizeof(BITMAPINFOHEADER)
        info.biWidth = win_w
        info.biHeight = -win_h  # 负值 = 自上而下的行序
        info.biPlanes = 1
        info.biBitCount = 32
        info.biCompression = BI_RGB
        buf = ctypes.create_string_buffer(win_w * win_h * 4)
        gdi32.GetDIBits(hdc_mem, bmp, 0, win_h, buf, ctypes.byref(info), DIB_RGB_COLORS)
        img = Image.frombuffer("RGB", (win_w, win_h), buf, "raw", "BGRX", 0, 1)
    finally:
        gdi32.DeleteObject(bmp)
        gdi32.DeleteDC(hdc_mem)
        user32.ReleaseDC(hwnd, hdc_win)
    return img.crop((off_x, off_y, off_x + client.right, off_y + client.bottom))


class Example:
    """一个正在运行的示例窗口。用 with 语句保证关闭。"""

    def __init__(self, exe: str | Path, workdir: str | Path | None = None):
        _set_dpi_aware()
        self.proc = subprocess.Popen(
            [str(exe)],
            cwd=str(workdir) if workdir else None,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        self.hwnd = find_main_window(self.proc.pid)
        # 以窗口出现的时刻为时间零点：与示例内 Time 的起点（首帧）基本对齐，
        # 不受渲染器初始化耗时（数秒、随机器而异）的影响
        self.t0 = time.perf_counter()
        user32.SetForegroundWindow(self.hwnd)

    def wait_until(self, t_since_window: float) -> None:
        """睡到窗口出现后第 t 秒（≈ 示例内 Time 的第 t 秒）。"""
        remain = self.t0 + t_since_window - time.perf_counter()
        if remain > 0:
            time.sleep(remain)

    def grab(self):
        return grab_window(self.hwnd)

    def shot(self, t: float):
        """启动后第 t 秒截一帧。"""
        self.wait_until(t)
        return self.grab()

    def record(self, start: float, dur: float, fps: int, size=None) -> list:
        """从启动后第 start 秒起，以 fps 录 dur 秒，返回帧列表（可选缩放）。"""
        frames = []
        total = int(dur * fps)
        for i in range(total):
            self.wait_until(start + i / fps)
            img = self.grab()
            if size:
                img = img.resize(size)
            frames.append(img)
        return frames

    def close(self) -> None:
        user32.PostMessageW(self.hwnd, WM_CLOSE, 0, 0)
        try:
            self.proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            self.proc.kill()

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        self.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_shot = sub.add_parser("shot", help="截单帧（或多个时刻多帧）")
    p_shot.add_argument("exe")
    p_shot.add_argument("out")
    p_shot.add_argument("--at", type=float, nargs="+", required=True)

    p_rec = sub.add_parser("record", help="录帧序列为 PNG")
    p_rec.add_argument("exe")
    p_rec.add_argument("outdir")
    p_rec.add_argument("--start", type=float, default=1.0)
    p_rec.add_argument("--dur", type=float, default=10.0)
    p_rec.add_argument("--fps", type=int, default=10)

    args = parser.parse_args()
    if args.cmd == "shot":
        with Example(args.exe) as ex:
            for i, t in enumerate(args.at):
                img = ex.shot(t)
                out = Path(args.out)
                if len(args.at) > 1:
                    out = out.with_stem(f"{out.stem}-{i + 1}")
                img.save(out)
                print(f"已保存 {out}（t={t}s，{img.size[0]}x{img.size[1]}）")
    elif args.cmd == "record":
        outdir = Path(args.outdir)
        outdir.mkdir(parents=True, exist_ok=True)
        with Example(args.exe) as ex:
            frames = ex.record(args.start, args.dur, args.fps)
        for i, img in enumerate(frames):
            img.save(outdir / f"frame-{i:04}.png")
        print(f"已保存 {len(frames)} 帧到 {outdir}")


if __name__ == "__main__":
    main()

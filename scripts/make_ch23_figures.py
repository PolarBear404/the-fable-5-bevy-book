# -*- coding: utf-8 -*-
"""一键重建第 23 章全部 14 张插图（10 张运行截图 + 1 张 WebP 动图 + 3 张手绘 SVG）。

    py -3.11 scripts/make_ch23_figures.py [图名筛选]

运行图：阿福首秀、素坯双头、换场重影、灯与素台、借机位两联、挂灯笼、
换漆两联、借漆同框、挥袖动图、走两步两联、夜场全景。
SVG（内容手绘、由本脚本落盘，保证一条命令全量重建）：glTF 指针链解剖、
前向罗盘、DCC 三栏映射。

键盘注入走 PostMessage 客户区消息（WM_KEYDOWN/WM_KEYUP）——本机 SendInput
经常拿不到前台焦点，投递窗口消息不依赖前台，实测可靠（工单 §5 的结论）。
"""

import ctypes
import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

sys.stdout.reconfigure(encoding="utf-8")
sys.stderr.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
CODE = ROOT / "code"
CRATE = CODE / "ch23-gltf"
EXAMPLES = CODE / "target" / "debug" / "examples"
OUT = ROOT / "book" / "src" / "images" / "ch23"

os.environ["BEVY_ASSET_ROOT"] = str(CRATE)

sys.path.insert(0, str(ROOT / "scripts"))
import capture  # noqa: E402
from capture import Example, find_main_window, grab_window  # noqa: E402

user32 = ctypes.windll.user32

FONT = ImageFont.truetype("C:/Windows/Fonts/msyh.ttc", 20)
LABEL_BG = (20, 22, 26)
LABEL_FG = (225, 225, 228)
GAP_COLOR = (58, 61, 68)
GAP = 4
LABEL_H = 36

# ------------------------------------------------- 键盘注入（PostMessage）

WM_KEYDOWN, WM_KEYUP = 0x0100, 0x0101
KEYS = {"SPACE": (0x20, 0x39)}  # 名字 -> (virtual-key, scancode)


def post_tap(ex: Example, name: str = "SPACE", hold: float = 0.06) -> None:
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


# ------------------------------------------------- 通用排版（沿 ch22 惯例）

def exe(name: str) -> Path:
    if name == "main":
        return CODE / "target" / "debug" / "ch23-gltf.exe"
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


HALF = 0.5  # 两联图里单帧缩到 640×360


def shrink(img: Image.Image) -> Image.Image:
    return img.resize((int(img.width * HALF), int(img.height * HALF)), Image.LANCZOS)


# ------------------------------------------------- 运行截图

def fig_01_first_unboxing() -> None:
    """Figure 23-1：阿福首秀——全身立于台板，平行光影子铺台面。"""
    with Example(exe("listing-23-01"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-23-01-first-unboxing.png")


def fig_03_two_blank_heads() -> None:
    """Figure 23-3：老鲁的两个素坯——左青瓷右乌釉，各拖一片影子。"""
    with Example(exe("listing-23-04"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-23-03-two-blank-heads.png")


def fig_04_double_exposure() -> None:
    """Figure 23-4：换场重影——两台 order 0 的相机各画一遍，叠成双曝光。"""
    with Example(exe("listing-23-05"), workdir=CODE) as ex:
        ex.wait_until(4.0)
        post_tap(ex)  # 换到 Workbench：MakerCam 进园即活，与我们的相机撞 order
        shot = logical(ex.shot(6.0))
    save_png(shot, "fig-23-04-double-exposure.png")


def fig_05_lamp_vs_plain() -> None:
    """Figure 23-5：load_lights 的一拨之差——左边亮摊位灯，右边平灰。"""
    with Example(exe("listing-23-06"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-23-05-lamp-vs-plain.png")


def _shoot_two_seats() -> tuple[Image.Image, Image.Image, bool]:
    """跑一遍 listing-23-07：t3.2 园子机位、SPACE@4、t5.0 作坊机位。

    stdout 落盘（文件重定向，不走 PIPE——ICU4X 刷 stderr 会塞死管道），
    结束后靠日志判定本次运行有没有踩到装载竞态（见 fig_06 注释）。
    """
    capture._set_dpi_aware()
    with tempfile.TemporaryFile() as log:
        proc = subprocess.Popen(
            [str(exe("listing-23-07"))],
            cwd=str(CODE),
            stdout=log,
            stderr=subprocess.STDOUT,
        )
        try:
            hwnd = find_main_window(proc.pid)
            t0 = time.perf_counter()

            def wait_until(t: float) -> None:
                remain = t0 + t - time.perf_counter()
                if remain > 0:
                    time.sleep(remain)

            wait_until(3.2)
            house = logical(grab_window(hwnd))
            wait_until(4.0)
            vk, scan = KEYS["SPACE"]
            user32.PostMessageW(hwnd, WM_KEYDOWN, vk, 1 | (scan << 16))
            time.sleep(0.06)
            user32.PostMessageW(hwnd, WM_KEYUP, vk, 1 | (scan << 16) | (1 << 30) | (1 << 31))
            wait_until(5.0)
            maker = logical(grab_window(hwnd))
            user32.PostMessageW(hwnd, 0x0010, 0, 0)  # WM_CLOSE
            proc.wait(timeout=8)
        except Exception:
            proc.kill()
            raise
        log.seek(0)
        tainted = b"Camera order ambiguities" in log.read()
    return house, maker, tainted


def fig_06_two_seats() -> None:
    """Figure 23-6：同一张工作台，两把椅子——园子全景 vs 作坊特写（两联）。

    注意：listing-23-07 对同一路径 afu.gltf 发了两次装载请求——Scene(1) 带
    load_cameras=false，Node(7) 走默认 settings。两个请求进异步装载器的先后
    不定；默认 settings 若抢先，load_cameras=false 被无视，MakerCam 以活相机
    进园：order 撞 0 双相机叠画，swap_seat 的 Single<&mut Transform,
    With<Camera3d>> 因出现第二台相机而失配罢工。正文描述的是干净行为，
    这里靠 stdout 里的 camera order 警告识别踩雷的运行并重试（竞态已回报
    主会话，listing 修复后重试自然归零）。
    """
    for attempt in range(1, 7):
        house, maker, tainted = _shoot_two_seats()
        if not tainted:
            break
        print(f"  listing-23-07 第 {attempt} 次运行踩到装载竞态（MakerCam 混进园子），重试……")
        time.sleep(0.5)
    else:
        raise RuntimeError("listing-23-07 连续 6 次踩到装载竞态——先修 listing 再来重建本图")
    save_png(
        hstack([shrink(house), shrink(maker)],
               ["园子机位：正面偏高的全景", "作坊机位 MakerCam：斜俯特写"]),
        "fig-23-06-two-seats.png",
    )


def fig_07_lantern_on_sleeve() -> None:
    """Figure 23-7：灯笼挂上左袖——袖口下方一颗发光小球，暖光晕开。"""
    with Example(exe("listing-23-09"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-23-07-lantern-on-sleeve.png")


def fig_08_repaint() -> None:
    """Figure 23-8：换漆两联——朱红 vs 月白，袍与双袖同变，头杆不动。"""
    with Example(exe("listing-23-10"), workdir=CODE) as ex:
        vermilion = logical(ex.shot(3.2))
        ex.wait_until(4.0)
        post_tap(ex)  # 上月白
        moon_white = logical(ex.shot(5.0))
    save_png(
        hstack([shrink(vermilion), shrink(moon_white)],
               ["出厂朱红", "按空格上月白——袍与双袖同变，头杆不动"]),
        "fig-23-08-repaint.png",
    )


def fig_09_borrowed_paint() -> None:
    """Figure 23-9：借漆上身——阿福与胶囊木人同框，红色完全一致。"""
    with Example(exe("listing-23-11"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.5))
    save_png(shot, "fig-23-09-borrowed-paint.png")


def fig_10_swing() -> None:
    """Figure 23-10：《Swing》动图——右袖起落、头随拍摆，2.4 s 一循环。

    恰好录满一个循环（24 帧 @10fps），首尾相接播起来无缝；
    起录点放在窗口出现 2.6 s 之后，渲染管线与动画都已就绪。
    """
    frames: list[Image.Image] = []
    with Example(exe("listing-23-12"), workdir=CODE) as ex:
        for i in range(24):
            ex.wait_until(2.6 + i / 10)
            frames.append(ex.grab().resize((640, 360), Image.LANCZOS))
    path = OUT / "fig-23-10-swing.webp"
    frames[0].save(path, save_all=True, append_images=frames[1:],
                   duration=100, loop=0, method=4, quality=70)
    size_kb = path.stat().st_size // 1024
    print(f"fig-23-10-swing.webp：{len(frames)} 帧 640x360，{size_kb} KB")
    assert size_kb <= 2048, "动图超过 2 MB 上限"


def fig_12_walk_test() -> None:
    """Figure 23-12：走两步两联——三步之后，甲倒退着走，乙正着走远。"""
    with Example(exe("listing-23-14"), workdir=CODE) as ex:
        start = logical(ex.shot(3.2))
        for t in (4.0, 4.6, 5.2):
            ex.wait_until(t)
            post_tap(ex)  # 老雷喊步：甲乙各沿自己的 forward() 迈 0.5 m
        walked = logical(ex.shot(7.0))
    save_png(
        hstack([shrink(start), shrink(walked)],
               ["开场：甲（左）正脸，乙（右）后脑勺", "三步之后：甲一路倒退，乙正常走远"]),
        "fig-23-12-walk-test.png",
    )


def fig_14_night_show() -> None:
    """Figure 23-14：《阿福亮相》夜场全景——挥袖相位，灯笼暖光，长影入深。"""
    with Example(exe("main"), workdir=CODE) as ex:
        shot = logical(ex.shot(4.0))
    save_png(shot, "fig-23-14-night-show.png")


# ------------------------------------------------- 手绘 SVG（内容即代码，落盘即重建）

SVG_02_GLTF_ANATOMY = """<svg viewBox="0 0 840 560" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr-a23" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#7a7468"/>
    </marker>
    <marker id="arr-a23c" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#c05a2e"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="840" height="560" rx="10" fill="#f7f5f0"/>

  <text x="420" y="34" text-anchor="middle" font-size="16" fill="#4a463f" font-weight="bold">glTF 的指针链：JSON 记账，二进制装货，贴图外挂</text>

  <!-- ============ 左：afu.gltf 大卡片，八层小格 ============ -->
  <rect x="28" y="58" width="292" height="470" rx="10" fill="#ffffff" stroke="#7a7468" stroke-width="1.6"/>
  <text x="48" y="84" font-size="14" fill="#4a463f" font-weight="bold">afu.gltf</text>
  <text x="112" y="84" font-size="11" fill="#7a7468">JSON 主档，10 KB</text>

  <!-- 八层小格：scenes → … → buffers -->
  <g>
    <rect x="48" y="96" width="228" height="34" rx="6" fill="#eef0f7" stroke="#274a91" stroke-width="1.4"/>
    <text x="60" y="118" font-size="12.5" fill="#274a91" font-weight="bold">scenes</text>
    <text x="264" y="118" font-size="10.5" fill="#7a7468" text-anchor="end">AfuShow / Workbench</text>

    <rect x="48" y="144" width="228" height="34" rx="6" fill="#eef0f7" stroke="#274a91" stroke-width="1.4"/>
    <text x="60" y="166" font-size="12.5" fill="#274a91" font-weight="bold">nodes</text>
    <text x="264" y="166" font-size="10.5" fill="#7a7468" text-anchor="end">AfuRoot、LeftArm…</text>

    <rect x="48" y="192" width="228" height="34" rx="6" fill="#eef0f7" stroke="#274a91" stroke-width="1.4"/>
    <text x="60" y="214" font-size="12.5" fill="#274a91" font-weight="bold">meshes</text>
    <text x="264" y="214" font-size="10.5" fill="#7a7468" text-anchor="end">primitive＝顶点＋材质号</text>

    <rect x="48" y="240" width="228" height="34" rx="6" fill="#eef0f7" stroke="#274a91" stroke-width="1.4"/>
    <text x="60" y="262" font-size="12.5" fill="#274a91" font-weight="bold">materials</text>
    <text x="264" y="262" font-size="10.5" fill="#7a7468" text-anchor="end">AfuFace、AfuRobe…</text>

    <rect x="48" y="288" width="228" height="34" rx="6" fill="#eef0f7" stroke="#274a91" stroke-width="1.4"/>
    <text x="60" y="310" font-size="12.5" fill="#274a91" font-weight="bold">animations</text>
    <text x="264" y="310" font-size="10.5" fill="#7a7468" text-anchor="end">Swing</text>

    <rect x="48" y="336" width="228" height="34" rx="6" fill="#eef0f7" stroke="#274a91" stroke-width="1.4"/>
    <text x="60" y="358" font-size="12.5" fill="#274a91" font-weight="bold">accessors</text>
    <text x="264" y="358" font-size="10.5" fill="#7a7468" text-anchor="end">怎么读一段二进制</text>

    <rect x="48" y="384" width="228" height="34" rx="6" fill="#eef0f7" stroke="#274a91" stroke-width="1.4"/>
    <text x="60" y="406" font-size="12.5" fill="#274a91" font-weight="bold">bufferViews</text>
    <text x="264" y="406" font-size="10.5" fill="#7a7468" text-anchor="end">划一段字节</text>

    <rect x="48" y="432" width="228" height="34" rx="6" fill="#eef0f7" stroke="#274a91" stroke-width="1.4"/>
    <text x="60" y="454" font-size="12.5" fill="#274a91" font-weight="bold">buffers</text>
    <text x="264" y="454" font-size="10.5" fill="#7a7468" text-anchor="end">货在箱外</text>
  </g>

  <!-- 层间箭头：顺着序号一层层向下指 -->
  <g stroke="#7a7468" stroke-width="1.5" fill="none">
    <path d="M96,130 L96,142" marker-end="url(#arr-a23)"/>
    <path d="M96,178 L96,190" marker-end="url(#arr-a23)"/>
    <path d="M96,226 L96,238" marker-end="url(#arr-a23)"/>
    <path d="M120,322 L120,334" marker-end="url(#arr-a23)"/>
    <path d="M96,370 L96,382" marker-end="url(#arr-a23)"/>
    <path d="M96,418 L96,430" marker-end="url(#arr-a23)"/>
    <!-- meshes 的顶点数据绕过材质/动画两层，直指 accessors -->
    <path d="M276,214 C 302,238 302,320 278,350" marker-end="url(#arr-a23)"/>
  </g>
  <g font-size="9.5" fill="#7a7468">
    <text x="104" y="140">nodes: [0]</text>
    <text x="104" y="188">mesh: 2</text>
    <text x="104" y="236">material: 1</text>
    <text x="128" y="332">input / output</text>
    <text x="104" y="380">bufferView: 0</text>
    <text x="104" y="428">buffer: 0</text>
    <text x="297" y="285" text-anchor="middle" transform="rotate(90 297 285)">POSITION / indices</text>
  </g>
  <text x="174" y="514" text-anchor="middle" font-size="10.5" fill="#7a7468">几乎所有引用都是序号，一路指到底</text>

  <!-- ============ 中：卡片外的两件货 ============ -->
  <!-- afu.bin：二进制条带 -->
  <rect x="372" y="130" width="104" height="176" rx="7" fill="#3a3d44"/>
  <g font-size="10" fill="#9aa0ac" font-family="Consolas, monospace">
    <text x="384" y="152">01101100</text>
    <text x="384" y="170">11010010</text>
    <text x="384" y="188">00101101</text>
    <text x="384" y="206">10011010</text>
    <text x="384" y="224">01010011</text>
    <text x="384" y="242">11001001</text>
    <text x="384" y="260">00110110</text>
    <text x="384" y="278">10100101</text>
    <text x="384" y="296">01011010</text>
  </g>
  <text x="424" y="326" text-anchor="middle" font-size="12" fill="#4a463f" font-weight="bold">afu.bin</text>
  <text x="424" y="342" text-anchor="middle" font-size="10.5" fill="#7a7468">二进制副档，36 KB</text>

  <!-- afu-face.png：贴图缩略（阿福的脸） -->
  <rect x="376" y="386" width="96" height="96" rx="7" fill="#ffffff" stroke="#7a7468" stroke-width="1.4"/>
  <g>
    <circle cx="424" cy="434" r="38" fill="#fdf3ea" stroke="#4a463f" stroke-width="1.2"/>
    <path d="M390,422 A38,38 0 0 1 458,422 L458,414 A38,38 0 0 0 390,414 z" fill="#2b2320"/>
    <path d="M392,420 A38,38 0 0 1 456,420 A46,40 0 0 0 392,420 z" fill="#2b2320"/>
    <circle cx="406" cy="446" r="7.5" fill="#e88c8c" opacity="0.75"/>
    <circle cx="442" cy="446" r="7.5" fill="#e88c8c" opacity="0.75"/>
    <path d="M400,430 q8,-7 16,0" stroke="#2b2320" stroke-width="2.4" fill="none" stroke-linecap="round"/>
    <path d="M432,430 q8,-7 16,0" stroke="#2b2320" stroke-width="2.4" fill="none" stroke-linecap="round"/>
    <circle cx="424" cy="424" r="3" fill="#b3402e"/>
    <path d="M416,456 q8,6 16,0" stroke="#b3402e" stroke-width="3" fill="none" stroke-linecap="round"/>
  </g>
  <text x="424" y="502" text-anchor="middle" font-size="12" fill="#4a463f" font-weight="bold">afu-face.png</text>
  <text x="424" y="518" text-anchor="middle" font-size="10.5" fill="#7a7468">贴图，1 KB</text>

  <!-- buffers 层伸出的两支 uri 箭头 -->
  <g stroke="#c05a2e" stroke-width="1.8" fill="none">
    <path d="M276,443 C 322,420 336,300 366,230" marker-end="url(#arr-a23c)"/>
    <path d="M276,458 C 320,470 336,440 370,434" marker-end="url(#arr-a23c)"/>
  </g>
  <g font-size="10" fill="#c05a2e">
    <text x="336" y="346">uri</text>
    <text x="330" y="478">uri</text>
  </g>

  <!-- ============ 右：afu.glb 单件箱 ============ -->
  <rect x="560" y="120" width="250" height="330" rx="10" fill="#ffffff" stroke="#274a91" stroke-width="2"/>
  <text x="580" y="148" font-size="14" fill="#4a463f" font-weight="bold">afu.glb</text>
  <text x="638" y="148" font-size="11" fill="#7a7468">单件箱</text>

  <rect x="580" y="166" width="210" height="76" rx="6" fill="#efe9dd" stroke="#7a7468" stroke-width="1.2"/>
  <text x="592" y="196" font-size="13" fill="#4a463f" font-weight="bold">JSON</text>
  <text x="592" y="216" font-size="10.5" fill="#7a7468">同一份账本</text>

  <rect x="580" y="252" width="210" height="76" rx="6" fill="#3a3d44"/>
  <text x="592" y="282" font-size="13" fill="#e6e2da" font-weight="bold">BIN</text>
  <text x="592" y="302" font-size="10.5" fill="#9aa0ac">同一批顶点与曲线</text>

  <rect x="580" y="338" width="210" height="76" rx="6" fill="#ffffff" stroke="#7a7468" stroke-width="1.2"/>
  <text x="592" y="368" font-size="13" fill="#4a463f" font-weight="bold">PNG</text>
  <text x="592" y="388" font-size="10.5" fill="#7a7468">同一张脸</text>
  <g>
    <circle cx="752" cy="376" r="22" fill="#fdf3ea" stroke="#4a463f" stroke-width="1"/>
    <path d="M733,369 A22,22 0 0 1 771,369 A27,23 0 0 0 733,369 z" fill="#2b2320"/>
    <circle cx="742" cy="382" r="4.2" fill="#e88c8c" opacity="0.75"/>
    <circle cx="762" cy="382" r="4.2" fill="#e88c8c" opacity="0.75"/>
    <path d="M738,373 q5,-4 9,0" stroke="#2b2320" stroke-width="1.6" fill="none" stroke-linecap="round"/>
    <path d="M757,373 q5,-4 9,0" stroke="#2b2320" stroke-width="1.6" fill="none" stroke-linecap="round"/>
    <path d="M746,388 q6,4 12,0" stroke="#b3402e" stroke-width="2" fill="none" stroke-linecap="round"/>
  </g>

  <text x="685" y="482" text-anchor="middle" font-size="12.5" fill="#4a463f">同一套内容，打成一件</text>
  <text x="685" y="502" text-anchor="middle" font-size="10.5" fill="#7a7468">标签照旧：#Scene0 还是 #Scene0</text>
</svg>
"""

SVG_11_FORWARD_COMPASS = """<svg viewBox="0 0 760 400" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr-c23" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#7a7468"/>
    </marker>
    <marker id="arr-c23o" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#c05a2e"/>
    </marker>
    <marker id="arr-c23b" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#274a91"/>
    </marker>
    <marker id="arr-c23d" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#4a463f"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="760" height="400" rx="10" fill="#f7f5f0"/>

  <text x="380" y="34" text-anchor="middle" font-size="16" fill="#4a463f" font-weight="bold">两家的「前」差半圈（俯视，+Y 出纸面）</text>

  <!-- ============ 左半：glTF ============ -->
  <text x="170" y="76" text-anchor="middle" font-size="15" fill="#c05a2e" font-weight="bold">glTF</text>
  <text x="170" y="94" text-anchor="middle" font-size="11" fill="#7a7468">规范：模型正脸朝 +Z</text>

  <!-- 轴 -->
  <g stroke="#7a7468" stroke-width="1.6" fill="none">
    <path d="M170,150 L170,296" marker-end="url(#arr-c23)"/>
    <path d="M110,220 L246,220" marker-end="url(#arr-c23)"/>
  </g>
  <text x="256" y="224" font-size="12" fill="#7a7468">+X</text>
  <text x="164" y="316" font-size="12" fill="#7a7468">+Z</text>

  <!-- 俯视小人立在原点：肩 + 头 + 鼻尖朝 +Z（画面下方） -->
  <g>
    <ellipse cx="170" cy="220" rx="33" ry="13" fill="#ffffff" stroke="#c05a2e" stroke-width="2"/>
    <circle cx="170" cy="220" r="12" fill="#fdf3ea" stroke="#c05a2e" stroke-width="2"/>
    <path d="M164,231 L176,231 L170,240 z" fill="#c05a2e"/>
  </g>
  <path d="M170,246 L170,282" stroke="#c05a2e" stroke-width="3" fill="none" marker-end="url(#arr-c23o)"/>
  <text x="182" y="268" font-size="12" fill="#c05a2e" font-weight="bold">脸朝 +Z</text>

  <!-- ============ 中：拧半圈 ============ -->
  <path d="M380,163 A 52,52 0 1 1 379,267" stroke="#4a463f" stroke-width="2.4" fill="none" marker-end="url(#arr-c23d)"/>
  <circle cx="380" cy="215" r="4" fill="none" stroke="#4a463f" stroke-width="1.6"/>
  <circle cx="380" cy="215" r="1.4" fill="#4a463f"/>
  <text x="380" y="238" text-anchor="middle" font-size="10" fill="#7a7468">Y 轴出纸面</text>
  <text x="380" y="308" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">rotate_scene_entity</text>
  <text x="380" y="326" text-anchor="middle" font-size="11" fill="#7a7468">装卸时把场景根绕 Y 拧半圈（180°）</text>

  <!-- ============ 右半：Bevy ============ -->
  <text x="590" y="76" text-anchor="middle" font-size="15" fill="#274a91" font-weight="bold">Bevy</text>
  <text x="590" y="94" text-anchor="middle" font-size="11" fill="#7a7468">Transform::forward() = −Z</text>

  <g stroke="#7a7468" stroke-width="1.6" fill="none">
    <path d="M590,150 L590,296" marker-end="url(#arr-c23)"/>
    <path d="M530,220 L666,220" marker-end="url(#arr-c23)"/>
  </g>
  <text x="676" y="224" font-size="12" fill="#7a7468">+X</text>
  <text x="584" y="316" font-size="12" fill="#7a7468">+Z</text>
  <text x="584" y="144" font-size="12" fill="#7a7468">−Z</text>

  <!-- 同一个小人立在原点，脸应当朝 −Z（画面上方） -->
  <g>
    <ellipse cx="590" cy="220" rx="33" ry="13" fill="#ffffff" stroke="#274a91" stroke-width="2"/>
    <circle cx="590" cy="220" r="12" fill="#fdf3ea" stroke="#274a91" stroke-width="2"/>
    <path d="M584,209 L596,209 L590,200 z" fill="#274a91"/>
  </g>
  <path d="M590,194 L590,162" stroke="#274a91" stroke-width="3" fill="none" marker-end="url(#arr-c23b)"/>
  <text x="602" y="182" font-size="12" fill="#274a91" font-weight="bold">脸朝 −Z</text>
</svg>
"""

SVG_13_DCC_PIPELINE = """<svg viewBox="0 0 840 540" xmlns="http://www.w3.org/2000/svg" font-family="-apple-system, 'Segoe UI', 'Microsoft YaHei', sans-serif">
  <defs>
    <marker id="arr-p23" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto">
      <path d="M0,0 L8,4.5 L0,9 z" fill="#7a7468"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="840" height="540" rx="10" fill="#f7f5f0"/>

  <text x="420" y="32" text-anchor="middle" font-size="16" fill="#4a463f" font-weight="bold">从作坊到园子：名字、材质、动画、附注，各走各的桥</text>

  <text x="150" y="62" text-anchor="middle" font-size="13.5" fill="#c05a2e" font-weight="bold">Blender（作坊）</text>
  <text x="420" y="62" text-anchor="middle" font-size="13.5" fill="#7a7468" font-weight="bold">glTF 字段（桥）</text>
  <text x="688" y="62" text-anchor="middle" font-size="13.5" fill="#274a91" font-weight="bold">Bevy（园子）</text>

  <!-- 行框：左 36..264，中 330..510，右 576..800；行高 70，框高 52 -->
  <g>
    <!-- 1 物体与层级 -->
    <rect x="36" y="80" width="228" height="52" rx="8" fill="#ffffff" stroke="#c05a2e" stroke-width="2"/>
    <text x="150" y="102" text-anchor="middle" font-size="13" fill="#c05a2e" font-weight="bold">物体与层级</text>
    <text x="150" y="121" text-anchor="middle" font-size="10.5" fill="#7a7468">大纲视图里的父子树</text>
    <rect x="330" y="80" width="180" height="52" rx="8" fill="#efe9dd" stroke="#7a7468" stroke-width="1.4"/>
    <text x="420" y="111" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">nodes</text>
    <rect x="576" y="80" width="228" height="52" rx="8" fill="#ffffff" stroke="#274a91" stroke-width="2"/>
    <text x="690" y="102" text-anchor="middle" font-size="13" fill="#274a91" font-weight="bold">实体树</text>
    <text x="690" y="121" text-anchor="middle" font-size="10.5" fill="#7a7468">ChildOf / Children</text>

    <!-- 2 物体名 -->
    <rect x="36" y="150" width="228" height="52" rx="8" fill="#ffffff" stroke="#c05a2e" stroke-width="2"/>
    <text x="150" y="172" text-anchor="middle" font-size="13" fill="#c05a2e" font-weight="bold">物体名</text>
    <text x="150" y="191" text-anchor="middle" font-size="10.5" fill="#7a7468">LeftArm、MainRod…</text>
    <rect x="330" y="150" width="180" height="52" rx="8" fill="#efe9dd" stroke="#7a7468" stroke-width="1.4"/>
    <text x="420" y="181" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">name</text>
    <rect x="576" y="150" width="228" height="52" rx="8" fill="#ffffff" stroke="#274a91" stroke-width="2"/>
    <text x="690" y="172" text-anchor="middle" font-size="13" fill="#274a91" font-weight="bold">Name 组件</text>
    <text x="690" y="191" text-anchor="middle" font-size="10.5" fill="#7a7468">按名找人、挂道具凭它</text>

    <!-- 3 材质名 -->
    <rect x="36" y="220" width="228" height="52" rx="8" fill="#ffffff" stroke="#c05a2e" stroke-width="2"/>
    <text x="150" y="242" text-anchor="middle" font-size="13" fill="#c05a2e" font-weight="bold">材质名</text>
    <text x="150" y="261" text-anchor="middle" font-size="10.5" fill="#7a7468">AfuRobe、RodWood…</text>
    <rect x="330" y="220" width="180" height="52" rx="8" fill="#efe9dd" stroke="#7a7468" stroke-width="1.4"/>
    <text x="420" y="251" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">materials</text>
    <rect x="576" y="220" width="228" height="52" rx="8" fill="#ffffff" stroke="#274a91" stroke-width="2"/>
    <text x="690" y="242" text-anchor="middle" font-size="12.5" fill="#274a91" font-weight="bold">GltfMaterialName</text>
    <text x="690" y="261" text-anchor="middle" font-size="10.5" fill="#7a7468">两本账：Material{N} 与 /std</text>

    <!-- 4 动画 -->
    <rect x="36" y="290" width="228" height="52" rx="8" fill="#ffffff" stroke="#c05a2e" stroke-width="2"/>
    <text x="150" y="312" text-anchor="middle" font-size="13" fill="#c05a2e" font-weight="bold">动画（Action）</text>
    <text x="150" y="331" text-anchor="middle" font-size="10.5" fill="#7a7468">关键帧烤成曲线</text>
    <rect x="330" y="290" width="180" height="52" rx="8" fill="#efe9dd" stroke="#7a7468" stroke-width="1.4"/>
    <text x="420" y="321" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">animations</text>
    <rect x="576" y="290" width="228" height="52" rx="8" fill="#ffffff" stroke="#274a91" stroke-width="2"/>
    <text x="690" y="312" text-anchor="middle" font-size="12.5" fill="#274a91" font-weight="bold">named_animations</text>
    <text x="690" y="331" text-anchor="middle" font-size="10.5" fill="#7a7468">AnimationClip，按名/按序取</text>

    <!-- 5 自定义属性 -->
    <rect x="36" y="360" width="228" height="52" rx="8" fill="#ffffff" stroke="#c05a2e" stroke-width="2"/>
    <text x="150" y="382" text-anchor="middle" font-size="13" fill="#c05a2e" font-weight="bold">自定义属性</text>
    <text x="150" y="401" text-anchor="middle" font-size="10.5" fill="#7a7468">Custom Properties</text>
    <rect x="330" y="360" width="180" height="52" rx="8" fill="#efe9dd" stroke="#7a7468" stroke-width="1.4"/>
    <text x="420" y="391" text-anchor="middle" font-size="12.5" fill="#4a463f" font-weight="bold">extras</text>
    <rect x="576" y="360" width="228" height="52" rx="8" fill="#ffffff" stroke="#274a91" stroke-width="2"/>
    <text x="690" y="382" text-anchor="middle" font-size="13" fill="#274a91" font-weight="bold">GltfExtras</text>
    <text x="690" y="401" text-anchor="middle" font-size="10.5" fill="#7a7468">JSON 原文，一字不动递到手</text>

    <!-- 6 相机与灯 -->
    <rect x="36" y="430" width="228" height="52" rx="8" fill="#ffffff" stroke="#c05a2e" stroke-width="2"/>
    <text x="150" y="452" text-anchor="middle" font-size="13" fill="#c05a2e" font-weight="bold">摆好的相机与灯</text>
    <text x="150" y="471" text-anchor="middle" font-size="10.5" fill="#7a7468">机位与照明随箱走</text>
    <rect x="330" y="430" width="180" height="52" rx="8" fill="#efe9dd" stroke="#7a7468" stroke-width="1.4"/>
    <text x="420" y="450" text-anchor="middle" font-size="12" fill="#4a463f" font-weight="bold">cameras</text>
    <text x="420" y="468" text-anchor="middle" font-size="10.5" fill="#4a463f">KHR_lights_punctual</text>
    <rect x="576" y="430" width="228" height="52" rx="8" fill="#ffffff" stroke="#274a91" stroke-width="2"/>
    <text x="690" y="452" text-anchor="middle" font-size="12.5" fill="#274a91" font-weight="bold">Camera3d / PointLight</text>
    <text x="690" y="471" text-anchor="middle" font-size="10.5" fill="#7a7468">默认照单全收（23.5 的开关）</text>
  </g>

  <!-- 桥上的箭头：两两相连 -->
  <g stroke="#7a7468" stroke-width="1.6" fill="none">
    <path d="M268,106 L326,106" marker-end="url(#arr-p23)"/>
    <path d="M514,106 L572,106" marker-end="url(#arr-p23)"/>
    <path d="M268,176 L326,176" marker-end="url(#arr-p23)"/>
    <path d="M514,176 L572,176" marker-end="url(#arr-p23)"/>
    <path d="M268,246 L326,246" marker-end="url(#arr-p23)"/>
    <path d="M514,246 L572,246" marker-end="url(#arr-p23)"/>
    <path d="M268,316 L326,316" marker-end="url(#arr-p23)"/>
    <path d="M514,316 L572,316" marker-end="url(#arr-p23)"/>
    <path d="M268,386 L326,386" marker-end="url(#arr-p23)"/>
    <path d="M514,386 L572,386" marker-end="url(#arr-p23)"/>
    <path d="M268,456 L326,456" marker-end="url(#arr-p23)"/>
    <path d="M514,456 L572,456" marker-end="url(#arr-p23)"/>
  </g>

  <text x="420" y="516" text-anchor="middle" font-size="10.5" fill="#7a7468">导出 .glb → 丢进 assets/ → 按本章的手艺开箱；名字是两边事实上的 API</text>
</svg>
"""


def fig_02_gltf_anatomy_svg() -> None:
    """Figure 23-2：glTF 指针链解剖 + .glb 对照（手绘 SVG）。"""
    save_svg(SVG_02_GLTF_ANATOMY, "fig-23-02-gltf-anatomy.svg")


def fig_11_forward_compass_svg() -> None:
    """Figure 23-11：glTF +Z / Bevy −Z 前向罗盘（手绘 SVG）。"""
    save_svg(SVG_11_FORWARD_COMPASS, "fig-23-11-forward-compass.svg")


def fig_13_dcc_pipeline_svg() -> None:
    """Figure 23-13：Blender → glTF → Bevy 三栏映射（手绘 SVG）。"""
    save_svg(SVG_13_DCC_PIPELINE, "fig-23-13-dcc-pipeline.svg")


# ------------------------------------------------- 主流程

ALL = [
    fig_01_first_unboxing,
    fig_02_gltf_anatomy_svg,
    fig_03_two_blank_heads,
    fig_04_double_exposure,
    fig_05_lamp_vs_plain,
    fig_06_two_seats,
    fig_07_lantern_on_sleeve,
    fig_08_repaint,
    fig_09_borrowed_paint,
    fig_10_swing,
    fig_11_forward_compass_svg,
    fig_12_walk_test,
    fig_13_dcc_pipeline_svg,
    fig_14_night_show,
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    print("构建本章二进制……")
    subprocess.run(
        ["cargo", "build", "-p", "ch23-gltf", "--bins", "--examples"],
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

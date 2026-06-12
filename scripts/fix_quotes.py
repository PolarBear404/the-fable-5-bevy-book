# 把 book/src 下 .md 正文里用作中文引号的 ASCII 直引号替换为 “ ”（U+201C/U+201D）。
# 不碰：围栏代码块、行内代码（反引号 span）、HTML 标签内部（属性）、{{#include}} 行。
# 引号开闭按"行内交替"判定（前置调查已确认全书 0 个奇数引号行，无跨行成对）。
import sys
from pathlib import Path

SRC = Path(__file__).resolve().parent.parent / "book" / "src"

LQ = "“"  # “
RQ = "”"  # ”


def convert_line(line: str):
    """对一行正文做替换。返回 (新行, 替换数, 是否引号数为奇数)。"""
    out = []
    i = 0
    n = len(line)
    expect_open = True
    replaced = 0
    while i < n:
        ch = line[i]
        if ch == "`":
            # 行内代码 span：N 个反引号开启，需恰好 N 个反引号闭合
            j = i
            while j < n and line[j] == "`":
                j += 1
            run = j - i
            k = j
            close_at = -1
            while k < n:
                if line[k] == "`":
                    m = k
                    while m < n and line[m] == "`":
                        m += 1
                    if m - k == run:
                        close_at = k
                        break
                    k = m
                else:
                    k += 1
            if close_at != -1:
                out.append(line[i : close_at + run])
                i = close_at + run
            else:
                out.append(line[i:j])
                i = j
            continue
        if ch == "<" and i + 1 < n and (line[i + 1].isalpha() or line[i + 1] == "/"):
            # HTML 标签：跳到最近的 '>'，属性引号原样保留
            gt = line.find(">", i)
            if gt != -1:
                out.append(line[i : gt + 1])
                i = gt + 1
                continue
        if ch == '"':
            out.append(LQ if expect_open else RQ)
            expect_open = not expect_open
            replaced += 1
            i += 1
            continue
        out.append(ch)
        i += 1
    return "".join(out), replaced, (not expect_open)


def is_fence_open(line: str):
    s = line.lstrip()
    for c in ("`", "~"):
        if s.startswith(c * 3):
            j = 0
            while j < len(s) and s[j] == c:
                j += 1
            info = s[j:]
            if c == "`" and "`" in info:
                continue  # backtick 围栏的 info string 不能含反引号
            return c, j
    return None


total = 0
odd_lines = []
for f in sorted(SRC.glob("*.md")):
    text = f.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)
    in_fence = False
    fence_char, fence_len = None, 0
    new_lines = []
    file_replaced = 0
    for lineno, line in enumerate(lines, 1):
        if in_fence:
            s = line.strip()
            if s and set(s) == {fence_char} and len(s) >= fence_len:
                in_fence = False
            new_lines.append(line)
            continue
        fence = is_fence_open(line)
        if fence:
            in_fence = True
            fence_char, fence_len = fence
            new_lines.append(line)
            continue
        if "{{#include" in line:
            new_lines.append(line)
            continue
        new_line, replaced, odd = convert_line(line)
        if odd:
            odd_lines.append(f"{f.name}:{lineno}")
        new_lines.append(new_line)
        file_replaced += replaced
    if file_replaced:
        f.write_text("".join(new_lines), encoding="utf-8")
        print(f"{f.name}: {file_replaced}")
        total += file_replaced

print(f"\n共替换 {total} 个引号")
if odd_lines:
    print("!! 以下行引号数为奇数，需人工核对：")
    for x in odd_lines:
        print(f"  {x}")
    sys.exit(1)

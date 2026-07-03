# 把 book/src 下 .md 正文里用作中文引号的 ASCII 直引号替换为 “ ”（U+201C/U+201D）。
# 不碰：围栏代码块、行内代码（反引号 span）、HTML 标签内部（属性引号，含属性跨行的标签，
# 如 <img src="…"␤ alt="…">）、{{#include}} 行。
# 引号开闭按"行内交替"判定（前置调查已确认全书 0 个奇数引号行，无跨行成对）。
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")

SRC = Path(__file__).resolve().parent.parent / "book" / "src"

LQ = "“"  # “
RQ = "”"  # ”


def convert_line(line: str, in_tag: bool = False):
    """对一行正文做替换。in_tag：行首是否处于上一行未闭合的 HTML 标签内。
    返回 (新行, 替换数, 是否引号数为奇数, 行尾是否仍在标签内)。"""
    out = []
    i = 0
    n = len(line)
    expect_open = True
    replaced = 0
    if in_tag:
        gt = line.find(">")
        if gt == -1:
            return line, 0, False, True
        out.append(line[: gt + 1])
        i = gt + 1
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
            # 行内没有 '>'：标签的属性跨行，剩余部分原样保留，未闭合状态带给下一行
            out.append(line[i:])
            return "".join(out), replaced, (not expect_open), True
        if ch == '"':
            out.append(LQ if expect_open else RQ)
            expect_open = not expect_open
            replaced += 1
            i += 1
            continue
        out.append(ch)
        i += 1
    return "".join(out), replaced, (not expect_open), False


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


def process_text(text: str):
    """处理一个文件的全文。返回 (新文本, 替换数, 奇数引号行号列表)。"""
    lines = text.splitlines(keepends=True)
    in_fence = False
    in_tag = False
    fence_char, fence_len = None, 0
    new_lines = []
    replaced_total = 0
    odd_linenos = []
    for lineno, line in enumerate(lines, 1):
        if not in_tag:
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
        new_line, replaced, odd, in_tag = convert_line(line, in_tag)
        if odd:
            odd_linenos.append(lineno)
        new_lines.append(new_line)
        replaced_total += replaced
    return "".join(new_lines), replaced_total, odd_linenos


def main():
    total = 0
    odd_lines = []
    for f in sorted(SRC.glob("*.md")):
        text = f.read_text(encoding="utf-8")
        new_text, file_replaced, odd_linenos = process_text(text)
        odd_lines += [f"{f.name}:{n}" for n in odd_linenos]
        if file_replaced:
            f.write_text(new_text, encoding="utf-8")
            print(f"{f.name}: {file_replaced}")
            total += file_replaced

    print(f"\n共替换 {total} 个引号")
    if odd_lines:
        print("!! 以下行引号数为奇数，需人工核对：")
        for x in odd_lines:
            print(f"  {x}")
        sys.exit(1)


if __name__ == "__main__":
    main()

# -*- coding: utf-8 -*-
"""一键重建第 23 章的 glTF 资产：杖头木偶「阿福」（纯 Python 手写 glTF 2.0，零外部工具，确定性输出）。

    py -3.11 scripts/make_ch23_assets.py

产物（输出到 code/ch23-gltf/assets/models/）：
  afu/afu.gltf       JSON 主档（pretty 打印，正文可直接摘录）
  afu/afu.bin        几何 + 动画采样数据
  afu/afu-face.png   脸贴图（PIL 手绘，256×256）
  afu.glb            同一只木偶的单文件装箱（JSON+BIN+PNG 三合一）

箱内清单（正文与 listing 的断言都以此为准）：
  场景  Scene0 "AfuShow"（只有阿福——干净的主角场）
        Scene1 "Workbench"（作坊的工作台：备用头 + 备用袖 + 作坊自己的摊位灯 BoothLamp
        与参考机位 MakerCam——load_lights/load_cameras 实验的伏笔）；默认场景 Scene0
  节点  AfuRoot > Body > {Head, LeftArm, RightArm}；AfuRoot > MainRod；
        BoothLamp（KHR_lights_punctual 点光，500 cd 暖光）；MakerCam（透视相机，看向阿福头部）
  网格  HeadMesh（球，贴脸）/ RobeMesh（锥台袍）/ SleeveMesh（锥台袖，左右臂与备用袖共用）/ RodMesh（主杆）
  材质  0 AfuFace（带贴图）、1 AfuRobe（朱红）、2 RodWood（木色）
  动画  "Swing"（2.4 s 循环：RightArm 挥袖 + Head 摆头）
  extras AfuRoot {"workshop":"Qiaoshouzhai"}、LeftArm {"slot":"lantern"}
  朝向  按 glTF 约定脸朝 +Z（23.8 convert_coordinates 实验以此为前提）
"""

import json
import math
import struct
import sys
from pathlib import Path

from PIL import Image, ImageDraw

sys.stdout.reconfigure(encoding="utf-8")

OUT = Path(__file__).resolve().parent.parent / "code" / "ch23-gltf" / "assets" / "models"

F32 = 5126   # glTF componentType: float
U16 = 5123   # glTF componentType: unsigned short
ARRAY_BUFFER = 34962          # bufferView.target: 顶点数据
ELEMENT_ARRAY_BUFFER = 34963  # bufferView.target: 索引数据


# ---------------------------------------------------------------- 数学小件

def quat_axis_angle(ax, ay, az, deg):
    """轴角转四元数 [x, y, z, w]（glTF 顺序）。"""
    n = math.sqrt(ax * ax + ay * ay + az * az)
    ax, ay, az = ax / n, ay / n, az / n
    half = math.radians(deg) / 2.0
    s = math.sin(half)
    return [ax * s, ay * s, az * s, math.cos(half)]


def quat_z(deg):
    return quat_axis_angle(0.0, 0.0, 1.0, deg)


def quat_y(deg):
    return quat_axis_angle(0.0, 1.0, 0.0, deg)


def look_at_quat(eye, target, up=(0.0, 1.0, 0.0)):
    """算一个「镜头对准 target」的节点旋转（相机沿自身 −Z 取景，glTF 与 Bevy 同约定）。"""
    fx, fy, fz = (target[0] - eye[0], target[1] - eye[1], target[2] - eye[2])
    fl = math.sqrt(fx * fx + fy * fy + fz * fz)
    fx, fy, fz = fx / fl, fy / fl, fz / fl
    # 相机的 +Z 指向视线反方向
    zx, zy, zz = -fx, -fy, -fz
    # x = up × z
    xx, xy, xz = (up[1] * zz - up[2] * zy, up[2] * zx - up[0] * zz, up[0] * zy - up[1] * zx)
    xl = math.sqrt(xx * xx + xy * xy + xz * xz)
    xx, xy, xz = xx / xl, xy / xl, xz / xl
    # y = z × x
    yx, yy, yz = (zy * xz - zz * xy, zz * xx - zx * xz, zx * xy - zy * xx)
    # 旋转矩阵（列向量 x/y/z）转四元数
    t = xx + yy + zz
    if t > 0:
        s = math.sqrt(t + 1.0) * 2
        return [(yz - zy) / s, (zx - xz) / s, (xy - yx) / s, 0.25 * s]
    if xx > yy and xx > zz:
        s = math.sqrt(1.0 + xx - yy - zz) * 2
        return [0.25 * s, (yx + xy) / s, (zx + xz) / s, (yz - zy) / s]
    if yy > zz:
        s = math.sqrt(1.0 + yy - xx - zz) * 2
        return [(yx + xy) / s, 0.25 * s, (zy + yz) / s, (zx - xz) / s]
    s = math.sqrt(1.0 + zz - xx - yy) * 2
    return [(zx + xz) / s, (zy + yz) / s, 0.25 * s, (xy - yx) / s]


# ---------------------------------------------------------------- 几何生成
# 约定：+Y 向上，脸朝 +Z（glTF 的「前」）；三角形逆时针为正面。

def uv_sphere(radius, rings=18, sectors=28):
    """经纬球。u=0.5 对准 +Z——脸贴图画在贴图正中即可落在脸上。"""
    pos, nrm, uv, idx = [], [], [], []
    for r in range(rings + 1):
        theta = math.pi * r / rings            # 0 顶 → π 底
        v = r / rings
        for s in range(sectors + 1):
            u = s / sectors
            phi = 2.0 * math.pi * (u - 0.5)    # u=0.5 → phi=0 → +Z
            x = radius * math.sin(theta) * math.sin(phi)
            y = radius * math.cos(theta)
            z = radius * math.sin(theta) * math.cos(phi)
            pos.append((x, y, z))
            nrm.append((x / radius, y / radius, z / radius))
            uv.append((u, v))
    stride = sectors + 1
    for r in range(rings):
        for s in range(sectors):
            a = r * stride + s
            b = a + stride
            idx += [a, b, a + 1, a + 1, b, b + 1]
    return pos, nrm, uv, idx


def frustum(r_bottom, r_top, height, y_min, sectors=28):
    """圆锥台（侧面 + 上下盖），底面在 y_min、顶面在 y_min+height。r 相等即圆柱。"""
    pos, nrm, uv, idx = [], [], [], []
    slope = r_bottom - r_top
    nl = math.sqrt(height * height + slope * slope)
    ny, nxz = slope / nl, height / nl          # 侧面法线的竖直/水平分量
    # 侧面（两圈顶点）
    for level, (radius, y, v) in enumerate(
        [(r_bottom, y_min, 1.0), (r_top, y_min + height, 0.0)]
    ):
        for s in range(sectors + 1):
            u = s / sectors
            phi = 2.0 * math.pi * (u - 0.5)
            c, sn = math.cos(phi), math.sin(phi)
            pos.append((radius * sn, y, radius * c))
            nrm.append((nxz * sn, ny, nxz * c))
            uv.append((u, v))
    stride = sectors + 1
    for s in range(sectors):
        a, b = s, s + stride                   # a 底圈，b 顶圈
        idx += [a, a + 1, b, a + 1, b + 1, b]
    # 上下盖（各自独立顶点，法线 ±Y）
    for radius, y, normal_y in [(r_top, y_min + height, 1.0), (r_bottom, y_min, -1.0)]:
        center = len(pos)
        pos.append((0.0, y, 0.0))
        nrm.append((0.0, normal_y, 0.0))
        uv.append((0.5, 0.5))
        for s in range(sectors + 1):
            phi = 2.0 * math.pi * s / sectors
            c, sn = math.cos(phi), math.sin(phi)
            pos.append((radius * sn, y, radius * c))
            nrm.append((0.0, normal_y, 0.0))
            uv.append((0.5 + 0.5 * sn, 0.5 - 0.5 * c))
        for s in range(sectors):
            a, b = center + 1 + s, center + 2 + s
            if normal_y > 0:
                idx += [center, a, b]
            else:
                idx += [center, b, a]
    return pos, nrm, uv, idx


# ---------------------------------------------------------------- 脸贴图

def paint_face(path):
    """阿福的脸：肤色底、黑发盖、弯眉笑眼、红颊红唇，外加眉心一点朱砂。"""
    size = 256
    img = Image.new("RGB", (size, size), (242, 213, 184))     # 肤色
    d = ImageDraw.Draw(img)
    # 黑发：贴图顶部一条（球的极区）+ 刘海弧
    d.rectangle([0, 0, size, 40], fill=(24, 18, 14))
    d.ellipse([-30, 8, size + 30, 78], fill=(24, 18, 14))
    # 眉心朱砂痣（阿福的招牌）
    d.ellipse([122, 96, 134, 108], fill=(178, 34, 34))
    # 眉毛：两道细弯眉
    d.arc([92, 106, 120, 124], start=200, end=340, fill=(30, 22, 18), width=3)
    d.arc([136, 106, 164, 124], start=200, end=340, fill=(30, 22, 18), width=3)
    # 眼睛：笑眼（下弯弧）
    d.arc([94, 122, 118, 142], start=20, end=160, fill=(20, 16, 12), width=5)
    d.arc([138, 122, 162, 142], start=20, end=160, fill=(20, 16, 12), width=5)
    # 红颊：两团腮红
    for cx in (88, 168):
        d.ellipse([cx - 14, 138, cx + 14, 162], fill=(233, 150, 138))
    # 嘴：一枚樱桃小口 + 笑弧
    d.ellipse([118, 156, 138, 170], fill=(190, 52, 46))
    d.arc([110, 148, 146, 176], start=20, end=160, fill=(130, 30, 26), width=3)
    img.save(path)
    return path


# ---------------------------------------------------------------- glTF 组装

class BinBuilder:
    """把顶点/索引/动画数据码进一个 buffer，顺手记下 bufferView 与 accessor。"""

    def __init__(self):
        self.blob = bytearray()
        self.views = []
        self.accessors = []

    def _pad4(self):
        while len(self.blob) % 4:
            self.blob += b"\x00"

    def _view(self, data: bytes, target=None):
        self._pad4()
        view = {"buffer": 0, "byteOffset": len(self.blob), "byteLength": len(data)}
        if target is not None:
            view["target"] = target
        self.blob += data
        self.views.append(view)
        return len(self.views) - 1

    def vec_accessor(self, vecs, kind, target=ARRAY_BUFFER, with_min_max=False):
        """vecs: [(...), ...]；kind: "VEC2"/"VEC3"/"VEC4"/"SCALAR"（float）。"""
        flat = [c for v in vecs for c in v] if kind != "SCALAR" else list(vecs)
        data = struct.pack(f"<{len(flat)}f", *flat)
        acc = {
            "bufferView": self._view(data, target),
            "componentType": F32,
            "count": len(vecs),
            "type": kind,
        }
        if with_min_max:
            if kind == "SCALAR":
                acc["min"] = [min(vecs)]
                acc["max"] = [max(vecs)]
            else:
                acc["min"] = [min(v[i] for v in vecs) for i in range(len(vecs[0]))]
                acc["max"] = [max(v[i] for v in vecs) for i in range(len(vecs[0]))]
        self.accessors.append(acc)
        return len(self.accessors) - 1

    def index_accessor(self, indices):
        data = struct.pack(f"<{len(indices)}H", *indices)
        acc = {
            "bufferView": self._view(data, ELEMENT_ARRAY_BUFFER),
            "componentType": U16,
            "count": len(indices),
            "type": "SCALAR",
        }
        self.accessors.append(acc)
        return len(self.accessors) - 1


def build_document():
    """组装整份 glTF：返回 (json_dict, bin_bytes)。"""
    b = BinBuilder()

    # ---- 四件网格
    def add_mesh(name, geom, material):
        pos, nrm, uv, idx = geom
        return {
            "name": name,
            "primitives": [{
                "attributes": {
                    "POSITION": b.vec_accessor(pos, "VEC3", with_min_max=True),
                    "NORMAL": b.vec_accessor(nrm, "VEC3"),
                    "TEXCOORD_0": b.vec_accessor(uv, "VEC2"),
                },
                "indices": b.index_accessor(idx),
                "material": material,
            }],
        }

    meshes = [
        add_mesh("HeadMesh", uv_sphere(0.16), 0),
        add_mesh("RobeMesh", frustum(0.24, 0.10, 0.50, -0.25), 1),
        add_mesh("SleeveMesh", frustum(0.065, 0.045, 0.34, -0.34), 1),
        add_mesh("RodMesh", frustum(0.03, 0.03, 0.60, -0.30), 2),
    ]

    # ---- 动画 "Swing"：右袖挥、头轻摆，2.4 s 循环
    times = [0.0, 0.6, 1.2, 1.8, 2.4]
    t_in = b.vec_accessor(times, "SCALAR", target=None, with_min_max=True)
    arm_out = b.vec_accessor(
        [quat_z(d) for d in (-15.0, -95.0, -65.0, -95.0, -15.0)], "VEC4", target=None
    )
    head_out = b.vec_accessor(
        [quat_y(d) for d in (0.0, -14.0, 0.0, 14.0, 0.0)], "VEC4", target=None
    )
    animation = {
        "name": "Swing",
        "samplers": [
            {"input": t_in, "interpolation": "LINEAR", "output": arm_out},
            {"input": t_in, "interpolation": "LINEAR", "output": head_out},
        ],
        "channels": [
            {"sampler": 0, "target": {"node": 4, "path": "rotation"}},   # RightArm
            {"sampler": 1, "target": {"node": 2, "path": "rotation"}},   # Head
        ],
    }

    doc = {
        "asset": {
            "version": "2.0",
            "generator": "Qiaoshouzhai Puppet Works (scripts/make_ch23_assets.py)",
        },
        "scene": 0,
        "scenes": [
            {"name": "AfuShow", "nodes": [0]},
            {"name": "Workbench", "nodes": [8, 9, 6, 7]},
        ],
        "nodes": [
            {   # 0
                "name": "AfuRoot",
                "children": [1, 5],
                "extras": {"workshop": "Qiaoshouzhai"},
            },
            {   # 1
                "name": "Body",
                "translation": [0.0, 0.62, 0.0],
                "mesh": 1,
                "children": [2, 3, 4],
            },
            {   # 2
                "name": "Head",
                "translation": [0.0, 0.44, 0.0],
                "mesh": 0,
            },
            {   # 3　按 glTF 约定脸朝 +Z，角色自己的左手边是 +X
                "name": "LeftArm",
                "translation": [0.19, 0.18, 0.0],
                "rotation": quat_z(15.0),
                "mesh": 2,
                "extras": {"slot": "lantern"},
            },
            {   # 4
                "name": "RightArm",
                "translation": [-0.19, 0.18, 0.0],
                "rotation": quat_z(-15.0),
                "mesh": 2,
            },
            {   # 5
                "name": "MainRod",
                "translation": [0.0, 0.32, 0.0],
                "mesh": 3,
            },
            {   # 6
                "name": "BoothLamp",
                "translation": [0.6, 1.2, 0.8],
                "extensions": {"KHR_lights_punctual": {"light": 0}},
            },
            {   # 7
                "name": "MakerCam",
                "translation": [1.0, 1.0, 2.0],
                "rotation": look_at_quat((1.0, 1.0, 2.0), (0.0, 0.12, 0.0)),
                "camera": 0,
            },
            {   # 8
                "name": "SpareHead",
                "translation": [-0.3, 0.16, 0.0],
                "rotation": quat_y(35.0),
                "mesh": 0,
            },
            {   # 9
                "name": "SpareArm",
                "translation": [0.25, 0.065, 0.0],
                "rotation": quat_z(90.0),
                "mesh": 2,
            },
        ],
        "meshes": meshes,
        "materials": [
            {
                "name": "AfuFace",
                "pbrMetallicRoughness": {
                    "baseColorTexture": {"index": 0},
                    "metallicFactor": 0.0,
                    "roughnessFactor": 0.7,
                },
            },
            {
                "name": "AfuRobe",
                "pbrMetallicRoughness": {
                    "baseColorFactor": [0.42, 0.032, 0.02, 1.0],
                    "metallicFactor": 0.0,
                    "roughnessFactor": 0.85,
                },
            },
            {
                "name": "RodWood",
                "pbrMetallicRoughness": {
                    "baseColorFactor": [0.16, 0.08, 0.03, 1.0],
                    "metallicFactor": 0.0,
                    "roughnessFactor": 0.9,
                },
            },
        ],
        "textures": [{"source": 0, "sampler": 0}],
        "samplers": [{"magFilter": 9729, "minFilter": 9729, "wrapS": 33071, "wrapT": 33071}],
        "images": [{"uri": "afu-face.png"}],
        "cameras": [{
            "name": "MakerCam",
            "type": "perspective",
            "perspective": {"yfov": 0.7, "znear": 0.1},
        }],
        "animations": [animation],
        "extensionsUsed": ["KHR_lights_punctual"],
        "extensions": {
            "KHR_lights_punctual": {
                "lights": [{
                    "name": "BoothGlow",
                    "type": "point",
                    "color": [1.0, 0.82, 0.6],
                    "intensity": 3000.0,
                }],
            },
        },
        "buffers": [{"uri": "afu.bin", "byteLength": 0}],   # byteLength 收尾时补
        "bufferViews": b.views,
        "accessors": b.accessors,
    }
    b._pad4()
    doc["buffers"][0]["byteLength"] = len(b.blob)
    return doc, bytes(b.blob)


# ---------------------------------------------------------------- 两种装箱

def write_gltf_trio(doc, blob, out_dir, png_path):
    """三件套：afu.gltf + afu.bin + afu-face.png（png 已就位）。"""
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / "afu.bin").write_bytes(blob)
    text = json.dumps(doc, indent=2, ensure_ascii=False)
    (out_dir / "afu.gltf").write_text(text, encoding="utf-8")
    return out_dir / "afu.gltf"


def write_glb(doc, blob, png_bytes, path):
    """单件 .glb：同一份 JSON，buffer 去掉 uri、贴图改从 BIN 块里取。"""
    doc = json.loads(json.dumps(doc))          # 深拷贝，别动三件套那份
    # 贴图追加进二进制块
    blob = bytearray(blob)
    while len(blob) % 4:
        blob += b"\x00"
    doc["bufferViews"].append(
        {"buffer": 0, "byteOffset": len(blob), "byteLength": len(png_bytes)}
    )
    blob += png_bytes
    doc["images"][0] = {
        "bufferView": len(doc["bufferViews"]) - 1,
        "mimeType": "image/png",
    }
    del doc["buffers"][0]["uri"]
    doc["buffers"][0]["byteLength"] = len(blob)

    json_bytes = json.dumps(doc, separators=(",", ":"), ensure_ascii=True).encode("ascii")
    while len(json_bytes) % 4:
        json_bytes += b" "                     # JSON 块按规范用空格补齐
    while len(blob) % 4:
        blob += b"\x00"
    total = 12 + 8 + len(json_bytes) + 8 + len(blob)
    with open(path, "wb") as f:
        f.write(struct.pack("<III", 0x46546C67, 2, total))            # magic "glTF"
        f.write(struct.pack("<II", len(json_bytes), 0x4E4F534A))      # "JSON"
        f.write(json_bytes)
        f.write(struct.pack("<II", len(blob), 0x004E4942))            # "BIN\0"
        f.write(blob)
    return path


def main():
    trio_dir = OUT / "afu"
    trio_dir.mkdir(parents=True, exist_ok=True)

    png_path = paint_face(trio_dir / "afu-face.png")
    doc, blob = build_document()
    gltf_path = write_gltf_trio(doc, blob, trio_dir, png_path)
    glb_path = write_glb(doc, blob, png_path.read_bytes(), OUT / "afu.glb")

    for p in (gltf_path, trio_dir / "afu.bin", png_path, glb_path):
        print(f"  {p.relative_to(OUT.parent.parent)}  {p.stat().st_size:,} bytes")
    print("阿福出箱，完工。")


if __name__ == "__main__":
    main()

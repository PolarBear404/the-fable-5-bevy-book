#!/usr/bin/env python3
"""ch23 资产生成器：用纯标准库手写一个带命名节点 + 一段节点动画的 glTF 木偶。

杖头木偶「阿福」的骨架：

    Puppet(根) → Torso → { Head, ArmLeft, ArmRight, LegLeft, LegRight }

四肢由「节点变换动画」驱动——一段 1 秒循环的「活络手脚」，靠的是给每个节点的
rotation 通道打关键帧，不含蒙皮骨骼（skin/joints）。真·蒙皮留到第 30 章。

这样做的用意：
  * 全过程零第三方依赖，只用 Python 标准库（struct/base64/json），与全书「一条命令
    重建资产」的铁律一致；
  * 输出是 .gltf 文本（JSON + 内嵌 base64 buffer），自给自足、可 diff，正文里能直接
    把它的片段贴出来给读者看 glTF 到底长什么样；
  * 命名节点（Head/ArmLeft…）+ 命名材质（Robe/Face）+ 一段命名动画（Swing），正好
    覆盖第 23 章要讲的 SceneRoot、按 Name 取实体、最小动画播放。

运行：

    py -3.11 scripts/make_ch23_assets.py

输出：code/ch23-gltf/assets/models/puppet.gltf
"""

import base64
import json
import math
import pathlib
import struct

ROOT = pathlib.Path(__file__).resolve().parent.parent
OUT = ROOT / "code" / "ch23-gltf" / "assets" / "models" / "puppet.gltf"

# glTF 常量
FLOAT = 5126
USHORT = 5123
ARRAY_BUFFER = 34962
ELEMENT_ARRAY_BUFFER = 34963

# 边写边攒：二进制缓冲区、bufferViews、accessors
blob = bytearray()
buffer_views = []
accessors = []
meshes = []


def _align4():
    """glTF 要求每个 bufferView 起点 4 字节对齐——不足就补零。"""
    while len(blob) % 4:
        blob.append(0)


def _add_view(data, target=None):
    _align4()
    offset = len(blob)
    blob.extend(data)
    view = {"buffer": 0, "byteOffset": offset, "byteLength": len(data)}
    if target is not None:
        view["target"] = target
    buffer_views.append(view)
    return len(buffer_views) - 1


def add_floats(values, type_str, count, target=None, mn=None, mx=None):
    data = struct.pack("<%df" % len(values), *values)
    view = _add_view(data, target)
    acc = {"bufferView": view, "componentType": FLOAT, "count": count, "type": type_str}
    if mn is not None:
        acc["min"] = mn
        acc["max"] = mx
    accessors.append(acc)
    return len(accessors) - 1


def add_indices(idx):
    data = struct.pack("<%dH" % len(idx), *idx)
    view = _add_view(data, ELEMENT_ARRAY_BUFFER)
    accessors.append(
        {"bufferView": view, "componentType": USHORT, "count": len(idx), "type": "SCALAR"}
    )
    return len(accessors) - 1


def box(center, half):
    """24 顶点平面法线立方体 + 36 索引（CCW 朝外）。

    center 是几何中心：四肢传一个偏下的 center，让顶点挂在节点原点「之下」，
    这样旋转该节点就是绕「肩 / 胯」摆，而不是绕自己中段转。
    """
    cx, cy, cz = center
    hx, hy, hz = half
    x0, x1 = cx - hx, cx + hx
    y0, y1 = cy - hy, cy + hy
    z0, z1 = cz - hz, cz + hz
    faces = [
        ((0, 0, 1), [(x0, y0, z1), (x1, y0, z1), (x1, y1, z1), (x0, y1, z1)]),
        ((0, 0, -1), [(x1, y0, z0), (x0, y0, z0), (x0, y1, z0), (x1, y1, z0)]),
        ((1, 0, 0), [(x1, y0, z1), (x1, y0, z0), (x1, y1, z0), (x1, y1, z1)]),
        ((-1, 0, 0), [(x0, y0, z0), (x0, y0, z1), (x0, y1, z1), (x0, y1, z0)]),
        ((0, 1, 0), [(x0, y1, z1), (x1, y1, z1), (x1, y1, z0), (x0, y1, z0)]),
        ((0, -1, 0), [(x0, y0, z0), (x1, y0, z0), (x1, y0, z1), (x0, y0, z1)]),
    ]
    pos, nor, idx = [], [], []
    for normal, verts in faces:
        base = len(pos) // 3
        for vx, vy, vz in verts:
            pos += [vx, vy, vz]
            nor += [normal[0], normal[1], normal[2]]
        idx += [base, base + 1, base + 2, base, base + 2, base + 3]
    return pos, nor, idx


def add_box_mesh(name, center, half, material):
    pos, nor, idx = box(center, half)
    xs, ys, zs = pos[0::3], pos[1::3], pos[2::3]
    a_pos = add_floats(
        pos, "VEC3", len(pos) // 3, ARRAY_BUFFER,
        [min(xs), min(ys), min(zs)], [max(xs), max(ys), max(zs)],
    )
    a_nor = add_floats(nor, "VEC3", len(nor) // 3, ARRAY_BUFFER)
    a_idx = add_indices(idx)
    meshes.append(
        {
            "name": name,
            "primitives": [
                {
                    "attributes": {"POSITION": a_pos, "NORMAL": a_nor},
                    "indices": a_idx,
                    "material": material,
                }
            ],
        }
    )
    return len(meshes) - 1


def quat_x(theta):
    """绕 X 轴转 theta 弧度的四元数 [x, y, z, w]。"""
    return [math.sin(theta / 2), 0.0, 0.0, math.cos(theta / 2)]


# ---- 材质：红袍 + 米白的脸 ----
materials = [
    {
        "name": "Robe",
        "pbrMetallicRoughness": {
            "baseColorFactor": [0.62, 0.12, 0.12, 1.0],
            "metallicFactor": 0.0,
            "roughnessFactor": 0.75,
        },
    },
    {
        "name": "Face",
        "pbrMetallicRoughness": {
            "baseColorFactor": [0.93, 0.86, 0.74, 1.0],
            "metallicFactor": 0.0,
            "roughnessFactor": 0.6,
        },
    },
]
ROBE, FACE = 0, 1

# ---- 网格：躯干 / 头是居中盒；四肢是「挂在原点之下」的盒 ----
torso_mesh = add_box_mesh("TorsoMesh", (0.0, 0.0, 0.0), (0.45, 0.70, 0.28), ROBE)
head_mesh = add_box_mesh("HeadMesh", (0.0, 0.0, 0.0), (0.35, 0.35, 0.33), FACE)
arm_mesh = add_box_mesh("ArmMesh", (0.0, -0.60, 0.0), (0.13, 0.60, 0.13), ROBE)
leg_mesh = add_box_mesh("LegMesh", (0.0, -0.85, 0.0), (0.16, 0.85, 0.16), ROBE)

# ---- 节点树：脚落在 y=0，头顶约 y=3.8 ----
nodes = [
    {"name": "Puppet", "children": [1]},
    {"name": "Torso", "mesh": torso_mesh, "translation": [0.0, 2.4, 0.0], "children": [2, 3, 4, 5, 6]},
    {"name": "Head", "mesh": head_mesh, "translation": [0.0, 1.05, 0.0]},
    {"name": "ArmLeft", "mesh": arm_mesh, "translation": [0.62, 0.50, 0.0]},
    {"name": "ArmRight", "mesh": arm_mesh, "translation": [-0.62, 0.50, 0.0]},
    {"name": "LegLeft", "mesh": leg_mesh, "translation": [0.24, -0.70, 0.0]},
    {"name": "LegRight", "mesh": leg_mesh, "translation": [-0.24, -0.70, 0.0]},
]

# ---- 动画 Swing：四肢绕 X 轴前后摆，1 秒一循环（首尾同姿，无缝衔接）----
times = [0.0, 0.5, 1.0]
t_in = add_floats(times, "SCALAR", len(times), None, [min(times)], [max(times)])

# (节点, 三个关键帧的摆角/弧度)；左右与手脚都反相，像在原地踏步
swing_specs = [
    (3, [0.5, -0.5, 0.5]),    # ArmLeft
    (4, [-0.5, 0.5, -0.5]),   # ArmRight
    (5, [-0.4, 0.4, -0.4]),   # LegLeft
    (6, [0.4, -0.4, 0.4]),    # LegRight
]
samplers, channels = [], []
for node_index, angles in swing_specs:
    out = []
    for a in angles:
        out += quat_x(a)
    a_out = add_floats(out, "VEC4", len(angles), None)
    samplers.append({"input": t_in, "output": a_out, "interpolation": "LINEAR"})
    channels.append({"sampler": len(samplers) - 1, "target": {"node": node_index, "path": "rotation"}})

animations = [{"name": "Swing", "samplers": samplers, "channels": channels}]

# ---- 组装并写出 ----
gltf = {
    "asset": {"version": "2.0", "generator": "the-bevy-book ch23 make_ch23_assets.py"},
    "scene": 0,
    "scenes": [{"name": "PuppetStage", "nodes": [0]}],
    "nodes": nodes,
    "meshes": meshes,
    "materials": materials,
    "animations": animations,
    "accessors": accessors,
    "bufferViews": buffer_views,
    "buffers": [
        {
            "byteLength": len(blob),
            "uri": "data:application/gltf-buffer;base64," + base64.b64encode(bytes(blob)).decode("ascii"),
        }
    ],
}

OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(gltf, indent=2, ensure_ascii=False), encoding="utf-8")
print(
    f"wrote {OUT.relative_to(ROOT)} — buffer {len(blob)} B, "
    f"{len(nodes)} nodes, {len(meshes)} meshes, {len(materials)} materials, "
    f"{len(channels)} anim channels"
)

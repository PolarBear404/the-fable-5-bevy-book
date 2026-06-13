# Picking 管线：从指针到实体事件

`bevy_picking` 先把输入抽象成 **Pointer**（指针）。鼠标是一个指针，触摸屏上的每根手指也可以是一个指针，未来你也可以写自己的虚拟指针。指针只知道位置、按键、滚轮这些输入事实；它还不知道场景里有什么。

第二步是 **backend**（拾取后端）。后端负责回答「这个指针下面有哪些实体」。mesh 后端把相机和光标位置变成射线，跟 3D mesh 相交；sprite 后端在 2D 相机里检查 sprite 的矩形或不透明像素；UI 后端按 UI 节点的渲染顺序找命中项。后端只提交命中结果，不负责决定最终谁收到事件。

第三步是 `HoverMap`。如果同一个指针同时命中一个 UI 面板、一张 sprite、一个 3D 箱子，Bevy 要按相机顺序、深度、`Pickable` 规则决定「真正 hover 的是谁」。默认规则很像你看到的画面：上面的东西挡住下面的东西。

最后才是 `Pointer<E>` 事件。常用事件分三组：

- `Pointer<Over>` / `Pointer<Out>` / `Pointer<Move>`：移入、移出、移动；
- `Pointer<Press>` / `Pointer<Release>` / `Pointer<Click>`：按下、松开、点击；
- `Pointer<DragStart>` / `Pointer<Drag>` / `Pointer<DragEnd>` / `Pointer<DragDrop>`：拖拽与拖放。

这些事件是第 8 章讲过的 **EntityEvent**：目标实体先收到，之后会沿 `ChildOf` 关系向父实体冒泡。你可以用 `MessageReader<Pointer<Click>>` 全局读，也可以更常见地把处理逻辑直接挂到实体上：

```rust
{{#include ../../code/ch25-picking-camera-control/examples/listing-25-01.rs:mesh_observers}}
```

<span class="caption">Listing 25-1（节选一）：把 `Pointer` 事件处理器直接挂在可点选实体上</span>

注意这段代码里没有「全局判断鼠标点在哪个箱子上」。我们只给箱子挂观察者：指针移入时变色，按下时变色，拖拽时旋转。哪个实体命中，Bevy 就触发哪个实体身上的观察者。

## 默认插件和后端

`DefaultPlugins` 在默认 feature 下已经包含 picking 的基础管线，也包含 sprite/UI 后端；但 **mesh 后端不是默认插件的一部分**，写 3D mesh 拾取时要自己加 `MeshPickingPlugin`：

```rust
{{#include ../../code/ch25-picking-camera-control/examples/listing-25-01.rs:plugins}}
```

<span class="caption">Listing 25-1（节选二）：3D mesh 拾取要额外添加 `MeshPickingPlugin`</span>

这句只是在 App 里装上 mesh 后端。装好以后，mesh 默认可被拾取；如果某个实体不该参与拾取，给它加 `Pickable::IGNORE`。地面、背景板、纯装饰物通常都该 ignore，否则它们会吃掉光标，挡住真正要点的角色或道具。


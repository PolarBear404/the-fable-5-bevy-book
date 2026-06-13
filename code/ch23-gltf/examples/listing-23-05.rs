//! Listing 23-5：让角儿动起来——加载并循环播放 glTF 里的一段动画

use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

fn main() {
    App::new()
        // —— 下面这段 WindowPlugin、以及 Update 里的 toggle_graph_on_click，都是网页
        //    demo 多带的料，落在正文 ANCHOR 截取区之外——ch23-05 印出来的代码不含它们。
        //    canvas/fit_canvas_to_parent 仅 Web 生效（把画面渲进页面里 id="bevy-ch23-anim"
        //    的 <canvas> 并随容器缩放）；桌面平台无效，`cargo run --example` 照旧开窗口。
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-ch23-anim".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.52, 0.55, 0.62)))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_graph_on_click)
        .run();
}

// ANCHOR: to_play
/// 记下「要播哪段动画」：动画图的句柄 + 这段动画在图里的节点编号。
#[derive(Component)]
struct AnimationToPlay {
    graph: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}
// ANCHOR_END: to_play

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: build_graph
    // 把第 0 段动画装进一张「动画图」。哪怕只有一段，也得先进图——
    // 第 30 章会用图来混合、过渡多段动画；这里只取它最简单的一面。
    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/puppet.gltf")),
    );
    let graph = graphs.add(graph);

    // 把「要播什么」记在实体上，再挂观察者；SceneRoot 负责把模型展开成实体
    commands
        .spawn((
            AnimationToPlay { graph, index },
            SceneRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("models/puppet.gltf"),
            )),
        ))
        .observe(play_when_ready);
    // ANCHOR_END: build_graph

    stage(&mut commands, &mut meshes, &mut materials);
}

// ANCHOR: play_when_ready
/// 场景展开后，glTF 加载器已经替我们在某个子孙实体上挂好了 AnimationPlayer。
/// 找到它，告诉它播哪段、循环播，并把动画图接上去——少了接图这一步，动作不会动。
fn play_when_ready(
    ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let Ok(anim) = to_play.get(ready.entity) else {
        return;
    };
    for child in children.iter_descendants(ready.entity) {
        if let Ok(mut player) = players.get_mut(child) {
            player.play(anim.index).repeat();
            commands
                .entity(child)
                .insert(AnimationGraphHandle(anim.graph.clone()));
        }
    }
}
// ANCHOR_END: play_when_ready

// 台子：地面、主光、机位、一点环境光
fn stage(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    commands.insert_resource(GlobalAmbientLight {
        brightness: 200.0,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.30, 0.33),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.5, -0.9, -0.4), Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::new(0.0, 1.9, 0.0), Vec3::Y),
    ));
}

// —— 网页 demo 专用，正文 Listing 不取——亲手撞一回本节那个「哑巴坑」。
// 点一下画面，就地把动画图从播放器上拔掉 / 接回去：
// 为什么「拔了动画图」就等于「动作停住」？推进动画时间的 advance_animations、和把动画
// 灌进 Transform 的 animate_targets，这两个系统的 Query 都**必需** &AnimationGraphHandle。
// 一旦从这个实体上移除它，两边都不再匹配——时间不走、姿势不更，阿福连帧带姿一起冻住，
// 还不 panic、不告警。再 insert 回去，又从冻住那一帧原样续上。这正是漏接图时的现象。
fn toggle_graph_on_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    players: Query<(Entity, Has<AnimationGraphHandle>), With<AnimationPlayer>>,
    to_play: Query<&AnimationToPlay>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    // 木偶只有一个 AnimationPlayer；场景还没就位时查不到，直接跳过这一帧
    let Ok((player, connected)) = players.single() else {
        return;
    };
    let Ok(anim) = to_play.single() else {
        return;
    };
    if connected {
        commands.entity(player).remove::<AnimationGraphHandle>();
    } else {
        commands
            .entity(player)
            .insert(AnimationGraphHandle(anim.graph.clone()));
    }
}

//! Listing 23-5：让角儿动起来——加载并循环播放 glTF 里的一段动画

use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.52, 0.55, 0.62)))
        .add_systems(Startup, setup)
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

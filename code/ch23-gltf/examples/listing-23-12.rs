//! Listing 23-12：放动画——谱（AnimationGraph）架上、锣鼓（AnimationPlayer）起

use bevy::{prelude::*, world_serialization::WorldInstanceReady};

const AFU: &str = "models/afu/afu.gltf";

// ANCHOR: to_play
/// 司鼓的戏单：graph 是整座谱架，node 是今晚要敲的那一折
#[derive(Component)]
struct AnimationToPlay {
    graph: Handle<AnimationGraph>,
    node: AnimationNodeIndex,
}
// ANCHOR_END: to_play

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.5, 1.4, 2.6).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 4.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(6.0, 6.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.30, 0.26),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // ANCHOR: build_graph
    // 第 0 折动画（就是 "Swing"）拉出来，搭一座单谱的谱架
    let clip: Handle<AnimationClip> =
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(AFU));
    let (graph, node) = AnimationGraph::from_clip(clip);
    let graph = graphs.add(graph);

    // 戏单跟阿福钉在同一个实体上，回执一到就开锣
    commands
        .spawn((
            WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(AFU))),
            AnimationToPlay { graph, node },
        ))
        .observe(strike_up);
    // ANCHOR_END: build_graph
}

// ANCHOR: strike
/// 司鼓：场子搭好了，找到树里的播放器，把谱递过去、开锣
fn strike_up(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let Ok(to_play) = to_play.get(ready.entity) else {
        return;
    };
    for child in children.iter_descendants(ready.entity) {
        if let Ok(mut player) = players.get_mut(child) {
            // 开锣：点这折、循环打
            player.play(to_play.node).repeat();
            // 递谱：播放器手里必须有 AnimationGraphHandle，缺了整台哑火
            commands
                .entity(child)
                .insert(AnimationGraphHandle(to_play.graph.clone()));
            println!("司鼓：谱架上好，起——《Swing》，循环。");
        }
    }
}
// ANCHOR_END: strike

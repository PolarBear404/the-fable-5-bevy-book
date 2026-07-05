//! Listing 23-13：哑巴坑——点一下左键抽走 AnimationGraphHandle，阿福僵住，日志无声

use bevy::{prelude::*, world_serialization::WorldInstanceReady};

const AFU: &str = "models/afu/afu.gltf";

/// 司鼓的戏单（与 Listing 23-12 相同）
#[derive(Component)]
struct AnimationToPlay {
    graph: Handle<AnimationGraph>,
    node: AnimationNodeIndex,
}

fn main() {
    App::new()
        // canvas/fit_canvas_to_parent 只在网页构建里生效（见 20.7），桌面照常
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-ch23-anim".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_graph)
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

    let clip: Handle<AnimationClip> =
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(AFU));
    let (graph, node) = AnimationGraph::from_clip(clip);
    let graph = graphs.add(graph);
    commands
        .spawn((
            WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(AFU))),
            AnimationToPlay { graph, node },
        ))
        .observe(strike_up);
    println!("老雷：开演后点一下左键抽谱、再点一下还谱——盯住阿福的袖子。");
}

/// 司鼓开锣（与 Listing 23-12 相同）
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
            player.play(to_play.node).repeat();
            commands
                .entity(child)
                .insert(AnimationGraphHandle(to_play.graph.clone()));
            println!("司鼓：谱架上好，起——《Swing》，循环。");
        }
    }
}

// ANCHOR: toggle
/// 老雷：左键一点，谱子抽走/还回——播放器还在，就是没了谱
fn toggle_graph(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    to_play: Single<&AnimationToPlay>,
    player: Single<(Entity, Has<AnimationGraphHandle>), With<AnimationPlayer>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let (entity, has_graph) = *player;
    if has_graph {
        commands.entity(entity).remove::<AnimationGraphHandle>();
        println!("老雷：谱子抽了——阿福僵在半空。听听，日志一声不吭。");
    } else {
        commands
            .entity(entity)
            .insert(AnimationGraphHandle(to_play.graph.clone()));
        println!("老雷：谱子还回去——从僵住那一拍原样接着来，连时间都没走。");
    }
}
// ANCHOR_END: toggle

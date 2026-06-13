//! Listing 23-6：角儿登场——加载场景、按名字给手里塞道具、放起动画，一台戏齐活

use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

const PUPPET: &str = "models/puppet.gltf";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.16, 0.13, 0.15)))
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: components
/// 记下要播的动画：动画图句柄 + 这段动画在图里的节点编号。
#[derive(Component)]
struct AnimationToPlay {
    graph: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}
// ANCHOR_END: components

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 第 0 段动画装进动画图，记在实体上；SceneRoot 负责加载并把模型展开成实体
    let (graph, index) =
        AnimationGraph::from_clip(asset_server.load(GltfAssetLabel::Animation(0).from_asset(PUPPET)));
    commands
        .spawn((
            AnimationToPlay { graph: graphs.add(graph), index },
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(PUPPET))),
        ))
        .observe(on_ready);

    stage(&mut commands, &mut meshes, &mut materials);
}
// ANCHOR_END: setup

// ANCHOR: on_ready
/// 场景一展开，就把这一趟的事一次办齐：沿子孙走一遍——
/// 撞见 AnimationPlayer 就放动画，撞见名叫 "ArmRight" 的节点就往它手里塞面小旗。
/// 小旗做了这条胳膊的子实体，于是它会跟着挥动的手一起摆。
fn on_ready(
    ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    names: Query<&Name>,
    to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(anim) = to_play.get(ready.entity) else {
        return;
    };
    let flag = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.82, 0.25),
        emissive: LinearRgba::rgb(0.6, 0.5, 0.1),
        ..default()
    });
    for entity in children.iter_descendants(ready.entity) {
        if let Ok(mut player) = players.get_mut(entity) {
            player.play(anim.index).repeat();
            commands
                .entity(entity)
                .insert(AnimationGraphHandle(anim.graph.clone()));
        }
        if names.get(entity).map_or(false, |n| n.as_str() == "ArmRight") {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.55, 0.4, 0.04))),
                MeshMaterial3d(flag.clone()),
                Transform::from_xyz(0.0, -1.05, 0.25),
                ChildOf(entity),
            ));
        }
    }
}
// ANCHOR_END: on_ready

// 一座暖色的小戏台：地面、主光、机位、一点环境光——都是第 21、22 章的老相识
fn stage(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    commands.insert_resource(GlobalAmbientLight {
        brightness: 180.0,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.13, 0.11),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 9000.0,
            color: Color::srgb(1.0, 0.93, 0.82),
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.4, -0.85, -0.45), Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.1, 7.5).looking_at(Vec3::new(0.0, 1.9, 0.0), Vec3::Y),
    ));
}

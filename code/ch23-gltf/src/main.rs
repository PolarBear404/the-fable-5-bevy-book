//! Listing 23-6：角儿登场——加载场景、按名字给手里塞道具、放起动画，一台戏齐活

use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

const PUPPET: &str = "models/puppet.gltf";

fn main() {
    App::new()
        // ANCHOR: web_window
        // 仅 Web 生效：把渲染塞进页面里 id="bevy-ch23" 的 <canvas>，并随容器缩放。
        // 这几个字段在桌面平台无效——同一份代码，桌面开窗口、网页进画布，两头通吃。
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-ch23".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        // ANCHOR_END: web_window
        .insert_resource(ClearColor(Color::srgb(0.16, 0.13, 0.15)))
        .add_systems(Startup, setup)
        .add_systems(Update, orbit_camera)
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

// ANCHOR: orbit_camera
/// 看货摇臂：按住鼠标左键拖动，相机绕木偶（戏台中心）公转。半径不变，
/// 只改方位角 yaw 与俯仰角 pitch——3D 模型就该转着看，这是网页比静态截图多给的东西。
/// 用光标的逐帧位移驱动（`cursor_position` 读的是画布内绝对坐标，桌面与网页都可靠，
/// 不碰浏览器要 pointer-lock 才给的原始位移）——同一套逻辑两头通跑。
const ORBIT_CENTER: Vec3 = Vec3::new(0.0, 1.9, 0.0);
const ORBIT_SPEED: f32 = 0.008;

fn orbit_camera(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    mut last_cursor: Local<Option<Vec2>>,
) {
    // 没按住左键、或光标移出画布：忘掉上一帧位置，下次重新起算（避免猛地跳一下）
    let Some(cursor) = window.cursor_position() else {
        *last_cursor = None;
        return;
    };
    if !mouse.pressed(MouseButton::Left) {
        *last_cursor = None;
        return;
    }
    // 按住后的第一帧只记录位置、先不转；之后每帧用「这帧光标 − 上帧光标」当位移
    let delta = match last_cursor.replace(cursor) {
        Some(prev) => cursor - prev,
        None => Vec2::ZERO,
    };
    if delta == Vec2::ZERO {
        return;
    }
    // 把相机当前位置换算成绕中心的球坐标，按光标位移拨动，再换算回位置、盯回中心
    let offset = camera.translation - ORBIT_CENTER;
    let radius = offset.length();
    let yaw = offset.x.atan2(offset.z) - delta.x * ORBIT_SPEED;
    let pitch = ((offset.y / radius).asin() + delta.y * ORBIT_SPEED).clamp(-1.4, 1.4);
    camera.translation = ORBIT_CENTER
        + Vec3::new(
            radius * pitch.cos() * yaw.sin(),
            radius * pitch.sin(),
            radius * pitch.cos() * yaw.cos(),
        );
    camera.look_at(ORBIT_CENTER, Vec3::Y);
}
// ANCHOR_END: orbit_camera

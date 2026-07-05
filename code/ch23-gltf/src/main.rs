//! ch23 全本《阿福亮相》（Listing 23-15）：
//! 单件 .glb 开箱、按名挂灯笼、《Swing》循环、左键拖动转台、空格歇锣/起锣

use bevy::{prelude::*, world_serialization::WorldInstanceReady};

const AFU_GLB: &str = "models/afu.glb";

/// 司鼓的戏单（23.8 的老朋友）
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
                canvas: Some("#bevy-ch23".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.016, 0.026, 0.048)))
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit_camera, gong))
        .run();
}

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.5, 1.47, 2.6).looking_at(Vec3::new(0.0, 0.62, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 3_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 4.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 台板与两根台柱
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(9.0, 9.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.26, 0.23),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    let pillar_wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.36, 0.10, 0.08),
        perceptual_roughness: 0.7,
        ..default()
    });
    for x in [-2.2_f32, 2.2] {
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.16, 3.2))),
            MeshMaterial3d(pillar_wood.clone()),
            Transform::from_xyz(x, 1.6, -1.5),
        ));
    }

    // 这回抬的是单件装箱的 afu.glb——箱内一切照旧，标签也照旧
    let clip: Handle<AnimationClip> =
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(AFU_GLB));
    let (graph, node) = AnimationGraph::from_clip(clip);
    let graph = graphs.add(graph);
    commands
        .spawn((
            WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(AFU_GLB))),
            AnimationToPlay { graph, node },
        ))
        .observe(backstage);

    println!("老雷：《阿福亮相》，开演——左键拖着转台，空格歇锣/起锣。");
}
// ANCHOR_END: setup

// ANCHOR: backstage
/// 后台一把抓：回执一到，跟包挂灯笼、司鼓递谱开锣
fn backstage(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    to_play: Query<&AnimationToPlay>,
    names: Query<&Name>,
    mut players: Query<&mut AnimationPlayer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(to_play) = to_play.get(ready.entity) else {
        return;
    };
    for entity in children.iter_descendants(ready.entity) {
        // 跟包：左袖口挂灯笼——袖子一挥，灯笼跟着走
        if names.get(entity).is_ok_and(|n| n.as_str() == "LeftArm") {
            commands.entity(entity).with_child((
                Mesh3d(meshes.add(Sphere::new(0.055))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.9, 0.25, 0.15),
                    emissive: LinearRgba::new(4.0, 0.9, 0.4, 1.0),
                    ..default()
                })),
                PointLight {
                    color: Color::srgb(1.0, 0.6, 0.35),
                    intensity: 3_000.0,
                    range: 3.0,
                    ..default()
                },
                Transform::from_xyz(0.0, -0.50, 0.0),
            ));
            println!("跟包：灯笼挂上左袖了。");
        }
        // 司鼓：找到播放器，递谱开锣
        if let Ok(mut player) = players.get_mut(entity) {
            player.play(to_play.node).repeat();
            commands
                .entity(entity)
                .insert(AnimationGraphHandle(to_play.graph.clone()));
            println!("司鼓：起——《Swing》，循环。");
        }
    }
}
// ANCHOR_END: backstage

// ANCHOR: orbit
/// 左键拖动转台：用 cursor_position 自己算位移（17.4 的手法，桌面网页通吃）
fn orbit_camera(
    window: Single<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    mut last_cursor: Local<Option<Vec2>>,
    mut yaw: Local<f32>,
) {
    if mouse.pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            if let Some(prev) = *last_cursor {
                *yaw -= (pos.x - prev.x) * 0.008;
            }
            *last_cursor = Some(pos);
        }
    } else {
        *last_cursor = None;
    }
    // 机位吊在半径 3 米的圆轨上，永远看着阿福
    let angle = 0.52 + *yaw;
    let center = Vec3::new(0.0, 0.62, 0.0);
    let seat = center + Vec3::new(3.0 * angle.sin(), 0.85, 3.0 * angle.cos());
    **camera = Transform::from_translation(seat).looking_at(center, Vec3::Y);
}
// ANCHOR_END: orbit

// ANCHOR: gong
/// 空格：歇锣/起锣——AnimationPlayer 的暂停开关
fn gong(keyboard: Res<ButtonInput<KeyCode>>, mut players: Query<&mut AnimationPlayer>) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    for mut player in &mut players {
        if player.all_paused() {
            player.resume_all();
            println!("司鼓：起锣——接着演。");
        } else {
            player.pause_all();
            println!("司鼓：歇锣——满台定格。");
        }
    }
}
// ANCHOR_END: gong

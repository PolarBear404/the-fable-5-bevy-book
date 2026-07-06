//! Listing 26-13：《定妆照》——夜戏开演前，盛师傅的画质开关面板：
//! 1~5 换磨边方案，B 辉光、F 景深、M 运动模糊、V 老镜头套餐、T 换冲印配方

use bevy::anti_alias::fxaa::Fxaa;
use bevy::anti_alias::smaa::Smaa;
use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::camera::Exposure;
use bevy::camera::Hdr;
use bevy::core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::post_process::dof::{DepthOfField, DepthOfFieldMode};
use bevy::post_process::effect_stack::{ChromaticAberration, LensDistortion, Vignette};
use bevy::post_process::motion_blur::MotionBlur;
use bevy::prelude::*;
use bevy::render::camera::{MipBias, TemporalJitter};

// ANCHOR: board
/// 面板状态：输入只改这块黑板，真正拆装组件的活儿归 apply_board
#[derive(Resource)]
struct Board {
    aa: usize,     // AA_GEARS 的下标
    bloom: bool,   // 辉光
    dof: bool,     // 景深
    motion: bool,  // 运动模糊
    vintage: bool, // 老镜头套餐：暗角 + 畸变 + 色差
    recipe: usize, // RECIPES 的下标
}

const AA_GEARS: [&str; 5] = ["素颜", "MSAA 4x", "FXAA", "SMAA", "TAA"];
const RECIPES: [(&str, Tonemapping); 4] = [
    ("TonyMcMapface", Tonemapping::TonyMcMapface),
    ("AcesFitted", Tonemapping::AcesFitted),
    ("AgX", Tonemapping::AgX),
    ("None", Tonemapping::None),
];
// ANCHOR_END: board

/// 走马灯的转轴标记
#[derive(Component)]
struct Carousel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "定妆照".into(),
                // 网页构建专用的两个字段（桌面下是空操作）：挂 <canvas>、随外框伸缩
                canvas: Some("#bevy-canvas".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GlobalAmbientLight::NONE)
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.035)))
        .insert_resource(Board {
            aa: 1, // 出厂即 MSAA 4x，跟引擎默认对齐
            bloom: true,
            dof: false,
            motion: true,
            vintage: false,
            recipe: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spin,
                press_keys,
                // 黑板没动就不碰相机——第 5 章的资源变更检测在管事
                (apply_board, repaint_sign).run_if(resource_changed::<Board>),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // ANCHOR: camera
    // 相机只带“底盘”：Hdr 底片常驻，效果组件全由 apply_board 按黑板拆装
    commands.spawn((
        Camera3d::default(),
        Exposure::INDOOR, // 夜戏进光口径——22.2 的室内档
        Hdr,
        Transform::from_xyz(0.0, 3.0, 9.5).looking_at(Vec3::new(0.0, 1.6, 0.0), Vec3::Y),
    ));
    // ANCHOR_END: camera

    // 台面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(26.0, 16.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.31, 0.34),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // 三盏灯笼高挂：辉光的主角，也是散景的光斑源
    let lanterns = [
        (
            -4.0,
            LinearRgba::new(9.0, 1.6, 0.8, 1.0),
            Color::srgb(1.0, 0.45, 0.3),
        ),
        (
            0.0,
            LinearRgba::new(9.0, 6.5, 2.0, 1.0),
            Color::srgb(1.0, 0.85, 0.45),
        ),
        (
            4.0,
            LinearRgba::new(0.9, 5.5, 6.5, 1.0),
            Color::srgb(0.45, 0.9, 1.0),
        ),
    ];
    for (x, emissive, light_color) in lanterns {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.38))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.1),
                emissive,
                ..default()
            })),
            Transform::from_xyz(x, 3.4, -1.0),
            children![(
                PointLight {
                    color: light_color,
                    intensity: 24_000.0,
                    range: 20.0,
                    ..default()
                },
                Transform::IDENTITY,
            )],
        ));
    }
    // 机位后一盏暖场补光：把栏杆、瓷柱和台面从夜色里托出来
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.93, 0.82),
            intensity: 220_000.0,
            range: 36.0,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 11.0),
    ));

    // 前中后三件货：琉璃盏（近）、堂鼓（中）、锦旗（远）——景深的三个纵深
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.42))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.75, 0.72),
            perceptual_roughness: 0.15,
            ..default()
        })),
        Transform::from_xyz(-1.6, 0.9, 4.2),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.75, 0.9))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.17, 0.13),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(0.6, 0.45, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.6, 2.4, 0.08))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.10, 0.35, 0.75),
            perceptual_roughness: 0.55,
            ..default()
        })),
        Transform::from_xyz(-0.8, 1.2, -5.2),
    ));

    // 白栏杆一排贴后墙：细几何，磨边方案的试纸
    let post = meshes.add(Cuboid::new(0.07, 1.5, 0.07));
    let white = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.83, 0.78),
        perceptual_roughness: 0.5,
        ..default()
    });
    for i in 0..13 {
        commands.spawn((
            Mesh3d(post.clone()),
            MeshMaterial3d(white.clone()),
            Transform::from_xyz(-6.0 + i as f32, 0.75, -6.5),
        ));
    }

    // 上釉瓷柱：高光锯齿的窝点，TAA 的表演位
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.16, 2.6).mesh().resolution(64))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.88, 0.82, 0.72),
            perceptual_roughness: 0.08,
            ..default()
        })),
        Transform::from_xyz(3.4, 1.3, 1.0),
    ));

    // 走马灯：运动模糊的表演位
    let panel_mesh = meshes.add(Cuboid::new(0.7, 1.0, 0.05));
    let colors = [
        Color::srgb(0.9, 0.2, 0.15),
        Color::srgb(0.95, 0.7, 0.2),
        Color::srgb(0.2, 0.8, 0.3),
        Color::srgb(0.2, 0.55, 0.95),
        Color::srgb(0.7, 0.3, 0.9),
        Color::srgb(0.95, 0.45, 0.7),
    ];
    commands
        .spawn((
            Carousel,
            Transform::from_xyz(-3.6, 1.6, 1.6),
            Visibility::default(),
            children![(
                Mesh3d(meshes.add(Cylinder::new(0.07, 2.2))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.45, 0.32, 0.18),
                    ..default()
                })),
            )],
        ))
        .with_children(|carousel| {
            for (i, color) in colors.into_iter().enumerate() {
                let angle = i as f32 / colors.len() as f32 * std::f32::consts::TAU;
                carousel.spawn((
                    Mesh3d(panel_mesh.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: color,
                        emissive: color.to_linear() * 1.6,
                        ..default()
                    })),
                    Transform::from_xyz(angle.sin() * 1.3, 0.0, angle.cos() * 1.3)
                        .with_rotation(Quat::from_rotation_y(angle)),
                ));
            }
        });

    // ANCHOR: sign
    // 屏上状态牌：UI 文本的正式讲解在第 28 章，这里先当一块会写字的板用
    commands.spawn((
        Text::new(""),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
            font_size: FontSize::Px(16.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(10),
            left: px(10),
            ..default()
        },
    ));
    // ANCHOR_END: sign

    println!("老雷：盛师傅，夜戏的定妆照全托您了。");
    println!("盛师傅：1~5 磨边，B 辉光，F 景深，M 运动模糊，V 老镜头，T 冲印配方。");
}

/// 走马灯匀速自转
fn spin(time: Res<Time>, mut carousel: Single<&mut Transform, With<Carousel>>) {
    carousel.rotate_y(1.8 * time.delta_secs());
}

// ANCHOR: press
/// 键盘只写黑板，不碰相机
fn press_keys(keyboard: Res<ButtonInput<KeyCode>>, mut board: ResMut<Board>) {
    let digits = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
    ];
    if let Some(gear) = digits.into_iter().position(|k| keyboard.just_pressed(k)) {
        // TAA 的片元着色器翻译不进 WebGL2（一张纹理配多只采样器，GL 语义装不下）——
        // 网页版把 5 号挡整个封存，免得读者一键把 demo 按退场
        if !(cfg!(target_arch = "wasm32") && gear == 4) {
            board.aa = gear;
        }
    }
    if keyboard.just_pressed(KeyCode::KeyB) {
        board.bloom = !board.bloom;
    }
    if keyboard.just_pressed(KeyCode::KeyF) {
        board.dof = !board.dof;
    }
    if keyboard.just_pressed(KeyCode::KeyM) {
        board.motion = !board.motion;
    }
    if keyboard.just_pressed(KeyCode::KeyV) {
        board.vintage = !board.vintage;
    }
    if keyboard.just_pressed(KeyCode::KeyT) {
        board.recipe = (board.recipe + 1) % RECIPES.len();
    }
}
// ANCHOR_END: press

// ANCHOR: apply
/// 黑板落实到相机：一处集中拆装，全场只有这个系统碰效果组件
fn apply_board(
    board: Res<Board>,
    camera: Single<(Entity, &mut Msaa, &mut Tonemapping), With<Camera3d>>,
    mut commands: Commands,
) {
    let (entity, mut msaa, mut tonemapping) = camera.into_inner();
    let mut cam = commands.entity(entity);

    // 磨边：先全下再上岗。注意只摘 TAA 三件，prepass 留着——
    // 走马灯的运动模糊还等着 MotionVectorPrepass 干活
    cam.remove::<(Fxaa, Smaa, TemporalAntiAliasing, TemporalJitter, MipBias)>();
    *msaa = if board.aa == 1 {
        Msaa::Sample4
    } else {
        Msaa::Off
    };
    match board.aa {
        2 => {
            cam.insert(Fxaa::default());
        }
        3 => {
            cam.insert(Smaa::default());
        }
        4 => {
            cam.insert(TemporalAntiAliasing::default());
        }
        _ => {}
    }

    // 效果四组：开就整件插，关就整件拔
    if board.bloom {
        cam.insert(Bloom::NATURAL);
    } else {
        cam.remove::<Bloom>();
    }
    if board.dof {
        cam.insert(DepthOfField {
            focal_distance: 9.8, // 对准堂鼓那一档纵深
            aperture_f_stops: 2.0,
            mode: DepthOfFieldMode::Bokeh,
            ..default()
        });
    } else {
        cam.remove::<DepthOfField>();
    }
    // WebGL2 造不出多重采样的 prepass 纹理（引擎注释：网页读深度纹理必须 Msaa::Off）——
    // 网页版拨进 MSAA 挡时，运动模糊连同两张 prepass 一起请下台；桌面不受此限
    let web_msaa = cfg!(target_arch = "wasm32") && board.aa == 1;
    if board.motion && !web_msaa {
        cam.insert(MotionBlur::default());
    } else {
        cam.remove::<MotionBlur>();
    }
    if web_msaa {
        cam.remove::<(DepthPrepass, MotionVectorPrepass)>();
    }
    if board.vintage {
        cam.insert((
            Vignette::default(),
            LensDistortion {
                intensity: 0.12,
                ..default()
            },
            ChromaticAberration::default(),
        ));
    } else {
        cam.remove::<(Vignette, LensDistortion, ChromaticAberration)>();
    }

    *tonemapping = RECIPES[board.recipe].1;
}
// ANCHOR_END: apply

// ANCHOR: sign_update
/// 黑板变了才重写状态牌
fn repaint_sign(board: Res<Board>, mut sign: Single<&mut Text>) {
    let onoff = |on: bool| if on { "开" } else { "关" };
    sign.0 = format!(
        "定妆照画质台\n磨边[1-5]：{}\n辉光[B]：{}　景深[F]：{}\n运动模糊[M]：{}　老镜头[V]：{}\n冲印[T]：{}",
        AA_GEARS[board.aa],
        onoff(board.bloom),
        onoff(board.dof),
        onoff(board.motion),
        onoff(board.vintage),
        RECIPES[board.recipe].0,
    );
}
// ANCHOR_END: sign_update

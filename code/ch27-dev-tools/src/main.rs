//! Listing 27-14《检场》：把第 20 章《打瓦》的玩法内核搬回工作台，
//! 叠一层只给自己看的调试皮——粉线、账本、水牌全归 ChalkPlugin 一个插件管，
//! 摘掉 add_plugins 里的一行，游戏本体一根汗毛不少。
//! ←/→ 移凳，空格发球，P 定格，F3 粉线总闸，F4 水牌开关。

use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;
use bevy::window::WindowResolution;

// ---------------------------------------------------------------- 台面尺寸（照抄第 20 章）

const WALL_THICKNESS: f32 = 14.0;
const LEFT_WALL: f32 = -430.0;
const RIGHT_WALL: f32 = 430.0;
const TOP_WALL: f32 = 250.0;
const ARENA_BOTTOM: f32 = -370.0;
const GUTTER_Y: f32 = -380.0;

const PADDLE_SIZE: Vec2 = Vec2::new(110.0, 18.0);
const PADDLE_Y: f32 = -280.0;
const PADDLE_SPEED: f32 = 520.0;
const PADDLE_MARGIN: f32 = 8.0;

const BALL_RADIUS: f32 = 11.0;
const BALL_SPEED: f32 = 380.0;
const GLUE_Y: f32 = PADDLE_Y + PADDLE_SIZE.y / 2.0 + BALL_RADIUS + 2.0;
const SERVE_DIRECTION: Vec2 = Vec2::new(0.6, 1.0);

const BRICK_COLUMNS: usize = 8;
const BRICK_ROWS: usize = 7;
const BRICK_SIZE: Vec2 = Vec2::new(92.0, 26.0);
const BRICK_GAP: f32 = 8.0;
const BRICK_TOP_Y: f32 = 212.0;
const GLAZED_ROWS: usize = 2;

const BACKDROP: Color = Color::srgb(0.10, 0.10, 0.14);
const WALL_COLOR: Color = Color::srgb(0.38, 0.32, 0.27);
const PADDLE_COLOR: Color = Color::srgb(0.84, 0.65, 0.37);
const BALL_COLOR: Color = Color::srgb(0.92, 0.36, 0.32);
const TILE_COLOR: Color = Color::srgb(0.52, 0.62, 0.66);
const GLAZED_COLOR: Color = Color::srgb(0.27, 0.43, 0.56);

// ---------------------------------------------------------------- 玩法内核

// ANCHOR: flow
/// 工作台只有两种时辰：跑着、定格——P 键来回拨
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum Flow {
    #[default]
    Running,
    Paused,
}
// ANCHOR_END: flow

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

/// 碰撞登记证：登记自己外接盒的尺寸
#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Component)]
struct Health(u8);

#[derive(Component)]
struct Glued;

#[derive(Resource)]
struct BallStock {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

/// 玩法内核的系统都在这个集合里——检场的粉线排在它后面画
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
struct PlaySet;

// ANCHOR: main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "检场".into(),
                // 比第 20 章高 80 逻辑像素：工作台要看得见台沿下的沟线
                resolution: WindowResolution::new(1280, 800),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKDROP))
        .init_state::<Flow>()
        // 检场插件：整层调试皮就这一行——摘掉它，游戏照演
        .add_plugins(chalk::ChalkPlugin)
        .add_systems(Startup, setup_court)
        .add_systems(OnEnter(Flow::Paused), hold_clock)
        .add_systems(OnExit(Flow::Paused), release_clock)
        .add_systems(Update, toggle_pause)
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                follow_paddle,
                serve_ball,
                apply_velocity,
                check_collisions,
                watch_gutter,
                check_cleared,
            )
                .chain()
                .in_set(PlaySet),
        )
        .run();
}
// ANCHOR_END: main

fn setup_court(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let side_height = TOP_WALL - ARENA_BOTTOM + WALL_THICKNESS;
    let side_center_y = (TOP_WALL + ARENA_BOTTOM) / 2.0;
    let top_width = RIGHT_WALL - LEFT_WALL + WALL_THICKNESS;
    let walls = [
        (
            Vec2::new(LEFT_WALL, side_center_y),
            Vec2::new(WALL_THICKNESS, side_height),
        ),
        (
            Vec2::new(RIGHT_WALL, side_center_y),
            Vec2::new(WALL_THICKNESS, side_height),
        ),
        (
            Vec2::new(0.0, TOP_WALL),
            Vec2::new(top_width, WALL_THICKNESS),
        ),
    ];
    for (position, size) in walls {
        commands.spawn((
            Sprite::from_color(WALL_COLOR, size),
            Transform::from_translation(position.extend(0.0)),
            Collider { size },
        ));
    }

    commands.spawn((
        Paddle,
        Sprite::from_color(PADDLE_COLOR, PADDLE_SIZE),
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
        Collider { size: PADDLE_SIZE },
    ));

    spawn_bricks(&mut commands);

    let stock = BallStock {
        mesh: meshes.add(Circle::new(BALL_RADIUS)),
        material: materials.add(BALL_COLOR),
    };
    spawn_ball(&mut commands, &stock);
    commands.insert_resource(stock);

    println!("老雷：工作台开张。检场，你的粉线伺候着。");
    println!("检场：得嘞——F3 粉线，F4 水牌，P 定格慢慢看。");
}

fn spawn_bricks(commands: &mut Commands) {
    let grid_width = BRICK_COLUMNS as f32 * (BRICK_SIZE.x + BRICK_GAP) - BRICK_GAP;
    let first_x = -grid_width / 2.0 + BRICK_SIZE.x / 2.0;
    for row in 0..BRICK_ROWS {
        let glazed = row < GLAZED_ROWS;
        for column in 0..BRICK_COLUMNS {
            commands.spawn((
                Health(if glazed { 2 } else { 1 }),
                Sprite::from_color(if glazed { GLAZED_COLOR } else { TILE_COLOR }, BRICK_SIZE),
                Transform::from_xyz(
                    first_x + column as f32 * (BRICK_SIZE.x + BRICK_GAP),
                    BRICK_TOP_Y - row as f32 * (BRICK_SIZE.y + BRICK_GAP),
                    0.0,
                ),
                Collider { size: BRICK_SIZE },
            ));
        }
    }
}

fn spawn_ball(commands: &mut Commands, stock: &BallStock) {
    commands.spawn((
        Ball,
        Glued,
        Velocity(Vec2::ZERO),
        Mesh2d(stock.mesh.clone()),
        MeshMaterial2d(stock.material.clone()),
        Transform::from_xyz(0.0, GLUE_Y, 1.0),
    ));
}

// ANCHOR: pause
/// P：定格/放行。捏住虚拟时钟，FixedUpdate 的鼓点就整个停了
fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<Flow>>,
    mut next: ResMut<NextState<Flow>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        next.set(match state.get() {
            Flow::Running => Flow::Paused,
            Flow::Paused => Flow::Running,
        });
    }
}

fn hold_clock(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn release_clock(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}
// ANCHOR_END: pause

fn move_paddle(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut paddle: Single<&mut Transform, With<Paddle>>,
) {
    let mut steer = 0.0;
    if keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        steer -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        steer += 1.0;
    }
    let reach = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_MARGIN;
    let next = paddle.translation.x + steer * PADDLE_SPEED * time.delta_secs();
    paddle.translation.x = next.clamp(-reach, reach);
}

fn follow_paddle(
    paddle: Single<&Transform, With<Paddle>>,
    mut glued: Query<&mut Transform, (With<Glued>, Without<Paddle>)>,
) {
    for mut transform in &mut glued {
        transform.translation.x = paddle.translation.x;
    }
}

fn serve_ball(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut glued: Query<(Entity, &mut Velocity), With<Glued>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    for (entity, mut velocity) in &mut glued {
        velocity.0 = SERVE_DIRECTION.normalize() * BALL_SPEED;
        commands.entity(entity).remove::<Glued>();
    }
}

fn apply_velocity(time: Res<Time>, mut movers: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut movers {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

/// 圆撞盒：不相交返回 None；相交则由“盒上最近点”判断撞的是哪一面
fn hit_side(ball: BoundingCircle, target: Aabb2d) -> Option<Side> {
    if !ball.intersects(&target) {
        return None;
    }
    let closest = target.closest_point(ball.center);
    let offset = ball.center - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x > 0.0 {
            Side::Right
        } else {
            Side::Left
        }
    } else if offset.y > 0.0 {
        Side::Top
    } else {
        Side::Bottom
    };
    Some(side)
}

fn check_collisions(
    mut commands: Commands,
    ball: Single<(&Transform, &mut Velocity), With<Ball>>,
    mut colliders: Query<(Entity, &Transform, &Collider, Option<(&mut Sprite, &mut Health)>)>,
) {
    let (ball_transform, mut velocity) = ball.into_inner();
    let bounding = BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS);

    for (entity, transform, collider, brick) in &mut colliders {
        let target = Aabb2d::new(transform.translation.truncate(), collider.size / 2.0);
        let Some(side) = hit_side(bounding, target) else {
            continue;
        };
        let (reflect_x, reflect_y) = match side {
            Side::Left => (velocity.x > 0.0, false),
            Side::Right => (velocity.x < 0.0, false),
            Side::Top => (false, velocity.y < 0.0),
            Side::Bottom => (false, velocity.y > 0.0),
        };
        if reflect_x {
            velocity.x = -velocity.x;
        }
        if reflect_y {
            velocity.y = -velocity.y;
        }
        if let Some((mut sprite, mut health)) = brick {
            health.0 -= 1;
            if health.0 == 0 {
                commands.entity(entity).despawn();
            } else {
                sprite.color = TILE_COLOR;
            }
        }
    }
}

/// 工作台不记命数：球掉沟就再补一只，试到尽兴为止
fn watch_gutter(
    mut commands: Commands,
    stock: Res<BallStock>,
    ball: Single<(Entity, &Transform), With<Ball>>,
) {
    let (entity, transform) = *ball;
    if transform.translation.y > GUTTER_Y {
        return;
    }
    commands.entity(entity).despawn();
    spawn_ball(&mut commands, &stock);
    println!("检场：球喂了沟——工作台管够，再来。");
}

/// 瓦阵清空就重砌一面，接着试
fn check_cleared(bricks: Query<(), With<Health>>, mut commands: Commands) {
    if bricks.is_empty() {
        spawn_bricks(&mut commands);
        println!("检场：瓦打光了，重砌一面接着来。");
    }
}

// ---------------------------------------------------------------- 检场插件

// ANCHOR: chalk_plugin
/// 调试层整个住在这个模块里：粉线、账本、水牌。
/// 它只“看”游戏（读组件、数数），从不改玩法的一个字段
mod chalk {
    use super::*;
    use bevy::dev_tools::diagnostics_overlay::{
        DiagnosticsOverlay, DiagnosticsOverlayItem, DiagnosticsOverlayPlugin,
        DiagnosticsOverlayStatistic,
    };
    use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
    use bevy::dev_tools::states::log_transitions;
    use bevy::diagnostic::{
        Diagnostic, DiagnosticPath, Diagnostics, EntityCountDiagnosticsPlugin, RegisterDiagnostic,
    };

    /// 台上还剩几片瓦
    const BRICKS_LEFT: DiagnosticPath = DiagnosticPath::const_new("dawa/bricks_left");
    /// 每拍做了多少对碰撞检查（球数 × 登记在册的盒子数）
    const HIT_CHECKS: DiagnosticPath = DiagnosticPath::const_new("dawa/hit_checks");

    /// 检场的粉线自成一组：虚线画沟，规格不惊动别家
    #[derive(Default, Reflect, GizmoConfigGroup)]
    struct GutterLine;

    pub struct ChalkPlugin;

    impl Plugin for ChalkPlugin {
        fn build(&self, app: &mut App) {
            app
                // 水牌 + 小窗管家 + 口数账（fps 账由水牌捎带挂上）
                .add_plugins((
                    FpsOverlayPlugin {
                        config: FpsOverlayConfig {
                            text_config: TextFont::from_font_size(22.0),
                            text_color: Color::srgb(0.91, 0.88, 0.80),
                            frame_time_graph_config: FrameTimeGraphConfig {
                                enabled: true,
                                min_fps: 30.0,
                                target_fps: 60.0,
                            },
                            ..default()
                        },
                    },
                    DiagnosticsOverlayPlugin,
                    EntityCountDiagnosticsPlugin::default(),
                ))
                // 两本自家账。后缀故意不写中文：账本小窗用引擎内置字模，
                // 中文会成豆腐块（27.4 的老问题换个场地再犯）
                .register_diagnostic(Diagnostic::new(BRICKS_LEFT))
                .register_diagnostic(Diagnostic::new(HIT_CHECKS))
                .init_gizmo_group::<GutterLine>()
                .add_systems(Startup, hang_ledger)
                // 账本窗默认出生在左上角，会与水牌叠住——搬去水牌下面
                .add_systems(PostStartup, place_ledger)
                // 粉线跟着鼓点画：排在玩法后面，标的都是本拍的新位置
                .add_systems(
                    FixedUpdate,
                    (chalk_court, chalk_ball, keep_ledger).after(PlaySet),
                )
                .add_systems(Update, (master_switch, log_transitions::<Flow>));
        }
    }

    /// 开台挂账本小窗，顺手把沟线的规格定了
    fn hang_ledger(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
        commands.spawn(DiagnosticsOverlay::new(
            "Dawa ledger",
            vec![
                DiagnosticsOverlayItem {
                    path: BRICKS_LEFT,
                    statistic: DiagnosticsOverlayStatistic::Value,
                    precision: 0,
                },
                DiagnosticsOverlayItem {
                    path: HIT_CHECKS,
                    statistic: DiagnosticsOverlayStatistic::Value,
                    precision: 0,
                },
                DiagnosticsOverlayItem {
                    path: EntityCountDiagnosticsPlugin::ENTITY_COUNT,
                    statistic: DiagnosticsOverlayStatistic::Value,
                    precision: 0,
                },
            ],
        ));

        let (config, _) = config_store.config_mut::<GutterLine>();
        config.line.width = 3.0;
        config.line.style = GizmoLineStyle::Dashed {
            gap_scale: 2.0,
            line_scale: 4.0,
        };
    }

    /// 小窗出生后把它挪到水牌下方（拖动手感不受影响，照样能拽走）
    fn place_ledger(mut windows: Query<&mut Node, With<DiagnosticsOverlay>>) {
        for mut node in &mut windows {
            node.top = Val::Px(96.0);
            node.left = Val::Px(8.0);
        }
    }
    // ANCHOR_END: chalk_plugin

    // ANCHOR: chalk_marks
    /// 台面记号：登记在册的盒子逐个描框，沟线用虚线画
    fn chalk_court(
        mut gizmos: Gizmos,
        mut gutter: Gizmos<GutterLine>,
        colliders: Query<(&Transform, &Collider, Has<Paddle>, Option<&Health>)>,
    ) {
        for (transform, collider, is_paddle, health) in &colliders {
            let color = match (is_paddle, health) {
                (true, _) => Color::srgb(1.0, 0.85, 0.3),            // 凳：金
                (_, Some(Health(2))) => Color::srgb(0.4, 0.9, 1.0),  // 上釉瓦：亮青
                (_, Some(_)) => Color::srgb(1.0, 1.0, 1.0),          // 素瓦：白
                _ => Color::srgb(0.6, 0.6, 0.6),                     // 墙：灰
            };
            gizmos.rect_2d(transform.translation.truncate(), collider.size, color);
        }
        // 沟线：球过了这条虚线就算喂沟
        gutter.line_2d(
            Vec2::new(LEFT_WALL, GUTTER_Y),
            Vec2::new(RIGHT_WALL, GUTTER_Y),
            Color::srgb(1.0, 0.3, 0.25),
        );
    }

    /// 球的记号：外接圆、速度箭头、随行速度牌；待发球画预告箭头
    fn chalk_ball(mut gizmos: Gizmos, balls: Query<(&Transform, &Velocity, Has<Glued>), With<Ball>>) {
        for (transform, velocity, glued) in &balls {
            let center = transform.translation.truncate();
            gizmos.circle_2d(center, BALL_RADIUS, Color::srgb(0.4, 1.0, 0.5));

            if glued {
                // 还趴在凳上：画一支半透明的发球预告箭头
                let preview = SERVE_DIRECTION.normalize() * BALL_SPEED * 0.25;
                gizmos.arrow_2d(center, center + preview, Color::srgba(1.0, 0.85, 0.3, 0.5));
            } else {
                // 飞行中：速度箭头 = 0.25 秒的路程，短平快一眼读出快慢与方向
                gizmos.arrow_2d(center, center + velocity.0 * 0.25, Color::srgb(1.0, 0.5, 0.2));
                gizmos.text_2d(
                    center + Vec2::Y * (BALL_RADIUS + 10.0),
                    &format!("v {:.0}", velocity.length()),
                    22.0,
                    Vec2::new(0.0, -0.5),
                    Color::srgb(0.4, 1.0, 0.5),
                );
            }
        }
    }

    /// 记账：瓦数、每拍碰撞检查对数
    fn keep_ledger(
        mut diagnostics: Diagnostics,
        bricks: Query<(), With<Health>>,
        balls: Query<(), With<Ball>>,
        colliders: Query<(), With<Collider>>,
    ) {
        diagnostics.add_measurement(&BRICKS_LEFT, || bricks.iter().count() as f64);
        diagnostics.add_measurement(&HIT_CHECKS, || {
            (balls.iter().count() * colliders.iter().count()) as f64
        });
    }
    // ANCHOR_END: chalk_marks

    // ANCHOR: switch
    /// F3 粉线总闸（默认组和沟线组一起拨）；F4 水牌开关
    fn master_switch(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut config_store: ResMut<GizmoConfigStore>,
        mut overlay: ResMut<FpsOverlayConfig>,
    ) {
        if keyboard.just_pressed(KeyCode::F3) {
            let flip = !config_store.config::<DefaultGizmoConfigGroup>().0.enabled;
            config_store.config_mut::<DefaultGizmoConfigGroup>().0.enabled = flip;
            config_store.config_mut::<GutterLine>().0.enabled = flip;
            println!("检场：粉线{}。", if flip { "上台" } else { "全下" });
        }
        if keyboard.just_pressed(KeyCode::F4) {
            overlay.enabled = !overlay.enabled;
            overlay.frame_time_graph_config.enabled = overlay.enabled;
            println!("检场：水牌{}。", if overlay.enabled { "挂出" } else { "收起" });
        }
    }
    // ANCHOR_END: switch
}

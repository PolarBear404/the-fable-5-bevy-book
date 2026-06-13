//! game：玩法本体——场地、凳、球、瓦阵，鼓点上的物理与规则。
//! 对外只承诺四样：`Knock` 消息、`Lives` 与 `Outcome` 资源、几个场地常量。

use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

use crate::{GameState, IsPaused};

// ---------------------------------------------------------------- 常量

pub const WALL_THICKNESS: f32 = 14.0;
pub const LEFT_WALL: f32 = -430.0;
pub const RIGHT_WALL: f32 = 430.0;
pub const TOP_WALL: f32 = 250.0;
const ARENA_BOTTOM: f32 = -370.0;
/// 球心低于这条线即落沟（已在窗外）
const GUTTER_Y: f32 = -380.0;

const PADDLE_SIZE: Vec2 = Vec2::new(110.0, 18.0);
const PADDLE_Y: f32 = -280.0;
const PADDLE_SPEED: f32 = 520.0;
const PADDLE_MARGIN: f32 = 8.0;

const BALL_RADIUS: f32 = 11.0;
const BALL_SPEED: f32 = 380.0;
/// 待发的球趴在凳面上的高度
const GLUE_Y: f32 = PADDLE_Y + PADDLE_SIZE.y / 2.0 + BALL_RADIUS + 2.0;
const SERVE_DIRECTION: Vec2 = Vec2::new(0.6, 1.0);
/// 一局的绣球数
pub const BALL_COUNT: u32 = 3;

const BRICK_COLUMNS: usize = 8;
const BRICK_ROWS: usize = 7;
const BRICK_SIZE: Vec2 = Vec2::new(92.0, 26.0);
const BRICK_GAP: f32 = 8.0;
const BRICK_TOP_Y: f32 = 212.0;
const GLAZED_ROWS: usize = 2;
/// 全场瓦数：56——第 1 章那张表说好的数
pub const TOTAL_BRICKS: u32 = (BRICK_COLUMNS * BRICK_ROWS) as u32;

const WALL_COLOR: Color = Color::srgb(0.38, 0.32, 0.27);
const PADDLE_COLOR: Color = Color::srgb(0.84, 0.65, 0.37);
const BALL_COLOR: Color = Color::srgb(0.92, 0.36, 0.32);
const TILE_COLOR: Color = Color::srgb(0.52, 0.62, 0.66);
const GLAZED_COLOR: Color = Color::srgb(0.27, 0.43, 0.56);

// ---------------------------------------------------------------- 对外合同

// ANCHOR: contract
/// 台上的一声动静。本插件只管写，谁爱听谁听（score 记账、audio 敲锣）
#[derive(Message)]
pub enum Knock {
    /// 球撞墙
    Wall,
    /// 球撞凳
    Paddle,
    /// 筒瓦掉了釉，还没碎
    Crack,
    /// 一片瓦碎了
    Shatter,
    /// 绣球落了沟
    Gutter,
}

/// 还剩几只绣球
#[derive(Resource, Default)]
pub struct Lives(pub u32);

/// 一局的结论——闭幕的瞬间写下，结算屏与锣鼓都看它
#[derive(Resource, Clone, Copy)]
pub enum Outcome {
    /// 瓦砸完了
    Cleared,
    /// 绣球用尽
    Spilled,
}
// ANCHOR_END: contract

// ---------------------------------------------------------------- 组件

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

/// 瓦的耐久：还经得起几下
#[derive(Component)]
struct Health(u8);

/// 粘在凳上待发的球
#[derive(Component)]
struct Glued;

/// 球的铸模：补球时直接克隆提货单
#[derive(Resource)]
struct BallStock {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

/// 意图层：设备往里写，玩法从里读
#[derive(Resource, Default)]
struct Intent {
    steer: f32,
    serve: bool,
}

// ---------------------------------------------------------------- 插件

// ANCHOR: plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Intent>()
            .init_resource::<Lives>()
            .add_message::<Knock>()
            .add_systems(OnEnter(GameState::Playing), setup_court)
            .add_systems(OnEnter(IsPaused::Paused), hold_clock)
            .add_systems(OnExit(IsPaused::Paused), release_clock)
            .add_systems(
                RunFixedMainLoop,
                collect_intent
                    .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop)
                    .run_if(in_state(IsPaused::Running)),
            )
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
                    .run_if(in_state(IsPaused::Running)),
            );
    }
}
// ANCHOR_END: plugin

// ---------------------------------------------------------------- 搭台

/// 开台：场地、凳、瓦阵、第一只待发的球——全部挂上 DespawnOnExit(Playing)
fn setup_court(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut lives: ResMut<Lives>,
) {
    lives.0 = BALL_COUNT;

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
            DespawnOnExit(GameState::Playing),
        ));
    }

    commands.spawn((
        Paddle,
        Sprite::from_color(PADDLE_COLOR, PADDLE_SIZE),
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
        Collider { size: PADDLE_SIZE },
        DespawnOnExit(GameState::Playing),
    ));

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
                DespawnOnExit(GameState::Playing),
            ));
        }
    }

    let stock = BallStock {
        mesh: meshes.add(Circle::new(BALL_RADIUS)),
        material: materials.add(BALL_COLOR),
    };
    spawn_ball(&mut commands, &stock);
    commands.insert_resource(stock);

    println!("场记：开台——{TOTAL_BRICKS} 片瓦，{BALL_COUNT} 只绣球。");
}

/// 出一只新绣球：趴在凳上等发球
fn spawn_ball(commands: &mut Commands, stock: &BallStock) {
    commands.spawn((
        Ball,
        Glued,
        Velocity(Vec2::ZERO),
        Mesh2d(stock.mesh.clone()),
        MeshMaterial2d(stock.material.clone()),
        Transform::from_xyz(0.0, GLUE_Y, 1.0),
        DespawnOnExit(GameState::Playing),
    ));
}

// ---------------------------------------------------------------- 输入

/// 收集这一帧手上的输入：steer 持续、serve 瞬时攒账（18.5 的招）
fn collect_intent(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut intent: ResMut<Intent>,
) {
    let mut steer = 0.0;
    if keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        steer -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        steer += 1.0;
    }
    if keyboard.just_pressed(KeyCode::Space) {
        intent.serve = true;
    }
    for gamepad in &gamepads {
        let stick = gamepad.left_stick().x;
        if stick.abs() > 0.15 {
            steer += stick;
        }
        steer += gamepad.dpad().x;
        if gamepad.just_pressed(GamepadButton::South) {
            intent.serve = true;
        }
    }
    intent.steer = steer.clamp(-1.0, 1.0);
}

// ---------------------------------------------------------------- 鼓点上的物理

fn move_paddle(
    intent: Res<Intent>,
    time: Res<Time>,
    mut paddle: Single<&mut Transform, With<Paddle>>,
) {
    let reach = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_MARGIN;
    let next = paddle.translation.x + intent.steer * PADDLE_SPEED * time.delta_secs();
    paddle.translation.x = next.clamp(-reach, reach);
}

/// 待发的球跟着凳走
fn follow_paddle(
    paddle: Single<&Transform, With<Paddle>>,
    mut glued: Query<&mut Transform, (With<Glued>, Without<Paddle>)>,
) {
    for mut transform in &mut glued {
        transform.translation.x = paddle.translation.x;
    }
}

/// 发球：消费 serve 意图——没人接就作废，不留到下一拍
fn serve_ball(
    mut intent: ResMut<Intent>,
    mut commands: Commands,
    mut glued: Query<(Entity, &mut Velocity), With<Glued>>,
) {
    let pressed = std::mem::take(&mut intent.serve);
    if !pressed {
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

// ---------------------------------------------------------------- 碰撞

/// 球撞上了盒子的哪一面（以盒子为参照）
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

/// 反弹、扣耐久，把动静报出去
fn check_collisions(
    mut commands: Commands,
    mut knocks: MessageWriter<Knock>,
    ball: Single<(&Transform, &mut Velocity), With<Ball>>,
    mut colliders: Query<(
        Entity,
        &Transform,
        &Collider,
        Has<Paddle>,
        Option<(&mut Sprite, &mut Health)>,
    )>,
) {
    let (ball_transform, mut velocity) = ball.into_inner();
    let bounding = BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS);

    for (entity, transform, collider, is_paddle, brick) in &mut colliders {
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

        match brick {
            Some((mut sprite, mut health)) => {
                health.0 -= 1;
                if health.0 == 0 {
                    commands.entity(entity).despawn();
                    knocks.write(Knock::Shatter);
                } else {
                    sprite.color = TILE_COLOR;
                    knocks.write(Knock::Crack);
                }
            }
            None => {
                knocks.write(if is_paddle {
                    Knock::Paddle
                } else {
                    Knock::Wall
                });
            }
        }
    }
}

// ---------------------------------------------------------------- 胜负

/// 盯着沟：球掉下去就收走；还有绣球就补一只，没有就闭幕
fn watch_gutter(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    mut knocks: MessageWriter<Knock>,
    mut next_state: ResMut<NextState<GameState>>,
    stock: Res<BallStock>,
    ball: Single<(Entity, &Transform), With<Ball>>,
) {
    let (entity, transform) = *ball;
    if transform.translation.y > GUTTER_Y {
        return;
    }
    commands.entity(entity).despawn();
    knocks.write(Knock::Gutter);
    lives.0 -= 1;
    if lives.0 > 0 {
        println!("场记：一只绣球喂了沟——还剩 {} 只。", lives.0);
        spawn_ball(&mut commands, &stock);
    } else {
        commands.insert_resource(Outcome::Spilled);
        next_state.set(GameState::GameOver);
    }
}

/// 盯着瓦阵：一片不剩就是满堂彩
fn check_cleared(
    bricks: Query<(), With<Health>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if bricks.is_empty() {
        commands.insert_resource(Outcome::Cleared);
        next_state.set(GameState::GameOver);
    }
}

// ---------------------------------------------------------------- 中场的钟

/// 中场协议的戏台钟侧（18.2）：规则停在 run_if，时间停在这儿
fn hold_clock(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn release_clock(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}

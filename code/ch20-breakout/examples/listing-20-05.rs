//! Listing 20-5：记分——碰撞改发消息，记分牌只管听
//! 在 Listing 20-4 之上：Knock 消息 + Score 资源 + 记分牌 Text2d。
//! 跑起来玩：左上角的记分牌随碎瓦走字，里程碑时控制台有一句台词。

use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;
use bevy::sprite::Anchor;

const WALL_THICKNESS: f32 = 14.0;
const LEFT_WALL: f32 = -430.0;
const RIGHT_WALL: f32 = 430.0;
const TOP_WALL: f32 = 250.0;
const ARENA_BOTTOM: f32 = -370.0;

const PADDLE_SIZE: Vec2 = Vec2::new(110.0, 18.0);
const PADDLE_Y: f32 = -280.0;
const PADDLE_SPEED: f32 = 520.0;
const PADDLE_MARGIN: f32 = 8.0;

const BALL_RADIUS: f32 = 11.0;
const BALL_SPEED: f32 = 380.0;
const BALL_DROP: Vec3 = Vec3::new(0.0, -120.0, 1.0);
const DROP_DIRECTION: Vec2 = Vec2::new(0.6, 1.0);

const BRICK_COLUMNS: usize = 8;
const BRICK_ROWS: usize = 7;
const BRICK_SIZE: Vec2 = Vec2::new(92.0, 26.0);
const BRICK_GAP: f32 = 8.0;
const BRICK_TOP_Y: f32 = 212.0;
const GLAZED_ROWS: usize = 2;
/// 全场瓦数：56
const TOTAL_BRICKS: u32 = (BRICK_COLUMNS * BRICK_ROWS) as u32;

/// 记分牌一行字的高度（顶墙上方）
const BOARD_Y: f32 = 302.0;

const BACKDROP: Color = Color::srgb(0.10, 0.10, 0.14);
const WALL_COLOR: Color = Color::srgb(0.38, 0.32, 0.27);
const PADDLE_COLOR: Color = Color::srgb(0.84, 0.65, 0.37);
const BALL_COLOR: Color = Color::srgb(0.92, 0.36, 0.32);
const TILE_COLOR: Color = Color::srgb(0.52, 0.62, 0.66);
const GLAZED_COLOR: Color = Color::srgb(0.27, 0.43, 0.56);
const TEXT_COLOR: Color = Color::srgb(0.91, 0.88, 0.80); // 米白

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Component)]
struct Health(u8);

// ANCHOR: knock
/// 台上的一声动静。碰撞系统只管写，谁爱听谁听——第 7 章碰碰车场的解耦
#[derive(Message)]
enum Knock {
    /// 球撞墙
    Wall,
    /// 球撞凳
    Paddle,
    /// 筒瓦掉了釉，还没碎
    Crack,
    /// 一片瓦碎了
    Shatter,
}

/// 战果：碎了几片瓦
#[derive(Resource, Default)]
struct Score(u32);
// ANCHOR_END: knock

// ANCHOR: scoreboard
/// 记分牌（左上角那行字）
#[derive(Component)]
struct ScoreBoard;

/// 听 Knock 记账：只有 Shatter 算分，里程碑配一句台词
fn tally(mut knocks: MessageReader<Knock>, mut score: ResMut<Score>) {
    for knock in knocks.read() {
        if matches!(knock, Knock::Shatter) {
            score.0 += 1;
            match score.0 {
                1 => println!("场记：头一片，开张。"),
                28 => println!("场记：过半了——还剩 28 片。"),
                50 => println!("场记：还剩 6 片，稳着点。"),
                _ => {}
            }
        }
    }
}

/// 分数变了才重排版——第 5 章的资源变更检测在调度层把关
fn refresh_scoreboard(score: Res<Score>, mut board: Single<&mut Text2d, With<ScoreBoard>>) {
    board.0 = format!("瓦 {}/{}", score.0, TOTAL_BRICKS);
}
// ANCHOR_END: scoreboard

#[derive(Debug, Clone, Copy)]
enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

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

// ANCHOR: check_collisions
/// 碰撞系统现在只剩本职：反弹、扣耐久，再把动静报出去
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
// ANCHOR_END: check_collisions

#[derive(Resource, Default)]
struct Intent {
    steer: f32,
}

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
    for gamepad in &gamepads {
        let stick = gamepad.left_stick().x;
        if stick.abs() > 0.15 {
            steer += stick;
        }
        steer += gamepad.dpad().x;
    }
    intent.steer = steer.clamp(-1.0, 1.0);
}

fn move_paddle(
    intent: Res<Intent>,
    time: Res<Time>,
    mut paddle: Single<&mut Transform, With<Paddle>>,
) {
    let reach = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_MARGIN;
    let next = paddle.translation.x + intent.steer * PADDLE_SPEED * time.delta_secs();
    paddle.translation.x = next.clamp(-reach, reach);
}

fn apply_velocity(time: Res<Time>, mut movers: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut movers {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

// ANCHOR: main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "打瓦".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKDROP))
        .init_resource::<Intent>()
        .init_resource::<Score>()
        .add_message::<Knock>()
        .add_systems(Startup, (setup_court, rig_scoreboard))
        .add_systems(
            RunFixedMainLoop,
            collect_intent.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
        )
        .add_systems(
            FixedUpdate,
            (move_paddle, apply_velocity, check_collisions).chain(),
        )
        .add_systems(
            Update,
            (tally, refresh_scoreboard.run_if(resource_changed::<Score>)).chain(),
        )
        .run();
}
// ANCHOR_END: main

// ANCHOR: rig_scoreboard
/// 挂记分牌：一行 Text2d，钉在顶墙左上方
fn rig_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ScoreBoard,
        Text2d::new(format!("瓦 0/{TOTAL_BRICKS}")),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-bold.otf"),
            font_size: 30.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
        Anchor::CENTER_LEFT,
        Transform::from_xyz(LEFT_WALL - WALL_THICKNESS / 2.0, BOARD_Y, 5.0),
    ));
}
// ANCHOR_END: rig_scoreboard

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

    commands.spawn((
        Ball,
        Velocity(DROP_DIRECTION.normalize() * BALL_SPEED),
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::from_translation(BALL_DROP),
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
            ));
        }
    }

    println!("老雷：散场不散人——后台的《打瓦》摊子支起来了。");
}

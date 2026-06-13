//! Listing 20-2：放球——绣球上场，沿直线飞行
//! 在 Listing 20-1 之上：Ball + Velocity + apply_velocity。
//! 跑起来看：球往右上飞，对三面墙视而不见，穿出台外一去不回。

use bevy::prelude::*;

const WALL_THICKNESS: f32 = 14.0;
const LEFT_WALL: f32 = -430.0;
const RIGHT_WALL: f32 = 430.0;
const TOP_WALL: f32 = 250.0;
const ARENA_BOTTOM: f32 = -370.0;

const PADDLE_SIZE: Vec2 = Vec2::new(110.0, 18.0);
const PADDLE_Y: f32 = -280.0;
const PADDLE_SPEED: f32 = 520.0;
const PADDLE_MARGIN: f32 = 8.0;

// ANCHOR: ball_consts
const BALL_RADIUS: f32 = 11.0;
const BALL_SPEED: f32 = 380.0;
// 试运行的出球点与方向——正式的发球规矩第 20.5 节才立
const BALL_DROP: Vec3 = Vec3::new(0.0, -120.0, 1.0);
const DROP_DIRECTION: Vec2 = Vec2::new(0.6, 1.0);
// ANCHOR_END: ball_consts

const BACKDROP: Color = Color::srgb(0.10, 0.10, 0.14);
const WALL_COLOR: Color = Color::srgb(0.38, 0.32, 0.27);
const PADDLE_COLOR: Color = Color::srgb(0.84, 0.65, 0.37);
const BALL_COLOR: Color = Color::srgb(0.92, 0.36, 0.32); // 绣球红

#[derive(Component)]
struct Paddle;

// ANCHOR: ball
/// 绣球
#[derive(Component)]
struct Ball;

/// 速度：每秒走多少世界单位——谁挂上它，谁就归 apply_velocity 管
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

/// 物理第一课：位置 += 速度 × 步长，在鼓点上结算
fn apply_velocity(time: Res<Time>, mut movers: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut movers {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}
// ANCHOR_END: ball

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
        .add_systems(Startup, setup_court)
        .add_systems(
            RunFixedMainLoop,
            collect_intent.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
        )
        .add_systems(FixedUpdate, (move_paddle, apply_velocity).chain())
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
        ));
    }

    commands.spawn((
        Paddle,
        Sprite::from_color(PADDLE_COLOR, PADDLE_SIZE),
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
    ));

    // ANCHOR: spawn_ball
    // 绣球：圆形网格 + 纯色材质——第 15 章“不用画的道具”
    commands.spawn((
        Ball,
        Velocity(DROP_DIRECTION.normalize() * BALL_SPEED),
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::from_translation(BALL_DROP),
    ));
    // ANCHOR_END: spawn_ball

    println!("老雷：散场不散人——后台的《打瓦》摊子支起来了。");
}

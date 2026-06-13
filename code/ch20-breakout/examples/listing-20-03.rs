//! Listing 20-3：碰撞——给球一双眼睛，看见墙与凳
//! 在 Listing 20-2 之上：Collider 登记尺寸 + bounding 判交 + 迎面反弹。
//! 跑起来玩：球在三面墙之间永动，凳子接得住就一直接；接不住，球落沟里一去不回。

use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
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

const BALL_RADIUS: f32 = 11.0;
const BALL_SPEED: f32 = 380.0;
const BALL_DROP: Vec3 = Vec3::new(0.0, -120.0, 1.0);
const DROP_DIRECTION: Vec2 = Vec2::new(0.6, 1.0);

const BACKDROP: Color = Color::srgb(0.10, 0.10, 0.14);
const WALL_COLOR: Color = Color::srgb(0.38, 0.32, 0.27);
const PADDLE_COLOR: Color = Color::srgb(0.84, 0.65, 0.37);
const BALL_COLOR: Color = Color::srgb(0.92, 0.36, 0.32);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

// ANCHOR: collider
/// 碰撞登记证：登记自己外接盒的尺寸——挂上它，球才看得见你
#[derive(Component)]
struct Collider {
    size: Vec2,
}
// ANCHOR_END: collider

// ANCHOR: hit_side
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
// ANCHOR_END: hit_side

// ANCHOR: check_collisions
/// 拿球逐个对照所有登记过的盒子：迎面撞上，就把速度的那一轴翻号
fn check_collisions(
    ball: Single<(&Transform, &mut Velocity), With<Ball>>,
    colliders: Query<(&Transform, &Collider)>,
) {
    let (ball_transform, mut velocity) = ball.into_inner();
    let bounding = BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS);

    for (transform, collider) in &colliders {
        let target = Aabb2d::new(transform.translation.truncate(), collider.size / 2.0);
        let Some(side) = hit_side(bounding, target) else {
            continue;
        };
        // 只有速度迎着撞面时才反弹——背着面飞说明正在脱身，别再拍回去
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
        .add_systems(Startup, setup_court)
        .add_systems(
            RunFixedMainLoop,
            collect_intent.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
        )
        .add_systems(
            FixedUpdate,
            (move_paddle, apply_velocity, check_collisions).chain(),
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

    // ANCHOR: spawn_with_collider
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
            Collider { size }, // 墙：领证
        ));
    }

    commands.spawn((
        Paddle,
        Sprite::from_color(PADDLE_COLOR, PADDLE_SIZE),
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
        Collider { size: PADDLE_SIZE }, // 凳：领证
    ));
    // ANCHOR_END: spawn_with_collider

    commands.spawn((
        Ball,
        Velocity(DROP_DIRECTION.normalize() * BALL_SPEED),
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::from_translation(BALL_DROP),
    ));

    println!("老雷：散场不散人——后台的《打瓦》摊子支起来了。");
}

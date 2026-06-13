//! Listing 20-4：瓦阵——8 列 × 7 行 = 56 片瓦，顶上两行是要砸两下的筒瓦
//! 在 Listing 20-3 之上：Health 组件 + 瓦阵生成 + 碰撞里的碎瓦分支。
//! 跑起来玩：球打上去瓦应声而碎；筒瓦先掉釉（变色）再碎。

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

// ANCHOR: brick_consts
// 瓦阵：8 列 × 7 行 = 56 片——第 1 章那张表说好的数
const BRICK_COLUMNS: usize = 8;
const BRICK_ROWS: usize = 7;
const BRICK_SIZE: Vec2 = Vec2::new(92.0, 26.0);
const BRICK_GAP: f32 = 8.0;
const BRICK_TOP_Y: f32 = 212.0; // 最上一行瓦的中心高度
const GLAZED_ROWS: usize = 2; // 顶上两行是筒瓦：带釉，要砸两下
// ANCHOR_END: brick_consts

const BACKDROP: Color = Color::srgb(0.10, 0.10, 0.14);
const WALL_COLOR: Color = Color::srgb(0.38, 0.32, 0.27);
const PADDLE_COLOR: Color = Color::srgb(0.84, 0.65, 0.37);
const BALL_COLOR: Color = Color::srgb(0.92, 0.36, 0.32);
// ANCHOR: brick_colors
const TILE_COLOR: Color = Color::srgb(0.52, 0.62, 0.66); // 素瓦的灰青
const GLAZED_COLOR: Color = Color::srgb(0.27, 0.43, 0.56); // 筒瓦的釉色
// ANCHOR_END: brick_colors

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

// ANCHOR: health
/// 瓦的耐久：还经得起几下。第 1 章那张表的 Health 列，在此兑现
#[derive(Component)]
struct Health(u8);
// ANCHOR_END: health

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
/// 反弹照旧；撞上的若是瓦（带 Health），扣它一条耐久
fn check_collisions(
    mut commands: Commands,
    ball: Single<(&Transform, &mut Velocity), With<Ball>>,
    mut colliders: Query<(
        Entity,
        &Transform,
        &Collider,
        Option<(&mut Sprite, &mut Health)>,
    )>,
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

        // 墙和凳没有 Health，这个分支只有瓦才进得来
        if let Some((mut sprite, mut health)) = brick {
            health.0 -= 1;
            if health.0 == 0 {
                commands.entity(entity).despawn(); // 瓦碎
            } else {
                sprite.color = TILE_COLOR; // 釉面剥落，露出素瓦
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

    // ANCHOR: spawn_bricks
    // 瓦阵：算出左上角第一片的圆心，按行列铺开
    let grid_width = BRICK_COLUMNS as f32 * (BRICK_SIZE.x + BRICK_GAP) - BRICK_GAP;
    let first_x = -grid_width / 2.0 + BRICK_SIZE.x / 2.0;
    for row in 0..BRICK_ROWS {
        let glazed = row < GLAZED_ROWS; // 自上而下数，前两行带釉
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
    // ANCHOR_END: spawn_bricks

    println!("老雷：散场不散人——后台的《打瓦》摊子支起来了。");
}

//! Listing 20-1：搭台——三面墙围出场地，一条长凳听候差遣
//! A/D 或 ←→ 推凳，手柄的左摇杆与十字键同样使唤。

use bevy::prelude::*;

// ANCHOR: consts
// 场地几何：默认 Camera2d 下 1 个世界单位 = 1 逻辑像素，原点在窗口正中
const WALL_THICKNESS: f32 = 14.0;
const LEFT_WALL: f32 = -430.0; // 左右墙与顶墙的中心线
const RIGHT_WALL: f32 = 430.0;
const TOP_WALL: f32 = 250.0;
const ARENA_BOTTOM: f32 = -370.0; // 台口下沿：侧墙垂到这里，底边不封口

const PADDLE_SIZE: Vec2 = Vec2::new(110.0, 18.0);
const PADDLE_Y: f32 = -280.0;
const PADDLE_SPEED: f32 = 520.0;
const PADDLE_MARGIN: f32 = 8.0; // 凳腿离墙的最小空隙

const BACKDROP: Color = Color::srgb(0.10, 0.10, 0.14); // 夜幕
const WALL_COLOR: Color = Color::srgb(0.38, 0.32, 0.27); // 戏台木框
const PADDLE_COLOR: Color = Color::srgb(0.84, 0.65, 0.37); // 条凳木色
// ANCHOR_END: consts

// ANCHOR: paddle
/// 条凳：场上唯一听玩家使唤的东西
#[derive(Component)]
struct Paddle;
// ANCHOR_END: paddle

// ANCHOR: intent
/// 意图层（第 17 章的骨架）：设备只管往里写，玩法只管从里读
#[derive(Resource, Default)]
struct Intent {
    /// 推凳方向：-1.0（左）到 +1.0（右），每帧由设备重写
    steer: f32,
}

/// 收集这一帧手上的输入——键盘给数字量，摇杆给模拟量，殊途同归
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
            steer += stick; // 摇杆的死区自己留——第 17 章的账
        }
        steer += gamepad.dpad().x;
    }
    intent.steer = steer.clamp(-1.0, 1.0);
}
// ANCHOR_END: intent

// ANCHOR: move
/// 推凳：在鼓点上结算位移，两头让墙拦住
fn move_paddle(
    intent: Res<Intent>,
    time: Res<Time>,
    mut paddle: Single<&mut Transform, With<Paddle>>,
) {
    let reach = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_MARGIN;
    let next = paddle.translation.x + intent.steer * PADDLE_SPEED * time.delta_secs();
    paddle.translation.x = next.clamp(-reach, reach);
}
// ANCHOR_END: move

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
        .add_systems(FixedUpdate, move_paddle)
        .run();
}
// ANCHOR_END: main

// ANCHOR: setup
/// 搭台：相机、三面墙、一条凳
fn setup_court(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 三面墙：左右两面垂到台口下沿，顶上一面横贯；底边不封——那是沟
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

    // 条凳
    commands.spawn((
        Paddle,
        Sprite::from_color(PADDLE_COLOR, PADDLE_SIZE),
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
    ));

    println!("老雷：散场不散人——后台的《打瓦》摊子支起来了。");
}
// ANCHOR_END: setup

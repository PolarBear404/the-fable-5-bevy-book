//! Listing 20-6：开幕与闭幕——状态机进场，游戏第一次有了“一局”
//! 在 Listing 20-5 之上：GameState 三态 + 发球/绣球命数 + 胜负判定 + 结算屏。
//! 跑起来玩：空格开局，球粘在凳上再按空格发球；瓦清完是“满堂彩”，
//! 三只绣球喂了沟是“绣球散尽”；结算屏空格再来、Esc 回后台。

use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;
use bevy::sprite::Anchor;

const WALL_THICKNESS: f32 = 14.0;
const LEFT_WALL: f32 = -430.0;
const RIGHT_WALL: f32 = 430.0;
const TOP_WALL: f32 = 250.0;
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
/// 发球方向：斜向右上（归一化后乘速度）
const SERVE_DIRECTION: Vec2 = Vec2::new(0.6, 1.0);
/// 一局的绣球数
const BALL_COUNT: u32 = 3;

const BRICK_COLUMNS: usize = 8;
const BRICK_ROWS: usize = 7;
const BRICK_SIZE: Vec2 = Vec2::new(92.0, 26.0);
const BRICK_GAP: f32 = 8.0;
const BRICK_TOP_Y: f32 = 212.0;
const GLAZED_ROWS: usize = 2;
const TOTAL_BRICKS: u32 = (BRICK_COLUMNS * BRICK_ROWS) as u32;

const BOARD_Y: f32 = 302.0;

const BACKDROP: Color = Color::srgb(0.10, 0.10, 0.14);
const WALL_COLOR: Color = Color::srgb(0.38, 0.32, 0.27);
const PADDLE_COLOR: Color = Color::srgb(0.84, 0.65, 0.37);
const BALL_COLOR: Color = Color::srgb(0.92, 0.36, 0.32);
const TILE_COLOR: Color = Color::srgb(0.52, 0.62, 0.66);
const GLAZED_COLOR: Color = Color::srgb(0.27, 0.43, 0.56);
const TEXT_COLOR: Color = Color::srgb(0.91, 0.88, 0.80);
const MUTED_COLOR: Color = Color::srgb(0.55, 0.57, 0.62); // 提示文字的灰

// ANCHOR: states
/// 全场只有三种活法：后台待客、一局进行中、一局收场
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

/// 一局的结论——闭幕的瞬间写下，结算屏（和下一节的锣鼓）都看它
#[derive(Resource, Clone, Copy)]
enum Outcome {
    /// 瓦砸完了
    Cleared,
    /// 绣球用尽
    Spilled,
}
// ANCHOR_END: states

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

// ANCHOR: glued
/// 粘在凳上待发的球
#[derive(Component)]
struct Glued;

/// 还剩几只绣球
#[derive(Resource, Default)]
struct Lives(u32);

/// 球的铸模：圆网格与红材质的提货单，补球时直接克隆（第 14 章的省钱诀窍）
#[derive(Resource)]
struct BallStock {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
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
// ANCHOR_END: glued

#[derive(Message)]
enum Knock {
    Wall,
    Paddle,
    Crack,
    Shatter,
    /// 绣球落了沟
    Gutter,
}

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Component)]
struct ScoreBoard;

/// 右上角的命数牌
#[derive(Component)]
struct LivesBoard;

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

fn refresh_scoreboard(score: Res<Score>, mut board: Single<&mut Text2d, With<ScoreBoard>>) {
    board.0 = format!("瓦 {}/{}", score.0, TOTAL_BRICKS);
}

fn refresh_lives(lives: Res<Lives>, mut board: Single<&mut Text2d, With<LivesBoard>>) {
    board.0 = format!("绣球 ×{}", lives.0);
}

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

// ANCHOR: intent
/// 意图层添了一项瞬时意图：发球
#[derive(Resource, Default)]
struct Intent {
    steer: f32,
    /// 这一拍要不要发球——收集系统攒着，鼓点上消费（18.5 的账）
    serve: bool,
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
    if keyboard.just_pressed(KeyCode::Space) {
        intent.serve = true; // 攒着，等鼓点来收
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
// ANCHOR_END: intent

fn move_paddle(
    intent: Res<Intent>,
    time: Res<Time>,
    mut paddle: Single<&mut Transform, With<Paddle>>,
) {
    let reach = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_MARGIN;
    let next = paddle.translation.x + intent.steer * PADDLE_SPEED * time.delta_secs();
    paddle.translation.x = next.clamp(-reach, reach);
}

// ANCHOR: serve
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
    let pressed = std::mem::take(&mut intent.serve); // 消费即清零
    if !pressed {
        return;
    }
    for (entity, mut velocity) in &mut glued {
        velocity.0 = SERVE_DIRECTION.normalize() * BALL_SPEED;
        commands.entity(entity).remove::<Glued>();
    }
}
// ANCHOR_END: serve

fn apply_velocity(time: Res<Time>, mut movers: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut movers {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

// ANCHOR: settle
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
// ANCHOR_END: settle

// ANCHOR: screens
/// 后台招牌：进 Menu 搭，出 Menu 引擎拆——DespawnOnExit 挂在根上，整棵树托管
fn show_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");
    let regular = asset_server.load("fonts/book-sans-sc-regular.otf");
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Transform::default(),
        Visibility::default(),
        children![
            (
                Text2d::new("打　瓦"),
                TextFont {
                    font: bold.into(),
                    font_size: FontSize::Px(110.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Transform::from_xyz(0.0, 110.0, 5.0),
            ),
            (
                Text2d::new("夜戏散场后的保留节目"),
                TextFont {
                    font: regular.clone().into(),
                    font_size: FontSize::Px(26.0),
                    ..default()
                },
                TextColor(MUTED_COLOR),
                Transform::from_xyz(0.0, 28.0, 5.0),
            ),
            (
                Text2d::new("空格 开局　　Esc 离场"),
                TextFont {
                    font: regular.into(),
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Transform::from_xyz(0.0, -96.0, 5.0),
            ),
        ],
    ));
}

/// 结算屏：标题看 Outcome，分数行读 Score
fn show_curtain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    outcome: Res<Outcome>,
    score: Res<Score>,
) {
    let (headline, verdict) = match *outcome {
        Outcome::Cleared => ("满堂彩！", format!("{TOTAL_BRICKS} 片瓦，一片不剩")),
        Outcome::Spilled => ("绣球散尽", format!("这局砸下 {} 片瓦", score.0)),
    };
    println!("场记：{headline}——{verdict}。");
    commands.spawn((
        DespawnOnExit(GameState::GameOver),
        Transform::default(),
        Visibility::default(),
        children![
            (
                Text2d::new(headline),
                TextFont {
                    font: asset_server.load("fonts/book-sans-sc-bold.otf").into(),
                    font_size: FontSize::Px(84.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Transform::from_xyz(0.0, 90.0, 5.0),
            ),
            (
                Text2d::new(verdict),
                TextFont {
                    font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                TextColor(MUTED_COLOR),
                Transform::from_xyz(0.0, 10.0, 5.0),
            ),
            (
                Text2d::new("空格 再来一局　　Esc 回后台"),
                TextFont {
                    font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Transform::from_xyz(0.0, -96.0, 5.0),
            ),
        ],
    ));
}
// ANCHOR_END: screens

// ANCHOR: keys
/// 后台的键：空格开局，Esc 离场
fn menu_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("老雷：收摊。各回各屋。");
        exit.write(AppExit::Success);
    }
}

/// 结算屏的键：空格再来，Esc 回后台
fn curtain_keys(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
// ANCHOR_END: keys

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
        .init_state::<GameState>()
        .init_resource::<Intent>()
        .init_resource::<Score>()
        .init_resource::<Lives>()
        .add_message::<Knock>()
        .add_systems(Startup, (rig_camera, greet))
        .add_systems(OnEnter(GameState::Menu), show_menu)
        .add_systems(OnEnter(GameState::Playing), (setup_court, rig_scoreboard))
        .add_systems(OnEnter(GameState::GameOver), show_curtain)
        .add_systems(
            RunFixedMainLoop,
            collect_intent
                .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop)
                .run_if(in_state(GameState::Playing)),
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
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                menu_keys.run_if(in_state(GameState::Menu)),
                curtain_keys.run_if(in_state(GameState::GameOver)),
                (
                    tally,
                    refresh_scoreboard.run_if(resource_changed::<Score>),
                    refresh_lives.run_if(resource_changed::<Lives>),
                )
                    .chain(),
            ),
        )
        .run();
}
// ANCHOR_END: main

fn rig_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn greet() {
    println!("老雷：夜戏散了，伙计们后台耍一局《打瓦》——空格开局。");
}

// ANCHOR: rig_scoreboard
/// 记分牌与命数牌：进局挂出来，离开 Playing 由引擎收走
fn rig_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    score.0 = 0; // 新的一局，旧账清零
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");
    commands.spawn((
        ScoreBoard,
        DespawnOnExit(GameState::Playing),
        Text2d::new(format!("瓦 0/{TOTAL_BRICKS}")),
        TextFont {
            font: bold.clone().into(),
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(TEXT_COLOR),
        Anchor::CENTER_LEFT,
        Transform::from_xyz(LEFT_WALL - WALL_THICKNESS / 2.0, BOARD_Y, 5.0),
    ));
    commands.spawn((
        LivesBoard,
        DespawnOnExit(GameState::Playing),
        Text2d::new(format!("绣球 ×{BALL_COUNT}")),
        TextFont {
            font: bold.into(),
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(TEXT_COLOR),
        Anchor::CENTER_RIGHT,
        Transform::from_xyz(RIGHT_WALL + WALL_THICKNESS / 2.0, BOARD_Y, 5.0),
    ));
    commands.spawn((
        DespawnOnExit(GameState::Playing),
        Text2d::new("A/D 推凳　　空格 发球"),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(MUTED_COLOR),
        Transform::from_xyz(0.0, BOARD_Y, 5.0),
    ));
}
// ANCHOR_END: rig_scoreboard

// ANCHOR: setup_court
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

    // 球的铸模存起来，落沟补球时再用
    let stock = BallStock {
        mesh: meshes.add(Circle::new(BALL_RADIUS)),
        material: materials.add(BALL_COLOR),
    };
    spawn_ball(&mut commands, &stock);
    commands.insert_resource(stock);

    println!("场记：开台——{TOTAL_BRICKS} 片瓦，{BALL_COUNT} 只绣球。");
}
// ANCHOR_END: setup_court

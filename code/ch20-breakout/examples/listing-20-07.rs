//! Listing 20-7：锣鼓与中场——音效、BGM、暂停子状态，游戏全须全尾
//! 在 Listing 20-6 之上：IsPaused 子状态（P 中场/继续，Esc 收摊）+
//! Knock 消息驱动的一次性音效 + 循环 BGM + GlobalVolume 总闸（-/= 调）。
//! 这也是单文件的最后一站：560 行——下一节拆插件。

use bevy::audio::Volume;
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;
use bevy::sprite::Anchor;

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
const BALL_COUNT: u32 = 3;

const BRICK_COLUMNS: usize = 8;
const BRICK_ROWS: usize = 7;
const BRICK_SIZE: Vec2 = Vec2::new(92.0, 26.0);
const BRICK_GAP: f32 = 8.0;
const BRICK_TOP_Y: f32 = 212.0;
const GLAZED_ROWS: usize = 2;
const TOTAL_BRICKS: u32 = (BRICK_COLUMNS * BRICK_ROWS) as u32;

const BOARD_Y: f32 = 302.0;

/// BGM 自己的基准音量（总闸另算）
const BGM_LEVEL: f32 = 0.45;

const BACKDROP: Color = Color::srgb(0.10, 0.10, 0.14);
const WALL_COLOR: Color = Color::srgb(0.38, 0.32, 0.27);
const PADDLE_COLOR: Color = Color::srgb(0.84, 0.65, 0.37);
const BALL_COLOR: Color = Color::srgb(0.92, 0.36, 0.32);
const TILE_COLOR: Color = Color::srgb(0.52, 0.62, 0.66);
const GLAZED_COLOR: Color = Color::srgb(0.27, 0.43, 0.56);
const TEXT_COLOR: Color = Color::srgb(0.91, 0.88, 0.80);
const MUTED_COLOR: Color = Color::srgb(0.55, 0.57, 0.62);

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

// ANCHOR: paused
/// 中场：只在一局进行中才存在的小状态机（第 10 章的 SubStates）
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::Playing)]
enum IsPaused {
    #[default]
    Running,
    Paused,
}
// ANCHOR_END: paused

#[derive(Resource, Clone, Copy)]
enum Outcome {
    Cleared,
    Spilled,
}

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

#[derive(Component)]
struct Glued;

#[derive(Resource, Default)]
struct Lives(u32);

#[derive(Resource)]
struct BallStock {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

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

#[derive(Message)]
enum Knock {
    Wall,
    Paddle,
    Crack,
    Shatter,
    Gutter,
}

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Component)]
struct ScoreBoard;

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

#[derive(Resource, Default)]
struct Intent {
    steer: f32,
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

fn move_paddle(
    intent: Res<Intent>,
    time: Res<Time>,
    mut paddle: Single<&mut Transform, With<Paddle>>,
) {
    let reach = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_MARGIN;
    let next = paddle.translation.x + intent.steer * PADDLE_SPEED * time.delta_secs();
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

// ANCHOR: intermission_screen
/// 中场幕布：进 Paused 搭，出 Paused 引擎拆
fn show_intermission(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        DespawnOnExit(IsPaused::Paused),
        Transform::default(),
        Visibility::default(),
        children![
            (
                Text2d::new("中　场"),
                TextFont {
                    font: asset_server.load("fonts/book-sans-sc-bold.otf").into(),
                    font_size: FontSize::Px(84.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Transform::from_xyz(0.0, 40.0, 10.0),
            ),
            (
                Text2d::new("P 继续　　Esc 收摊回后台"),
                TextFont {
                    font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                TextColor(MUTED_COLOR),
                Transform::from_xyz(0.0, -60.0, 10.0),
            ),
        ],
    ));
}
// ANCHOR_END: intermission_screen

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

fn curtain_keys(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

// ANCHOR: pause_keys
/// P：中场与开演来回切
fn pause_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    paused: Res<State<IsPaused>>,
    mut next_paused: ResMut<NextState<IsPaused>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        next_paused.set(match paused.get() {
            IsPaused::Running => IsPaused::Paused,
            IsPaused::Paused => IsPaused::Running,
        });
    }
}

/// 中场里按 Esc：弃局回后台
fn quit_from_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("场记：这局不打了，瓦留给明儿个。");
        next_state.set(GameState::Menu);
    }
}

/// 中场的另一半（18.2 的戏台钟）：钟也得停
fn hold_clock(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn release_clock(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}
// ANCHOR_END: pause_keys

// ---------------------------------------------------------------- 锣鼓

// ANCHOR: sound_bank
/// 武场的家伙什：全部音效的提货单
#[derive(Resource)]
struct SoundBank {
    clack: Handle<AudioSource>,
    shatter: Handle<AudioSource>,
    drum: Handle<AudioSource>,
    win: Handle<AudioSource>,
    lose: Handle<AudioSource>,
}

/// 循环 BGM 的标记——总闸调音量时要找到它
#[derive(Component)]
struct Bgm;

/// 开张就位：备好提货单，序曲循环起播
fn setup_band(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundBank {
        clack: asset_server.load("sfx/clack.wav"),
        shatter: asset_server.load("sfx/shatter.wav"),
        drum: asset_server.load("sfx/drum.wav"),
        win: asset_server.load("sfx/win.wav"),
        lose: asset_server.load("sfx/lose.wav"),
    });
    commands.spawn((
        Bgm,
        AudioPlayer::new(asset_server.load("music/changfeng-overture.wav")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(BGM_LEVEL)),
    ));
}
// ANCHOR_END: sound_bank

// ANCHOR: play_knocks
/// 武场只听 Knock：一声动静一个 DESPAWN 实体（19.2 的标准答案）。
/// clack 一份素材三种用场——速度即音高，凳高墙平瓦裂闷
fn play_knocks(mut knocks: MessageReader<Knock>, bank: Res<SoundBank>, mut commands: Commands) {
    for knock in knocks.read() {
        let (source, speed) = match knock {
            Knock::Wall => (&bank.clack, 1.0),
            Knock::Paddle => (&bank.clack, 1.3),
            Knock::Crack => (&bank.clack, 0.6),
            Knock::Shatter => (&bank.shatter, 1.0),
            Knock::Gutter => (&bank.drum, 1.0),
        };
        commands.spawn((
            AudioPlayer::new(source.clone()),
            PlaybackSettings::DESPAWN.with_speed(speed),
        ));
    }
}

/// 闭幕一声定音：满堂彩上行，绣球散尽下行
fn verdict_sting(outcome: Res<Outcome>, bank: Res<SoundBank>, mut commands: Commands) {
    let source = match *outcome {
        Outcome::Cleared => &bank.win,
        Outcome::Spilled => &bank.lose,
    };
    commands.spawn((AudioPlayer::new(source.clone()), PlaybackSettings::DESPAWN));
}
// ANCHOR_END: play_knocks

// ANCHOR: sinks_and_dial
/// 中场协议的声卡侧（19.3 的教训）：戏台钟管不到声卡，sink 的闸自己拧
fn hold_sinks(sinks: Query<&AudioSink>) {
    sinks.iter().for_each(AudioSink::pause);
}

fn release_sinks(sinks: Query<&AudioSink>) {
    sinks.iter().for_each(AudioSink::play);
}

/// 总闸：-/= 拧 GlobalVolume——新开播的声音自动吃到新值
fn master_dial(keyboard: Res<ButtonInput<KeyCode>>, mut master: ResMut<GlobalVolume>) {
    let step = i32::from(keyboard.just_pressed(KeyCode::Equal))
        - i32::from(keyboard.just_pressed(KeyCode::Minus));
    if step != 0 {
        let turned = (master.volume.to_linear() + step as f32 * 0.1).clamp(0.0, 1.0);
        master.volume = Volume::Linear(turned);
        println!("老雷：总闸拧到 {turned:.1}。");
    }
}

/// 19.4 的坑：总闸不管已经在播的——BGM 这种长寿声音得自己补一遍
fn apply_master(master: Res<GlobalVolume>, mut bgm: Query<&mut AudioSink, With<Bgm>>) {
    for mut sink in &mut bgm {
        sink.set_volume(Volume::Linear(BGM_LEVEL) * master.volume);
    }
}
// ANCHOR_END: sinks_and_dial

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
        .add_sub_state::<IsPaused>()
        .init_resource::<Intent>()
        .init_resource::<Score>()
        .init_resource::<Lives>()
        .add_message::<Knock>()
        .add_systems(Startup, (rig_camera, setup_band, greet))
        .add_systems(OnEnter(GameState::Menu), show_menu)
        .add_systems(OnEnter(GameState::Playing), (setup_court, rig_scoreboard))
        .add_systems(OnEnter(GameState::GameOver), (show_curtain, verdict_sting))
        .add_systems(
            OnEnter(IsPaused::Paused),
            (show_intermission, hold_clock, hold_sinks),
        )
        .add_systems(OnExit(IsPaused::Paused), (release_clock, release_sinks))
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
        )
        .add_systems(
            Update,
            (
                menu_keys.run_if(in_state(GameState::Menu)),
                curtain_keys.run_if(in_state(GameState::GameOver)),
                pause_keys.run_if(in_state(GameState::Playing)),
                quit_from_pause.run_if(in_state(IsPaused::Paused)),
                (
                    tally,
                    refresh_scoreboard.run_if(resource_changed::<Score>),
                    refresh_lives.run_if(resource_changed::<Lives>),
                )
                    .chain(),
                play_knocks,
                master_dial,
                apply_master.run_if(resource_changed::<GlobalVolume>),
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

fn rig_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    score.0 = 0;
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
        Text2d::new("A/D 推凳　　空格 发球　　P 中场"),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(MUTED_COLOR),
        Transform::from_xyz(0.0, BOARD_Y, 5.0),
    ));
}

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

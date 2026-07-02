//! Listing 17-9：体验场《来者不拒》——键盘、鼠标、手柄、触屏使唤同一个阿燕
//! 所有设备只负责把“意图”写进 Intent 资源；走位、出剑、动画各干各的，
//! 谁也不认识键盘长什么样。改键、加设备，都只动“听”的那一半。

use bevy::input::common_conditions::input_just_pressed;
use bevy::input::gamepad::{
    GamepadConnection, GamepadConnectionEvent, GamepadRumbleIntensity, GamepadRumbleRequest,
};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::time::Duration;

const FLOOR: f32 = -180.0;
const STAGE_HALF: f32 = 560.0;
const WALK_SPEED: f32 = 240.0;
const DASH_SPEED: f32 = 340.0;
/// 木人桩的站位与出剑够得着的距离
const DUMMY_X: f32 = 380.0;
const REACH: f32 = 150.0;
/// 自留的摇杆死区（17.5 节的账）
const STICK_DEADZONE: f32 = 0.15;

// ANCHOR: intent
/// 哪路看客在发话——只影响通报与 HUD，不影响动作
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Source {
    Keyboard,
    Mouse,
    Gamepad,
    Touch,
}

impl Source {
    fn label(self) -> &'static str {
        match self {
            Source::Keyboard => "键盘",
            Source::Mouse => "鼠标",
            Source::Gamepad => "手柄",
            Source::Touch => "触屏",
        }
    }
}

/// 意图层：设备们往里写“想做什么”，动作系统从里读
/// walk 与 strike 是瞬时意图，每帧清空重填；dash_to 是持续意图，到站才销
#[derive(Resource, Default)]
struct Intent {
    walk: f32,
    strike: bool,
    dash_to: Option<f32>,
    source: Option<Source>,
}
// ANCHOR_END: intent

/// 阿燕：朝向与这一帧是否真的在走（动画用）
#[derive(Component)]
struct Ayan {
    facing: f32,
    moving: bool,
}

/// 帧动画的节拍器
#[derive(Component)]
struct FrameClock(Timer);

/// 标记：令旗 / 木人桩 / HUD 文本根
#[derive(Component)]
struct Flag;
#[derive(Component)]
struct Dummy {
    wobble: Timer,
}
#[derive(Component)]
struct Hud;

/// 剑光：一闪即逝
#[derive(Component)]
struct SlashGlow {
    life: Timer,
}

/// 记分：中桩多少记
#[derive(Resource, Default)]
struct Hits(u32);

// ANCHOR: sets
/// 先听后动：所有设备读完，动作系统才开工
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum BoothSystems {
    Listen,
    Act,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .init_resource::<Intent>()
        .init_resource::<Hits>()
        .configure_sets(
            Update,
            (BoothSystems::Listen, BoothSystems::Act).chain(),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                begin,
                (read_keyboard, read_mouse, read_gamepad, read_touch),
            )
                .chain()
                .in_set(BoothSystems::Listen),
        )
        .add_systems(
            Update,
            (
                (act_move, act_strike),
                (animate, fade_slash, wobble_dummy, hud, door),
                exit_booth.run_if(input_just_pressed(KeyCode::Escape)),
            )
                .chain()
                .in_set(BoothSystems::Act),
        )
        .run();
}
// ANCHOR_END: sets

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    let regular = asset_server.load("fonts/book-sans-sc-regular.otf");
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");

    // 台面与木人桩
    commands.spawn((
        Sprite {
            image: asset_server.load("props/dock-plank.png"),
            custom_size: Some(Vec2::new(1400.0, 56.0)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 4.0,
            },
            ..default()
        },
        Transform::from_xyz(0.0, FLOOR - 28.0, 0.0),
    ));
    commands.spawn((
        Dummy {
            wobble: Timer::from_seconds(0.25, TimerMode::Once),
        },
        Sprite::from_image(asset_server.load("props/wooden-dummy.png")),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(DUMMY_X, FLOOR, 2.0).with_scale(Vec3::splat(4.0)),
    ));

    // 阿燕：十二格连环画整本带上——前六格养神，后六格走路
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        6,
        2,
        None,
        None,
    ));
    commands.spawn((
        Ayan {
            facing: 1.0,
            moving: false,
        },
        FrameClock(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Sprite::from_atlas_image(
            asset_server.load("actors/ayan-sheet.png"),
            TextureAtlas { layout, index: 0 },
        ),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(-150.0, FLOOR, 3.0).with_scale(Vec3::splat(4.0)),
    ));

    // 令旗
    commands.spawn((
        Flag,
        Sprite::from_color(Color::srgb(0.95, 0.78, 0.22), Vec2::splat(18.0)),
        Transform::from_xyz(0.0, FLOOR + 9.0, 0.5)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
        Visibility::Hidden,
    ));

    // HUD：正在听谁的 + 中桩计数（第 16 章的富文本）
    commands.spawn((
        Hud,
        Text2d::new("听差："),
        TextFont {
            font: bold.clone().into(),
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.72, 0.29)),
        Anchor::CENTER_LEFT,
        Transform::from_xyz(-600.0, 300.0, 5.0),
        children![
            (
                TextSpan::new("虚位以待"),
                TextFont {
                    font: bold.into(),
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                TextColor(Color::srgb(0.55, 0.57, 0.62)),
            ),
            (
                TextSpan::new("　中桩 ×0"),
                TextFont {
                    font: regular.into(),
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.55, 0.57, 0.62)),
            ),
        ],
    ));

    println!("老雷：体验场《来者不拒》开张——键盘鼠标手柄触屏，哪路看客都接。");
    println!("场记：A/D 走，空格出剑；点台面插旗；摇杆南键同理。Esc 散场。");
}

/// 开帧清账：瞬时意图归零，持续意图（令旗）留着
fn begin(mut intent: ResMut<Intent>) {
    intent.walk = 0.0;
    intent.strike = false;
}

// ANCHOR: read_keyboard
/// 键盘：A/D 与左右箭头写 walk，空格写 strike
fn read_keyboard(keyboard: Res<ButtonInput<KeyCode>>, mut intent: ResMut<Intent>) {
    let mut walk = 0.0;
    if keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        walk -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        walk += 1.0;
    }
    if walk != 0.0 {
        intent.walk = walk;
        intent.source = Some(Source::Keyboard);
    }
    if keyboard.just_pressed(KeyCode::Space) {
        intent.strike = true;
        intent.source = Some(Source::Keyboard);
    }
}
// ANCHOR_END: read_keyboard

/// 鼠标：左键点到哪，旗插到哪
fn read_mouse(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut flag: Single<(&mut Transform, &mut Visibility), With<Flag>>,
    mut intent: ResMut<Intent>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let (camera, camera_transform) = *camera;
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor) else {
        return;
    };
    let (flag_transform, flag_visibility) = &mut *flag;
    flag_transform.translation = point.extend(0.5);
    **flag_visibility = Visibility::Visible;
    intent.dash_to = Some(point.x.clamp(-STAGE_HALF, STAGE_HALF));
    intent.source = Some(Source::Mouse);
}

// ANCHOR: read_gamepad
/// 手柄：摇杆/十字键写 walk，南键写 strike——出剑命中另有一阵震动犒赏
fn read_gamepad(
    gamepads: Query<(Entity, &Gamepad)>,
    mut intent: ResMut<Intent>,
    mut rumbles: MessageWriter<GamepadRumbleRequest>,
) {
    for (entity, gamepad) in &gamepads {
        let stick = gamepad.left_stick().x;
        let stick = if stick.abs() > STICK_DEADZONE { stick } else { 0.0 };
        let push = (stick + gamepad.dpad().x).clamp(-1.0, 1.0);
        if push != 0.0 {
            intent.walk = push; // 模拟量原样交上去：半推半速
            intent.source = Some(Source::Gamepad);
        }
        if gamepad.just_pressed(GamepadButton::South) {
            intent.strike = true;
            intent.source = Some(Source::Gamepad);
            rumbles.write(GamepadRumbleRequest::Add {
                duration: Duration::from_millis(300),
                intensity: GamepadRumbleIntensity::strong_motor(0.6),
                gamepad: entity,
            });
        }
    }
}
// ANCHOR_END: read_gamepad

/// 触屏：落指即插旗，和鼠标走同一条反算路
fn read_touch(
    touches: Res<Touches>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut flag: Single<(&mut Transform, &mut Visibility), With<Flag>>,
    mut intent: ResMut<Intent>,
) {
    let (camera, camera_transform) = *camera;
    for touch in touches.iter_just_pressed() {
        let Ok(point) = camera.viewport_to_world_2d(camera_transform, touch.position()) else {
            continue;
        };
        let (flag_transform, flag_visibility) = &mut *flag;
        flag_transform.translation = point.extend(0.5);
        **flag_visibility = Visibility::Visible;
        intent.dash_to = Some(point.x.clamp(-STAGE_HALF, STAGE_HALF));
        intent.source = Some(Source::Touch);
    }
}

// ANCHOR: act_move
/// 走位只认 Intent：手上的 walk 优先，令旗其次——谁写进来的，它不问
fn act_move(
    time: Res<Time>,
    mut intent: ResMut<Intent>,
    mut ayan: Single<(&mut Transform, &mut Ayan)>,
    mut flag_visibility: Single<&mut Visibility, (With<Flag>, Without<Ayan>)>,
) {
    let (transform, ayan) = &mut *ayan;
    ayan.moving = false;

    if intent.walk != 0.0 {
        // 亲手走位夺回指挥权：撤旗
        if intent.dash_to.take().is_some() {
            **flag_visibility = Visibility::Hidden;
        }
        transform.translation.x = (transform.translation.x
            + intent.walk * WALK_SPEED * time.delta_secs())
        .clamp(-STAGE_HALF, STAGE_HALF);
        ayan.facing = intent.walk.signum();
        ayan.moving = true;
    } else if let Some(goal_x) = intent.dash_to {
        let step = DASH_SPEED * time.delta_secs();
        let distance = goal_x - transform.translation.x;
        ayan.facing = distance.signum();
        if distance.abs() <= step {
            transform.translation.x = goal_x;
            intent.dash_to = None;
            **flag_visibility = Visibility::Hidden;
        } else {
            transform.translation.x += step * distance.signum();
            ayan.moving = true;
        }
    }
}
// ANCHOR_END: act_move

/// 出剑也只认 Intent：剑光一闪，够得着木桩就记一记
fn act_strike(
    intent: Res<Intent>,
    mut hits: ResMut<Hits>,
    mut commands: Commands,
    ayan: Single<(&Transform, &Ayan)>,
    mut dummy: Single<&mut Dummy>,
) {
    if !intent.strike {
        return;
    }
    let (transform, ayan) = *ayan;

    // 剑光：朝向一侧斜出的一道白
    commands.spawn((
        SlashGlow {
            life: Timer::from_seconds(0.18, TimerMode::Once),
        },
        Sprite::from_color(Color::srgb(0.92, 0.96, 1.0), Vec2::new(110.0, 10.0)),
        Transform::from_xyz(
            transform.translation.x + ayan.facing * 95.0,
            FLOOR + 110.0,
            4.0,
        )
        .with_rotation(Quat::from_rotation_z(ayan.facing * -0.5)),
    ));

    // 够得着、且面朝木桩，才算中桩
    let offset = DUMMY_X - transform.translation.x;
    if offset.abs() <= REACH && offset.signum() == ayan.facing {
        hits.0 += 1;
        dummy.wobble.reset();
        match hits.0 {
            1 => println!("阿燕：头一记，开张。"),
            n if n % 5 == 0 => println!("场记：中桩 {n} 记——这位看客是练家子。"),
            _ => {}
        }
    }
}

/// 帧动画：走路用后六格，养神用前六格；朝向翻 flip_x
fn animate(time: Res<Time>, mut ayan: Single<(&mut Sprite, &mut FrameClock, &Ayan)>) {
    let (sprite, clock, ayan) = &mut *ayan;
    let Some(atlas) = sprite.texture_atlas.as_mut() else {
        return;
    };
    let (start, end) = if ayan.moving { (6, 11) } else { (0, 5) };
    if !(start..=end).contains(&atlas.index) {
        atlas.index = start;
    }
    if clock.0.tick(time.delta()).just_finished() {
        atlas.index = if atlas.index >= end {
            start
        } else {
            atlas.index + 1
        };
    }
    sprite.flip_x = ayan.facing < 0.0;
}

/// 剑光转瞬即逝
fn fade_slash(
    time: Res<Time>,
    mut glows: Query<(Entity, &mut SlashGlow, &mut Sprite)>,
    mut commands: Commands,
) {
    for (entity, mut glow, mut sprite) in &mut glows {
        glow.life.tick(time.delta());
        sprite.color.set_alpha(1.0 - glow.life.fraction());
        if glow.life.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// 挨打的木桩晃两下（第 16 章的老相识）
fn wobble_dummy(time: Res<Time>, mut dummy: Single<(&mut Dummy, &mut Transform)>) {
    let (dummy, transform) = &mut *dummy;
    dummy.wobble.tick(time.delta());
    let strength = 1.0 - dummy.wobble.fraction();
    transform.rotation =
        Quat::from_rotation_z(0.05 * strength * (time.elapsed_secs() * 40.0).sin());
}

// ANCHOR: hud
/// HUD：换了设备就改“听差”，中了桩就改计数；头回上手的设备登记通报
fn hud(
    intent: Res<Intent>,
    hits: Res<Hits>,
    board: Single<Entity, With<Hud>>,
    mut writer: Text2dWriter,
    mut shown: Local<(Option<Source>, u32)>,
    mut greeted: Local<Vec<Source>>,
) {
    if *shown == (intent.source, hits.0) {
        return;
    }
    *shown = (intent.source, hits.0);

    if let Some(source) = intent.source {
        *writer.text(*board, 1) = source.label().to_string();
        writer.color(*board, 1).0 = Color::srgb(0.92, 0.93, 0.96);
        if !greeted.contains(&source) {
            greeted.push(source);
            println!("场记：{}看客上手了。", source.label());
        }
    }
    *writer.text(*board, 2) = format!("　中桩 ×{}", hits.0);
}
// ANCHOR_END: hud

/// 门房：手柄进出场通报（与 Listing 17-6 同款）
fn door(mut connections: MessageReader<GamepadConnectionEvent>) {
    for event in connections.read() {
        match &event.connection {
            GamepadConnection::Connected { name, .. } => {
                println!("场记：手柄看客进场——“{name}”。");
            }
            GamepadConnection::Disconnected => {
                println!("场记：手柄 {} 拔线离场。", event.gamepad);
            }
        }
    }
}

/// 散场：Esc 一声令下，App 退出（第 7 章的 AppExit）
fn exit_booth(mut exit: MessageWriter<AppExit>) {
    println!("老雷：散场。今儿的看客都伺候好了。");
    exit.write(AppExit::Success);
}

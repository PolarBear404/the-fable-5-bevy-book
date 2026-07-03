//! Listing 18-6：收官加彩——「过一会儿再办」的活计交给延迟命令
//! 开演前三息定场（延迟 spawn＋预约撤牌＋延迟放行）；中桩弹「中！」字，0.6 秒自撤。
//! 空格出手（冷却 0.8 秒照旧是 Timer），P 中场/开戏——驿站也归戏台钟管。

use bevy::prelude::*;
use bevy::sprite::Anchor;

const FLOOR: f32 = -200.0;
const AYAN_X: f32 = -380.0;
const DUMMY_X: f32 = 380.0;
const DART_SPEED: f32 = 900.0;

/// 出手的硬直：反复用、要报余数——这种表还得自己养（Timer）
#[derive(Resource)]
struct Cooldown(Timer);

impl Default for Cooldown {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(0.8, TimerMode::Once);
        timer.finish(); // 开演第一手就是稳的
        Self(timer)
    }
}

/// 开演令牌：三息之后才由驿站送进 World
#[derive(Resource)]
struct CurtainUp;

/// 定场牌：牌面即台词，落牌那一帧由场记喊出来
#[derive(Component)]
struct StageCall(&'static str);

/// 飞着的袖箭
#[derive(Component)]
struct Dart;

/// 挨打的木桩：晃劲要逐帧衰减——要过程的活，还是 Timer 的
#[derive(Component)]
struct Dummy {
    wobble: Timer,
}

/// 中桩数
#[derive(Resource, Default)]
struct Hits(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .init_resource::<Cooldown>()
        .init_resource::<Hits>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (announce, curtain_call.run_if(resource_added::<CurtainUp>)).chain(),
                tick_cooldown,
                throw.run_if(resource_exists::<CurtainUp>),
                fly,
                wobble_dummy,
                intermission,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");
    let regular = asset_server.load("fonts/book-sans-sc-regular.otf");

    // 台面、阿燕（站桩帧）、木人桩——与《连珠箭》同一套班底
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
        Sprite {
            image: asset_server.load("actors/ayan-sheet.png"),
            rect: Some(Rect::new(0.0, 0.0, 32.0, 40.0)),
            ..default()
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(AYAN_X, FLOOR, 3.0).with_scale(Vec3::splat(4.0)),
    ));
    commands.spawn((
        Dummy {
            wobble: Timer::from_seconds(0.25, TimerMode::Once),
        },
        Sprite::from_image(asset_server.load("props/wooden-dummy.png")),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(DUMMY_X, FLOOR, 2.0).with_scale(Vec3::splat(4.0)),
    ));
    commands.spawn((
        Text2d::new("空格出手　　P 中场/开戏"),
        TextFont {
            font: regular.into(),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(0.55, 0.57, 0.62)),
        Transform::from_xyz(0.0, 320.0, 5.0),
    ));

    // ANCHOR: curtain
    // 定场三息：三块牌各在第 1、2、3 秒落下，各挂 0.8 秒；三息一到，戏就放行。
    // 全部只是“排单”——setup 本身一瞬就跑完了，此后没有任何系统再管这些牌。
    let mut delayed = commands.delayed();
    for (i, call) in ["一息——", "两息——", "三息——"].into_iter().enumerate() {
        let at = 1.0 + i as f32;
        let sign = delayed
            .secs(at)
            .spawn((
                StageCall(call),
                Text2d::new(call),
                TextFont {
                    font: bold.clone().into(),
                    font_size: FontSize::Px(72.0),
                    ..default()
                },
                TextColor(Color::srgb(0.91, 0.72, 0.29)),
                Transform::from_xyz(0.0, 120.0, 8.0),
            ))
            .id(); // 牌还没落，号已经领好
        delayed.secs(at + 0.8).entity(sign).despawn(); // 出生之前，后事已经预约
    }
    delayed.secs(3.0).insert_resource(CurtainUp);
    // ANCHOR_END: curtain

    println!("老雷：《连珠箭》收官加彩——三息定场，中桩报「中」。空格出手，P 中场。");
}

// ANCHOR: announce
/// 落牌即喊话：Added 过滤器（第 4 章）逮住驿站刚送到的牌，顺带给两只钟对表
fn announce(calls: Query<&StageCall, Added<StageCall>>, real: Res<Time<Real>>, stage: Res<Time>) {
    for call in &calls {
        println!(
            "场记：{}（怀表 {:.1} 秒，戏台钟 {:.1} 秒）",
            call.0,
            real.elapsed_secs(),
            stage.elapsed_secs()
        );
    }
}

/// 令牌进门那一帧喊开演——resource_added 只在插入的当帧为真
fn curtain_call() {
    println!("老雷：开演——手都亮出来！");
}
// ANCHOR_END: announce

/// 冷却表照旧每帧喂——这只表要反复 reset、要报余数，delayed 替不了它
fn tick_cooldown(time: Res<Time>, mut cooldown: ResMut<Cooldown>) {
    cooldown.0.tick(time.delta());
}

/// 出手两问：缓过劲了吗（Timer 的活）；开没开演由 run_if 把门
fn throw(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cooldown: ResMut<Cooldown>,
    mut commands: Commands,
    mut thrown: Local<u32>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    if !cooldown.0.is_finished() {
        println!(
            "阿燕：手上还没缓过劲——还差 {:.1} 秒。",
            cooldown.0.remaining_secs()
        );
        return;
    }
    cooldown.0.reset();
    commands.spawn((
        Dart,
        Sprite::from_color(Color::srgb(0.92, 0.96, 1.0), Vec2::new(30.0, 6.0)),
        Transform::from_xyz(AYAN_X + 70.0, FLOOR + 110.0, 4.0),
    ));
    *thrown += 1;
    if *thrown == 1 {
        println!("阿燕：看箭。");
    }
}

// ANCHOR: hit
/// 袖箭中桩：弹一个「中！」字，撤牌的事当场交给驿站——不养组件，不写清扫系统
fn fly(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut darts: Query<(Entity, &mut Transform), With<Dart>>,
    mut dummy: Single<&mut Dummy>,
    mut hits: ResMut<Hits>,
    mut commands: Commands,
) {
    for (entity, mut transform) in &mut darts {
        transform.translation.x += DART_SPEED * time.delta_secs();
        if transform.translation.x >= DUMMY_X - 40.0 {
            commands.entity(entity).despawn();
            dummy.wobble.reset();
            hits.0 += 1;
            if hits.0 == 1 {
                println!("场记：中桩头一记——「中」字挂 0.6 秒，到点自己撤。");
            }
            let sign = commands
                .spawn((
                    Text2d::new("中！"),
                    TextFont {
                        font: asset_server.load("fonts/book-sans-sc-bold.otf").into(),
                        font_size: FontSize::Px(48.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.53, 0.32)),
                    Transform::from_xyz(DUMMY_X, FLOOR + 300.0, 7.0),
                ))
                .id();
            commands.delayed().secs(0.6).entity(sign).despawn();
        }
    }
}
// ANCHOR_END: hit

/// 木桩的余韵（与《连珠箭》同款）：每帧按 fraction 衰减——要过程的活，还是 Timer 的
fn wobble_dummy(time: Res<Time>, mut dummy: Single<(&mut Dummy, &mut Transform)>) {
    let (dummy, transform) = &mut *dummy;
    dummy.wobble.tick(time.delta());
    let strength = 1.0 - dummy.wobble.fraction();
    transform.rotation =
        Quat::from_rotation_z(0.05 * strength * (time.elapsed_secs() * 40.0).sin());
}

/// P 中场/开戏：旋钮照旧拧在 Time<Virtual> 上——驿站的件也跟着冻
fn intermission(keyboard: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if !keyboard.just_pressed(KeyCode::KeyP) {
        return;
    }
    if time.is_paused() {
        time.unpause();
        println!("老雷：开戏。");
    } else {
        time.pause();
        println!("老雷：中场——驿站也归戏台钟管，在途的件全冻住。");
    }
}

//! Listing 18-4：连珠箭——冷却是只走一程的 Timer，补箭是循环的 Timer
//! 空格掷袖箭：手上要缓 0.8 秒，匣里每 2.5 秒补一支（至多三支）。

use bevy::prelude::*;
use bevy::sprite::Anchor;

const FLOOR: f32 = -200.0;
const AYAN_X: f32 = -380.0;
const DUMMY_X: f32 = 380.0;
const DART_SPEED: f32 = 900.0;

// ANCHOR: quiver
/// 箭匣账本：两只定时器、一格库存、一本战绩
#[derive(Resource)]
struct Quiver {
    cooldown: Timer, // 出手后的硬直：只走一程（Once）
    refill: Timer,   // 补箭的节拍：循环走（Repeating）
    darts: u32,
    thrown: u32,
    hits: u32,
}

impl Default for Quiver {
    fn default() -> Self {
        let mut cooldown = Timer::from_seconds(0.8, TimerMode::Once);
        cooldown.finish(); // 开场手是稳的：把冷却直接拨到走完
        Self {
            cooldown,
            refill: Timer::from_seconds(2.5, TimerMode::Repeating),
            darts: 3,
            thrown: 0,
            hits: 0,
        }
    }
}
// ANCHOR_END: quiver

/// 飞着的袖箭
#[derive(Component)]
struct Dart;

/// 挨打的木桩：晃一阵的余韵也是只 Timer
#[derive(Component)]
struct Dummy {
    wobble: Timer,
}

/// HUD：冷却条的填充段与三枚箭标
#[derive(Component)]
struct CooldownFill;
#[derive(Component)]
struct DartPip(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .init_resource::<Quiver>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            ((tick_quiver, throw).chain(), fly, wobble_dummy, hud),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let regular = asset_server.load("fonts/book-sans-sc-regular.otf");

    // 台面、阿燕（出手姿势用站桩帧）、木人桩
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
            rect: Some(Rect::new(0.0, 0.0, 32.0, 40.0)), // 整本连环画只取第一格
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

    // HUD：冷却条（底 + 填充）与箭匣三枚箭标
    commands.spawn((
        Text2d::new("袖箭"),
        TextFont {
            font: regular,
            font_size: 26.0,
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.88, 0.80)),
        Anchor::CENTER_LEFT,
        Transform::from_xyz(-600.0, 308.0, 5.0),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.22, 0.22, 0.27), Vec2::new(200.0, 16.0)),
        Anchor::CENTER_LEFT,
        Transform::from_xyz(-520.0, 308.0, 5.0),
    ));
    commands.spawn((
        CooldownFill,
        Sprite::from_color(Color::srgb(0.49, 0.76, 0.44), Vec2::new(200.0, 16.0)),
        Anchor::CENTER_LEFT,
        Transform::from_xyz(-520.0, 308.0, 6.0),
    ));
    for i in 0..3 {
        commands.spawn((
            DartPip(i),
            Sprite::from_color(Color::srgb(0.85, 0.78, 0.48), Vec2::new(26.0, 8.0)),
            Anchor::CENTER_LEFT,
            Transform::from_xyz(-296.0 + i as f32 * 38.0, 308.0, 5.0)
                .with_rotation(Quat::from_rotation_z(0.5)),
        ));
    }

    println!("老雷：练《连珠箭》——空格出手。手上要缓劲，匣里要等补。");
}

// ANCHOR: tick
/// 拨表：两只定时器每帧各喂一次 delta；补箭只在“循环走完”的那一帧动手
fn tick_quiver(time: Res<Time>, mut quiver: ResMut<Quiver>) {
    quiver.cooldown.tick(time.delta());
    if quiver.refill.tick(time.delta()).just_finished() && quiver.darts < 3 {
        quiver.darts += 1;
        println!("场记：补一支——匣 {}/3。", quiver.darts);
    }
}
// ANCHOR_END: tick

// ANCHOR: throw
/// 出手三问：按了吗、缓过劲了吗、匣里还有吗
fn throw(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut quiver: ResMut<Quiver>,
    mut commands: Commands,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    if !quiver.cooldown.is_finished() {
        println!(
            "阿燕：手上还没缓过劲——还差 {:.1} 秒。",
            quiver.cooldown.remaining_secs()
        );
        return;
    }
    if quiver.darts == 0 {
        println!("场记：匣空了——等补箭。");
        return;
    }
    quiver.darts -= 1;
    quiver.thrown += 1;
    quiver.cooldown.reset(); // 冷却从头再走一程
    commands.spawn((
        Dart,
        Sprite::from_color(Color::srgb(0.92, 0.96, 1.0), Vec2::new(30.0, 6.0)),
        Transform::from_xyz(AYAN_X + 70.0, FLOOR + 110.0, 4.0),
    ));
    if quiver.thrown == 1 {
        println!("阿燕：看箭。");
    }
}
// ANCHOR_END: throw

/// 袖箭飞行：到桩即中
fn fly(
    time: Res<Time>,
    mut darts: Query<(Entity, &mut Transform), With<Dart>>,
    mut dummy: Single<&mut Dummy>,
    mut quiver: ResMut<Quiver>,
    mut commands: Commands,
) {
    for (entity, mut transform) in &mut darts {
        transform.translation.x += DART_SPEED * time.delta_secs();
        if transform.translation.x >= DUMMY_X - 40.0 {
            commands.entity(entity).despawn();
            dummy.wobble.reset();
            quiver.hits += 1;
            if quiver.hits % 5 == 0 {
                println!("场记：中桩 {} 记。", quiver.hits);
            }
        }
    }
}

/// 木桩的余韵：fraction 从 0 走到 1，晃劲随之衰减
fn wobble_dummy(time: Res<Time>, mut dummy: Single<(&mut Dummy, &mut Transform)>) {
    let (dummy, transform) = &mut *dummy;
    dummy.wobble.tick(time.delta());
    let strength = 1.0 - dummy.wobble.fraction();
    transform.rotation =
        Quat::from_rotation_z(0.05 * strength * (time.elapsed_secs() * 40.0).sin());
}

// ANCHOR: hud
/// 冷却条直接画 fraction：出手瞬间归零，0.8 秒长回满格
fn hud(
    quiver: Res<Quiver>,
    mut fill: Single<&mut Transform, With<CooldownFill>>,
    mut pips: Query<(&DartPip, &mut Visibility)>,
) {
    fill.scale.x = quiver.cooldown.fraction();
    for (pip, mut visibility) in &mut pips {
        *visibility = if pip.0 < quiver.darts {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
// ANCHOR_END: hud

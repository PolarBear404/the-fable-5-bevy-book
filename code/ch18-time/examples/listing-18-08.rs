//! Listing 18-8：接拍——帧上收招、记在账上，鼓点上一笔结清
//! 解法：瞬时输入由每帧必跑的系统收集成“意图”，FixedUpdate 只消费意图。
//! 与 Listing 18-7 同两种场：[1] 慢板（丢拍场）[2] 拖戏（重复场）——这回账都两清。

use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;

/// 候拍的意图：攒下的出招数。鼓点上消费，消费完清零
#[derive(Resource, Default)]
struct Queued {
    strikes: u32,
}

#[derive(Resource, Default)]
struct Tally {
    seen: u32,
    caught: u32,
}

#[derive(Resource, Default)]
struct Dragging(bool);

#[derive(Component)]
struct Hud;

// ANCHOR: app
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .insert_resource(Time::<Fixed>::from_hz(4.0))
        .init_resource::<Queued>()
        .init_resource::<Tally>()
        .init_resource::<Dragging>()
        .add_systems(Startup, setup)
        // 收招放在固定主循环之前：每帧必跑一次，鼓点再稀也漏不掉
        .add_systems(
            RunFixedMainLoop,
            collect_strikes.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
        )
        .add_systems(FixedUpdate, drummer_settles)
        .add_systems(Update, (switch_scene, drag, hud))
        .run();
}
// ANCHOR_END: app

// ANCHOR: queue
/// 场记收招：just_pressed 还是每帧问，但只往账上记，不当场办
fn collect_strikes(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut queued: ResMut<Queued>,
    mut tally: ResMut<Tally>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        queued.strikes += 1;
        tally.seen += 1;
        println!("场记：记下第 {} 招，候拍。", tally.seen);
    }
}

/// 鼓师结账：每拍把攒下的招一笔结清，结完清零——后头的拍子自然没账可重复
fn drummer_settles(mut queued: ResMut<Queued>, mut tally: ResMut<Tally>) {
    if queued.strikes == 0 {
        return;
    }
    tally.caught += queued.strikes;
    println!(
        "鼓师：这一拍结清 {} 招——共接 {} 招。",
        queued.strikes, tally.caught
    );
    queued.strikes = 0;
}
// ANCHOR_END: queue

/// 换场（与 Listing 18-7 同款）：两种场里账目都该两清
fn switch_scene(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut fixed: ResMut<Time<Fixed>>,
    mut dragging: ResMut<Dragging>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        fixed.set_timestep_hz(4.0);
        dragging.0 = false;
        println!("老雷：慢板——鼓点 4 拍/秒，帧照常跑。");
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        fixed.set_timestep_hz(64.0);
        dragging.0 = true;
        println!("老雷：拖戏——鼓点恢复 64 拍/秒，每帧硬卡 150 毫秒。");
    }
}

fn drag(dragging: Res<Dragging>) {
    if dragging.0 {
        std::thread::sleep(Duration::from_millis(150));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Hud,
        Text2d::new(""),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-bold.otf").into(),
            font_size: FontSize::Px(34.0),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.88, 0.80)),
        Transform::from_xyz(0.0, 60.0, 5.0),
    ));
    commands.spawn((
        Text2d::new("空格出招　[1] 慢板（鼓点 4 拍/秒）　[2] 拖戏（64 拍/秒 + 每帧卡 150 毫秒）"),
        TextFont {
            font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(0.55, 0.57, 0.62)),
        Transform::from_xyz(0.0, -60.0, 5.0),
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("actors/ayan-sheet.png"),
            rect: Some(Rect::new(0.0, 0.0, 32.0, 40.0)),
            ..default()
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(0.0, -320.0, 3.0).with_scale(Vec3::splat(4.0)),
    ));
    println!("老雷：再验一遍招——这回场记只管记账，鼓师按拍结账。");
}

fn hud(
    tally: Res<Tally>,
    fixed: Res<Time<Fixed>>,
    mut text: Single<&mut Text2d, With<Hud>>,
    mut shown: Local<(u32, u32, u32)>,
) {
    let tempo = (1.0 / fixed.timestep().as_secs_f64()).round() as u32;
    if *shown == (tally.seen, tally.caught, tempo) {
        return;
    }
    *shown = (tally.seen, tally.caught, tempo);
    text.0 = format!(
        "场记记了 {} 招　　鼓师接了 {} 招\n鼓点 {tempo} 拍/秒",
        tally.seen, tally.caught,
    );
}

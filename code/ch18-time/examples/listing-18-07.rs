//! Listing 18-7：丢拍与重复——同一个 just_pressed，场记帧帧在看，鼓师只在鼓点上看
//! 空格出招。[1] 慢板：鼓点 4 拍/秒，比帧率慢——丢拍；
//! [2] 拖戏：鼓点恢复默认 64 拍/秒，但每帧人为卡 150 毫秒——一帧好几拍，重复。

use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;

/// 两本账：场记在 Update 记，鼓师在 FixedUpdate 记
#[derive(Resource, Default)]
struct Tally {
    seen: u32,
    caught: u32,
}

/// 拖戏开关：开着就每帧硬卡一阵，模拟“老机器/卡顿”
#[derive(Resource, Default)]
struct Dragging(bool);

#[derive(Component)]
struct Hud;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        // 开场是慢板：鼓点放慢到 4 拍/秒，丢拍丢得明明白白
        .insert_resource(Time::<Fixed>::from_hz(4.0))
        .init_resource::<Tally>()
        .init_resource::<Dragging>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, drummer_counts)
        .add_systems(Update, (clerk_counts, switch_scene, drag, hud))
        .run();
}

// ANCHOR: counts
/// 场记的账：Update 每帧跑一次，一次按下就是一招
fn clerk_counts(keyboard: Res<ButtonInput<KeyCode>>, mut tally: ResMut<Tally>) {
    if keyboard.just_pressed(KeyCode::Space) {
        tally.seen += 1;
        println!("场记：第 {} 招。", tally.seen);
    }
}

/// 鼓师的账：同一句问话搬进 FixedUpdate——本帧没鼓点就看不见，一帧几拍就看几遍
fn drummer_counts(keyboard: Res<ButtonInput<KeyCode>>, mut tally: ResMut<Tally>) {
    if keyboard.just_pressed(KeyCode::Space) {
        tally.caught += 1;
        println!("鼓师：鼓点上接到第 {} 招！", tally.caught);
    }
}
// ANCHOR_END: counts

// ANCHOR: scenes
/// 换场：慢板拧鼓点，拖戏拧帧率
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

/// 拖戏：把一帧人为拖长，下一帧鼓师就得连补好几拍
fn drag(dragging: Res<Dragging>) {
    if dragging.0 {
        std::thread::sleep(Duration::from_millis(150));
    }
}
// ANCHOR_END: scenes

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
    println!("老雷：验招——场记帧帧盯着，鼓师只在鼓点上抬头。都按空格记账。");
}

/// 记分牌：两本账并排示众
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

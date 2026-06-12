//! Listing 17-3：一下、按住与松手——同一块键盘的快照与流水
//! 空格出剑（一下），左 Shift 运劲（按住），松开收势（松手）；
//! 场记另开一本流水账，逐条速记 KeyboardInput 消息。

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::Anchor;

const FLOOR: f32 = -180.0;

/// 标记：阿燕
#[derive(Component)]
struct Ayan;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .add_systems(Startup, setup)
        // chain 只为让两本账的打印顺序固定（第 6 章）
        .add_systems(Update, (moves, transcript).chain())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
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
        Ayan,
        Sprite {
            image: asset_server.load("actors/ayan-sheet.png"),
            rect: Some(Rect::new(0.0, 40.0, 32.0, 80.0)),
            ..default()
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(0.0, FLOOR, 1.0).with_scale(Vec3::splat(4.0)),
    ));
    println!("老雷：试招台。空格出剑，左 Shift 运劲，松手收势——场记记流水。");
}

// ANCHOR: moves
/// 三种问法，三种招式：一下出剑、按住运劲、松手收势
fn moves(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ayan: Single<&mut Sprite, With<Ayan>>,
    mut charge: Local<f32>,
    mut slashes: Local<u32>,
) {
    // just_pressed：只在按下的那一帧为真——按住不放也只出一剑
    if keyboard.just_pressed(KeyCode::Space) {
        *slashes += 1;
        let name = ["拨云", "断浪", "归鞘"][(*slashes as usize - 1) % 3];
        println!("阿燕：青霜剑——{name}！（第 {} 剑）", *slashes);
    }

    // pressed：按住期间每帧都为真——劲随时间越运越足
    if keyboard.pressed(KeyCode::ShiftLeft) {
        *charge = (*charge + time.delta_secs()).min(2.0);
        let heat = Color::srgb(1.0, 0.84, 0.40);
        ayan.color = Color::WHITE.mix(&heat, *charge / 2.0);
    }

    // just_released：只在松开的那一帧为真——正好用来结算
    if keyboard.just_released(KeyCode::ShiftLeft) {
        println!("场记：收势。这口劲运了 {:.1} 秒。", *charge);
        *charge = 0.0;
        ayan.color = Color::WHITE;
    }
}
// ANCHOR_END: moves

// ANCHOR: transcript
/// 流水账：每条 KeyboardInput 消息原样过一遍场记的笔
fn transcript(mut events: MessageReader<KeyboardInput>) {
    for event in events.read() {
        let state = if event.state.is_pressed() {
            "按下"
        } else {
            "松开"
        };
        let repeat = if event.repeat { "（系统重复）" } else { "" };
        let text = match &event.text {
            Some(t) => format!("，落字 {t:?}"),
            None => String::new(),
        };
        println!(
            "场记速记：{:?} {state}{repeat}，逻辑键 {:?}{text}",
            event.key_code, event.logical_key
        );
    }
}
// ANCHOR_END: transcript

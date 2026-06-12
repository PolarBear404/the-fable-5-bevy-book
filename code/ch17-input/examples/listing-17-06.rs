//! Listing 17-6：手柄进场——Gamepad 是实体
//! 左摇杆（或方向键）走人，南键出剑外加一阵手柄震动；
//! 插拔手柄，场记照实通报。没有手柄也能跑——台上只是没人接活。

use bevy::input::gamepad::{
    GamepadConnection, GamepadConnectionEvent, GamepadRumbleIntensity, GamepadRumbleRequest,
};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::time::Duration;

const FLOOR: f32 = -180.0;
const STAGE_HALF: f32 = 560.0;
const WALK_SPEED: f32 = 240.0;
/// 自留的死区：left_stick() 给的是原始读数，回中不归零的旧摇杆全靠这道闸
const STICK_DEADZONE: f32 = 0.15;

/// 标记：阿燕
#[derive(Component)]
struct Ayan;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .add_systems(Startup, setup)
        .add_systems(Update, (door, drive))
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
    println!("老雷：手柄口开着，插上就认。摇杆走人，南键出剑。");
}

// ANCHOR: door
/// 门房：手柄的进场与离场都有通报
fn door(mut connections: MessageReader<GamepadConnectionEvent>) {
    for event in connections.read() {
        match &event.connection {
            GamepadConnection::Connected { name, .. } => {
                println!("场记：手柄看客进场——“{name}”（实体 {}）。", event.gamepad);
            }
            GamepadConnection::Disconnected => {
                println!("场记：手柄 {} 拔线离场。", event.gamepad);
            }
        }
    }
}
// ANCHOR_END: door

// ANCHOR: drive
/// 在场的每只手柄都能使唤阿燕：摇杆是模拟量，推几分走几分
fn drive(
    gamepads: Query<(Entity, &Gamepad)>,
    time: Res<Time>,
    mut ayan: Single<(&mut Transform, &mut Sprite), With<Ayan>>,
    mut rumbles: MessageWriter<GamepadRumbleRequest>,
) {
    for (entity, gamepad) in &gamepads {
        // 左摇杆横轴在 -1.0..=1.0 之间连续取值；十字键凑成的 dpad() 只有 ±1 和 0
        let stick = gamepad.left_stick().x;
        let stick = if stick.abs() > STICK_DEADZONE { stick } else { 0.0 };
        let push = (stick + gamepad.dpad().x).clamp(-1.0, 1.0);
        if push != 0.0 {
            let (transform, sprite) = &mut *ayan;
            transform.translation.x = (transform.translation.x
                + push * WALK_SPEED * time.delta_secs())
            .clamp(-STAGE_HALF, STAGE_HALF);
            sprite.flip_x = push < 0.0;
        }

        // 南键出剑：快照三问对手柄按键同样适用
        if gamepad.just_pressed(GamepadButton::South) {
            println!("阿燕：剑出鞘——这把手柄有分量。");
            // 回敬一阵震动：强马达七分劲，半秒内收住
            rumbles.write(GamepadRumbleRequest::Add {
                duration: Duration::from_millis(400),
                intensity: GamepadRumbleIntensity::strong_motor(0.7),
                gamepad: entity,
            });
        }
    }
}
// ANCHOR_END: drive

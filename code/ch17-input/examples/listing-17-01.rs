//! Listing 17-1：键盘牵动阿燕——ButtonInput<KeyCode> 初见
//! 体验场开张第一桩：A/D（或左右箭头）按住就走、松手就停。

use bevy::prelude::*;
use bevy::sprite::Anchor;

/// 桥板地面的高度
const FLOOR: f32 = -180.0;
/// 台口两端：阿燕的活动范围
const STAGE_HALF: f32 = 560.0;
/// 步速（世界单位 / 秒）
const WALK_SPEED: f32 = 240.0;

/// 标记：阿燕
#[derive(Component)]
struct Ayan;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .add_systems(Startup, setup)
        .add_systems(Update, walk)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 桥板：第 15 章的平铺贴片
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

    // 阿燕：从十二格连环画里裁出侧身那格（第二行第一格），先不管腿动不动
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

    println!("老雷：体验场开张。键盘看客先请——A、D 走人，箭头键也认。");
}

// ANCHOR: walk
/// 每帧问一遍键盘：哪边的键正按着，阿燕就往哪边走
fn walk(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ayan: Single<(&mut Transform, &mut Sprite), With<Ayan>>,
    mut greeted: Local<bool>,
) {
    let mut direction = 0.0;
    if keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        direction -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        direction += 1.0;
    }
    if direction == 0.0 {
        return; // 没人按键，这一帧不挪窝
    }
    if !*greeted {
        *greeted = true;
        println!("阿燕：来了？我脚下听你的。");
    }

    let (transform, sprite) = &mut *ayan;
    transform.translation.x = (transform.translation.x
        + direction * WALK_SPEED * time.delta_secs())
    .clamp(-STAGE_HALF, STAGE_HALF);
    sprite.flip_x = direction < 0.0; // 画稿朝右，往左走就镜像
}
// ANCHOR_END: walk

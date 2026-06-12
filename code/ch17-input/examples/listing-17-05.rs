//! Listing 17-5：导播摇臂——位移、滚轮与抓光标
//! 按住右键拖动台面，滚轮推拉镜头；G 进摇臂模式（光标锁住、藏起来，
//! 移动鼠标直接摇镜头），Esc 退出。

use bevy::input::common_conditions::input_just_pressed;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::{CursorGrabMode, CursorOptions};

const FLOOR: f32 = -180.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .add_systems(Startup, setup)
        // ANCHOR: run_if
        .add_systems(
            Update,
            (
                crane,
                enter_crane.run_if(input_just_pressed(KeyCode::KeyG)),
                exit_crane.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        )
        // ANCHOR_END: run_if
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 加长的台面：桥板一路铺过去，每 200 个单位钉一枚金桩当参照物
    commands.spawn((
        Sprite {
            image: asset_server.load("props/dock-plank.png"),
            custom_size: Some(Vec2::new(2800.0, 56.0)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 4.0,
            },
            ..default()
        },
        Transform::from_xyz(0.0, FLOOR - 28.0, 0.0),
    ));
    for i in -6..=6 {
        commands.spawn((
            Sprite::from_color(Color::srgb(0.91, 0.72, 0.29), Vec2::new(10.0, 26.0)),
            Anchor::BOTTOM_CENTER,
            Transform::from_xyz(i as f32 * 200.0, FLOOR, 0.5),
        ));
    }
    commands.spawn((
        Sprite {
            image: asset_server.load("actors/ayan-sheet.png"),
            rect: Some(Rect::new(0.0, 0.0, 32.0, 40.0)),
            ..default()
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(-120.0, FLOOR, 1.0).with_scale(Vec3::splat(4.0)),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("props/wooden-dummy.png")),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(640.0, FLOOR, 1.0).with_scale(Vec3::splat(4.0)),
    ));

    println!("老雷：摇臂就位。右键拖台面，滚轮推拉；按 G 上摇臂，Esc 下来。");
}

// ANCHOR: crane
/// 摇臂本体：滚轮缩放取景框，拖动平移机位
fn crane(
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    cursor_options: Single<&CursorOptions>,
    camera: Single<(&mut Transform, &mut Projection), With<Camera2d>>,
) {
    let (mut transform, mut projection) = camera.into_inner();
    let Projection::Orthographic(ortho) = &mut *projection else {
        return;
    };

    // 滚轮推拉：滚一格改一档投影 scale（第 13 章的旋钮）。
    // 鼠标滚轮按“行”报数，触控板按“像素”报数——先折算成行
    if scroll.delta.y != 0.0 {
        let lines = match scroll.unit {
            MouseScrollUnit::Line => scroll.delta.y,
            MouseScrollUnit::Pixel => {
                scroll.delta.y / MouseScrollUnit::SCROLL_UNIT_CONVERSION_FACTOR
            }
        };
        ortho.scale = (ortho.scale * 0.9_f32.powf(lines)).clamp(0.35, 2.5);
    }

    // 平移：摇臂模式下光标已锁死，位移直接驱动机位；自由模式下要按住右键。
    // 注意一减一加——窗口坐标 y 朝下，世界坐标 y 朝上
    let craning = cursor_options.grab_mode == CursorGrabMode::Locked;
    if (craning || mouse.pressed(MouseButton::Right)) && motion.delta != Vec2::ZERO {
        transform.translation.x =
            (transform.translation.x - motion.delta.x * ortho.scale).clamp(-1200.0, 1200.0);
        transform.translation.y =
            (transform.translation.y + motion.delta.y * ortho.scale).clamp(-300.0, 300.0);
    }
}
// ANCHOR_END: crane

// ANCHOR: grab
/// 上摇臂：光标锁在原地、藏起来——位移消息照流不误
fn enter_crane(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false;
    println!("场记：上摇臂。光标锁住藏起，手上的位移直接进镜头。");
}

/// 下摇臂：还看客一个能自由出窗的光标
fn exit_crane(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.grab_mode = CursorGrabMode::None;
    cursor_options.visible = true;
    println!("场记：下摇臂，光标放行。");
}
// ANCHOR_END: grab

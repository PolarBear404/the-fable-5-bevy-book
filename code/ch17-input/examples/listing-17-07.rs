//! Listing 17-7：触摸——一窝会编号的“光标”
//! 触屏设备上：点哪儿插旗，阿燕跑过去；几根手指按着，场记数得清。
//! 没有触屏的机器照样能跑，只是 Touches 永远是空的。

use bevy::prelude::*;
use bevy::sprite::Anchor;

const FLOOR: f32 = -180.0;
const STAGE_HALF: f32 = 560.0;
const RUN_SPEED: f32 = 340.0;

/// 标记：阿燕
#[derive(Component)]
struct Ayan;

/// 标记：令旗
#[derive(Component)]
struct Flag;

/// 阿燕这趟要跑到的横坐标
#[derive(Resource, Default)]
struct DashTarget(Option<f32>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .init_resource::<DashTarget>()
        .add_systems(Startup, setup)
        .add_systems(Update, (taps, dash).chain())
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
    commands.spawn((
        Flag,
        Sprite::from_color(Color::srgb(0.95, 0.78, 0.22), Vec2::splat(18.0)),
        Transform::from_xyz(0.0, FLOOR + 9.0, 0.5)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
        Visibility::Hidden,
    ));
    println!("老雷：触屏看客请上手。点哪儿走哪儿，几根手指我们都数着。");
}

// ANCHOR: taps
/// 落指即插旗：触点的窗口坐标走光标同一条反算路
fn taps(
    touches: Res<Touches>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut flag: Single<(&mut Transform, &mut Visibility), With<Flag>>,
    mut target: ResMut<DashTarget>,
    mut last_count: Local<usize>,
) {
    let (camera, camera_transform) = *camera;
    for touch in touches.iter_just_pressed() {
        let Ok(point) = camera.viewport_to_world_2d(camera_transform, touch.position()) else {
            continue;
        };
        let (flag_transform, flag_visibility) = &mut *flag;
        flag_transform.translation = point.extend(0.5);
        **flag_visibility = Visibility::Visible;
        target.0 = Some(point.x.clamp(-STAGE_HALF, STAGE_HALF));
        println!(
            "场记：{} 号手指点了 ({:.0}, {:.0})，旗插上了。",
            touch.id(),
            point.x,
            point.y
        );
    }

    // 同时按着几根手指？Touches 替每个触点记着账
    let count = touches.iter().count();
    if count != *last_count {
        *last_count = count;
        if count > 1 {
            println!("场记：台口同时按着 {count} 根手指。");
        }
    }
}
// ANCHOR_END: taps

/// 朝令旗跑：与鼠标那版一字不差——目标定了，怎么定的不重要
fn dash(
    time: Res<Time>,
    mut target: ResMut<DashTarget>,
    mut ayan: Single<(&mut Transform, &mut Sprite), With<Ayan>>,
    mut flag_visibility: Single<&mut Visibility, (With<Flag>, Without<Ayan>)>,
) {
    let Some(goal_x) = target.0 else { return };
    let (transform, sprite) = &mut *ayan;
    let step = RUN_SPEED * time.delta_secs();
    let distance = goal_x - transform.translation.x;
    sprite.flip_x = distance < 0.0;
    if distance.abs() <= step {
        transform.translation.x = goal_x;
        target.0 = None;
        **flag_visibility = Visibility::Hidden;
    } else {
        transform.translation.x += step * distance.signum();
    }
}

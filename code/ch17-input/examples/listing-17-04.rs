//! Listing 17-4：指哪打哪——鼠标按键与光标反算
//! 左键点台面安一面小令旗，阿燕跑过去；右键叫停。
//! 光标出窗，阿燕的目光就追不上了——cursor_position 给 None。

use bevy::prelude::*;
use bevy::sprite::Anchor;

const FLOOR: f32 = -180.0;
const STAGE_HALF: f32 = 560.0;
const RUN_SPEED: f32 = 340.0;

/// 标记：阿燕
#[derive(Component)]
struct Ayan;

/// 标记：点出来的小令旗
#[derive(Component)]
struct Flag;

/// 阿燕这趟要跑到的横坐标（None = 原地候场）
#[derive(Resource, Default)]
struct DashTarget(Option<f32>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .init_resource::<DashTarget>()
        .add_systems(Startup, setup)
        .add_systems(Update, (point_and_call, dash).chain())
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
    // 令旗先候着：一面金色小三角，点哪儿插哪儿
    commands.spawn((
        Flag,
        Sprite::from_color(Color::srgb(0.95, 0.78, 0.22), Vec2::splat(18.0)),
        Transform::from_xyz(0.0, FLOOR + 9.0, 0.5)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
        Visibility::Hidden,
    ));
    println!("老雷：换鼠标看客。左键把令旗插在台上，阿燕跑过去；右键叫停。");
}

// ANCHOR: point
/// 左键安旗：把光标的窗口坐标反算成世界坐标，旗插到点的位置
fn point_and_call(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut flag: Single<(&mut Transform, &mut Visibility), With<Flag>>,
    mut target: ResMut<DashTarget>,
    mut was_inside: Local<bool>,
) {
    // 光标在不在窗口里？不在就拿不到坐标——这正是 Option 的用处
    let Some(cursor) = window.cursor_position() else {
        if *was_inside {
            *was_inside = false;
            println!("场记：看客的手出了台口，光标没影了。");
        }
        return;
    };
    *was_inside = true;

    if mouse.just_pressed(MouseButton::Left) {
        // 窗口坐标（原点左上、y 朝下）→ 世界坐标（原点居中、y 朝上）
        let (camera, camera_transform) = *camera;
        let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor) else {
            return;
        };
        let (flag_transform, flag_visibility) = &mut *flag;
        flag_transform.translation = point.extend(0.5);
        **flag_visibility = Visibility::Visible;
        target.0 = Some(point.x.clamp(-STAGE_HALF, STAGE_HALF));
        println!("场记：令旗插在 ({:.0}, {:.0})。", point.x, point.y);
    }
    if mouse.just_pressed(MouseButton::Right) && target.0.take().is_some() {
        println!("阿燕：叫停就停。");
    }
}
// ANCHOR_END: point

// ANCHOR: dash
/// 朝令旗跑：到站收旗
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
        println!("阿燕：到位。下一处？");
    } else {
        transform.translation.x += step * distance.signum();
    }
}
// ANCHOR_END: dash

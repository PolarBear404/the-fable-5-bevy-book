//! Listing 25-14：2D 的现成脚架——PanCamera 平移、缩放、转场

use bevy::camera_controller::pan_camera::{PanCamera, PanCameraPlugin};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanCameraPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, report)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ANCHOR: camera
    commands.spawn((
        Camera2d,
        PanCamera {
            // 键盘平移快一点，别的用出厂值
            pan_speed: 700.0,
            ..default()
        },
    ));
    // ANCHOR_END: camera

    // 长街一条：五盏灯笼一字排开，阿燕站正中——平移缩放才有的看
    let lantern = asset_server.load("sprites/lantern.png");
    for i in -2..=2_i32 {
        commands.spawn((
            Sprite::from_image(lantern.clone()),
            Transform::from_xyz(i as f32 * 320.0, 140.0, 0.0).with_scale(Vec3::splat(6.0)),
        ));
    }
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ayan-still.png")),
        Transform::from_xyz(0.0, -60.0, 1.0).with_scale(Vec3::splat(8.0)),
    ));
    println!("老雷：夜市长街——WASD 移镜头，QE 转，+/- 或滚轮推拉，左键一拖就走。");
    println!("小棠：空格报机位。");
}

// ANCHOR: report
/// 空格报账：位置在 Transform 上，缩放档在 PanCamera 自己身上
fn report(keys: Res<ButtonInput<KeyCode>>, camera: Single<(&Transform, &PanCamera)>) {
    if keys.just_pressed(KeyCode::Space) {
        let (seat, pan) = *camera;
        println!(
            "场记：机位 ({:.0}, {:.0})，转角 {:.0} 度，变焦 {:.2}。",
            seat.translation.x,
            seat.translation.y,
            seat.rotation.to_euler(EulerRot::ZYX).0.to_degrees(),
            pan.zoom_factor
        );
    }
}
// ANCHOR_END: report

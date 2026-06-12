//! Listing 12-6：彗星的机头——让朝向追上运动方向
//! 取“下一刻位置 − 当前位置”当方向，把精灵的 +Y 转过去对齐

use bevy::prelude::*;

/// 标记：扫过天文馆穹顶的彗星
#[derive(Component)]
struct Comet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, fly_comet)
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 太阳留在原点做参照物
    commands.spawn(Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(80.0)));

    // 彗星是个窄长条：竖放时窄边在上，所以“机头”是它自己的 +Y 方向
    commands.spawn((
        Comet,
        Sprite::from_color(Color::srgb(0.85, 0.95, 1.0), Vec2::new(14.0, 36.0)),
        Transform::from_xyz(320.0, 0.0, 0.0),
    ));
}
// ANCHOR_END: setup

// ANCHOR: fly
/// 沿椭圆飞行，机头始终指向运动方向
fn fly_comet(time: Res<Time>, mut comets: Query<&mut Transform, With<Comet>>) {
    let t = time.elapsed_secs() * 0.8;
    for mut transform in &mut comets {
        // 椭圆轨道的参数方程：横半轴 320，纵半轴 180
        let next = Vec3::new(320.0 * t.cos(), 180.0 * t.sin(), 0.0);

        // 运动方向 = 下一处落点 − 当前位置（z 分量不参与，截掉）
        let motion = (next - transform.translation).truncate();
        transform.translation = next;

        // Dir2::new 拒收零向量：原地未动时保持旧朝向，而不是突然朝乱
        if let Ok(heading) = Dir2::new(motion) {
            // 把精灵自带的 +Y（机头）转到 heading 指的方向
            transform.rotation = Quat::from_rotation_arc(Vec3::Y, heading.extend(0.0));
        }
    }
}
// ANCHOR_END: fly

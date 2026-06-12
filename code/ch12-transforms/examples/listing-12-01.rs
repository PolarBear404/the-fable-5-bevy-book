//! Listing 12-1：罗盘四方块——亲眼确认坐标系往哪边长
//! 原点在窗口正中央，+X 向右，+Y 向上

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 白色小方块：不写 Transform，落在默认位置——世界原点 (0, 0)
    commands.spawn(Sprite::from_color(Color::WHITE, Vec2::splat(20.0)));

    // 红色方块：x = +300，如果 +X 真朝右，它该出现在原点右边
    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.3, 0.3), Vec2::splat(40.0)),
        Transform::from_xyz(300.0, 0.0, 0.0),
    ));

    // 绿色方块：y = +200，如果 +Y 真朝上，它该出现在原点上方
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.8, 0.4), Vec2::splat(40.0)),
        Transform::from_xyz(0.0, 200.0, 0.0),
    ));

    // 蓝色方块：两个坐标都为负，应该落在左下角
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.5, 0.9), Vec2::splat(40.0)),
        Transform::from_xyz(-300.0, -200.0, 0.0),
    ));
}

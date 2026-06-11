use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_sprite)
        .run();
}

fn setup(mut commands: Commands) {
    // 没有相机就什么都看不到；Camera2d 是 2D 世界的取景器
    commands.spawn(Camera2d);
    // 一个 120×120 的纯色方块
    commands.spawn(Sprite::from_color(
        Color::srgb(0.25, 0.65, 0.95),
        Vec2::new(120.0, 120.0),
    ));
}

fn move_sprite(time: Res<Time>, mut sprites: Query<&mut Transform, With<Sprite>>) {
    for mut transform in &mut sprites {
        // 位置随时间正弦摆动：x 在 -200 到 +200 之间来回
        transform.translation.x = time.elapsed_secs().sin() * 200.0;
    }
}

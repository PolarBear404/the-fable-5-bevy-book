//! Listing 18-5：掐表——Stopwatch 是只往前数、由你喂时间的表
//! 左 Shift 运劲（表走），松手收势（报数、归零）。第 17 章的桩功，这回用对工具。

use bevy::color::Mix;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::time::Stopwatch; // 不在 prelude 里，要自己请

const FLOOR: f32 = -200.0;

// ANCHOR: charge
/// 运劲的账：一只秒表
#[derive(Component)]
struct Charge {
    watch: Stopwatch,
}

/// 按住才喂时间，松手报数归零——表走不走，全看你喂不喂
fn practice(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ayan: Single<(&mut Charge, &mut Sprite)>,
) {
    let (charge, sprite) = &mut *ayan;
    if keyboard.pressed(KeyCode::ShiftLeft) {
        charge.watch.tick(time.delta());
    }
    if keyboard.just_released(KeyCode::ShiftLeft) {
        println!("场记：收势——这口劲运了 {:.1} 秒。", charge.watch.elapsed_secs());
        charge.watch.reset();
    }
    // 劲越足，戏服越金（两秒到顶）
    let heat = (charge.watch.elapsed_secs() / 2.0).min(1.0);
    sprite.color = Color::WHITE.mix(&Color::srgb(1.0, 0.84, 0.40), heat);
}
// ANCHOR_END: charge

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .add_systems(Startup, setup)
        .add_systems(Update, practice)
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
        Charge {
            watch: Stopwatch::new(),
        },
        Sprite {
            image: asset_server.load("actors/ayan-sheet.png"),
            rect: Some(Rect::new(0.0, 0.0, 32.0, 40.0)),
            ..default()
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(0.0, FLOOR, 3.0).with_scale(Vec3::splat(4.0)),
    ));
    println!("老雷：今儿练桩功——左 Shift 运劲，松手收势，场记掐表。");
}

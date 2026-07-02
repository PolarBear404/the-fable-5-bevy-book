//! Listing 18-3：两面钟——怀表走人间（Real），戏台钟走戏里（Virtual）
//! 空格中场/开戏，↑↓ 给戏台钟换挡。台上的一切跟着戏台钟，怀表谁也管不住。

use std::f32::consts::TAU;
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::time::common_conditions::on_real_timer;

const FLOOR: f32 = -200.0;

/// 钟的指针：一分钟一圈
#[derive(Component)]
enum Hand {
    Real,
    Stage,
}

/// 阿燕与她的帧动画节拍器
#[derive(Component)]
struct Ayan;
#[derive(Component)]
struct FrameClock(Timer);

/// HUD 文本
#[derive(Component)]
struct Hud;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.10, 0.14)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                conduct,
                turn_hands,
                walk_ayan,
                animate,
                hud.run_if(on_real_timer(Duration::from_millis(100))),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    let bold = asset_server.load("fonts/book-sans-sc-bold.otf");
    let regular = asset_server.load("fonts/book-sans-sc-regular.otf");

    // 台面
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

    // ANCHOR: clocks
    // 两面钟：表盘是 Mesh2d 圆（第 15 章），指针钉在下端、绕钉子转（第 15 章的 Anchor）
    let face = meshes.add(Circle::new(72.0));
    let pivot = meshes.add(Circle::new(7.0));
    let face_paint = materials.add(Color::srgb(0.93, 0.91, 0.85));
    let pivot_paint = materials.add(Color::srgb(0.16, 0.15, 0.13));
    for (x, label, hand, color) in [
        (-330.0, "怀表（Real）", Hand::Real, Color::srgb(0.22, 0.34, 0.62)),
        (330.0, "戏台钟（Virtual）", Hand::Stage, Color::srgb(0.76, 0.30, 0.18)),
    ] {
        commands.spawn((
            Mesh2d(face.clone()),
            MeshMaterial2d(face_paint.clone()),
            Transform::from_xyz(x, 150.0, 1.0),
        ));
        commands.spawn((
            hand,
            Sprite::from_color(color, Vec2::new(7.0, 62.0)),
            Anchor::BOTTOM_CENTER,
            Transform::from_xyz(x, 150.0, 2.0),
        ));
        commands.spawn((
            Mesh2d(pivot.clone()),
            MeshMaterial2d(pivot_paint.clone()),
            Transform::from_xyz(x, 150.0, 3.0),
        ));
        commands.spawn((
            Text2d::new(label),
            TextFont {
                font: regular.clone().into(),
                font_size: FontSize::Px(24.0),
                ..default()
            },
            TextColor(color),
            Transform::from_xyz(x, 52.0, 1.0),
        ));
    }
    // ANCHOR_END: clocks

    // 阿燕在台上走台步（后六格是走路帧，第 15 章的图集）
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        6,
        2,
        None,
        None,
    ));
    commands.spawn((
        Ayan,
        FrameClock(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Sprite::from_atlas_image(
            asset_server.load("actors/ayan-sheet.png"),
            TextureAtlas { layout, index: 6 },
        ),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(0.0, FLOOR, 3.0).with_scale(Vec3::splat(4.0)),
    ));

    // HUD：读数与状态
    commands.spawn((
        Hud,
        Text2d::new(""),
        TextFont {
            font: bold.into(),
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.88, 0.80)),
        Transform::from_xyz(0.0, 320.0, 5.0),
    ));

    println!("老雷：排练厅挂两面钟——怀表走人间，戏台钟走戏里。");
    println!("场记：空格中场/开戏，↑↓ 给戏台钟换挡。");
}

// ANCHOR: conduct
const SPEEDS: [f32; 5] = [0.25, 0.5, 1.0, 2.0, 4.0];

/// 指挥席：暂停与变速都拧在 Time<Virtual> 这只钟上
fn conduct(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
    mut gear: Local<Option<usize>>,
) {
    let gear = gear.get_or_insert(2); // 开场挂在 ×1.0 那一挡
    if keyboard.just_pressed(KeyCode::Space) {
        if time.is_paused() {
            time.unpause();
            println!("老雷：开戏——戏台钟接着走。");
        } else {
            time.pause();
            println!("老雷：中场——台上歇住，怀表不歇。");
        }
    }
    let step: isize = i32::from(keyboard.just_pressed(KeyCode::ArrowUp)) as isize
        - i32::from(keyboard.just_pressed(KeyCode::ArrowDown)) as isize;
    if step != 0 {
        let next = gear.saturating_add_signed(step).min(SPEEDS.len() - 1);
        if next != *gear {
            *gear = next;
            time.set_relative_speed(SPEEDS[next]);
            println!("鼓师：换挡——戏台钟 ×{}。", SPEEDS[next]);
        }
    }
}
// ANCHOR_END: conduct

// ANCHOR: hands
/// 转表针：一分钟一圈。两根针各问各的钟
fn turn_hands(
    real: Res<Time<Real>>,
    stage: Res<Time<Virtual>>,
    mut hands: Query<(&mut Transform, &Hand)>,
) {
    for (mut transform, hand) in &mut hands {
        let elapsed = match hand {
            Hand::Real => real.elapsed_secs(),
            Hand::Stage => stage.elapsed_secs(),
        };
        transform.rotation = Quat::from_rotation_z(-elapsed * TAU / 60.0);
    }
}
// ANCHOR_END: hands

// ANCHOR: walk
/// 台步照旧问 Res<Time>——在 Update 里它就是那只戏台钟（Virtual）
fn walk_ayan(time: Res<Time>, mut ayan: Single<(&mut Transform, &mut Sprite), With<Ayan>>) {
    let beat = time.elapsed_secs() * 0.6;
    let (transform, sprite) = &mut *ayan;
    transform.translation.x = beat.sin() * 340.0;
    sprite.flip_x = beat.cos() < 0.0; // 往西走就照镜子
}
// ANCHOR_END: walk

/// 帧动画的节拍器喂的也是 Res<Time> 的 delta——中场时 delta 为 0，阿燕定格
fn animate(time: Res<Time>, mut ayan: Single<(&mut Sprite, &mut FrameClock), With<Ayan>>) {
    let (sprite, clock) = &mut *ayan;
    if clock.0.tick(time.delta()).just_finished()
        && let Some(atlas) = sprite.texture_atlas.as_mut()
    {
        atlas.index = if atlas.index >= 11 { 6 } else { atlas.index + 1 };
    }
}

/// 读数牌：两只钟的 elapsed 各自报数
fn hud(
    real: Res<Time<Real>>,
    stage: Res<Time<Virtual>>,
    mut text: Single<&mut Text2d, With<Hud>>,
) {
    let status = if stage.is_paused() { "中场" } else { "开戏" };
    text.0 = format!(
        "怀表 {:6.1} 秒　　戏台钟 {:6.1} 秒\n速度 ×{:.2}　　{status}",
        real.elapsed_secs(),
        stage.elapsed_secs(),
        stage.relative_speed(),
    );
}

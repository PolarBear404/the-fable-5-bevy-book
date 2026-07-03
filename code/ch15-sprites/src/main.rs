//! Listing 15-12：《渡口夜话》带妆彩排——本章全部家伙什一台戏用齐：
//! 图集帧动画（阿燕走/停、梢公摇橹）、染色与翻面、锚点、九宫格字幕框、
//! 平铺的江水与栈桥、Mesh2d 铸的月亮光晕、bevy_color 调的灯笼呼吸

use bevy::prelude::*;
use bevy::sprite::Anchor;

/// 栈桥桥面的高度（演员脚底贴着它走）
const DOCK_TOP: f32 = -50.0;
/// 阿燕的两个站位：西头戏台口与东头渡口
const WEST_MARK: f32 = -500.0;
const EAST_MARK: f32 = 20.0;

/// 帧动画的节拍与范围（与 Listing 15-4 同款）
#[derive(Component)]
struct FrameClock {
    timer: Timer,
    first: usize,
    last: usize,
}

impl FrameClock {
    fn new(interval: f32, first: usize, last: usize) -> Self {
        FrameClock {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            first,
            last,
        }
    }
}

// ANCHOR: direction
/// 阿燕的台位安排：走到目标点，或停在原地养神到某一时刻
#[derive(Component)]
enum StageDirection {
    Resting { until: f32 },
    Walking { to_x: f32 },
}
// ANCHOR_END: direction

/// 标记：阿燕
#[derive(Component)]
struct Ayan;

/// 标记：乌篷船（梢公是它的子实体，跟船一起沉浮）
#[derive(Component)]
struct Boat;

/// 标记：灯笼的光晕（材质颜色随呼吸明灭）
#[derive(Component)]
struct LanternGlow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.035, 0.045, 0.09)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (direct_ayan, advance_frames, rock_the_boat, breathe_lantern),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // —— 天幕：月亮、光晕、星斗、远山（Mesh2d 现铸） ——
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(56.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.93, 0.91, 0.82))),
        Transform::from_xyz(340.0, 215.0, 0.6),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(56.0, 80.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.93, 0.91, 0.82, 0.06))),
        Transform::from_xyz(340.0, 215.0, 0.55),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(80.0, 106.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.93, 0.91, 0.82, 0.03))),
        Transform::from_xyz(340.0, 215.0, 0.5),
    ));

    let star_mesh = meshes.add(RegularPolygon::new(3.5, 4));
    let star_material = materials.add(Color::srgba(0.92, 0.94, 1.00, 0.8));
    for i in 0..16 {
        let x = ((i * 167) % 1240) as f32 - 620.0;
        let y = ((i * 89) % 220) as f32 + 110.0;
        commands.spawn((
            Mesh2d(star_mesh.clone()),
            MeshMaterial2d(star_material.clone()),
            Transform::from_xyz(x, y, 0.3),
        ));
    }

    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(-300.0, -100.0),
            Vec2::new(300.0, -100.0),
            Vec2::new(-20.0, 100.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.06, 0.09, 0.15))),
        Transform::from_xyz(-340.0, -20.0, 0.7),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(-260.0, -75.0),
            Vec2::new(260.0, -75.0),
            Vec2::new(50.0, 75.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.045, 0.07, 0.12))),
        Transform::from_xyz(300.0, -45.0, 0.65),
    ));

    // —— 江水与栈桥：两块 16×16 贴片铺出来（Tiled） ——
    commands.spawn((
        Sprite {
            image: asset_server.load("props/water-tile.png"),
            custom_size: Some(Vec2::new(1280.0, 230.0)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 4.0,
            },
            ..default()
        },
        Transform::from_xyz(0.0, -205.0, 1.0),
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("props/dock-plank.png"),
            custom_size: Some(Vec2::new(800.0, 56.0)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 4.0,
            },
            ..default()
        },
        Transform::from_xyz(-240.0, DOCK_TOP - 28.0, 2.0),
    ));

    // —— 灯笼：挂梁上（TOP_CENTER 锚点），光晕单独一只会呼吸的圆环 ——
    commands.spawn((
        Sprite::from_color(Color::srgb(0.24, 0.19, 0.14), Vec2::new(150.0, 10.0)),
        Transform::from_xyz(-560.0, 160.0, 4.0),
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("props/lantern.png"),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Anchor::TOP_CENTER,
        Transform::from_xyz(-560.0, 155.0, 5.0),
    ));
    commands.spawn((
        LanternGlow,
        Mesh2d(meshes.add(Annulus::new(34.0, 62.0))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.62, 0.20, 0.10))),
        Transform::from_xyz(-560.0, 123.0, 4.5),
    ));

    // —— 阿燕：十二格连环画在身，先在西头养神 ——
    // ANCHOR: ayan
    let ayan_sheet = asset_server.load("actors/ayan-sheet.png");
    let ayan_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        6,
        2,
        None,
        None,
    ));
    commands.spawn((
        Ayan,
        Sprite::from_atlas_image(
            ayan_sheet,
            TextureAtlas {
                layout: ayan_layout,
                index: 0,
            },
        ),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(WEST_MARK, DOCK_TOP, 4.0).with_scale(Vec3::splat(4.0)),
        FrameClock::new(0.18, 0, 5),
        StageDirection::Resting { until: 2.5 },
    ));
    // ANCHOR_END: ayan

    // —— 乌篷船与梢公：梢公是船的子实体，跟着船一起沉浮 ——
    let boatman_sheet = asset_server.load("actors/shaogong-sheet.png");
    let boatman_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 40),
        4,
        1,
        None,
        None,
    ));
    commands
        .spawn((
            Boat,
            Sprite::from_image(asset_server.load("props/ferry-boat.png")),
            Transform::from_xyz(450.0, -140.0, 3.0).with_scale(Vec3::splat(4.0)),
        ))
        .with_child((
            Sprite::from_atlas_image(
                boatman_sheet,
                TextureAtlas {
                    layout: boatman_layout,
                    index: 0,
                },
            ),
            FrameClock::new(0.25, 0, 3),
            Transform::from_xyz(8.0, 10.0, 0.5),
        ));

    // —— 字幕框：九宫格裱出来的画框，词儿等下一章的 Text2d ——
    commands.spawn((
        Sprite {
            image: asset_server.load("props/scroll-panel.png"),
            custom_size: Some(Vec2::new(760.0, 100.0)),
            color: Color::srgba(1.0, 1.0, 1.0, 0.92),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(12.0),
                max_corner_scale: 4.0,
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, -300.0, 10.0),
    ));

    println!("老雷：《渡口夜话》带妆彩排——月亮、江水、灯笼、船，各就各位。");
    println!("小棠：阿燕的十二格连环画都在身上，梢公的橹也上了弦。");
}

// ANCHOR: direct
/// 给阿燕递口令：养神到点就起步，走到站位就收步
fn direct_ayan(
    time: Res<Time>,
    mut ayan: Single<
        (&mut StageDirection, &mut Transform, &mut Sprite, &mut FrameClock),
        With<Ayan>,
    >,
    mut cues: Local<u32>,
) {
    let now = time.elapsed_secs();
    let (direction, transform, sprite, clock) = &mut *ayan;

    match **direction {
        StageDirection::Resting { until } if now >= until => {
            // 起步：换走路那一行帧，朝另一头去
            let to_x = if transform.translation.x < (WEST_MARK + EAST_MARK) / 2.0 {
                EAST_MARK
            } else {
                WEST_MARK
            };
            **direction = StageDirection::Walking { to_x };
            **clock = FrameClock::new(0.11, 6, 11);
            sprite.texture_atlas.as_mut().unwrap().index = 6;
            sprite.flip_x = to_x < transform.translation.x;

            if *cues < 2 {
                *cues += 1;
                let side = if sprite.flip_x { "西" } else { "东" };
                println!("老雷：阿燕，走到{}头去。", side);
            }
        }
        StageDirection::Walking { to_x } => {
            let step = 150.0 * time.delta_secs();
            let dx = to_x - transform.translation.x;
            if dx.abs() <= step {
                // 收步：回到正面那一行帧，养神三秒半
                transform.translation.x = to_x;
                **direction = StageDirection::Resting { until: now + 3.5 };
                **clock = FrameClock::new(0.18, 0, 5);
                sprite.texture_atlas.as_mut().unwrap().index = 0;

                if *cues == 2 {
                    *cues += 1;
                    println!("场记：到位。后台收声，戏自己走。");
                }
            } else {
                transform.translation.x += step.copysign(dx);
            }
        }
        StageDirection::Resting { .. } => {}
    }
}
// ANCHOR_END: direct

/// 与 Listing 15-4 同款的走马灯：节拍一到拨下一帧
fn advance_frames(time: Res<Time>, mut query: Query<(&mut FrameClock, &mut Sprite)>) {
    for (mut clock, mut sprite) in &mut query {
        if !clock.timer.tick(time.delta()).just_finished() {
            continue;
        }
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index >= clock.last {
                clock.first
            } else {
                atlas.index + 1
            };
        }
    }
}

/// 船在水上轻轻沉浮，梢公作为子实体跟着一起动
fn rock_the_boat(time: Res<Time>, mut boat: Single<&mut Transform, With<Boat>>) {
    let t = time.elapsed_secs();
    boat.translation.y = -140.0 + 4.0 * (t * 1.3).sin();
    boat.rotation = Quat::from_rotation_z(0.02 * (t * 1.1).sin());
}

// ANCHOR: breathe
/// 灯笼的呼吸：在暗一档与亮一档之间来回 mix 光晕材质的颜色
fn breathe_lantern(
    time: Res<Time>,
    glow: Single<&MeshMaterial2d<ColorMaterial>, With<LanternGlow>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let dim = Color::srgba(1.0, 0.62, 0.20, 0.06);
    let warm = Color::srgba(1.0, 0.68, 0.24, 0.18);
    let breath = 0.5 + 0.5 * (time.elapsed_secs() * 2.1).sin();
    if let Some(mut material) = materials.get_mut(&glow.0) {
        material.color = dim.mix(&warm, breath);
    }
}
// ANCHOR_END: breathe

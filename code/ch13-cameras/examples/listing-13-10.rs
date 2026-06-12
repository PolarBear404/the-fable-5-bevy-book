//! Listing 13-10：右上角挂沙盘——小地图相机与 RenderLayers 防穿帮

use bevy::camera::visibility::RenderLayers;
use bevy::camera::{ScalingMode, Viewport};
use bevy::prelude::*;
use bevy::window::WindowResized;

/// 工作层：马克点、图例住这里——正式机位看不见
const CREW_LAYER: usize = 1;

/// 标记：侠客阿燕
#[derive(Component)]
struct Hero;

/// 标记：主机位
#[derive(Component)]
struct MainLens;

/// 标记：沙盘机位（小地图）
#[derive(Component)]
struct MinimapLens;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.15)))
        .add_systems(Startup, setup)
        .add_systems(Update, ((walk_hero, follow_hero).chain(), dock_minimap))
        .run();
}

fn setup(mut commands: Commands) {
    // ANCHOR: cameras
    // 主机位：什么都不挂，默认只看第 0 层
    commands.spawn((
        Camera2d,
        MainLens,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 600.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    // 沙盘机：order 靠后（叠在主画面上方），看第 0 层和工作层
    commands.spawn((
        Camera2d,
        MinimapLens,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.09, 0.08, 0.07)),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            // 锁定取景 1600×1200 世界单位：整个片场一览无余
            scaling_mode: ScalingMode::Fixed {
                width: 1600.0,
                height: 1200.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        RenderLayers::from_layers(&[0, CREW_LAYER]),
    ));
    // ANCHOR_END: cameras

    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.13, 0.11), Vec2::new(1400.0, 900.0)),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
    for i in -3..=3 {
        for y in [-350.0, 350.0] {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.95, 0.75, 0.25), Vec2::splat(22.0)),
                Transform::from_xyz(i as f32 * 200.0, y, -5.0),
            ));
        }
    }

    // ANCHOR: crew_props
    // 马克点：阿燕的走位记号，只许工作层看见
    for (x, y) in [(0.0, 0.0), (500.0, 0.0), (-500.0, 0.0), (354.0, 250.0), (-354.0, -250.0)] {
        commands.spawn((
            Sprite::from_color(Color::srgb(1.0, 0.3, 0.7), Vec2::splat(16.0)),
            Transform::from_xyz(x, y, -4.0),
            RenderLayers::layer(CREW_LAYER),
        ));
    }

    // 阿燕本体在第 0 层；头顶的沙盘图例在工作层——RenderLayers 不沿层级继承，得自己挂
    commands.spawn((
        Hero,
        Sprite::from_color(Color::srgb(0.85, 0.2, 0.2), Vec2::splat(30.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        children![(
            Sprite::from_color(Color::srgb(1.0, 0.4, 0.4), Vec2::splat(72.0)),
            Transform::from_xyz(0.0, 0.0, 5.0),
            RenderLayers::layer(CREW_LAYER),
        )],
    ));
    // ANCHOR_END: crew_props

    println!("老雷：右上角挂沙盘。马克点只许监视器看见，正片不许穿帮！");
}

// ANCHOR: dock
/// 沙盘停靠右上角：窗口一变就重新量一次
fn dock_minimap(
    windows: Query<&Window>,
    mut resized: MessageReader<WindowResized>,
    mut minimap: Single<&mut Camera, With<MinimapLens>>,
) {
    for message in resized.read() {
        let Ok(window) = windows.get(message.window) else {
            continue;
        };
        // 沙盘宽占窗口四分之一，比例锁 4:3，与取景范围 1600×1200 一致
        let width = window.physical_width() / 4;
        let size = UVec2::new(width, width * 3 / 4);
        minimap.viewport = Some(Viewport {
            // 视口坐标原点在窗口左上角，y 朝下
            physical_position: UVec2::new(window.physical_width() - size.x - 16, 16),
            physical_size: size,
            ..default()
        });
    }
}
// ANCHOR_END: dock

fn walk_hero(time: Res<Time>, mut hero: Single<&mut Transform, With<Hero>>) {
    let t = time.elapsed_secs() * 0.5;
    hero.translation.x = 500.0 * t.sin();
    hero.translation.y = 250.0 * (2.0 * t).sin();
}

fn follow_hero(
    time: Res<Time>,
    mut lens: Single<&mut Transform, (With<MainLens>, Without<Hero>)>,
    hero: Single<&Transform, With<Hero>>,
) {
    let target = hero.translation.with_z(lens.translation.z);
    lens.translation.smooth_nudge(&target, 2.0, time.delta_secs());
}

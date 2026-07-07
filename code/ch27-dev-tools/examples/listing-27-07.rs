//! Listing 27-7：一劳永逸的粉线——保留模式。
//! 后台定位线网几百根线只搭一次，存成资产挂在实体上；方向键整体挪动。

use bevy::color::palettes::css::*;
use bevy::prelude::*;

// ANCHOR: build
/// 标记：挂着定位线网的那只实体
#[derive(Component)]
struct LocatorGrid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (chalk_live_cursor, nudge_grid))
        .run();
}

fn setup(mut commands: Commands, mut gizmo_assets: ResMut<Assets<GizmoAsset>>) {
    commands.spawn(Camera2d);

    // 一次性把整张定位线网画进 GizmoAsset——画法和 Gizmos 一模一样
    let mut locator = GizmoAsset::new();

    // 纵横基准线：23 纵 + 11 横 = 34 根
    for i in -11..=11 {
        let x = i as f32 * 56.0;
        locator.line_2d(Vec2::new(x, -330.0), Vec2::new(x, 330.0), GRAY.with_alpha(0.6));
    }
    for j in -5..=5 {
        let y = j as f32 * 60.0;
        locator.line_2d(Vec2::new(-620.0, y), Vec2::new(620.0, y), GRAY.with_alpha(0.6));
    }
    // 交叉点上的定位星：11×23 组小叉，共 253 个
    for i in -11..=11 {
        for j in -5..=5 {
            let point = Vec2::new(i as f32 * 56.0, j as f32 * 60.0);
            locator.cross_2d(point, 6.0, TEAL);
        }
    }
    // 台心的招牌记号
    locator.circle_2d(Vec2::ZERO, 90.0, GOLD).resolution(48);
    locator.text_2d(Isometry2d::IDENTITY, "CENTER", 24.0, Vec2::ZERO, GOLD);

    // 交货前清点：GizmoAsset 里攒下了多少顶点，账本自己会说话
    let view = locator.buffer();
    println!(
        "检场：定位线网一次搭好——独立线段 {} 根、折线顶点 {} 个，再不重画。方向键整体挪。",
        view.list_positions.len() / 2,
        view.strip_positions.len()
    );

    // 挂上实体：Gizmo 组件 + 逐实体的线宽配置
    commands.spawn((
        LocatorGrid,
        Gizmo {
            handle: gizmo_assets.add(locator),
            line_config: GizmoLineConfig {
                width: 1.5,
                ..default()
            },
            depth_bias: 0.0,
        },
    ));
}
// ANCHOR_END: build

// ANCHOR: coexist
/// 即时模式照常干活：光标圈每帧跟着鼠标画，与保留线网同台不打架
fn chalk_live_cursor(
    mut gizmos: Gizmos,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = cameras.single() else { return };
    let Some(cursor) = window.cursor_position() else { return };
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor) else { return };
    gizmos.circle_2d(point, 24.0, ORANGE_RED);
}

/// 保留模式的线网挂在实体上，就吃实体的 Transform：整体平移一行搞定
fn nudge_grid(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut grids: Query<&mut Transform, With<LocatorGrid>>,
    time: Res<Time>,
) {
    let mut step = Vec2::ZERO;
    if keyboard.pressed(KeyCode::ArrowLeft) { step.x -= 1.0; }
    if keyboard.pressed(KeyCode::ArrowRight) { step.x += 1.0; }
    if keyboard.pressed(KeyCode::ArrowDown) { step.y -= 1.0; }
    if keyboard.pressed(KeyCode::ArrowUp) { step.y += 1.0; }
    for mut transform in &mut grids {
        transform.translation += (step * 160.0 * time.delta_secs()).extend(0.0);
    }
}
// ANCHOR_END: coexist

//! Listing 28-14：玻璃跟着哪台相机走——默认跟 order 最高的；
//! IsDefaultUiCamera 一枚官印钦点；UiTargetCamera 点名到户，谁也不用抢。
//! 按 I 轮转官印（无 → 给全景 → 两台都给），按 T 拨点名，空格报布局用的画幅。

use bevy::camera::Viewport;
use bevy::prelude::*;

/// 全景相机（order 0，铺满窗口）
#[derive(Component)]
struct WideCamera;

/// 角落相机（order 1，只占右下一角）
#[derive(Component)]
struct CornerCamera;

/// 那块用来试跟班的牌子（UI 树的根）
#[derive(Component)]
struct Billboard;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_seal, toggle_assignment, report_canvas))
        .run();
}

// ANCHOR: setup
fn setup(mut commands: Commands, window: Single<&Window>) {
    // 台上摆一块釉青色板作布景。没有它，哪台相机视野里都空无一物——
    // 2D 渲染遇到没东西可画的相机干脆不开工，连清屏色都不落地，
    // 角落相机的"地盘"就看不出来了
    commands.spawn(Sprite::from_color(
        Color::srgb(0.38, 0.65, 0.66),
        Vec2::new(220.0, 120.0),
    ));

    // 全景相机：order 0，清屏黛蓝
    commands.spawn((
        WideCamera,
        Camera2d,
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.13, 0.17, 0.22)),
            ..default()
        },
    ));

    // 角落相机：order 1，视口只占窗口右下四分之一。
    // 不写任何指派时，UI 跟的是它——order 大者当家。
    // （它的视野里也有那块色板：画幅的地界就靠色板的第二次亮相来辨认——
    // 同一窗口上后画的相机，自己的清屏色是不落地的，别指望靠底色划界）
    let quarter = window.physical_size() / 2;
    commands.spawn((
        CornerCamera,
        Camera2d,
        Camera {
            order: 1,
            viewport: Some(Viewport {
                physical_position: quarter,
                physical_size: quarter,
                ..default()
            }),
            ..default()
        },
    ));

    // 一块占画幅八成宽的牌子：画幅一换，它的实际尺寸跟着换
    commands.spawn((
        Billboard,
        Node {
            width: percent(80),
            height: px(80),
            margin: UiRect::all(px(20)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.55, 0.17, 0.12)),
    ));

    println!("水牌师傅：两台相机开着。I 轮官印，T 点名，空格报画幅。");
}
// ANCHOR_END: setup

// ANCHOR: seal
/// I：官印轮着盖——第一下给全景相机，第二下两台都盖（看引擎怎么骂），
/// 第三下全收回
fn rotate_seal(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    wide: Single<Entity, With<WideCamera>>,
    corner: Single<Entity, With<CornerCamera>>,
    mut stage: Local<u8>,
) {
    if !keys.just_pressed(KeyCode::KeyI) {
        return;
    }
    *stage = (*stage + 1) % 3;
    match *stage {
        1 => {
            commands.entity(*wide).insert(IsDefaultUiCamera);
            println!("  官印盖给全景相机——UI 应声搬回大画幅");
        }
        2 => {
            commands.entity(*corner).insert(IsDefaultUiCamera);
            println!("  两台相机都盖了官印——听引擎发落");
        }
        _ => {
            commands.entity(*wide).remove::<IsDefaultUiCamera>();
            commands.entity(*corner).remove::<IsDefaultUiCamera>();
            println!("  官印全收回——回到 order 说了算");
        }
    }
}
// ANCHOR_END: seal

// ANCHOR: assign
/// T：给牌子挂上/摘下 UiTargetCamera(全景相机)——点名到户，压过一切默认规则
fn toggle_assignment(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    wide: Single<Entity, With<WideCamera>>,
    billboard: Single<(Entity, Option<&UiTargetCamera>), With<Billboard>>,
) {
    if !keys.just_pressed(KeyCode::KeyT) {
        return;
    }
    let (entity, assigned) = *billboard;
    if assigned.is_some() {
        commands.entity(entity).remove::<UiTargetCamera>();
        println!("  摘下点名——回去听默认规则的");
    } else {
        commands.entity(entity).insert(UiTargetCamera(*wide));
        println!("  点名全景相机——官印、order 一概不看");
    }
}
// ANCHOR_END: assign

/// 空格：报牌子眼下按哪块画幅排版
fn report_canvas(
    keys: Res<ButtonInput<KeyCode>>,
    billboard: Single<(&ComputedNode, &ComputedUiRenderTargetInfo), With<Billboard>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    let (computed, target) = *billboard;
    let size = computed.size() * computed.inverse_scale_factor();
    println!(
        "  画幅 {} × {} 逻辑像素，牌子实测 {:.0} × {:.0}",
        target.logical_size().x,
        target.logical_size().y,
        size.x,
        size.y
    );
}

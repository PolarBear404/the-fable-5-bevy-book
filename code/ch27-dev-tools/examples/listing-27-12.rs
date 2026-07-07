//! Listing 27-12：开发工具箱开箱——FPS 水牌、诊断小窗、状态换场播报。
//! 空格搬一垛箱子进出（看口数账跳）；R 换场；C 换水牌颜色；4/5 拨水牌显隐与帧时图。

use bevy::dev_tools::diagnostics_overlay::{
    DiagnosticsOverlay, DiagnosticsOverlayItem, DiagnosticsOverlayPlugin,
    DiagnosticsOverlayStatistic,
};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::dev_tools::states::log_transitions;
use bevy::diagnostic::{DiagnosticPath, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

// ANCHOR: states
/// 后台的两种时辰：歇场与彩排——R 键来回换
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum Backstage {
    #[default]
    Resting,
    Rehearsing,
}
// ANCHOR_END: states

/// 空格搬进搬出的那垛备用箱
#[derive(Component)]
struct SpareCrate;

// ANCHOR: app
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // FPS 水牌：字号、颜色、刷新节奏、帧时图全在 config 里
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont::from_font_size(28.0),
                    text_color: Color::srgb(0.3, 1.0, 0.4),
                    refresh_interval: core::time::Duration::from_millis(100),
                    enabled: true,
                    frame_time_graph_config: FrameTimeGraphConfig {
                        enabled: true,
                        min_fps: 30.0,  // 低于它出红条
                        target_fps: 60.0, // 高于它出绿条
                    },
                },
            },
            // 诊断小窗管家 + 口数账（fps 账由 FpsOverlayPlugin 捎带挂上了）
            DiagnosticsOverlayPlugin,
            EntityCountDiagnosticsPlugin::default(),
        ))
        .init_state::<Backstage>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                haul_crates,
                switch_backstage,
                tune_overlay,
                // 状态换场播报员：泛型系统，塞进哪个状态就播报哪个
                log_transitions::<Backstage>,
            ),
        )
        .run();
}
// ANCHOR_END: app

// ANCHOR: windows
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 现成的 fps 小窗：标题栏可拖动、单击标题可折叠
    commands.spawn(DiagnosticsOverlay::fps());

    // 自订的"场记台账"小窗：口数账 + 一条没立过的账（看它怎么显示）
    commands.spawn(DiagnosticsOverlay::new(
        "Stage ledger",
        vec![
            DiagnosticsOverlayItem {
                path: EntityCountDiagnosticsPlugin::ENTITY_COUNT,
                statistic: DiagnosticsOverlayStatistic::Value,
                precision: 0,
            },
            DiagnosticsOverlayItem {
                path: FrameTimeDiagnosticsPlugin::FRAME_TIME,
                statistic: DiagnosticsOverlayStatistic::Smoothed,
                precision: 2,
            },
            // 故意记一条从没 register 过的名目
            DiagnosticPath::new("stage/confetti").into(),
        ],
    ));

    println!("检场：水牌挂好。空格搬箱，R 换场，C 换色，4 收水牌，5 收帧时图。");
}
// ANCHOR_END: windows

/// 空格：一垛 200 只备用箱搬进/搬出——口数账应声起落
fn haul_crates(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    spares: Query<Entity, With<SpareCrate>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    if spares.is_empty() {
        for i in 0..200u32 {
            let col = (i % 20) as f32;
            let row = (i / 20) as f32;
            commands.spawn((
                SpareCrate,
                Sprite::from_color(
                    Color::srgb(0.42, 0.36, 0.28),
                    Vec2::splat(26.0),
                ),
                Transform::from_xyz(-310.0 + col * 32.0, -260.0 + row * 32.0, 0.0),
            ));
        }
        println!("检场：两百只备用箱搬进后台。");
    } else {
        for entity in &spares {
            commands.entity(entity).despawn();
        }
        println!("检场：备用箱全数搬走。");
    }
}

/// R：歇场 ↔ 彩排。播报的活儿归 log_transitions，这里只管拨状态
fn switch_backstage(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<Backstage>>,
    mut next: ResMut<NextState<Backstage>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        next.set(match state.get() {
            Backstage::Resting => Backstage::Rehearsing,
            Backstage::Rehearsing => Backstage::Resting,
        });
    }
}

// ANCHOR: tune
/// 水牌的一切都在 FpsOverlayConfig 资源里：改资源，牌面立刻跟着变
fn tune_overlay(keyboard: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        overlay.text_color = if overlay.text_color == Color::srgb(0.3, 1.0, 0.4) {
            Color::srgb(1.0, 0.75, 0.25)
        } else {
            Color::srgb(0.3, 1.0, 0.4)
        };
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        overlay.enabled = !overlay.enabled;
    }
    if keyboard.just_pressed(KeyCode::Digit5) {
        overlay.frame_time_graph_config.enabled = !overlay.frame_time_graph_config.enabled;
    }
}
// ANCHOR_END: tune

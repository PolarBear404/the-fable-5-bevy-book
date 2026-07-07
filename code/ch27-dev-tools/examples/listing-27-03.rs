//! Listing 27-3：粉线的规格与分组——走位线、安全线各归各的配置组，各有各的开关。
//! 1/2 各关一组；←/→ 拨走位线线宽；U 换线型；J 换拐角。

use bevy::color::palettes::css::*;
use bevy::prelude::*;

// ANCHOR: groups
/// 走位线：演员照着走的路线，归检场管
#[derive(Default, Reflect, GizmoConfigGroup)]
struct WalkLines;

/// 安全线：台口危险边界，归舞台监督管
#[derive(Default, Reflect, GizmoConfigGroup)]
struct SafetyLines;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_gizmo_group::<WalkLines>()
        .init_gizmo_group::<SafetyLines>()
        .add_systems(Startup, setup)
        .add_systems(Update, (chalk_lines, tune_config))
        .run();
}
// ANCHOR_END: groups

// ANCHOR: setup
/// 开台先定规格：走位线粗、安全线虚——改的是配置，不是画法
fn setup(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    commands.spawn(Camera2d);

    let (walk_config, _) = config_store.config_mut::<WalkLines>();
    walk_config.line.width = 8.0;
    walk_config.line.joints = GizmoLineJoint::Miter;

    let (safety_config, _) = config_store.config_mut::<SafetyLines>();
    safety_config.line.width = 4.0;
    safety_config.line.style = GizmoLineStyle::Dashed {
        gap_scale: 2.0,  // 空当长 = 2 倍线宽
        line_scale: 4.0, // 实线长 = 4 倍线宽
    };

    println!("检场：走位线 8 像素带尖角，安全线虚线伺候。");
    println!("检场：1/2 各关一组，左右方向键拨线宽，U 换线型，J 换拐角。");
}
// ANCHOR_END: setup

// ANCHOR: draw
/// 画线的系统只管画：往哪个组画，就受哪个组的规格管
fn chalk_lines(mut walk: Gizmos<WalkLines>, mut safety: Gizmos<SafetyLines>) {
    // 走位线：之字急拐，拐角样式一眼可辨
    walk.linestrip_2d(
        [
            Vec2::new(-480.0, -160.0),
            Vec2::new(-200.0, 140.0),
            Vec2::new(40.0, -140.0),
            Vec2::new(300.0, 120.0),
            Vec2::new(480.0, -60.0),
        ],
        GOLD,
    );

    // 安全线：台口一圈虚线框
    safety.rect_2d(Isometry2d::IDENTITY, Vec2::new(1100.0, 500.0), ORANGE_RED);
}
// ANCHOR_END: draw

// ANCHOR: tune
/// 拨规格的系统只碰 GizmoConfigStore，一根线也不画
fn tune_config(keyboard: Res<ButtonInput<KeyCode>>, mut config_store: ResMut<GizmoConfigStore>) {
    let (walk_config, _) = config_store.config_mut::<WalkLines>();

    if keyboard.just_pressed(KeyCode::Digit1) {
        walk_config.enabled = !walk_config.enabled;
        println!(
            "检场：走位线{}。",
            if walk_config.enabled { "回来了" } else { "全下" }
        );
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        walk_config.line.width = (walk_config.line.width + 0.5).min(40.0);
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        walk_config.line.width = (walk_config.line.width - 0.5).max(1.0);
    }
    if keyboard.just_pressed(KeyCode::KeyU) {
        walk_config.line.style = match walk_config.line.style {
            GizmoLineStyle::Solid => GizmoLineStyle::Dotted,
            GizmoLineStyle::Dotted => GizmoLineStyle::Dashed {
                gap_scale: 3.0,
                line_scale: 5.0,
            },
            _ => GizmoLineStyle::Solid,
        };
        println!("检场：走位线改 {:?}。", walk_config.line.style);
    }
    if keyboard.just_pressed(KeyCode::KeyJ) {
        walk_config.line.joints = match walk_config.line.joints {
            GizmoLineJoint::None => GizmoLineJoint::Miter,
            GizmoLineJoint::Miter => GizmoLineJoint::Round(8),
            GizmoLineJoint::Round(_) => GizmoLineJoint::Bevel,
            GizmoLineJoint::Bevel => GizmoLineJoint::None,
        };
        println!("检场：拐角改 {:?}。", walk_config.line.joints);
    }

    if keyboard.just_pressed(KeyCode::Digit2) {
        let (safety_config, _) = config_store.config_mut::<SafetyLines>();
        safety_config.enabled = !safety_config.enabled;
        println!(
            "舞台监督：安全线{}。",
            if safety_config.enabled { "挂回去" } else { "摘了" }
        );
    }
}
// ANCHOR_END: tune

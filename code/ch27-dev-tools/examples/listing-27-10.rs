//! Listing 27-10：自记一笔——给自家数据立一本账。
//! 空格撒一把纸屑，账本记"台上纸屑数"；E 合上账本，看测量当场停摆。

use bevy::diagnostic::{
    Diagnostic, DiagnosticPath, Diagnostics, DiagnosticsStore, LogDiagnosticsPlugin,
    RegisterDiagnostic,
};
use bevy::prelude::*;

// ANCHOR: register
/// 账目要有独一无二的名目：`/` 分层，const 可当常量用
const CONFETTI_COUNT: DiagnosticPath = DiagnosticPath::const_new("stage/confetti");

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LogDiagnosticsPlugin::default()))
        // 先立账：名目 + 单位后缀 + 留 60 条历史（默认 120）
        .register_diagnostic(
            Diagnostic::new(CONFETTI_COUNT)
                .with_suffix(" 片")
                .with_max_history_length(60),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (throw_confetti, fade_confetti, count_confetti, close_book))
        .run();
}
// ANCHOR_END: register

/// 一片纸屑：往下飘，寿数尽了自己走
#[derive(Component)]
struct Confetti {
    fall_speed: f32,
    life: Timer,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    println!("检场：空格撒纸屑，E 合账本再撒一把试试。");
}

// ANCHOR: spawn
/// 空格撒一把：80 片，位置与落速用手搓的伪随机撒开
fn throw_confetti(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    for i in 0..80u32 {
        // 定数伪随机：黄金角散点，别拉 rand 依赖
        let angle = i as f32 * 2.399_963;
        let radius = (i as f32).sqrt() * 34.0;
        let position = Vec2::new(angle.cos(), angle.sin()) * radius + Vec2::Y * 200.0;
        let hue = (i as f32 * 47.0) % 360.0;
        commands.spawn((
            Confetti {
                fall_speed: 60.0 + (i % 7) as f32 * 25.0,
                life: Timer::from_seconds(2.0 + (i % 5) as f32 * 0.8, TimerMode::Once),
            },
            Sprite::from_color(Color::hsl(hue, 0.8, 0.6), Vec2::splat(9.0)),
            Transform::from_translation(position.extend(0.0)),
        ));
    }
    println!("检场：撒——80 片下去了。");
}

fn fade_confetti(
    mut commands: Commands,
    mut confetti: Query<(Entity, &mut Confetti, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut piece, mut transform) in &mut confetti {
        transform.translation.y -= piece.fall_speed * time.delta_secs();
        if piece.life.tick(time.delta()).is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
// ANCHOR_END: spawn

// ANCHOR: measure
/// 记账系统：每帧添一条测量。闭包懒求值——账本合着时连数都不数
fn count_confetti(mut diagnostics: Diagnostics, confetti: Query<(), With<Confetti>>) {
    diagnostics.add_measurement(&CONFETTI_COUNT, || confetti.iter().count() as f64);
}

/// E 键合上/翻开这本账：停的是测量与播报，历史还留着
fn close_book(keyboard: Res<ButtonInput<KeyCode>>, mut store: ResMut<DiagnosticsStore>) {
    if keyboard.just_pressed(KeyCode::KeyE)
        && let Some(diagnostic) = store.get_mut(&CONFETTI_COUNT)
    {
        diagnostic.is_enabled = !diagnostic.is_enabled;
        println!(
            "场记：纸屑这本账{}。",
            if diagnostic.is_enabled { "重新开记" } else { "合上了" }
        );
    }
}
// ANCHOR_END: measure

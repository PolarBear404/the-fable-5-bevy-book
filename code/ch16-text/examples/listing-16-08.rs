//! Listing 16-8：打字机——运行时改 Text2d，排版自动跟上

use bevy::prelude::*;
use bevy::text::TextBounds;

// ANCHOR: resource
/// 提词器：整句台词攥在手里，按节拍一个字一个字递出去
#[derive(Resource)]
struct Teleprompter {
    script: Vec<char>,
    handed_out: usize,
    beat: Timer,
}
// ANCHOR_END: resource

/// 标记：字幕框里那行正在长出来的词
#[derive(Component)]
struct SubtitleLine;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Teleprompter {
            script: "夜渡无人，秋水自横。客官，要渡江么？".chars().collect(),
            handed_out: 0,
            beat: Timer::from_seconds(0.15, TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, type_out)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("props/scroll-panel.png"),
            custom_size: Some(Vec2::new(720.0, 100.0)),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(12.0),
                max_corner_scale: 4.0,
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, -240.0, 0.0),
        children![(
            SubtitleLine,
            // 一开始是空字符串——词都还在提词器手里
            Text2d::new(""),
            TextFont {
                font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
                font_size: FontSize::Px(30.0),
                ..default()
            },
            TextColor(Color::srgb(0.24, 0.16, 0.08)),
            TextBounds::new_horizontal(660.0),
            Transform::from_translation(Vec3::Z),
        )],
    ));

    println!("场记：提词就位。一拍一个字。");
}

// ANCHOR: type_out
/// 节拍一到就往 Text2d 里添一个字。改组件本身，重排版引擎自己来
fn type_out(
    time: Res<Time>,
    mut prompter: ResMut<Teleprompter>,
    mut line: Single<&mut Text2d, With<SubtitleLine>>,
) {
    if !prompter.beat.tick(time.delta()).just_finished() {
        return;
    }
    if let Some(&next) = prompter.script.get(prompter.handed_out) {
        line.push(next);
        prompter.handed_out += 1;
        if prompter.handed_out == prompter.script.len() {
            println!("场记：整句递完，{} 个字。", prompter.handed_out);
        }
    }
}
// ANCHOR_END: type_out

//! score：记账与挂牌——听 game 的 Knock 消息记分，把分数与命数写上牌面。
//! 对外只承诺一样：`Score` 资源。

use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::game::{BALL_COUNT, Knock, LEFT_WALL, Lives, RIGHT_WALL, TOTAL_BRICKS, WALL_THICKNESS};
use crate::{FONT_BOLD, FONT_REGULAR, GameState, MUTED_COLOR, TEXT_COLOR};

/// 记分牌一行字的高度（顶墙上方）
const BOARD_Y: f32 = 302.0;

/// 战果：碎了几片瓦
#[derive(Resource, Default)]
pub struct Score(pub u32);

/// 记分牌（左上角那行字）
#[derive(Component)]
struct ScoreBoard;

/// 右上角的命数牌
#[derive(Component)]
struct LivesBoard;

// ANCHOR: plugin
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnEnter(GameState::Playing), rig_scoreboard)
            .add_systems(
                Update,
                (
                    tally,
                    refresh_scoreboard.run_if(resource_changed::<Score>),
                    refresh_lives.run_if(resource_changed::<Lives>),
                )
                    .chain(),
            );
    }
}
// ANCHOR_END: plugin

/// 记分牌与命数牌：进局挂出来，离开 Playing 由引擎收走
fn rig_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    score.0 = 0; // 新的一局，旧账清零
    let bold = asset_server.load(FONT_BOLD);
    commands.spawn((
        ScoreBoard,
        DespawnOnExit(GameState::Playing),
        Text2d::new(format!("瓦 0/{TOTAL_BRICKS}")),
        TextFont {
            font: bold.clone().into(),
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(TEXT_COLOR),
        Anchor::CENTER_LEFT,
        Transform::from_xyz(LEFT_WALL - WALL_THICKNESS / 2.0, BOARD_Y, 5.0),
    ));
    commands.spawn((
        LivesBoard,
        DespawnOnExit(GameState::Playing),
        Text2d::new(format!("绣球 ×{BALL_COUNT}")),
        TextFont {
            font: bold.into(),
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(TEXT_COLOR),
        Anchor::CENTER_RIGHT,
        Transform::from_xyz(RIGHT_WALL + WALL_THICKNESS / 2.0, BOARD_Y, 5.0),
    ));
    commands.spawn((
        DespawnOnExit(GameState::Playing),
        Text2d::new("A/D 推凳　　空格 发球　　P 中场"),
        TextFont {
            font: asset_server.load(FONT_REGULAR).into(),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(MUTED_COLOR),
        Transform::from_xyz(0.0, BOARD_Y, 5.0),
    ));
}

/// 听 Knock 记账：只有 Shatter 算分，里程碑配一句台词
fn tally(mut knocks: MessageReader<Knock>, mut score: ResMut<Score>) {
    for knock in knocks.read() {
        if matches!(knock, Knock::Shatter) {
            score.0 += 1;
            match score.0 {
                1 => println!("场记：头一片，开张。"),
                28 => println!("场记：过半了——还剩 28 片。"),
                50 => println!("场记：还剩 6 片，稳着点。"),
                _ => {}
            }
        }
    }
}

/// 分数变了才重排版——第 5 章的资源变更检测在调度层把关
fn refresh_scoreboard(score: Res<Score>, mut board: Single<&mut Text2d, With<ScoreBoard>>) {
    board.0 = format!("瓦 {}/{}", score.0, TOTAL_BRICKS);
}

fn refresh_lives(lives: Res<Lives>, mut board: Single<&mut Text2d, With<LivesBoard>>) {
    board.0 = format!("绣球 ×{}", lives.0);
}

//! menu：画面流程——后台招牌、结算屏、中场幕布，以及驱动状态机的那几个键。
//! 它不懂玩法，只看 `Outcome` 与 `Score` 的脸色行事。

use bevy::prelude::*;

use crate::game::{Outcome, TOTAL_BRICKS};
use crate::score::Score;
use crate::{FONT_BOLD, FONT_REGULAR, GameState, IsPaused, MUTED_COLOR, TEXT_COLOR};

// ANCHOR: plugin
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, greet)
            .add_systems(OnEnter(GameState::Menu), show_menu)
            .add_systems(OnEnter(GameState::GameOver), show_curtain)
            .add_systems(OnEnter(IsPaused::Paused), show_intermission)
            .add_systems(
                Update,
                (
                    menu_keys.run_if(in_state(GameState::Menu)),
                    curtain_keys.run_if(in_state(GameState::GameOver)),
                    pause_keys.run_if(in_state(GameState::Playing)),
                    quit_from_pause.run_if(in_state(IsPaused::Paused)),
                ),
            );
    }
}
// ANCHOR_END: plugin

fn greet() {
    println!("老雷：夜戏散了，伙计们后台耍一局《打瓦》——空格开局。");
}

/// 后台招牌：进 Menu 搭，出 Menu 引擎拆
fn show_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold = asset_server.load(FONT_BOLD);
    let regular = asset_server.load(FONT_REGULAR);
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Transform::default(),
        Visibility::default(),
        children![
            (
                Text2d::new("打　瓦"),
                TextFont {
                    font: bold,
                    font_size: 110.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Transform::from_xyz(0.0, 110.0, 5.0),
            ),
            (
                Text2d::new("夜戏散场后的保留节目"),
                TextFont {
                    font: regular.clone(),
                    font_size: 26.0,
                    ..default()
                },
                TextColor(MUTED_COLOR),
                Transform::from_xyz(0.0, 28.0, 5.0),
            ),
            (
                Text2d::new("空格 开局　　Esc 离场"),
                TextFont {
                    font: regular,
                    font_size: 30.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Transform::from_xyz(0.0, -96.0, 5.0),
            ),
        ],
    ));
}

// ANCHOR: show_curtain
/// 结算屏：标题看 Outcome，分数行读 Score——这两样是别的插件的承诺
fn show_curtain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    outcome: Res<Outcome>,
    score: Res<Score>,
) {
    let (headline, verdict) = match *outcome {
        Outcome::Cleared => ("满堂彩！", format!("{TOTAL_BRICKS} 片瓦，一片不剩")),
        Outcome::Spilled => ("绣球散尽", format!("这局砸下 {} 片瓦", score.0)),
    };
    println!("场记：{headline}——{verdict}。");
    commands.spawn((
        DespawnOnExit(GameState::GameOver),
        Transform::default(),
        Visibility::default(),
        children![
            (
                Text2d::new(headline),
                TextFont {
                    font: asset_server.load(FONT_BOLD),
                    font_size: 84.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Transform::from_xyz(0.0, 90.0, 5.0),
            ),
            (
                Text2d::new(verdict),
                TextFont {
                    font: asset_server.load(FONT_REGULAR),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(MUTED_COLOR),
                Transform::from_xyz(0.0, 10.0, 5.0),
            ),
            (
                Text2d::new("空格 再来一局　　Esc 回后台"),
                TextFont {
                    font: asset_server.load(FONT_REGULAR),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Transform::from_xyz(0.0, -96.0, 5.0),
            ),
        ],
    ));
}
// ANCHOR_END: show_curtain

/// 中场幕布：进 Paused 搭，出 Paused 引擎拆
fn show_intermission(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        DespawnOnExit(IsPaused::Paused),
        Transform::default(),
        Visibility::default(),
        children![
            (
                Text2d::new("中　场"),
                TextFont {
                    font: asset_server.load(FONT_BOLD),
                    font_size: 84.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Transform::from_xyz(0.0, 40.0, 10.0),
            ),
            (
                Text2d::new("P 继续　　Esc 收摊回后台"),
                TextFont {
                    font: asset_server.load(FONT_REGULAR),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(MUTED_COLOR),
                Transform::from_xyz(0.0, -60.0, 10.0),
            ),
        ],
    ));
}

/// 后台的键：空格开局，Esc 离场
fn menu_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("老雷：收摊。各回各屋。");
        exit.write(AppExit::Success);
    }
}

/// 结算屏的键：空格再来，Esc 回后台
fn curtain_keys(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

/// P：中场与开演来回切
fn pause_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    paused: Res<State<IsPaused>>,
    mut next_paused: ResMut<NextState<IsPaused>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        next_paused.set(match paused.get() {
            IsPaused::Running => IsPaused::Paused,
            IsPaused::Paused => IsPaused::Running,
        });
    }
}

/// 中场里按 Esc：弃局回后台
fn quit_from_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("场记：这局不打了，瓦留给明儿个。");
        next_state.set(GameState::Menu);
    }
}

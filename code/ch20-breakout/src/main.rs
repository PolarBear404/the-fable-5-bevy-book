//! 《打瓦》——本书第三部分的收官实战（与 Listing 20-7 行为完全相同，换了住址）
//! main.rs 只干三件事：声明模块、定义全场公认的状态机与配色、装配 App。
//! 玩法在 game，画面流程在 menu，记分在 score，锣鼓在 audio。

// ANCHOR: mods
mod audio;
mod game;
mod menu;
mod score;
// ANCHOR_END: mods

use bevy::prelude::*;

// ANCHOR: states
/// 全场只有三种活法：后台待客、一局进行中、一局收场——所有插件都认它
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

/// 中场：只在一局进行中才存在的小状态机（第 10 章的 SubStates）
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::Playing)]
pub enum IsPaused {
    #[default]
    Running,
    Paused,
}
// ANCHOR_END: states

// ANCHOR: theme
// 全书统一的“美术指导”：幕布、字色与字模，谁的招牌都从这儿取料
const BACKDROP: Color = Color::srgb(0.10, 0.10, 0.14);
pub const TEXT_COLOR: Color = Color::srgb(0.91, 0.88, 0.80);
pub const MUTED_COLOR: Color = Color::srgb(0.55, 0.57, 0.62);
pub const FONT_BOLD: &str = "fonts/book-sans-sc-bold.otf";
pub const FONT_REGULAR: &str = "fonts/book-sans-sc-regular.otf";
// ANCHOR_END: theme

// ANCHOR: main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "打瓦".into(),
                // 下面两行只在网页版里生效（桌面构建原样忽略）：
                // 把画面挂到页面上指定的 <canvas>，尺寸跟着它的父元素走
                canvas: Some("#bevy-canvas".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKDROP))
        .init_state::<GameState>()
        .add_sub_state::<IsPaused>()
        .add_plugins((
            game::GamePlugin,
            menu::MenuPlugin,
            score::ScorePlugin,
            audio::SoundPlugin,
        ))
        .add_systems(Startup, rig_camera)
        .run();
}

/// 相机不归任何一摊管，留在总装线上
fn rig_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
// ANCHOR_END: main

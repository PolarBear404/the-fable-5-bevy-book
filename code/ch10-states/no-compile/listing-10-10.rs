//! Listing 10-10：行不通——计算状态由 compute 推导，没有 NextState 可写

use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing {
        demo: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InAction;

impl ComputedStates for InAction {
    type SourceStates = GameState;

    fn compute(source: GameState) -> Option<Self> {
        matches!(source, GameState::Playing { .. }).then_some(InAction)
    }
}

fn main() {
    App::new().add_systems(Update, cheat).run();
}

// ANCHOR: cheat
/// 想绕过投币直接把机器拧到"过招中"——行不通
fn cheat(mut next: ResMut<NextState<InAction>>) {
    next.set(InAction);
}
// ANCHOR_END: cheat

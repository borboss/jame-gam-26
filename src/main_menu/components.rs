use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    InGame,
    #[default] MainMenu,
    PausedGame,
    GameOver
}
use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
           .add_systems(Startup, setup_game)
           .add_systems(Update, check_game_over_condition);
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Resource)]
pub struct PlayerResource {
    pub gold: u32,
    pub lives: u32,
    pub score: u32,
}

fn setup_game() {
    // Initialize game state and resources
}

fn check_game_over_condition() {
    // Check if player has lost or won
}


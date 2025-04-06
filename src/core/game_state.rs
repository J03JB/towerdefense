use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
           .init_resource::<PlayerResource>()
           .add_systems(Startup, setup_game)
           .add_systems(OnEnter(GameState::Playing), reset_player_resources)
           .add_systems(Update, check_game_over_condition.run_if(in_state(GameState::Playing)))
           .add_systems(Update, handle_pause.run_if(in_state(GameState::Playing)))
           .add_systems(Update, handle_resume.run_if(in_state(GameState::Paused)));
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Editor,
}

#[derive(Resource, Default)]
pub struct PlayerResource {
    pub money: u32,
    pub health: u32,
    pub score: u32,
}

fn setup_game(mut commands: Commands) {
    commands.insert_resource(PlayerResource {
        money: 100,
        health: 100,
        score: 0,
    });
}

fn check_game_over_condition(
    player_resource: Res<PlayerResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Game over when player runs out of lives
    if player_resource.health == 0 {
        next_state.set(GameState::GameOver);
    }
}

// Reset player resources when starting a new game
pub fn reset_player_resources(mut player_resource: ResMut<PlayerResource>) {
    *player_resource = PlayerResource {
        money: 100,
        health: 100,
        score: 0,
    };
}

fn handle_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn handle_resume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Playing);
    }
}

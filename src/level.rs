use bevy::prelude::*;
use crate::enemy::EnemyType;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level)
           .add_systems(Update, check_wave_progress);
    }
}

#[derive(Resource)]
pub struct Level {
    pub current_level: u32,
    pub path: Vec<Vec2>,
    pub waves: Vec<Wave>,
    pub current_wave: usize,
}

pub struct Wave {
    pub enemy_counts: Vec<(EnemyType, u32)>,
    pub spawn_interval: f32,
    pub wave_delay: f32,
}

fn setup_level() {
    // Initialize the game map and paths
}

fn check_wave_progress() {
    // Check if current wave is complete and spawn next wave
}



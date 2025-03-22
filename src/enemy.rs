// src/enemy.rs
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
           .add_systems(Update, (move_enemies, check_enemy_health));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub reward: u32,
    pub enemy_type: EnemyType,
}

#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    Basic,
    Fast,
    Tank,
    Boss,
}

fn setup_enemies() {
    // Initialize enemy-related resources
}

fn move_enemies() {
    // Logic for enemy movement along path
}

fn check_enemy_health() {
    // Check if enemies are defeated or reached the end
}



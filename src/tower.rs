// src/tower.rs
use bevy::prelude::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_towers)
           .add_systems(Update, (tower_targeting, tower_shooting, handle_tower_upgrades));
    }
}

#[derive(Component)]
pub struct Tower {
    pub tower_type: TowerType,
    pub range: f32,
    pub damage: f32,
    pub fire_rate: f32,
    pub last_shot: f32,
    pub level: u32,
    pub target: Option<Entity>,
}

#[derive(Debug, Clone, Copy)]
pub enum TowerType {
    Basic,
    Sniper,
    Splash,
    Slow,
}

fn setup_towers() {
    // Initialize tower-related resources
}

fn tower_targeting() {
    // Logic for towers to find and select targets
}

fn tower_shooting() {
    // Logic for towers to shoot at targets
}

fn handle_tower_upgrades() {
    // Handle tower upgrades and improvements
}


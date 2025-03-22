// src/ui.rs
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, (update_ui, handle_ui_interaction));
    }
}

fn setup_ui() {
    // Set up game UI elements like health, resources, tower selection
}

fn update_ui() {
    // Update UI based on game state
}

fn handle_ui_interaction() {
    // Handle player interaction with UI
}


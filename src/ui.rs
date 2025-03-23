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
    fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI container anchored to the top-right
    commands.spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            width: Val::Px(200.0),
            height: Val::Px(600.0),
            ..default()
    },
        )
    .with_children(|parent| {
        // Add UI elements as children
        // ...
    });
}
    // Set up game UI elements like health, resources, tower selection
}

fn update_ui() {
    // Update UI based on game state
}

fn handle_ui_interaction() {
    // Handle player interaction with UI
}


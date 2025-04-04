use bevy::prelude::*;

pub mod components;
pub mod data;
pub mod resources;
pub mod state;
pub mod systems;

pub use state::EditorState;

use crate::core::game_state::GameState;
use crate::level_editor::resources::{EditorData, EditorTextInput};
use crate::levels::level_textures::{cleanup_texture_selector, setup_texture_selector};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<EditorState>()
            .init_resource::<EditorTextInput>()
            .init_resource::<EditorData>() 
            .add_systems(Startup, check_editor_launch_arg)
            .add_systems(
                Update,
                (
                    (
                        systems::editor_input_handler,
                        systems::toggle_editor_tool,
                        systems::spawn_context_menu,
                        systems::handle_context_menu_interaction,
                    )
                        .run_if(|input: Res<EditorTextInput>| !input.dialog_open),
                    systems::update_tool_display_text, 
                    systems::export_level_data, 
                    systems::handle_save_dialog,
                    systems::handle_text_input, 
                    systems::render_editor_path,
                )
                    .run_if(in_state(EditorState::Active)),
            )
            .add_systems(
                OnEnter(EditorState::Active),
                (
                    systems::setup_editor_ui,
                    crate::levels::level_textures::setup_texture_selector,
                ),
            )
            .add_systems(
                OnExit(EditorState::Active),
                (
                    cleanup_editor_ui,
                    cleanup_texture_selector,
                ),
            );

        // TODO: Add systems for OnExit(EditorState::Active) to clean up UI/markers
    }
}

fn check_editor_launch_arg(
    mut editor_state: ResMut<NextState<EditorState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--editor") {
        editor_state.set(EditorState::Active);
        // TODO: add GameState::Editor
        game_state.set(GameState::Playing);
    } else {
        editor_state.set(EditorState::Inactive);
    }
}

fn cleanup_editor_ui(
    mut commands: Commands,
    ui_query: Query<
        Entity,
        Or<(
            With<components::EditorToolDisplay>,
            With<components::SaveDialog>,
            With<components::ContextMenu>,
        )>,
    >,
    marker_query: Query<Entity, With<components::EditorPathMarker>>,
) {
    info!("Cleaning up editor UI and markers...");
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in marker_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// src/levels/level_textures.rs

use bevy::prelude::*;
// Remove direct dependency on EditorData and EditorTool from level_editor
// use crate::level_editor::resources::{EditorData, EditorTool};
use crate::level_editor::resources::EditorData; // Need EditorData to react to state
use crate::level_editor::resources::EditorTool; // Need EditorTool to react to state
use crate::level_editor::state::EditorState; // Needed for system run conditions maybe
use serde::{Deserialize, Serialize};

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AvailableTextures>() // Initialize resource always
            // Systems that run only when editor is active
            .add_systems(
                Update,
                (
                    handle_texture_selection, // Handles clicks inside the panel
                    update_panel_visibility,  // Show/hide panel based on EditorTool
                )
                    .run_if(in_state(EditorState::Active)), // Run only when editor is active
            );
        // setup_texture_selector and cleanup are called by EditorPlugin's OnEnter/OnExit
    }
}

// --- Resources ---

#[derive(Resource, Debug)] // Added Debug
pub struct AvailableTextures {
    pub paths: Vec<String>,
    pub selected: Option<String>, // Keep track of the selected texture path
}

impl Default for AvailableTextures {
    fn default() -> Self {
        let paths = vec![
            "textures/path/path_straight_horizontal.png".to_string(),
            "textures/path/path_straight_vertical.png".to_string(),
            "textures/path/path_corner_bottom_left.png".to_string(),
            "textures/path/path_corner_bottom_right.png".to_string(),
            "textures/path/path_corner_top_left.png".to_string(),
            "textures/path/path_corner_top_right.png".to_string(),
            // Add more paths as needed
            // "textures/path_junction.png".to_string(),
        ];
        Self {
            // Select the first texture by default
            selected: paths.first().cloned(),
            paths,
        }
    }
}

// --- Components ---

#[derive(Component)]
pub struct TextureButton(pub String); // Holds the texture path this button represents

#[derive(Component)]
pub struct TextureSelectorPanel; // Marker component for the root panel node

// --- Structs ---

// Path texture information for serialization (used by level_editor::systems::export)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathTexture {
    pub position: Vec<u32>, // [x, y] grid coordinates
    pub texture: String,    // Texture asset path
}

// --- Systems ---

// This system is now called by the EditorPlugin on entering EditorState::Active
// This system is now called by the EditorPlugin on entering EditorState::Active
pub fn setup_texture_selector(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    textures: Res<AvailableTextures>, // Use the initialized resource
) {
    info!("Setting up texture selector panel...");
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute, // Position relative to window
                right: Val::Px(10.0),
                top: Val::Px(60.0),    // Below the main editor bar
                width: Val::Px(250.0), // Slightly wider for previews
                height: Val::Auto,     // Adjust height based on content
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column, // Stack items vertically
                row_gap: Val::Px(5.0),                 // Space between items
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
            BorderColor(Color::srgb(0.7, 0.7, 0.7)),
            // z_index: ZIndex::Global(50), // Ensure it's above game elements but below context/save menus
            Visibility::Hidden, // Start hidden, visibility controlled by update_panel_visibility
            TextureSelectorPanel, // Mark this node
        ))
        .with_children(|parent| {
            // Panel Title
            parent.spawn(
                (
                    // Using TextBundle here is generally fine even if avoiding others
                    Text::new("Path Textures"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor::WHITE,
                ), // Closing parenthesis for spawn
            ); // Semicolon for the spawn statement

            // Scrollable area for buttons (optional but good for many textures)
            // This block is now correctly nested inside the first with_children
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        height: Val::Px(300.0), // Set a fixed height for scrolling
                        overflow: Overflow::clip_y(), // Enable vertical clipping/scrolling
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
                ))
                .with_children(|parent| {
                    // Spawn buttons for each available texture
                    for texture_path in &textures.paths {
                        // Extract a display name (e.g., file name without extension)
                        let texture_name = texture_path
                            .split('/')
                            .last() // Get filename part
                            .unwrap_or(texture_path)
                            .split('.')
                            .next() // Get part before extension
                            .unwrap_or(texture_path)
                            .replace('_', " ") // Nicer formatting
                            .to_string();

                        // Determine initial background color based on current selection
                        let bg_color = if textures.selected.as_deref() == Some(texture_path) {
                            Color::srgb(0.3, 0.5, 0.3) // Highlight selected
                        } else {
                            Color::srgb(0.15, 0.15, 0.15) // Default
                        };

                        parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::SpaceBetween, // Space out text and image
                                    align_items: AlignItems::Center,
                                    padding: UiRect::horizontal(Val::Px(10.0)),
                                    ..default()
                                },
                                BackgroundColor(bg_color),
                                TextureButton(texture_path.clone()), // Store path in button
                            ))
                            .with_children(|parent| {
                                // Display texture name
                                parent.spawn(
                                    (
                                        // TextBundle is still generally okay
                                        Text::new(texture_name),
                                        TextFont {
                                            font_size: 14.0, // Smaller font
                                            ..default()
                                        },
                                        TextColor::WHITE,
                                    ), // Closing parenthesis for spawn
                                ); // Semicolon for spawn

                                // Display texture preview image
                                // Keeping original structure for ImageNode + Node
                                parent.spawn((
                                    ImageNode {
                                        // Use ImageNode as originally intended
                                        image: asset_server.load(texture_path).into(),
                                        ..default()
                                    },
                                    Node {
                                        // Separate Node for sizing/layout
                                        width: Val::Px(32.0), // Match image size
                                        height: Val::Px(32.0),
                                        ..default()
                                    },
                                )); // Closing parenthesis and semicolon for spawn
                            }); // End button children
                    } // End for loop
                }); // End scrollable area children
        }); // End main panel children
} // End function setup_texture_selector

// Handles clicks on the texture buttons in the panel
pub fn handle_texture_selection(
    mut textures: ResMut<AvailableTextures>, // Resource access is fine
    // Use ParamSet to manage conflicting queries
    mut button_param_set: ParamSet<(
        // Query 0: Find the interacted button (read-only needed for this part)
        Query<(&Interaction, &TextureButton), (Changed<Interaction>, With<Button>)>,
        // Query 1: Update all button colors (mutable access needed here)
        Query<(&TextureButton, &mut BackgroundColor), With<Button>>,
    )>,
) {
    let mut newly_selected_path: Option<String> = None;

    // --- Phase 1: Detect which button was pressed ---
    // Access the first query (index 0) in the ParamSet.
    // We only need to read Interaction and TextureButton here.
    for (interaction, texture_button) in button_param_set.p0().iter() {
        if matches!(interaction, Interaction::Pressed) {
            let clicked_path = texture_button.0.clone();
            // Check if the selection actually changed
            if textures.selected.as_deref() != Some(&clicked_path) {
                info!("Texture selected: {}", clicked_path);
                // Update the resource (this is allowed alongside component access)
                textures.selected = Some(clicked_path.clone());
                // Store the newly selected path to update colors later
                newly_selected_path = Some(clicked_path);
            }
            // Handle only one press per system run for simplicity
            break;
        }
    }

    // --- Phase 2: Update button colors if a new texture was selected ---
    // This part uses the *second* query (index 1) which has mutable access to BackgroundColor.
    // Because we finished using p0() before accessing p1(), Bevy allows this.
    if let Some(selected_path) = newly_selected_path {
        // Access the second query mutably
        for (button, mut color) in button_param_set.p1().iter_mut() {
            // Highlight the selected button, reset others
            if button.0 == selected_path {
                *color = Color::srgb(0.3, 0.5, 0.3).into(); // Highlight color
            } else {
                *color = Color::srgb(0.15, 0.15, 0.15).into(); // Default color
            }
        }
    }
}
// Controls the visibility of the texture panel based on the current EditorTool
pub fn update_panel_visibility(
    editor_data: Res<EditorData>, // Read the current tool state
    mut panel_query: Query<&mut Visibility, With<TextureSelectorPanel>>,
) {
    if editor_data.is_changed() {
        // Only run if editor data (like current_tool) changed
        if let Ok(mut visibility) = panel_query.get_single_mut() {
            let should_be_visible = editor_data.current_tool == EditorTool::TextureSelector;

            let current_visibility = match *visibility {
                Visibility::Hidden => false,
                _ => true,
            };

            if should_be_visible && !current_visibility {
                *visibility = Visibility::Visible;
                info!("Texture panel made visible.");
            } else if !should_be_visible && current_visibility {
                *visibility = Visibility::Hidden;
                info!("Texture panel hidden.");
            }
        }
    }
}

// Helper function used by level_editor to get the currently selected texture
pub fn get_selected_texture(textures: &AvailableTextures) -> String {
    // Return the selected texture, or the first one as a fallback
    textures
        .selected
        .clone()
        .unwrap_or_else(|| textures.paths.first().cloned().unwrap_or_default()) // Provide a default empty string if paths is empty
}

// System to despawn the panel when exiting editor mode
// This is called by the EditorPlugin on exiting EditorState::Active
pub fn cleanup_texture_selector(
    mut commands: Commands,
    panel_query: Query<Entity, With<TextureSelectorPanel>>,
) {
    info!("Cleaning up texture selector panel...");
    for entity in panel_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

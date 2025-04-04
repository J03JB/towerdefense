use bevy::prelude::*;

use super::super::components::*; // Use components from parent module
use super::super::resources::*;
// use crate::levels::level_editor::EditorData; // Use resources from parent module

// This function now only handles spawning the main editor UI bar and buttons
pub fn setup_editor_ui(mut commands: Commands) {
    // Create editor UI Top Bar
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                column_gap: Val::Px(10.0), // Add some gap between items
                position_type: PositionType::Absolute, // Optional: Keep it at the top
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            EditorToolDisplay, // Keep marker component on the root node
        ))
        .with_children(|parent| {
            // Display for current tool text
            parent.spawn((
                Text::new("Editor Mode: Path Placement"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor::WHITE,
            ));

            // --- Tool Buttons ---
            let button_style = Node {
                width: Val::Px(120.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            };
            let button_color = Color::srgb(0.15, 0.15, 0.15);

            // Path Tool Button
            parent
                .spawn((
                    Button,
                    button_style.clone(),
                    BackgroundColor(button_color.into()),
                    EditorButton(EditorTool::PathPlacer),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Path Tool (P)"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor::WHITE,
                    ));
                });

            // Start Point Button
            parent
                .spawn((
                    Button,
                    button_style.clone(),
                    BackgroundColor(button_color),
                    EditorButton(EditorTool::StartPoint),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start (S)"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor::WHITE,
                    ));
                });

            // End Point Button
            parent
                .spawn((
                    Button,
                    button_style.clone(),
                    BackgroundColor(button_color.into()),
                    EditorButton(EditorTool::EndPoint),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("End (E)"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Buildable Area Button
            parent
                .spawn((
                    Button,
                    button_style.clone(),
                    BackgroundColor(button_color.into()),
                    EditorButton(EditorTool::BuildableArea),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Buildable (B)"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor::WHITE,
                    ));
                });
 // Texture Selector Button
            parent
                .spawn((
                    Button,
                        button_style.clone(), // Use the same style as other tool buttons
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    EditorButton(EditorTool::TextureSelector), // Associate with the tool
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Texture Selector"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor::WHITE,
                    ));
                });

            // --- Export Button ---
            // Place it further to the right, maybe?
            parent
                .spawn((
                    Button,
                        Node {
                            margin: UiRect {
                                left: Val::Auto,
                                ..default()
                            }, // Pushes it right
                            ..button_style.clone()
                    },
                        BackgroundColor(Color::srgb(0.1, 0.4, 0.1).into()),
                    ExportButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Export"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor::WHITE,
                    ));
                });
        });
}

// System to handle button clicks for changing tools
pub fn toggle_editor_tool(
    mut interaction_query: Query<
        (&Interaction, &EditorButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut editor_data: ResMut<EditorData>,
    // Query the specific Text child of the EditorToolDisplay node
    mut text_query: Query<(&mut Text, &Parent)>,
    parent_query: Query<&Parent, With<Text>>, // To find the parent
    display_query: Query<Entity, With<EditorToolDisplay>>, // To identify the correct parent
) {
    let display_entity = display_query.get_single().ok(); // Get the UI bar entity

    for (interaction, button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if editor_data.current_tool != button.0 {
                    editor_data.current_tool = button.0.clone();
                    info!("Tool changed via button to: {:?}", editor_data.current_tool);

                    // Update the display text
                    if let Some(display_parent_entity) = display_entity {
                        for (mut text, text_parent) in text_query.iter_mut() {
                            // Check if this text node is a child of the main display node
                            if parent_query.get(text_parent.get()).is_ok()
                                && text_parent.get() == display_parent_entity
                            {
                                *text = Text::new(format!("Editor Mode: {:?}", editor_data.current_tool));
                                break; // Assume only one text display to update
                            }
                        }
                    }
                }
                *color = Color::srgb(0.35, 0.35, 0.35).into(); // Pressed color
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into(); // Hover color
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into(); // Default color
            }
        }
    }
}

// Optional: System to update the tool display text when the tool changes via keyboard
pub fn update_tool_display_text(
    editor_data: Res<EditorData>,
    mut text_query: Query<(&mut Text, &Parent)>,
    parent_query: Query<&Parent, With<Text>>,       // Query parents of text nodes
    display_query: Query<Entity, With<EditorToolDisplay>>, // Query the display node
) {
    if editor_data.is_changed() {
        // Only run if the tool might have changed
        if let Ok(display_entity) = display_query.get_single() {
            for (mut text, text_parent) in text_query.iter_mut() {
                // Check if this text node is a child of the EditorToolDisplay node
                if parent_query.get(text_parent.get()).is_ok()
                    && text_parent.get() == display_entity
                {
                    *text = Text::new(format!("Editor Mode: {:?}", editor_data.current_tool));
                    break; // Found the text to update
                }
            }
        }
    }
}

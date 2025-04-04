use bevy::prelude::*;
use std::path::PathBuf; // For potential future path joining

use crate::core::utils; 

use super::super::components::*; 
use super::super::resources::*; 
use super::export::export_level;


pub fn export_level_data(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExportButton>)>,
    mut editor_text_input: ResMut<EditorTextInput>,
    mut commands: Commands,
    dialog_query: Query<Entity, With<SaveDialog>>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) && !editor_text_input.dialog_open {
            editor_text_input.dialog_open = true;
            for entity in dialog_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            spawn_save_dialog(&mut commands, &editor_text_input); 
        }
    }
}

pub fn handle_text_input(
    mut editor_text_input: ResMut<EditorTextInput>,
    editor_data: Res<EditorData>, 
    mut text_query: Query<&mut Text, With<LevelNameInput>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    dialog_query: Query<Entity, With<SaveDialog>>,
) {
    if !editor_text_input.dialog_open {
        return; 
    }

    let mut name_changed = false;

    for keycode in keyboard_input.get_just_pressed() {
        match keycode {
            KeyCode::Backspace => {
                if !editor_text_input.level_name.is_empty() {
                    editor_text_input.level_name.pop();
                    name_changed = true;
                }
            }
            KeyCode::Space => {
                editor_text_input.level_name.push('_');
                name_changed = true;
            }
            KeyCode::Minus => {
                editor_text_input.level_name.push('-');
                name_changed = true;
            }
            KeyCode::Enter | KeyCode::NumpadEnter => {
                if !editor_text_input.level_name.is_empty() {
                    export_level(&editor_data, &editor_text_input.level_name);
                    editor_text_input.dialog_open = false;
                    for entity in dialog_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    return;
                }
            }
            KeyCode::Escape => {
                editor_text_input.dialog_open = false;
                for entity in dialog_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }
            _ => {
                if let Some(char) = utils::key_to_char(*keycode) {
                    if char.is_alphanumeric() || char == '-' || char == '_' {
                        editor_text_input.level_name.push(char);
                        name_changed = true;
                    }
                }
            }
        }
    }

    if name_changed {
        for mut text in text_query.iter_mut() {
            *text = Text::new(editor_text_input.level_name.clone());
        }
    }
}

pub fn handle_save_dialog(
    mut commands: Commands,
    mut editor_text_input: ResMut<EditorTextInput>,
    cancel_interaction: Query<&Interaction, (Changed<Interaction>, With<CancelSaveButton>)>,
    save_interaction: Query<&Interaction, (Changed<Interaction>, With<ConfirmSaveButton>)>,
    dialog_query: Query<Entity, With<SaveDialog>>,
    editor_data: Res<EditorData>, 
) {
    if !editor_text_input.dialog_open {
        return; 
    }

    for interaction in cancel_interaction.iter() {
        if matches!(interaction, Interaction::Pressed) {
            editor_text_input.dialog_open = false;
            for entity in dialog_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            return; 
        }
    }

    for interaction in save_interaction.iter() {
        if matches!(interaction, Interaction::Pressed) {
            if !editor_text_input.level_name.is_empty() {
                export_level(&editor_data, &editor_text_input.level_name);
                editor_text_input.dialog_open = false;
                for entity in dialog_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return; 
            } else {
                warn!("Level name cannot be empty.");
            }
        }
    }
}

pub fn spawn_save_dialog(commands: &mut Commands, editor_text_input: &EditorTextInput) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute, 
                left: Val::Percent(30.0),              
                top: Val::Percent(30.0),
                width: Val::Px(400.0),
                height: Val::Auto,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column, 
                row_gap: Val::Px(15.0),                
                align_items: AlignItems::Center,       
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor(Color::srgb(0.7, 0.7, 0.7)),
            // z_index: ZIndex::Global(101), // Ensure it's above context menu
            SaveDialog, 
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Save Level As"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor::WHITE,
            ));

            parent
                .spawn((
                    Node {
                        width: Val::Percent(90.0), 
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(1.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::FlexStart, 
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1).into()),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5).into()),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(if editor_text_input.level_name.is_empty() {
                            "Enter level name...".to_string()
                        } else {
                            editor_text_input.level_name.clone()
                        }),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor::WHITE,
                        LevelNameInput, 
                    ));
                });

            parent
                .spawn(Node {
                    width: Val::Percent(90.0),
                    justify_content: JustifyContent::FlexEnd, 
                    column_gap: Val::Px(10.0),                
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.5, 0.1, 0.1).into()),
                            CancelSaveButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Cancel (Esc)"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor::WHITE,
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.5, 0.1).into()),
                            ConfirmSaveButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Save (Enter)"), // Hint
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor::WHITE,
                            ));
                        });
                });
        });
}

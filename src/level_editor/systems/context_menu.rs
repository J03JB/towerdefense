use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::config::CELL_SIZE;
use crate::core::map::Map;
use crate::level_editor::systems::save_dialog::spawn_save_dialog;

use super::super::components::*;
use super::super::resources::*;

fn spawn_menu_option(parent: &mut ChildBuilder, text: &str, option: ContextMenuOption) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                margin: UiRect::vertical(Val::Px(2.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
            option,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor::WHITE,
            ));
        });
}

pub fn spawn_context_menu(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    context_menu_query: Query<Entity, With<ContextMenu>>,
) {
    let window = windows.single();

    if let Some(cursor_translate) = window.cursor_position() {
        let relative_position = (
            (cursor_translate.y / window.height()).abs(),
            (cursor_translate.x / window.width()).abs(),
        );

    if mouse_input.just_pressed(MouseButton::Left) {
        for entity in context_menu_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    if mouse_input.just_pressed(MouseButton::Right) {
        for entity in context_menu_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
            let mut style = Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(relative_position.0.abs() * 100.),
                left: Val::Percent(relative_position.1.abs() * 100.),

                width: Val::Percent(15.0),
                height: Val::Percent(15.0),
                ..default()
            };

            if relative_position.1.abs() > 0.9 {
                style.left = Val::Auto;
                style.right = Val::Percent(100. - relative_position.1.abs() * 100.);
            }
            if relative_position.0.abs() > 0.9 {
                style.top = Val::Auto;
                style.bottom = Val::Percent(100. - relative_position.0.abs() * 100.);
            }

            commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(relative_position.1.abs() * 100.),
                        top: Val::Percent(relative_position.0.abs() * 100.),
                        width: Val::Percent(15.0),
                        height: Val::Percent(15.0),
                        padding: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        // flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    BorderColor(Color::srgb(0.7, 0.7, 0.7)),
                    ContextMenu,
                ))
                .with_children(|parent| {
                    spawn_menu_option(parent, "Path Tool", ContextMenuOption::PathTool);
                    spawn_menu_option(parent, "Start Point", ContextMenuOption::StartPoint);
                    spawn_menu_option(parent, "End Point", ContextMenuOption::EndPoint);
                    spawn_menu_option(parent, "Buildable Area", ContextMenuOption::BuildableArea);

                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            margin: UiRect::vertical(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    ));

                    spawn_menu_option(parent, "Delete Tile", ContextMenuOption::Delete);
                    spawn_menu_option(parent, "Save Level", ContextMenuOption::Save);
                });
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_context_menu_interaction(
    mut commands: Commands,
    interaction_query: Query<
        (&Interaction, &ContextMenuOption, &Parent), // Get parent to despawn menu
        (Changed<Interaction>, With<Button>),
    >,
    menu_query: Query<Entity, With<ContextMenu>>,
    mut editor_data: ResMut<EditorData>,
    mut editor_text_input: ResMut<EditorTextInput>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    map: Option<Res<Map>>,
    markers_query: Query<(Entity, &Transform), With<EditorPathMarker>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    let calculate_grid_pos = |cursor_pos: Vec2| -> Option<UVec2> {
        camera
            .viewport_to_world_2d(camera_transform, cursor_pos)
            .ok()
            .map(|world_pos| {
                if let Some(map_res) = map.as_ref() {
                    map_res.world_to_grid(world_pos)
                } else {
                    // Fallback - ensure this matches your actual grid setup if no map
                    let grid_start_x = -crate::core::config::WINDOW_WIDTH / 2.0;
                    let grid_start_y = crate::core::config::WINDOW_HEIGHT / 2.0;
                    let x = ((world_pos.x - grid_start_x) / CELL_SIZE).floor() as u32;
                    let y = ((grid_start_y - world_pos.y) / CELL_SIZE).floor() as u32;
                    UVec2::new(x, y)
                }
            })
    };
    let grid_to_world = |grid_pos: UVec2| -> Vec2 {
        if let Some(map_res) = map.as_ref() {
            map_res.grid_to_world(grid_pos)
        } else {
            let grid_start_x = -crate::core::config::WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
            let grid_start_y = crate::core::config::WINDOW_HEIGHT / 2.0 - CELL_SIZE / 2.0;
            Vec2::new(
                grid_start_x + grid_pos.x as f32 * CELL_SIZE,
                grid_start_y - grid_pos.y as f32 * CELL_SIZE,
            )
        }
    };

    for (interaction, option, parent_menu_button) in interaction_query.iter() {
        if matches!(interaction, Interaction::Pressed) {
            let mut close_menu = true;

            match option {
                ContextMenuOption::PathTool => {
                    editor_data.current_tool = EditorTool::PathPlacer;
                }
                ContextMenuOption::StartPoint => {
                    editor_data.current_tool = EditorTool::StartPoint;
                }
                ContextMenuOption::EndPoint => {
                    editor_data.current_tool = EditorTool::EndPoint;
                }
                ContextMenuOption::BuildableArea => {
                    editor_data.current_tool = EditorTool::BuildableArea;
                }
                ContextMenuOption::Delete => {
                    if let Some(cursor_position) = window.cursor_position() {
                        if let Some(grid_pos) = calculate_grid_pos(cursor_position) {
                            let world_pos = grid_to_world(grid_pos);
                            let mut deleted_something = false;

                            let original_path_len = editor_data.path.len();
                            editor_data.path.retain(|(pos, _)| *pos != grid_pos);
                            if editor_data.path.len() < original_path_len {
                                deleted_something = true;
                            }

                            let original_buildable_len = editor_data.buildable_areas.len();
                            editor_data.buildable_areas.retain(|&pos| pos != grid_pos);
                            if editor_data.buildable_areas.len() < original_buildable_len {
                                deleted_something = true;
                            }

                            if editor_data.start == Some(grid_pos) {
                                editor_data.start = None;
                                deleted_something = true;
                            }
                            if editor_data.end == Some(grid_pos) {
                                editor_data.end = None;
                                deleted_something = true;
                            }

                            for (entity, transform) in markers_query.iter() {
                                if transform.translation.truncate().distance(world_pos) < 1.0 {
                                    commands.entity(entity).despawn();
                                    deleted_something = true;
                                }
                            }
                            if deleted_something {
                                info!("Deleted elements at grid position: {:?}", grid_pos);
                            } else {
                                info!("Nothing to delete at grid position: {:?}", grid_pos);
                            }
                        } else {
                            warn!("Could not get grid position for delete action.");
                            close_menu = false;
                        }
                    } else {
                        warn!("Could not get cursor position for delete action.");
                        close_menu = false;
                    }
                }
                ContextMenuOption::Save => {
                    editor_text_input.dialog_open = true;
                    spawn_save_dialog(&mut commands, &mut editor_text_input);
                }
            }

            if close_menu {
                if menu_query.get(parent_menu_button.get()).is_ok() {
                    commands
                        .entity(parent_menu_button.get())
                        .despawn_recursive();
                } else {
                    warn!("Could not find specific context menu parent, despawning all.");
                    for entity in menu_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH}; 
use crate::core::map::Map;
use crate::levels::level_textures::{get_selected_texture, AvailableTextures};

use super::super::components::*; 
use super::super::resources::*; 
use super::export::export_level; 

#[allow(clippy::too_many_arguments)]
pub fn editor_input_handler(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_press: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut editor_data: ResMut<EditorData>,
    editor_text_input: Res<EditorTextInput>, 
    map: Option<Res<Map>>, 
    asset_server: Res<AssetServer>,
    textures: Res<AvailableTextures>, 
    mut markers_query: Query<(Entity, &Transform, &mut Sprite), With<EditorPathMarker>>, 
    start_end_markers: Query<Entity, With<EditorPathMarker>>, 
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
                    let grid_start_x = -WINDOW_WIDTH / 2.0;
                    let grid_start_y = WINDOW_HEIGHT / 2.0;
                    let x = ((world_pos.x - grid_start_x) / CELL_SIZE).floor() as u32;
                    let y = ((grid_start_y - world_pos.y) / CELL_SIZE).floor() as u32;
                    UVec2::new(x.min(GRID_WIDTH as u32 - 1), y.min(GRID_HEIGHT as u32- 1))
                }
            })
    };

    let grid_to_world = |grid_pos: UVec2| -> Vec2 {
        if let Some(map_res) = map.as_ref() {
            map_res.grid_to_world(grid_pos)
        } else {
            let grid_start_x = -WINDOW_WIDTH / 2.0 + CELL_SIZE / 2.0;
            let grid_start_y = WINDOW_HEIGHT / 2.0 - CELL_SIZE / 2.0;
            Vec2::new(
                grid_start_x + grid_pos.x as f32 * CELL_SIZE,
                grid_start_y - grid_pos.y as f32 * CELL_SIZE,
            )
        }
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            if let Some(grid_pos) = calculate_grid_pos(cursor_position) {
                let world_pos = grid_to_world(grid_pos);

                match editor_data.current_tool {
                    EditorTool::PathPlacer => {
                        let texture_path = get_selected_texture(&textures);
                        let existing_path_index = editor_data
                            .path
                            .iter()
                            .position(|(pos, _)| *pos == grid_pos);

                        if let Some(idx) = existing_path_index {
                            editor_data.path[idx].1 = texture_path.clone();

                            for (entity, transform, mut sprite) in markers_query.iter_mut() {
                                if transform.translation.truncate().distance(world_pos) < 1.0 {
                                    sprite.image = asset_server.load(&texture_path);
                                    break; // Found and updated
                                }
                            }
                        } else {
                            editor_data.path.push((grid_pos, texture_path.clone()));

                            commands.spawn((
                                Sprite {
                                    image: asset_server.load(&texture_path),
                                        custom_size: Some(Vec2::splat(CELL_SIZE)),
                                        ..default()
                                    },
                                    Transform::from_translation(world_pos.extend(1.0)), // Ensure Z-order
                                EditorPathMarker,
                            ));
                        }
                    }
                    EditorTool::StartPoint => {
                        if let Some(old_start) = editor_data.start {
                             let old_world_pos = grid_to_world(old_start);
                             for (entity, transform, _) in markers_query.iter() {
                                 if transform.translation.truncate().distance(old_world_pos) < 1.0 {
                                      commands.entity(entity).despawn();
                                      break;
                                 }
                             }
                        }

                        editor_data.start = Some(grid_pos);
                        commands.spawn((
                                Sprite {
                                    color: Color::srgba(0.2, 0.9, 0.2, 0.7),
                                    custom_size: Some(Vec2::splat(CELL_SIZE)),
                                    ..default()
                                },
                                Transform::from_translation(world_pos.extend(1.5)), 
                            EditorPathMarker, 
                        ));
                    }
                    EditorTool::EndPoint => {
                        if let Some(old_end) = editor_data.end {
                             let old_world_pos = grid_to_world(old_end);
                             for (entity, transform, _) in markers_query.iter() {
                                 if transform.translation.truncate().distance(old_world_pos) < 1.0 {
                                      commands.entity(entity).despawn();
                                      break;
                                 }
                             }
                        }
                        editor_data.end = Some(grid_pos);
                        commands.spawn((
                            Sprite {
                                    color: Color::srgba(0.9, 0.1, 0.1, 0.7),
                                    custom_size: Some(Vec2::splat(CELL_SIZE)),
                                    ..default()
                                },
                                Transform::from_translation(world_pos.extend(1.5)), 
                            EditorPathMarker, 
                        ));
                    }
                    EditorTool::BuildableArea => {
                        if !editor_data.buildable_areas.contains(&grid_pos) {
                            editor_data.buildable_areas.push(grid_pos);
                            commands.spawn((
                                    Sprite {
                                        color: Color::srgba(0.2, 0.5, 0.8, 0.4),
                                        custom_size: Some(Vec2::splat(CELL_SIZE)),
                                        ..default()
                                    },
                                    Transform::from_translation(world_pos.extend(0.5)), 
                                EditorPathMarker, 
                            ));
                        }
                    }
                    EditorTool::TextureSelector => {
                    }
                }
            }
        }
    }

    if mouse_input.just_pressed(MouseButton::Right)
        && editor_data.current_tool == EditorTool::PathPlacer
    {
        if let Some((removed_pos, _)) = editor_data.path.pop() {
             let world_pos = grid_to_world(removed_pos);
             for (entity, transform, _) in markers_query.iter() {
                 if transform.translation.truncate().distance(world_pos) < 1.0 {
                     commands.entity(entity).despawn();
                     break; 
                 }
             }
        }
    }

    let mut new_tool = None;
    if key_press.just_pressed(KeyCode::KeyP) || key_press.just_pressed(KeyCode::Enter) { 
        new_tool = Some(EditorTool::PathPlacer);
    } else if key_press.just_pressed(KeyCode::KeyS) && !key_press.pressed(KeyCode::ControlLeft) { 
        new_tool = Some(EditorTool::StartPoint);
    } else if key_press.just_pressed(KeyCode::KeyE) {
        new_tool = Some(EditorTool::EndPoint);
    } else if key_press.just_pressed(KeyCode::KeyB) {
        new_tool = Some(EditorTool::BuildableArea);
    } else if key_press.just_pressed(KeyCode::KeyT) { 
        new_tool = Some(EditorTool::TextureSelector);
    }

    if let Some(tool) = new_tool {
        editor_data.current_tool = tool;
        info!("Switched tool to: {:?}", editor_data.current_tool);
    }


    if key_press.just_pressed(KeyCode::KeyS) && key_press.pressed(KeyCode::ControlLeft) {
        // export_level(&editor_data, &editor_text_input.level_name);
        info!("Ctrl+S pressed - Save dialog should open via ExportButton press / context menu");
    }

    if key_press.just_pressed(KeyCode::KeyG) {
        editor_data.grid_overlay = !editor_data.grid_overlay;
        info!("Toggled grid overlay: {}", editor_data.grid_overlay);
    }
}

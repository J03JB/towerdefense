use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::levels::level_editor::{EditorData, LevelData, EditorPathMarker};

// Function to read level files from the assets/levels directory
pub fn get_level_files() -> Vec<String> {
    let levels_path = "assets/levels";
    let mut level_files = Vec::new();

    if let Ok(entries) = fs::read_dir(levels_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                        level_files.push(file_name.to_string());
                    }
                }
            }
        }
    } else {
        error!("Could not read level files from assets/levels");
    }

    level_files
}

// Function to load level data from a JSON file
pub fn load_level_data(file_name: &str) -> Option<LevelData> {
    let file_path = format!("assets/levels/{}", file_name);
    if let Ok(contents) = fs::read_to_string(file_path) {
        match serde_json::from_str(&contents) {
            Ok(level_data) => Some(level_data),
            Err(e) => {
                error!("Failed to parse level data: {}", e);
                None
            }
        }
    } else {
        error!("Failed to read level file: {}", file_name);
        None
    }
}

// Function to apply level data to the editor
pub fn apply_level_data(
    level_data: &LevelData,
    mut editor_data: ResMut<EditorData>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut markers_query: Query<(Entity, &Transform), With<EditorPathMarker>>,
) {
    // Clear existing editor data
    editor_data.path.clear();
    editor_data.start = None;
    editor_data.end = None;
    editor_data.buildable_areas.clear();

     // Despawn existing markers
     for (entity, _) in markers_query.iter() {
        commands.entity(entity).despawn();
    }

    // Apply level data
    editor_data.path = level_data
        .path_textures
        .iter()
        .map(|path_texture| {
            (
                UVec2::new(path_texture.position[0], path_texture.position[1]),
                path_texture.texture.clone(),
            )
        })
        .collect();

    if !level_data.start.is_empty() {
        editor_data.start = Some(UVec2::new(level_data.start[0], level_data.start[1]));
    }

    if !level_data.end.is_empty() {
        editor_data.end = Some(UVec2::new(level_data.end[0], level_data.end[1]));
    }

    editor_data.buildable_areas = level_data
        .buildable_areas
        .iter()
        .map(|area| UVec2::new(area[0], area[1]))
        .collect();

    // Spawn new markers
    let grid_size = Vec2::new(48.0, 48.0); // Assuming grid size is 48x48

    for (grid_pos, texture_path) in &editor_data.path {
        let world_pos = Vec2::new(
            grid_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
            grid_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
        );

        commands.spawn((
            Sprite {
                image: asset_server.load(texture_path),
                custom_size: Some(Vec2::new(48.0, 48.0)),
                ..default()
            },
            Transform::from_translation(world_pos.extend(1.0)),
            EditorPathMarker,
        ));
    }

    if let Some(start_pos) = editor_data.start {
        let world_pos = Vec2::new(
            start_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
            start_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
        );

        commands.spawn((
            Sprite {
                color: Color::srgba(0.2, 0.9, 0.2, 0.7),
                custom_size: Some(Vec2::new(grid_size.x, grid_size.y)),
                ..default()
            },
            Transform::from_translation(world_pos.extend(1.0)),
            EditorPathMarker,
        ));
    }

    if let Some(end_pos) = editor_data.end {
        let world_pos = Vec2::new(
            end_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
            end_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
        );

        commands.spawn((
            Sprite {
                color: Color::srgba(0.9, 0.1, 0.1, 0.7),
                custom_size: Some(Vec2::new(grid_size.x, grid_size.y)),
                ..default()
            },
            Transform::from_translation(world_pos.extend(1.0)),
            EditorPathMarker,
        ));
    }

    for buildable_area in &editor_data.buildable_areas {
        let world_pos = Vec2::new(
            buildable_area.x as f32 * grid_size.x + grid_size.x * 0.5,
            buildable_area.y as f32 * grid_size.y + grid_size.y * 0.5,
        );

        commands.spawn((
            Sprite {
                color: Color::srgba(0.2, 0.5, 0.8, 0.4),
                custom_size: Some(Vec2::new(grid_size.x, grid_size.y)),
                ..default()
            },
            Transform::from_translation(world_pos.extend(0.5)),
            EditorPathMarker,
        ));
    }
}


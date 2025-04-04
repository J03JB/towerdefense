use std::fs::File;
use std::io::Write;
use bevy::log::{error, info, warn};
use bevy::prelude::UVec2;

use crate::levels::level_textures::PathTexture;
use crate::level_editor::resources::EditorData;
use super::super::data::LevelData;

pub fn export_level(editor_data: &EditorData, level_name: &str) {
    if level_name.is_empty() {
        warn!("Cannot export level with an empty name.");
        return;
    }

    let sanitized_name = level_name
        .replace(' ', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>();

    if sanitized_name.is_empty() {
        warn!("Level name contains no valid characters after sanitization.");
        return;
    }


    let mut path_coords = Vec::new();
    let mut path_textures = Vec::new();

    for (pos, texture) in &editor_data.path {
        path_coords.push(vec![pos.x, pos.y]);
        path_textures.push(PathTexture {
            position: vec![pos.x, pos.y], 
            texture: texture.clone(),
        });
    }

    let start_point = editor_data.start.unwrap_or(UVec2::ZERO);
    let end_point = editor_data.end.unwrap_or(UVec2::ZERO);

    let level_data = LevelData {
        path: path_coords,
        path_textures,
        start: vec![start_point.x, start_point.y],
        end: vec![end_point.x, end_point.y],
        buildable_areas: editor_data
            .buildable_areas
            .iter()
            .map(|pos| vec![pos.x, pos.y])
            .collect(),
        // TODO: Get dimensions dynamically or from config? Hardcoding for now.
        dimensions: vec![
            crate::core::config::GRID_WIDTH as u32,
            crate::core::config::GRID_HEIGHT as u32
        ],
    };

    match serde_json::to_string_pretty(&level_data) {
        Ok(json_string) => {
            let dir_path = "assets/levels";
            if let Err(e) = std::fs::create_dir_all(dir_path) {
                error!("Failed to create levels directory '{}': {}", dir_path, e);
                return;
            }

            let file_path = format!("{}/{}.json", dir_path, sanitized_name);
            match File::create(&file_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(json_string.as_bytes()) {
                        error!("Failed to write level data to '{}': {}", file_path, e);
                    } else {
                        info!("Successfully exported level data to '{}'", file_path);
                        // Optionally trigger success feedback UI
                    }
                }
                Err(e) => {
                    error!("Failed to create level file '{}': {}", file_path, e);
                }
            }
        }
        Err(e) => {
            error!("Failed to serialize level data: {}", e);
        }
    }
}

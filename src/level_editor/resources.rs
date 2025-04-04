use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct EditorTextInput {
    pub level_name: String,
    pub dialog_open: bool,
}

impl Default for EditorTextInput {
    fn default() -> Self {
        Self {
            level_name: "".to_string(),
            dialog_open: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditorTool {
    #[default]
    PathPlacer,
    StartPoint,
    EndPoint,
    BuildableArea,
    TextureSelector, 
}

#[derive(Resource, Default)]
pub struct EditorData {
    pub path: Vec<(UVec2, String)>, 
    pub start: Option<UVec2>,
    pub end: Option<UVec2>,
    pub buildable_areas: Vec<UVec2>,
    pub current_tool: EditorTool,
    pub grid_overlay: bool,
}

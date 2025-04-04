use crate::levels::level_textures::PathTexture;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LevelData {
    pub path: Vec<Vec<u32>>,
    pub path_textures: Vec<PathTexture>,
    pub start: Vec<u32>,
    pub end: Vec<u32>,
    pub buildable_areas: Vec<Vec<u32>>,
    pub dimensions: Vec<u32>,
}

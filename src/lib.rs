pub mod core {
    pub mod config;
    pub mod game_state;
    pub mod map;
    pub mod utils;
}

pub mod entities {
    pub mod enemy;
    pub mod projectile;
    pub mod tower;
    pub mod pathfinding;
}

pub mod ui {
    pub mod main_menu;
    pub mod render;
    pub mod ui_components;
    pub mod overlay;
}

pub mod levels {
    pub mod level;
    pub mod level_editor;
    pub mod level_loader;
    pub mod level_textures;
}

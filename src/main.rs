mod enemy;
mod game_state;
mod level;
mod map;
mod overlay;
mod projectile;
mod render;
mod tower;
mod ui;
mod utils;
mod grid;
mod config;
// mod level_loader;
mod level_editor;

use bevy::prelude::*;

use crate::config::WINDOW_HEIGHT;
use crate::config::WINDOW_WIDTH;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: "Tower Defense Game".to_string(),
                resizable: true,
                ..default()
            }),
            ..default()
        }
        ).set(ImagePlugin::default_nearest()))
        .add_plugins(game_state::GameStatePlugin)
        .add_plugins(render::RenderPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(tower::TowerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(projectile::ProjectilePlugin)
        .add_plugins(level_editor::EditorPlugin)
        // .add_plugins(grid::GridPlugin)
        // .add_plugins(ui::UiPlugin)
        // .add_plugins(overlay::OverlayPlugin)
        .run();
}

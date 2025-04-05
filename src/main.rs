use bevy::prelude::*;

use towerdefense::{
    core::{config::{WINDOW_HEIGHT, WINDOW_WIDTH}, game_state::GameStatePlugin},
    entities::{enemy::EnemyPlugin, projectile::ProjectilePlugin, tower::TowerPlugin, pathfinding::PathfindingPlugin},
    levels::{level::LevelPlugin, level_textures::TexturesPlugin},
    level_editor::EditorPlugin,
    ui::{main_menu::MainMenuPlugin, render::RenderPlugin, overlay::OverlayPlugin},
};

fn main() {
    App::new()
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
        .add_plugins(GameStatePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(RenderPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(PathfindingPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(TowerPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(EditorPlugin)
        .add_plugins(TexturesPlugin)
        // .add_plugins(grid::GridPlugin)
        // .add_plugins(ui::UiPlugin)
        .add_plugins(OverlayPlugin)
        .run();
}

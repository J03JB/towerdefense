mod enemy;
mod game_state;
mod level;
mod map;
mod projectile;
mod render;
mod tower;
mod ui;
mod utils;

use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280.0, 720.0),
                title: "Tower Defense Game".to_string(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(game_state::GameStatePlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(render::RenderPlugin)
        .add_plugins(tower::TowerPlugin)
        .add_plugins(projectile::ProjectilePlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(bevy::asset::AssetPlugin)
        .init_asset_loader::<LevelAsssetLoader>()
        .run();
}

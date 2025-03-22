mod enemy;
mod level;
mod render;
mod tower;
mod projectile;
mod ui;
mod game_state;
mod utils;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game_state::GameStatePlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(render::RenderPlugin)
        .add_plugins(tower::TowerPlugin)
        .add_plugins(projectile::ProjectilePlugin)
        .add_plugins(ui::UiPlugin)
        .run();
}





use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::overlay::MainCamera;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_sprites);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn update_sprites() {
    // Update sprite positions and animations
}

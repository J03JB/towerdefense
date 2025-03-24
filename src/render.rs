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
    // commands.spawn(Camera2d);
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical { viewport_height: 1600.0 },
        // scale: 0.01,
        ..OrthographicProjection::default_2d()
    });

    commands.spawn((
        Camera2d {},
        projection,
        Transform::from_xyz(100.0, 200.0, 0.0),
    ));
}

fn update_sprites() {
    // Update sprite positions and animations
}

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyWorldCoords>();
        // app.add_systems(Startup, setup)
            app.add_systems(Update, my_cursor_system);

    }
}


/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;


// pub fn setup(mut commands: Commands) {
    // Make sure to add the marker component when you set up your camera
    // commands.spawn(Camera2d {
    //     MainCameram
    //     ..default()
    //
    // });
     // commands.spawn((Camera2d, OverLayCamera));
// }

pub fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

if let Some(cursor_position) = window.cursor_position() {
    // Convert cursor position to viewport position
    let viewport_position = cursor_position;
    
    // Use the new viewport_to_world_2d which returns a Result instead of Option
    match camera.viewport_to_world_2d(camera_transform, viewport_position) {
        Ok(world_position) => {
            mycoords.0 = world_position;
            eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        },
        Err(_) => {
            // Handle error - cursor outside viewport or other conversion error
        }
    }
}
    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
//     if let Some(cursor_position) = window.cursor_position() {
//         if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, viewport_position) {
//
//         mycoords.0 = world_position;
//         eprintln!("World coords: {}/{}", world_position.x, world_position.y);
//     }
// }
}

use crate::map::Map;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EditorState>()
            .add_systems(Startup, setup_editor)
            .add_systems(
                Update,
                (
                    editor_input_handler,
                    render_editor_path,
                    export_level_data,
                    toggle_editor_tool,
                )
                    .run_if(in_state(EditorState::Active)),
            );
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default, Reflect)]
pub enum EditorState {
    #[default]
    Inactive,
    Active,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditorTool {
    #[default]
    PathPlacer,
    StartPoint,
    EndPoint,
    BuildableArea,
}

#[derive(Resource, Default)]
pub struct EditorData {
    pub path: Vec<UVec2>,
    pub start: Option<UVec2>,
    pub end: Option<UVec2>,
    pub buildable_areas: Vec<UVec2>,
    pub current_tool: EditorTool,
    pub grid_overlay: bool,
}

#[derive(Component)]
pub struct EditorPathMarker;

#[derive(Component)]
pub struct EditorToolDisplay;

#[derive(Serialize, Deserialize)]
pub struct LevelData {
    pub path: Vec<Vec<u32>>,            // Stored as [[x, y], [x, y], ...]
    pub start: Vec<u32>,                // [x, y]
    pub end: Vec<u32>,                  // [x, y]
    pub buildable_areas: Vec<Vec<u32>>, // [[x, y], [x, y], ...]
    pub dimensions: Vec<u32>,           // [width, height]
}

fn setup_editor(
    mut commands: Commands,
    mut editor_state: ResMut<State<EditorState>>,
    mut next_state: ResMut<NextState<EditorState>>,
    asset_server: Res<AssetServer>,
) {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--editor") {
        // Create editor UI
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3).into()),
                EditorToolDisplay,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Editor Mode: Path Placement"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor::WHITE,
                ));

                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Path Tool"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    })
                    .insert(EditorButton(EditorTool::PathPlacer));
                let button_style = Node {
                    width: Val::Px(120.0),
                    height: Val::Px(40.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                };

                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Start Point"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    })
                    .insert(EditorButton(EditorTool::StartPoint));

                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("End Point"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    })
                    .insert(EditorButton(EditorTool::EndPoint));

                parent
                    .spawn((
                        Button,
                        button_style.clone(),
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Buildable Area"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    })
                    .insert(EditorButton(EditorTool::BuildableArea));

                parent
                    .spawn((
                        Button,
                        button_style.clone(),
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Export JSON"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    })
                    .insert(ExportButton);
            });

        // Create a grid overlay
        // This would be a system that draws grid lines

        // Initialize editor data resource
        commands.insert_resource(EditorData::default());

        // Set editor state to active
        // editor_state.set(EditorState::Active);
        next_state.set(EditorState::Active);
    }
}

#[derive(Component)]
struct EditorButton(EditorTool);

#[derive(Component)]
struct ExportButton;

fn editor_input_handler(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_press: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut editor_data: ResMut<EditorData>,
    map: Option<Res<Map>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();

        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(world_position) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
            {
                let grid_pos = if let Some(map) = map.as_ref() {
                    map.world_to_grid(world_position)
                } else {
                    let grid_size = Vec2::new(48.0, 48.0);
                    UVec2::new(
                        (world_position.x / grid_size.x).floor() as u32,
                        (world_position.y / grid_size.y).floor() as u32,
                    )
                };

                let grid_size = Vec2::new(48.0, 48.0);
                match editor_data.current_tool {
                    EditorTool::PathPlacer => {
                        if !editor_data.path.contains(&grid_pos) {
                            editor_data.path.push(grid_pos);

                            let world_pos = if let Some(map) = map.as_ref() {
                                map.grid_to_world(grid_pos)
                            } else {
                                let grid_size = Vec2::new(48.0, 48.0);
                                Vec2::new(
                                    grid_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
                                    grid_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
                                )
                            };

                            commands.spawn((
                                Sprite {
                                    color: Color::srgba(0.8, 0.3, 0.3, 0.7),
                                    custom_size: Some(Vec2::new(48.0, 48.0)),
                                    ..default()
                                },
                                Transform::from_translation(world_pos.extend(1.0)),
                                EditorPathMarker,
                            ));
                        }
                    }
                    EditorTool::StartPoint => {
                        editor_data.start = Some(grid_pos);

                        // Clear previous start marker and create a new one
                        commands.spawn((
                            Sprite {
                                color: Color::srgba(0.2, 0.9, 0.2, 0.7),
                                custom_size: Some(Vec2::new(48.0, 48.0)),
                                ..default()
                            },
                            Transform::from_translation(
                                Vec2::new(
                                    grid_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
                                    grid_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
                                )
                                .extend(1.0),
                            ),
                            EditorPathMarker,
                        ));
                    }
                    EditorTool::EndPoint => {
                        editor_data.end = Some(grid_pos);

                        // Clear previous end marker and create a new one
                        commands.spawn((
                            Sprite {
                                color: Color::srgba(0.9, 0.1, 0.1, 0.7),
                                custom_size: Some(Vec2::new(grid_size.x, grid_size.y)),
                                ..default()
                            },
                            Transform::from_translation(
                                Vec2::new(
                                    grid_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
                                    grid_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
                                )
                                .extend(1.0),
                            ),
                            EditorPathMarker,
                        ));
                    }
                    EditorTool::BuildableArea => {
                        if !editor_data.buildable_areas.contains(&grid_pos) {
                            editor_data.buildable_areas.push(grid_pos);

                            commands.spawn((
                                Sprite {
                                    color: Color::srgba(0.2, 0.5, 0.8, 0.4),
                                    custom_size: Some(Vec2::new(grid_size.x, grid_size.y)),
                                    ..default()
                                },
                                Transform::from_translation(
                                    Vec2::new(
                                        grid_pos.x as f32 * grid_size.x + grid_size.x * 0.5,
                                        grid_pos.y as f32 * grid_size.y + grid_size.y * 0.5,
                                    )
                                    .extend(0.5),
                                ),
                                EditorPathMarker,
                            ));
                        }
                    }
                }
            }
        }
    }

    if mouse_input.just_pressed(MouseButton::Right)
        && editor_data.current_tool == EditorTool::PathPlacer
    {
        editor_data.path.pop();
    }

    if key_press.just_pressed(KeyCode::Enter) {
        editor_data.current_tool = EditorTool::PathPlacer;
    } else if key_press.just_pressed(KeyCode::KeyS) {
        editor_data.current_tool = EditorTool::StartPoint;
    } else if key_press.just_pressed(KeyCode::KeyE) {
        editor_data.current_tool = EditorTool::EndPoint;
    } else if key_press.just_pressed(KeyCode::KeyB) {
        editor_data.current_tool = EditorTool::BuildableArea;
    } else if key_press.just_pressed(KeyCode::KeyS) && key_press.pressed(KeyCode::ControlLeft) {
        // Ctrl+S to export
        export_level(&editor_data);
    } else if key_press.just_pressed(KeyCode::KeyG) {
        editor_data.grid_overlay = !editor_data.grid_overlay;
        info!("overlay")
    }
}

fn render_editor_path(editor_data: Res<EditorData>, mut gizmos: Gizmos, map: Option<Res<Map>>) {
    let grid_size = if let Some(map) = map.as_ref() {
        map.grid_size
    } else {
        Vec2::new(48.0, 48.0)
    };

    // info!("grid size: {}", grid_size);

    if editor_data.path.len() >= 2 {
        for i in 0..editor_data.path.len() - 1 {
            let start = editor_data.path[i];
            let end = editor_data.path[i + 1];

            let start_world = Vec2::new(
                start.x as f32 * grid_size.x + grid_size.x * 0.5,
                start.y as f32 * grid_size.y + grid_size.y * 0.5,
            );

            let end_world = Vec2::new(
                end.x as f32 * grid_size.x + grid_size.x * 0.5,
                end.y as f32 * grid_size.y + grid_size.y * 0.5,
            );

            gizmos.line_2d(start_world, end_world, Color::srgb(0.9, 0.3, 0.7));
        }
    }

    if editor_data.grid_overlay {
        let dimensions = if let Some(map) = map.as_ref() {
            map.dimensions
        } else {
            UVec2::new(27, 15) // Default dimensions if map not available
        };

        // info!("dimensions: {}", dimensions);

        let grid_size = if let Some(map) = map.as_ref() {
            map.grid_size
        } else {
            Vec2::new(48.0, 48.0) // Default grid size if map not available
        };

        // info!("overlay grid size: {}", grid_size);
        // Calculate grid boundaries
        let grid_start_pos = if let Some(map) = map.as_ref() {
            map.grid_to_world(UVec2::new(0, 0))
        } else {
            // Fallback calculation
            let grid_width = dimensions.x as f32 * grid_size.x;
            let grid_height = dimensions.y as f32 * grid_size.y;
            Vec2::new(-grid_width / 2.0, grid_height / 2.0)
        };

        // Draw vertical grid lines
        for x in 0..=dimensions.x {
            let x_pos = grid_start_pos.x + (x as f32 * grid_size.x);
            info!("xpos: {}", x_pos);
            let start = Vec2::new(640.0, 360.0);
            info!("start: {}", start);
            // let end = Vec2::new(-640.0, -360.0);
            let end = Vec2::new(
                x_pos,
                grid_start_pos.y - (dimensions.y as f32 * grid_size.y),
            );
            // info!("end pos: {}", end);
            gizmos.line_2d(start, end, Color::srgba(0.5, 0.5, 0.5, 0.3));
        }

        // Draw horizontal grid lines
        for y in 0..=dimensions.y {
            let y_pos = grid_start_pos.y - (y as f32 * grid_size.y);
            let start = Vec2::new(grid_start_pos.x, y_pos);
            let end = Vec2::new(
                grid_start_pos.x + (dimensions.x as f32 * grid_size.x),
                y_pos,
            );
            gizmos.line_2d(start, end, Color::srgba(0.5, 0.5, 1.5, 0.3));
        }
    }
}

fn toggle_editor_tool(
    interaction_query: Query<(&Interaction, &EditorButton), (Changed<Interaction>, With<Button>)>,
    mut editor_data: ResMut<EditorData>,
    mut tool_text_query: Query<&mut Text, With<EditorToolDisplay>>,
) {
    for (interaction, button) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            editor_data.current_tool = button.0.clone();

            // Update the tool text
            if let Ok(mut text) = tool_text_query.get_single_mut() {
                // Create a new TextSection
                *text = Text::new(format!("Editor Mode: {:?}", editor_data.current_tool));
            }
        }
    }
}

fn export_level_data(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExportButton>)>,
    editor_data: Res<EditorData>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            export_level(&editor_data);
        }
    }
}

fn export_level(editor_data: &EditorData) {
    let level_data = LevelData {
        path: editor_data
            .path
            .iter()
            .map(|pos| vec![pos.x, pos.y])
            .collect(),
        start: if let Some(start) = editor_data.start {
            vec![start.x, start.y]
        } else {
            vec![0, 0]
        },
        end: if let Some(end) = editor_data.end {
            vec![end.x, end.y]
        } else {
            vec![0, 0]
        },
        buildable_areas: editor_data
            .buildable_areas
            .iter()
            .map(|pos| vec![pos.x, pos.y])
            .collect(),
        dimensions: vec![27, 15], // Hard-coded for simplicity
    };

    // Serialize the data to JSON
    if let Ok(json_string) = serde_json::to_string_pretty(&level_data) {
        // Create a file for writing
        if let Ok(mut file) = File::create("assets/levels/level_1.json") {
            if let Err(e) = file.write_all(json_string.as_bytes()) {
                eprintln!("Failed to write level data: {}", e);
            } else {
                println!("Successfully exported level data to 'assets/levels/level_1.json'");
            }
        } else {
            eprintln!("Failed to create level file");
        }
    } else {
        eprintln!("Failed to serialize level data");
    }
}

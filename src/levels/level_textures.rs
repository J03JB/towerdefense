use bevy::prelude::*;
use crate::levels::level_editor::{EditorData, EditorTool};
use serde::{Deserialize, Serialize};

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_available_textures)
           .add_systems(Update, (
               handle_texture_selection,
               toggle_texture_panel,
           ));
    }
}

// Available textures - could be loaded from config or dynamically
#[derive(Resource)]
pub struct AvailableTextures {
    pub paths: Vec<String>,
    pub selected: Option<String>,
}

impl Default for AvailableTextures {
    fn default() -> Self {
        Self {
            paths: vec![
                "textures/path01.png".to_string(),
                "textures/path02.png".to_string(),
                "textures/path03.png".to_string(),
                "textures/path_corner.png".to_string(),
                "textures/path_junction.png".to_string(),
            ],
            selected: Some("textures/path01.png".to_string()),
        }
    }
}

#[derive(Component)]
pub struct TextureButton(pub String);

#[derive(Component)]
pub struct TextureSelectorPanel;

// Path texture information for serialization
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathTexture {
    pub position: Vec<u32>,  // [x, y]
    pub texture: String,     // Texture name/path
}

fn setup_available_textures(mut commands: Commands) {
    commands.init_resource::<AvailableTextures>();
}

pub fn setup_texture_selector(mut commands: Commands, asset_server: Res<AssetServer>, textures: Res<AvailableTextures>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(60.0),
                width: Val::Px(200.0),
                height: Val::Auto,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor(Color::srgb(0.7, 0.7, 0.7)),
            TextureSelectorPanel,
            Visibility::Visible,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Path Textures"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor::WHITE,
            ));

            // Texture buttons
            for texture_path in &textures.paths {
                let texture_name = texture_path
                    .split('/')
                    .next_back()
                    .unwrap_or(texture_path)
                    .to_string();
                
                let bg_color = if Some(texture_path.clone()) == textures.selected {
                    Color::srgb(0.3, 0.5, 0.3)
                } else {
                    Color::srgb(0.15, 0.15, 0.15)
                };
                
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(40.0),
                            margin: UiRect::vertical(Val::Px(2.0)),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::horizontal(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(bg_color),
                        TextureButton(texture_path.clone()),
                    ))
                    .with_children(|parent| {
                        // Display texture name
                        parent.spawn((
                            Text::new(texture_name),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                        
                        // Display texture preview
                        parent.spawn((
                            Sprite {
                                image: asset_server.load(texture_path),
                                custom_size: Some(Vec2::new(30.0, 30.0)),
                                ..default()
                            },
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                ..default()
                            },
                        ));
                    });
            }
        });
}

pub fn handle_texture_selection(
    interaction_query: Query<(&Interaction, &TextureButton), (Changed<Interaction>, With<Button>)>,
    mut editor_data: Option<ResMut<EditorData>>,
    mut textures: ResMut<AvailableTextures>,
    mut button_query: Query<(&TextureButton, &mut BackgroundColor), With<Button>>,
) {
    // Handle button interactions
    for (interaction, texture_button) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            // Update selected texture
            textures.selected = Some(texture_button.0.clone());
            
            // Update editor data if available
            if let Some(mut editor_data) = editor_data.as_mut() {
                editor_data.selected_texture = texture_button.0.clone();
                editor_data.current_tool = EditorTool::PathPlacer;
            }
            
            // Update button colors to show selection
            for (button, mut color) in button_query.iter_mut() {
                if button.0 == texture_button.0 {
                    *color = BackgroundColor(Color::srgb(0.3, 0.5, 0.3));
                } else {
                    *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
                }
            }
        }
    }
}

pub fn toggle_texture_panel(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut editor_data: Option<ResMut<EditorData>>,
    mut panel_query: Query<&mut Visibility, With<TextureSelectorPanel>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        if let Ok(mut visibility) = panel_query.get_single_mut() {
            // Toggle panel visibility
            *visibility = match *visibility {
                Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
            };
            if let Some(mut editor_data) = editor_data.as_mut() {
                editor_data.current_tool  = match *visibility {
                    Visibility::Visible => EditorTool::TextureSelector,
                    _ => EditorTool::PathPlacer,
                };
            }
        }
    }
}

pub fn get_selected_texture(textures: &Res<AvailableTextures>) -> String {
    textures.selected.clone().unwrap_or_else(|| "textures/path01.png".to_string())
}

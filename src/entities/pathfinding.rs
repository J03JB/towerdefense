use crate::core::map::Map;
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlowFieldDebugConfig>()
            .add_systems(Update, update_flow_field_visualization)
            .add_systems(Update, toggle_flow_field_visualization);
    }
}
#[derive(Resource)]
pub struct FlowFieldDebugConfig {
    pub show_visualization: bool,
}

impl Default for FlowFieldDebugConfig {
    fn default() -> Self {
        Self {
            show_visualization: false,
        }
    }
}

/// Flow field type - each cell points to the next cell in path
#[derive(Resource, Clone, Default)]
pub struct FlowField {
    pub width: usize,
    pub height: usize,
    pub field: Vec<Option<FlowDirection>>,
    pub integration_field: Vec<u32>,
    pub is_initialized: bool,
}

/// Direction to the next cell
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlowDirection {
    North,
    South,
    East,
    West,
    None,
}

impl FlowDirection {
    /// Convert direction to a normalized 2D vector
    pub fn to_vec2(&self) -> Vec2 {
        match self {
            FlowDirection::North => Vec2::new(0.0, 1.0),
            FlowDirection::South => Vec2::new(0.0, -1.0),
            FlowDirection::East => Vec2::new(1.0, 0.0),
            FlowDirection::West => Vec2::new(-1.0, 0.0),
            FlowDirection::None => Vec2::ZERO,
        }
    }
}

/// Component for visualizing flow field directions
#[derive(Component)]
pub struct FlowFieldVisualizer;

impl FlowField {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            field: vec![None; width * height],
            integration_field: vec![u32::MAX; width * height],
            is_initialized: false,
        }
    }
    /// Get 1D index from 2D coordinates
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        // Ensure coordinates are within bounds
        if x >= self.width || y >= self.height {
            return 0; // Return a safe default index
        }
        y * self.width + x
    }

    /// Get 2D coordinates from 1D index
    pub fn get_coordinates(&self, index: usize) -> (usize, usize) {
        // Ensure index is within bounds
        if index >= self.width * self.height {
            return (0, 0); // Return a safe default coordinate
        }
        let y = index / self.width;
        let x = index % self.width;
        (x, y)
    }

        pub fn compute(&mut self, map: &Map, goal_pos: UVec2) {
        // Reset fields
        self.integration_field = vec![u32::MAX; self.width * self.height];
        self.field = vec![None; self.width * self.height];

        // Initialize a queue for Dijkstra's algorithm
        let mut queue = VecDeque::new();

        // Check if goal position is valid
        let goal_x = goal_pos.x as usize;
        let goal_y = goal_pos.y as usize;

        if goal_x >= self.width || goal_y >= self.height {
            error!(
                "Goal position ({}, {}) is out of bounds ({}, {})",
                goal_x, goal_y, self.width, self.height
            );
            return;
        }

        // Convert goal to index
        let goal_index = self.get_index(goal_x, goal_y);

        // Set goal distance to 0 and add to queue
        self.integration_field[goal_index] = 0;
        queue.push_back(goal_index);

        // Debug info about path
        info!("Computing flow field from goal {:?} to start", goal_pos);
        info!("Path has {} tiles", map.path_tiles.len());
        
        // Print out the path tiles for debugging
        for (i, &pos) in map.path_tiles.iter().enumerate() {
            info!("Path tile {}: {:?}", i, pos);
        }

        // Process the queue (Dijkstra's algorithm)
        while let Some(current_index) = queue.pop_front() {
            let current_cost = self.integration_field[current_index];
            let (x, y) = self.get_coordinates(current_index);

            // Define potential neighbors
            let neighbors = [
                (x + 1, y, 10),     // East
                (x.wrapping_sub(1), y, 10), // West
                (x, y + 1, 10),     // South (in grid coordinates)
                (x, y.wrapping_sub(1), 10), // North (in grid coordinates)
            ];

            for (nx, ny, cost) in neighbors {
                // Skip out-of-bounds neighbors
                if nx >= self.width || ny >= self.height {
                    continue;
                }

                let neighbor_index = self.get_index(nx, ny);

                // Consider a cell walkable if it's in the path tiles
                let is_walkable = map.path_tiles.contains(&UVec2::new(nx as u32, ny as u32));
                
                // Skip non-walkable cells
                if !is_walkable {
                    continue;
                }

                // Calculate new cost
                let new_cost = current_cost + cost;

                // If new cost is better, update and add to queue
                if new_cost < self.integration_field[neighbor_index] {
                    self.integration_field[neighbor_index] = new_cost;
                    queue.push_back(neighbor_index);
                }
            }
        }

        // Generate flow field from integration field
        for &path_pos in &map.path_tiles {
            let x = path_pos.x as usize;
            let y = path_pos.y as usize;
            
            if x >= self.width || y >= self.height {
                continue;
            }
            
            let index = self.get_index(x, y);
            
            // Skip unreachable cells
            if self.integration_field[index] == u32::MAX {
                info!("Path tile {:?} is not reachable!", path_pos);
                continue;
            }

            // Find the neighbor with the lowest cost
            let mut best_direction = FlowDirection::None;
            let mut best_cost = self.integration_field[index];

            // Define neighbors: East, West, South, North
            // Note: In your world coordinates, Y increases upward, but in grid coords
            // it increases downward, so we need to reverse the meaning of North/South
            let neighbors = [
                (x + 1, y, FlowDirection::East),
                (x.wrapping_sub(1), y, FlowDirection::West),
                (x, y + 1, FlowDirection::South), // Going down in grid coords
                (x, y.wrapping_sub(1), FlowDirection::North), // Going up in grid coords
            ];

            for (nx, ny, direction) in neighbors {
                // Skip out-of-bounds neighbors
                if nx >= self.width || ny >= self.height {
                    continue;
                }

                let neighbor_index = self.get_index(nx, ny);

                // Skip non-path or unreachable cells
                if !map.path_tiles.contains(&UVec2::new(nx as u32, ny as u32)) || 
                   self.integration_field[neighbor_index] == u32::MAX {
                    continue;
                }

                // If this neighbor has a lower cost, update best direction
                if self.integration_field[neighbor_index] < best_cost {
                    best_cost = self.integration_field[neighbor_index];
                    best_direction = direction;
                }
            }

            // Set the direction for this cell
            self.field[index] = Some(best_direction);
        }

        // Debug output to verify directions
        for &path_pos in &map.path_tiles {
            let x = path_pos.x as usize;
            let y = path_pos.y as usize;
            
            if x >= self.width || y >= self.height {
                continue;
            }
            
            let index = self.get_index(x, y);
            
            if index < self.field.len() {
                if let Some(dir) = self.field[index] {
                    info!("Path tile at {:?} has direction: {:?}", path_pos, dir);
                } else {
                    info!("Path tile at {:?} has NO direction", path_pos);
                }
            }
        }

        self.is_initialized = true;
    }

    /// Get flow direction at given grid coordinates
    pub fn get_direction(&self, x: usize, y: usize) -> Option<FlowDirection> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = self.get_index(x, y);
        if index >= self.field.len() {
            return None;
        }
        self.field[index]
    }

    /// Get flow vector at given grid coordinates
    pub fn get_flow_vector(&self, x: usize, y: usize) -> Vec2 {
        // Ensure coordinates are within bounds
        if x >= self.width || y >= self.height {
            return Vec2::ZERO;
        }

        if let Some(direction) = self.get_direction(x, y) {
            direction.to_vec2()
        } else {
            Vec2::ZERO
        }
    }
}

fn toggle_flow_field_visualization(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_config: ResMut<FlowFieldDebugConfig>,
    mut arrows: Query<&mut Visibility, With<FlowFieldVisualizer>>,
) {
    // Toggle visualization with F key
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        debug_config.show_visualization = !debug_config.show_visualization;
        info!("you pressed f!");

        // Update visibility of all existing arrows
        let visibility = if debug_config.show_visualization {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        for mut vis in arrows.iter_mut() {
            *vis = visibility;
        }
    }
}

/// Visualize flow field for debugging
fn update_flow_field_visualization(
    mut commands: Commands,
    flow_field: Option<Res<FlowField>>,
    map: Res<Map>,
    arrows: Query<Entity, With<FlowFieldVisualizer>>,
    debug_config: Res<FlowFieldDebugConfig>,
) {
    // If visualization is disabled or there's no flow field, do nothing
    if !debug_config.show_visualization
        || flow_field.is_none()
        || !flow_field.as_ref().unwrap().is_initialized
    {
        // info!("no flow man!!");
        return;
    }

    // If there's no flow field, or it hasn't changed, do nothing
    if flow_field.is_none() || !flow_field.as_ref().unwrap().is_initialized {
        return;
    }
    let flow_field = flow_field.unwrap();

    // Remove old arrows
    for entity in arrows.iter() {
        commands.entity(entity).despawn();
    }

    // Create new arrows showing directions
    for y in 0..flow_field.height {
        for x in 0..flow_field.width {
            if let Some(direction) = flow_field.get_direction(x, y) {
                if direction == FlowDirection::None {
                    continue;
                }

                let world_pos = map.grid_to_world(UVec2::new(x as u32, y as u32));

                // Calculate rotation angle based on direction
                let angle = match direction {
                    FlowDirection::North => 0.0,
                    FlowDirection::East => std::f32::consts::PI * 0.5,
                    FlowDirection::South => std::f32::consts::PI,
                    FlowDirection::West => std::f32::consts::PI * 1.5,
                    FlowDirection::None => 0.0,
                };

                // Create an arrow shape (very simple representation)
                commands.spawn((
                    Sprite {
                        color: Color::srgba(1.0, 1.0, 0.0, 0.5),
                        custom_size: Some(Vec2::new(10.0, 20.0)),
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(world_pos.x, world_pos.y, 2.0),
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                    FlowFieldVisualizer,
                ));
            }
        }
    }
}

use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default, Reflect)]
pub enum EditorState {
    #[default]
    Inactive,
    Active,
}

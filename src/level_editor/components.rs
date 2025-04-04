use bevy::prelude::*;
use super::resources::EditorTool;

#[derive(Component)]
pub struct LevelNameInput;

#[derive(Component)]
pub struct SaveDialog;

#[derive(Component)]
pub struct EditorPathMarker;

#[derive(Component)]
pub struct EditorToolDisplay;

#[derive(Component)]
pub struct EditorButton(pub EditorTool); 

#[derive(Component)]
pub struct ExportButton;

#[derive(Component)]
pub struct ConfirmSaveButton;

#[derive(Component)]
pub struct CancelSaveButton;

#[derive(Component)]
pub struct ContextMenu;

#[derive(Component)]
pub enum ContextMenuOption {
    PathTool,
    StartPoint,
    EndPoint,
    BuildableArea,
    Delete,
    Save,
}

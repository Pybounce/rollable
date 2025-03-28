
use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    WorldSelect,
    StageSelect,
    InStage
}
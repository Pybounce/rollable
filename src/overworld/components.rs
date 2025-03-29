
use bevy::prelude::*;

use crate::world_select::components::WorldType;

#[derive(Resource)]
pub struct LoadOverworldConfig {
    pub world_type: WorldType
}

impl LoadOverworldConfig {
    pub fn new(world_type: WorldType) -> Self {
        return Self { world_type: world_type };
    }
}

#[derive(Component)]
pub struct OverworldEntity;
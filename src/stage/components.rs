
use bevy::prelude::*;

#[derive(Component)]
pub struct Ground;

#[derive(Resource)]
pub struct LoadStageConfig {
    stage_id: usize
}

impl LoadStageConfig {
    pub fn new(stage_id: usize) -> Self {
        return Self { stage_id: stage_id };
    }
    pub fn stage_id(&self) -> usize {
        return self.stage_id;
    }
}
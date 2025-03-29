

use bevy::{prelude::*, utils::hashbrown::HashMap};

/// Marker component to describe an entity that is part of the world select scene
#[derive(Component)]
pub struct WorldSelectEntity;

#[derive(Default, Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub enum WorldType {
    #[default]
    Grasslands = 1,
    SomethingElse = 2
}

#[derive(Resource)]
pub struct WorldSelectController {
    pub current_world: WorldType,
    pub max_camera_speed: f32,
    pub min_camera_speed: f32,
    pub select_world_key: KeyCode
}

impl WorldSelectController {
    pub fn cycle_next_world(&mut self) {
        self.current_world = match self.current_world {
            WorldType::Grasslands => WorldType::SomethingElse,
            WorldType::SomethingElse => WorldType::Grasslands,
        }
    }
    pub fn cycle_prev_world(&mut self) {
        self.current_world = match self.current_world {
            WorldType::Grasslands => WorldType::SomethingElse,
            WorldType::SomethingElse => WorldType::Grasslands,
        }
    }

}

impl Default for WorldSelectController {
    fn default() -> Self {
        Self { 
            current_world: Default::default(), 
            max_camera_speed: 8.0, 
            min_camera_speed: 0.001,
            select_world_key: KeyCode::Space, 
        }
    }
}

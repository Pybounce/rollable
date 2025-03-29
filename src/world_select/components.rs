

use bevy::{prelude::*, utils::hashbrown::HashMap};

/// Marker component to describe an entity that is part of the world select scene
#[derive(Component)]
pub struct WorldSelectEntity;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub enum WorldId {
    #[default]
    Grasslands = 1,
    SomethingElse = 2
}

#[derive(Resource, Default)]
pub struct WorldSelectController {
    pub current_world: WorldId,
    world_entities: HashMap<WorldId, Entity>
}

impl WorldSelectController {
    pub fn add_world(&mut self, id: WorldId, entity: Entity) {
        self.world_entities.insert(id, entity);
    }
    pub fn current_world_entity(&self) -> Entity {
        return self.world_entities[&self.current_world];
    }
}


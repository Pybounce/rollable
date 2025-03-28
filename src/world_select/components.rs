
use bevy::prelude::*;

/// Marker component to describe an entity that is part of the world select scene
#[derive(Component)]
pub struct WorldSelectEntity;

#[derive(Default, Clone)]
pub enum World {
    #[default]
    Grasslands = 1,
    SomethingElse = 2
}
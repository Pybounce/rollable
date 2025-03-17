
use bevy::prelude::*;

#[derive(Component)]
pub struct Bouncy {
    pub factor: f32
}

impl Default for Bouncy {
    fn default() -> Self {
        Self { factor: 10.0 }
    }
}

#[derive(Component)]
pub struct Bounceable;




use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Resource)]
pub struct ParticleEffects {
    pub player_ground_movement: Handle<EffectAsset>
}


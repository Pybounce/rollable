
use bevy::prelude::*;
use bevy_hanabi::EffectAsset;

#[derive(Resource)]
pub struct SharedAssets {
    pub base_material: Handle<StandardMaterial>,
}
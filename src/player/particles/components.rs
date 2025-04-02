
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Resource)]
pub struct PlayerParticleEffects {
    pub player_running_eff_handle: Handle<EffectAsset>,
    pub player_landing_eff_handle: Handle<EffectAsset>
}

#[derive(Component)]
pub struct PlayerGroundRunningParticleEmiter;

#[derive(Component)]
pub struct PlayerGroundLandingParticleEmiter;
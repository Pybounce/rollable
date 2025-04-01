
use avian3d::prelude::LinearVelocity;
use bevy::{prelude::*, render::mesh::primitives};
use bevy_hanabi::prelude::*;

use crate::player::components::{Grounded, Player};

use super::{components::PlayerParticleEffects, functions::*};


pub fn register_player_particles(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands
) {
    let running_effect_handle = effects.add(create_player_ground_running_particles(&mut meshes));
    let landing_effect_handle = effects.add(create_player_ground_landing_particles(&mut meshes));

    commands.insert_resource(PlayerParticleEffects {
        player_running_eff_handle: running_effect_handle,
        player_landing_eff_handle: landing_effect_handle,
    });

}


pub fn animate_player_particles(
    mut query: Query<&mut EffectInitializers>,
    query2: Query<(Option<&Grounded>, &LinearVelocity), With<Player>>,
) {
    if let Ok(mut props) = query.get_single_mut() {
        if let Ok((grounded_opt, linvel)) = query2.get_single() {
            props.set_active(linvel.length() > 3.0 && !grounded_opt.is_none());
        }
    }
}


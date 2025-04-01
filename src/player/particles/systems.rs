
use avian3d::prelude::LinearVelocity;
use bevy::{prelude::*, render::mesh::primitives};
use bevy_hanabi::prelude::*;

use crate::player::components::{Grounded, Player};

use super::components::ParticleEffects;


pub fn register_player_ground_movement_particles(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands
) {
    let mesh = meshes.add(primitives::SphereMeshBuilder::default());
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.9, 0.9, 0.9, 1.0));
    gradient.add_key(1.0, Vec4::new(0.9, 0.9, 0.9, 1.0));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.01),
        dimension: ShapeDimension::Volume,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(1.),
    };



    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(0.8); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0.0, 3.0, 0.0));
    let update_accel = AccelModifier::new(accel);
    
    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.0));
    size_gradient.add_key(0.1, Vec3::splat(0.7));
    size_gradient.add_key(0.3, Vec3::splat(0.7));
    size_gradient.add_key(1.0, Vec3::splat(0.0));
    let size_modifer = SizeOverLifetimeModifier {
        gradient: size_gradient,
        screen_space_size: false
    };

    // Create the effect asset
    let effect = EffectAsset::new(
      // Maximum number of particles alive at a time
      32768,
      // Spawn at a rate of 5 particles per second
      Spawner::rate(30.0.into()),
      // Move the expression module into the asset
      module
    )
    .with_name("MyEffect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .update(update_accel)
    .mesh(mesh)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(size_modifer)
    .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);
    commands.insert_resource(ParticleEffects {
        player_ground_movement: effect_handle,
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



use bevy_hanabi::prelude::*;
use bevy::{math::VectorSpace, prelude::*, render::mesh::primitives};

pub fn create_player_ground_running_particles(meshes: &mut ResMut<Assets<Mesh>>) -> EffectAsset {
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
      SpawnerSettings::rate(30.0.into()),
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
    .render(ColorOverLifetimeModifier { gradient, ..default() });

    return effect;
}


pub fn create_player_ground_landing_particles(meshes: &mut ResMut<Assets<Mesh>>) -> EffectAsset {
    let mesh = meshes.add(primitives::SphereMeshBuilder::default());
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.9, 0.9, 0.9, 1.0));
    gradient.add_key(1.0, Vec4::new(0.9, 0.9, 0.9, 1.0));

    // Create a new expression module
    let mut module = Module::default();

    let init_pos = SetPositionCircleModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.5),
        dimension: ShapeDimension::Surface,
        axis: module.lit(Vec3::Y),
    };

    let init_vel = SetVelocityCircleModifier {
        axis: module.lit(Vec3::Y),
        center: module.lit(Vec3::ZERO),
        speed: module.lit(6.),    
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
      SpawnerSettings::once(30.0.into()),
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
    .render(ColorOverLifetimeModifier { gradient, ..default() });

    return effect;
}
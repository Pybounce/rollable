
use std::f32::consts::{FRAC_PI_4, PI};

use avian3d::prelude::{NoAutoCenterOfMass, RigidBody};
use bevy::prelude::*;

use crate::{loading::components::SharedAssets, stage::stage_builder::*};

use super::components::{WorldType, WorldSelectEntity};


pub fn island_pos_from_world(world_id: WorldType) -> Vec3 {
    return match world_id {
        WorldType::Grasslands => Quat::from_rotation_y(0.0) * Vec3::new(0.0, 0.0, -45.0),
        WorldType::SomethingElse => Quat::from_rotation_y(90.0 * (PI / 180.0)) * Vec3::new(0.0, 0.0, -45.0),
    };
}

pub fn build_world_one<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets,
) -> Entity {
    let floor0 = build_floor(commands, server, shared_assets, Vec3::ZERO, Vec3::new(30.0, 2.0, 30.0), Floor::Octagon).id();

    let tree = build_tree_m(commands, server, shared_assets, Vec3::new(3.0, 0.0, 0.0)).id();
    let loon = build_air_loon(commands, server, shared_assets, Vec3::new(8.0, 13.0, 8.0), Vec3::ONE * 2.0).id();
    let goal = build_goal(commands, server, shared_assets, Vec3::new(-3.0, 0.0, 0.0)).id();

    let mut entity = commands.spawn((
        WorldSelectEntity,
        Transform::from_translation(island_pos_from_world(WorldType::Grasslands)).with_rotation(Quat::from_rotation_y(FRAC_PI_4)),
        RigidBody::Kinematic,
        NoAutoCenterOfMass,
        //AngularVelocity(Vec3::new(0.0, 0.5, 0.0))
    ));
    entity.add_child(floor0);
    entity.add_child(tree);
    entity.add_child(goal);
    entity.add_child(loon);

    return entity.id();
}

pub fn build_world_two<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets,
) -> Entity {
    let floor0 = build_floor(commands, server, shared_assets, Vec3::ZERO, Vec3::new(30.0, 2.0, 30.0), Floor::Octagon).id();

    let tree = build_tree_m(commands, server, shared_assets, Vec3::new(3.0, 0.0, 0.0)).id();
    let goal = build_goal(commands, server, shared_assets, Vec3::new(-3.0, 0.0, 0.0)).id();

    let mut entity = commands.spawn((
        WorldSelectEntity,
        Transform::from_translation(island_pos_from_world(WorldType::SomethingElse)),
        RigidBody::Kinematic,
        NoAutoCenterOfMass,
        //AngularVelocity(Vec3::new(0.0, 0.5, 0.0))
    ));
    entity.add_child(floor0);
    entity.add_child(tree);
    entity.add_child(goal);

    return entity.id();
}







use std::sync::Arc;

use avian3d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;

use crate::{loading::components::SharedAssets, physics::GamePhysicsLayer, shared::bouncy::components::Bouncy};

use super::{components::Ground, stage_builder::{build_bounce_pad, build_floor_large}};


pub fn spawn_temp_stage(
    mut commands: Commands,
    server: Res<AssetServer>,
    shared_assets: Res<SharedAssets>
) {

    build_bounce_pad(&mut commands, &server, &shared_assets, Vec3::new(0.0, 1.0, 0.0));
    let a = build_floor_large(&mut commands, &server, &shared_assets, Vec3::ZERO);

}


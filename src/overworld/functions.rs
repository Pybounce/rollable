
use bevy::prelude::*;

use crate::{loading::components::SharedAssets, player::spawner::PlayerSpawner, stage::stage_builder::*};

use super::components::OverworldEntity;

pub fn build_grasslands_overworld<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
) {
    build_player_spawner(commands, Vec3::Y * 3.0).insert(OverworldEntity);

    let _floor = build_floor(commands, server, shared_assets, Vec3::new(5.0, -5.0, -5.0), Vec3::new(40.0, 30.0, 40.0), Floor::Rectangle).try_insert(OverworldEntity);
    build_stage_teleport(commands, server, shared_assets, Vec3::new(15.0, 1.0, -5.0), 1).insert(OverworldEntity);
}
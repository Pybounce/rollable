
use bevy::prelude::*;

use crate::{loading::components::SharedAssets, stage::stage_builder::*};

use super::components::OverworldEntity;

pub fn build_grasslands_overworld<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
) {
    let floor = build_floor(commands, server, shared_assets, Vec3::new(5.0, -5.0, -5.0), Vec3::new(40.0, 30.0, 40.0), Floor::Rectangle).try_insert(OverworldEntity);

}
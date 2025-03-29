
use bevy::prelude::*;

use crate::loading::components::SharedAssets;

use super::{components::{LoadOverworldConfig, OverworldEntity}, functions::*};

pub fn spawn_overworld_stage(
    mut commands: Commands,
    server: Res<AssetServer>,
    shared_assets: Res<SharedAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    overworld_config: Res<LoadOverworldConfig>,

) {
    match overworld_config.world_type {
        crate::world_select::components::WorldType::Grasslands => build_grasslands_overworld(&mut commands, &server, &shared_assets),
        crate::world_select::components::WorldType::SomethingElse => todo!(),
    }
}

pub fn teardown_overworld(
    mut commands: Commands,
    query: Query<Entity, With<OverworldEntity>>
) {
    for e in &query {
        commands.entity(e).try_despawn_recursive();
    }
}
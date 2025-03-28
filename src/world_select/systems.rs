
use avian3d::prelude::AngularVelocity;
use bevy::prelude::*;

use crate::{loading::components::SharedAssets, stage::stage_builder::{build_floor, Floor}, states::AppState};

use super::components::WorldSelectEntity;

pub fn build_world_select(
    mut commands: Commands,
    server: Res<AssetServer>,
    shared_assets: Res<SharedAssets>,
    mut cam_query: Query<&mut Transform, With<Camera3d>>
) {
    if let Ok(mut cam_transform) = cam_query.get_single_mut() {
        cam_transform.translation = Vec3::default();
        cam_transform.rotation = Quat::default();
    }
    build_world_one(&mut commands, &server, &shared_assets);
}

pub fn teardown_world_select(
    mut commands: Commands,
    query: Query<Entity, With<WorldSelectEntity>>
) {
    for e in &query {
        commands.entity(e).try_despawn_recursive();
    }
}

pub fn select_world(
    mut app_state: ResMut<NextState<AppState>>
) {
    
}

fn build_world_one<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets,
) {
    let mut floor = build_floor(commands, server, shared_assets, Vec3::new(0.0, -1.2, -50.0), Vec3::new(10.0, 2.0, 10.0), Floor::Rectangle);
    floor.try_insert(AngularVelocity(Vec3::new(0.0, 1.0, 0.0)));
}
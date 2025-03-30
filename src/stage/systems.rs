

use std::sync::Arc;

use avian3d::prelude::{Collider, CollisionLayers, LinearVelocity, RigidBody};
use bevy::{color::palettes::css::{BLUE, SILVER}, prelude::*};

use crate::{loading::components::SharedAssets, physics::GamePhysicsLayer, shared::{bouncy::components::Bouncy, mover::components::OffsetMover}};

use super::{components::LoadStageConfig, stage_builder::*};

pub fn build_stage(
    load_stage_config: Res<LoadStageConfig>,
    mut commands: Commands,
    server: Res<AssetServer>,
    shared_assets: Res<SharedAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>
) {
    let water_mat = StandardMaterial {
        perceptual_roughness: 0.6,
        base_color: Color::linear_rgb(35.0/255.0, 137.0/255.0, 218.0/255.0),
        ..default()
    };

    let mut water_offset_mover = OffsetMover::bobbing_offset(0.5);
    water_offset_mover.with_speed(0.2);
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
        MeshMaterial3d(mats.add(water_mat)),
        Transform::from_translation(Vec3::new(0.0, -2.0, 0.0)),
    ));

    match load_stage_config.stage_id() {
        1 => build_stage_1(&mut commands, &server, &shared_assets),
        _ => ()
    };

}

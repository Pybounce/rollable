

use std::sync::Arc;

use avian3d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::{color::palettes::css::{BLUE, SILVER}, prelude::*};

use crate::{loading::components::SharedAssets, physics::GamePhysicsLayer, shared::{bouncy::components::Bouncy, mover::components::OffsetMover}};

use super::stage_builder::*;


pub fn spawn_temp_stage(
    mut commands: Commands,
    server: Res<AssetServer>,
    shared_assets: Res<SharedAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>
) {

    let water_mat = StandardMaterial {
        perceptual_roughness: 1.0,
        base_color: Color::linear_rgb(35.0/255.0, 137.0/255.0, 218.0/255.0),
        ..default()
    };

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2000.0, 2000.0).subdivisions(10))),
        MeshMaterial3d(mats.add(water_mat)),
        Transform::from_translation(Vec3::new(0.0, -10.0, 0.0))
    ));


    build_bounce_pad(&mut commands, &server, &shared_assets, Vec3::new(0.0, 1.0, 0.0));
    build_floor_large(&mut commands, &server, &shared_assets, Vec3::ZERO);
    build_floor_large(&mut commands, &server, &shared_assets, Vec3::new(30.0, -11.0, 0.0));
    build_pillar_m(&mut commands, &server, &shared_assets, Vec3::new(30.0, 0.0, 0.0)).try_insert(OffsetMover::bobbing_offset(10.0));
    build_pillar_m(&mut commands, &server, &shared_assets, Vec3::new(40.0, 10.0, 10.0)).try_insert(OffsetMover::bobbing_offset(-10.0));
    build_pillar_m(&mut commands, &server, &shared_assets, Vec3::new(50.0, 0.0, 0.0)).try_insert(OffsetMover::bobbing_offset(10.0));
    build_pillar_m(&mut commands, &server, &shared_assets, Vec3::new(24.0, 0.0, -10.0)).try_insert(OffsetMover::from_offsets(vec![Vec3::new(10.0, 0.0, 0.0), Vec3::new(-10.0, 0.0, 0.0)]));
    build_floor_large(&mut commands, &server, &shared_assets, Vec3::new(57.0, 0.0, 0.0));

}


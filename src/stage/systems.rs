

use std::sync::Arc;

use avian3d::prelude::{Collider, CollisionLayers, LinearVelocity, RigidBody};
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
    ));//.try_insert(water_offset_mover);
    

    //main floor
    build_floor(&mut commands, &server, &shared_assets, Vec3::ZERO, Vec3::new(40.0, 20.0, 40.0), Floor::Rectangle);
    build_floor(&mut commands, &server, &shared_assets, Vec3::new(-15.0, 5.0, 15.0), Vec3::new(20.0, 30.0, 20.0), Floor::Rectangle);
    build_floor(&mut commands, &server, &shared_assets, Vec3::new(5.0, -5.0, -5.0), Vec3::new(40.0, 30.0, 40.0), Floor::Rectangle);
    

    build_bounce_pad(&mut commands, &server, &shared_assets, Vec3::new(0.0, 1.0, 0.0));
    build_pillar_m(&mut commands, &server, &shared_assets, Vec3::new(33.0, 0.0, 0.0)).try_insert(OffsetMover::bobbing_offset(10.0));
    build_pillar_m(&mut commands, &server, &shared_assets, Vec3::new(47.0, 10.0, 10.0)).try_insert(OffsetMover::bobbing_offset(-10.0));
    build_floor(&mut commands, &server, &shared_assets, Vec3::new(77.0, 0.0, 0.0), Vec3::new(40.0, 20.0, 40.0), Floor::Octagon);
    build_floor(&mut commands, &server, &shared_assets, Vec3::new(23.0, 0.0, -10.0), Vec3::new(3.0, 0.5, 3.0), Floor::Octagon).try_insert(OffsetMover::from_offsets(vec![Vec3::new(15.0, 0.0, 0.0), Vec3::new(-15.0, 0.0, 0.0)]));
    build_floor(&mut commands, &server, &shared_assets, Vec3::new(53.0, 0.0, -6.0), Vec3::new(3.0, 0.5, 3.0), Floor::Octagon).try_insert(OffsetMover::from_offsets(vec![Vec3::new(-15.0, 0.0, 0.0), Vec3::new(15.0, 0.0, 0.0)]));
    build_tree_m(&mut commands, &server, &shared_assets, Vec3::ZERO);
    build_tree_m_patch(&mut commands, &server, &shared_assets, Vec3::new(77.0, 0.0, 0.0));
    build_rock(&mut commands, &server, &shared_assets, Vec3::new(5.0, 0.0, 5.0), Vec3::ONE);


    build_rock(&mut commands, &server, &shared_assets, Vec3::new(40.0, -10.0, 5.0), Vec3::ONE);
    build_rock(&mut commands, &server, &shared_assets, Vec3::new(10.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.2));
    build_goal(&mut commands, &server, &shared_assets, Vec3::new(77.0, 0.0, -5.0));
    
    build_obstacle_sweeper(&mut commands, &server, &shared_assets, Vec3::new(-50.0, 0.0, 0.0), 4.5, 10.0, 2);
    build_obstacle_sweeper(&mut commands, &server, &shared_assets, Vec3::new(-75.0, 4.0, 10.0), 4.5, 10.0, 4);
    build_obstacle_sweeper(&mut commands, &server, &shared_assets, Vec3::new(-100.0, 0.0, 0.0), 4.5, 10.0, 2);
    build_floor(&mut commands, &server, &shared_assets, Vec3::new(-50.0, 0.0, 0.0), Vec3::new(20.0, 20.0, 20.0), Floor::Octagon);
    build_floor(&mut commands, &server, &shared_assets, Vec3::new(-75.0, 4.0, 10.0), Vec3::new(20.0, 20.0, 20.0), Floor::Octagon);
    build_floor(&mut commands, &server, &shared_assets, Vec3::new(-100.0, 0.0, 0.0), Vec3::new(20.0, 20.0, 20.0), Floor::Octagon);

}

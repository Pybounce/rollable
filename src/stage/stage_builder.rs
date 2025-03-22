
use avian3d::prelude::{Collider, CollisionLayers, GravityScale, LinearVelocity, RigidBody};
use bevy::prelude::*;

use crate::{loading::components::SharedAssets, physics::GamePhysicsLayer, shared::{bouncy::components::Bouncy, mover::components::OffsetMover}};

use super::components::Ground;

pub fn build_bounce_pad<'c>(
    commands: &'c mut Commands, 
    server: &Res<AssetServer>, 
    shared_assets: &SharedAssets, 
    pos: Vec3) -> EntityCommands<'c> {

    let bouncepad_frame: Handle<Mesh> = server.load("bouncepad_frame.glb#Mesh0/Primitive0");
    let bouncepad_platform: Handle<Mesh> = server.load("bouncepad_platform.glb#Mesh0/Primitive0");
    let mat = shared_assets.base_material.clone();

    let mut entity_commands = commands.spawn((GlobalTransform::default(), Transform::default()));

    entity_commands.with_children(|p| {
        p.spawn((
            Mesh3d(bouncepad_platform.clone()),
            MeshMaterial3d(mat.clone()),
            Collider::cuboid(2.9, 0.5, 2.9),
            RigidBody::Static,
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_x(-45.0)),
            CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
            Bouncy::default()
        ));
        p.spawn((
            Mesh3d(bouncepad_frame.clone()),
            MeshMaterial3d(mat.clone()),
            Collider::cuboid(3.4, 0.25, 3.4),
            RigidBody::Static,
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_x(-45.0)),
            CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
            Ground
        ));
    });

    return entity_commands;
}

pub fn build_floor_large<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
    pos: Vec3) -> EntityCommands<'c> {

    let mesh: Handle<Mesh> = server.load("floor_large.glb#Mesh0/Primitive0");
    let mat = shared_assets.base_material.clone();

    return commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat.clone()),
        Collider::cuboid(40.0, 20.0, 40.0),
        RigidBody::Static,
        Transform::from_translation(pos - Vec3::new(0.0, 10.0, 0.0)),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Ground
    ));
}

pub fn build_pillar_m<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
    pos: Vec3) -> EntityCommands<'c> {

    let mesh: Handle<Mesh> = server.load("pillar_m.glb#Mesh0/Primitive0");
    let mat = shared_assets.base_material.clone();

    return commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat.clone()),
        Collider::cylinder(1.5, 20.0),
        RigidBody::Kinematic,
        Transform::from_translation(pos - Vec3::new(0.0, 10.0, 0.0)),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Ground,
        LinearVelocity::default(),
    ));
}

pub fn build_floor_s<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
    pos: Vec3) -> EntityCommands<'c> {

    let mesh: Handle<Mesh> = server.load("floor_s.glb#Mesh0/Primitive0");
    let mat = shared_assets.base_material.clone();

    return commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat.clone()),
        Collider::cuboid(3.75, 1.35, 3.75),
        RigidBody::Kinematic,
        Transform::from_translation(pos - Vec3::new(0.0, 0.0, 0.0)),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Ground,
        LinearVelocity::default(),
    ));
}

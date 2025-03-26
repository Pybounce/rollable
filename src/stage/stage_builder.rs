
use avian3d::{math::{FRAC_PI_2, PI}, prelude::{AngularVelocity, CenterOfMass, Collider, CollisionLayers, FixedJoint, Friction, GravityScale, LinearVelocity, NoAutoCenterOfMass, RigidBody}};
use bevy::{math::VectorSpace, prelude::*};

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

pub fn build_rock<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
    pos: Vec3,
    scale: Vec3) -> EntityCommands<'c> {

    let mesh: Handle<Mesh> = server.load("rock_01.glb#Mesh0/Primitive0");
    let mat = shared_assets.base_material.clone();

    return commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat.clone()),
        Collider::cylinder(1.5, 20.0),
        RigidBody::Kinematic,
        Transform::from_translation(pos).with_scale(scale),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Ground,
        LinearVelocity::default(),
    ));
}

pub enum Floor {
    Rectangle,
    Octagon
}

pub fn build_floor<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
    pos: Vec3,
    scale: Vec3,
    floor_type: Floor) -> EntityCommands<'c> {

    let (base_mesh, top_mesh) = match floor_type {
        Floor::Rectangle => (server.load("floor_rect_base.glb#Mesh0/Primitive0"), server.load("floor_rect_top.glb#Mesh0/Primitive0")),
        Floor::Octagon => (server.load("floor_oct_base.glb#Mesh0/Primitive0"), server.load("floor_oct_top.glb#Mesh0/Primitive0")),
    };

    let mat = shared_assets.base_material.clone();
    let mut entity_commands = commands.spawn((
        GlobalTransform::default(), 
        Transform::default(), 
        RigidBody::Kinematic,
        LinearVelocity::default(),
    ));
    entity_commands.with_children(|p| {
        p.spawn((
            Mesh3d(top_mesh.clone()),
            MeshMaterial3d(mat.clone()),
            Collider::cuboid(1.0, 1.0, 1.0),
            Transform::from_translation(pos - Vec3::new(0.0, 0.25, 0.0)).with_scale(Vec3::new(scale.x + 0.5, 0.5, scale.z + 0.5)),
            Ground,
            CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        ));
        p.spawn((
            Mesh3d(base_mesh.clone()),
            MeshMaterial3d(mat.clone()),
            Collider::cuboid(1.0, 1.0, 1.0),
            Transform::from_translation(pos - Vec3::new(0.0, (scale.y / 2.0) + 0.5, 0.0)).with_scale(scale),
            Ground,
            CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        ));
    });

    return entity_commands;
}

pub fn build_tree_m<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
    pos: Vec3) -> EntityCommands<'c> {

    let mesh: Handle<Mesh> = server.load("tree_01.glb#Mesh0/Primitive0");
    let mat = shared_assets.base_material.clone();

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat.clone()),
        Collider::cylinder(1.5, 20.0),
        RigidBody::Kinematic,
        Transform::from_translation(pos),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Ground,
        LinearVelocity::default(),
    ));
    let mut entity_commands = commands.spawn((GlobalTransform::default(), Transform::default()));

    entity_commands.with_children(|p| {
        p.spawn((
            Collider::cuboid(2.9, 0.5, 2.9),
            RigidBody::Kinematic,
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_x(-45.0)),
            CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        ));
        p.spawn((
            Collider::cuboid(2.9, 0.5, 2.9),
            RigidBody::Kinematic,
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_x(-45.0)),
            CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        ));
    });

    return entity_commands;
}

pub fn build_tree_m_patch(
    commands: &mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
    pos: Vec3) {

        build_tree_m(commands, server, shared_assets, pos + Vec3::new(0.0, 0.0, 6.0));
        build_tree_m(commands, server, shared_assets, pos + Vec3::new(6.0, -2.0, 0.0));
        build_tree_m(commands, server, shared_assets, pos + Vec3::new(-6.0, 0.0, 0.0));
}

pub fn build_goal<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
    pos: Vec3) -> EntityCommands<'c> {

    let mesh: Handle<Mesh> = server.load("goal.glb#Mesh0/Primitive0");
    let mat = shared_assets.base_material.clone();

    return commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat.clone()),
        //Collider::cylinder(1.5, 20.0),
        RigidBody::Kinematic,
        Transform::from_translation(pos),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Ground,
        LinearVelocity::default(),
    ));
}
pub fn build_obstacle_sweeper<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
    pos: Vec3,
    rotation: Quat,
    speed: f32,
    arm_length: f32,
    arm_count: u8) -> EntityCommands<'c> {

    let post: Handle<Mesh> = server.load("post.glb#Mesh0/Primitive0");
    let sweeper_join: Handle<Mesh> = server.load("sweeper_join.glb#Mesh0/Primitive0");
    let sweeper_arm: Handle<Mesh> = server.load("sweeper_arm.glb#Mesh0/Primitive0");
    let mat = shared_assets.base_material.clone();

    let mut entity_commands = commands.spawn((
        GlobalTransform::default(), 
        Transform::from_translation(pos).with_rotation(rotation),
        RigidBody::Kinematic,
        AngularVelocity(Vec3::new(0.0, speed, 0.0)),
        NoAutoCenterOfMass
    ));

    entity_commands.with_children(|p| {
        p.spawn((
            Mesh3d(post.clone()),
            MeshMaterial3d(mat.clone()),
            Transform::from_translation(Vec3::ZERO),
        )).with_child((
            Transform::from_translation(Vec3::new(0.0, 1.25, 0.0)),
            Collider::cylinder(0.75, 2.0),
            CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        ));

        let angle_offset = (2.0 * PI) / arm_count as f32;

        for i in 0..arm_count {
            let angle = i as f32 * angle_offset;
            p.spawn((
                Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2) * Quat::from_rotation_x(angle)),
            )).with_children(|r| {
                r.spawn((
                    Mesh3d(sweeper_join.clone()),
                    MeshMaterial3d(mat.clone()),
                    Collider::cylinder(0.25, 1.0),
                    Transform::from_translation(Vec3::new(1.0, 1.0, 0.0)),
                    CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
                ));
                r.spawn((
                    Mesh3d(sweeper_arm.clone()),
                    MeshMaterial3d(mat.clone()),
                    Collider::cylinder(0.5, 1.0),
                    Transform::from_translation(Vec3 { x: 1.0, y: 1.5 + (arm_length / 2.0), z: 0.0 }).with_scale(Vec3::new(1.0, arm_length, 1.0)),
                    CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
                ));
            });
        }


    });


    return entity_commands;
}

pub fn build_air_loon<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets, 
    pos: Vec3,
    scale: Vec3) -> EntityCommands<'c> {

    let mesh: Handle<Mesh> = server.load("hot_air_loon.glb#Mesh0/Primitive0");
    let mat = shared_assets.base_material.clone();

    return commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(pos).with_scale(scale),
    ));
}

































use avian3d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;

use crate::{physics::GamePhysicsLayer, shared::bouncy::components::Bouncy};

use super::components::Ground;


pub fn spawn_temp_stage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let cuboid_mesh = meshes.add(Cuboid::new(50.0, 1.0, 50.0));
    let material = materials.add(StandardMaterial::default());

    commands.spawn((
        Mesh3d(cuboid_mesh),
        MeshMaterial3d(material),
        Collider::cuboid(50.0, 1.0, 50.0),
        RigidBody::Static,
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Ground
    ));

}

pub fn spawn_temp_bouncepad(
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    let bouncepad_frame: Handle<Mesh> = server.load("bouncepad_frame.glb#Mesh0/Primitive0");
    let bouncepad_platform: Handle<Mesh> = server.load("bouncepad_platform.glb#Mesh0/Primitive0");
    let pink = materials.add(Color::srgb_u8(246, 161, 255));
    let blue = materials.add(Color::srgb_u8(161, 213, 255));


    commands.spawn((
        Mesh3d(bouncepad_platform.clone()),
        MeshMaterial3d(blue.clone()),
        Collider::cuboid(2.9, 0.5, 2.9),
        RigidBody::Static,
        Transform::from_translation(Vec3::new(20.0, 0.6, 0.0)),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Bouncy::default()
    ));

    commands.spawn((
        Mesh3d(bouncepad_frame.clone()),
        MeshMaterial3d(pink.clone()),
        Collider::cuboid(3.4, 0.25, 3.4),
        RigidBody::Static,
        Transform::from_translation(Vec3::new(20.0, 0.6, 0.0)),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Ground
    ));

    commands.spawn((
        Mesh3d(bouncepad_platform),
        MeshMaterial3d(blue),
        Collider::cuboid(2.9, 0.5, 2.9),
        RigidBody::Static,
        Transform::from_translation(Vec3::new(20.0, 0.5, 5.0)),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Bouncy::default()
    ));

    commands.spawn((
        Mesh3d(bouncepad_frame),
        MeshMaterial3d(pink),
        Collider::cuboid(3.4, 0.25, 3.4),
        RigidBody::Static,
        Transform::from_translation(Vec3::new(20.0, 0.5, 5.0)),
        CollisionLayers::new(GamePhysicsLayer::Ground, [GamePhysicsLayer::Ball]),
        Ground
    ));

}




use avian3d::prelude::{Collider, Gravity, GravityScale, LinearVelocity, RigidBody};
use bevy::prelude::*;

use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mesh = meshes.add(Sphere::default());
    let material = materials.add(StandardMaterial::default());

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Player,
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
        Collider::sphere(0.5),
        RigidBody::Dynamic,
        GravityScale(1.0),
        LinearVelocity::default()
    ));
    
}
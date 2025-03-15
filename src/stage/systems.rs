

use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;


pub fn spawn_temp_stage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let cuboid_mesh = meshes.add(Cuboid::default());
    let material = materials.add(StandardMaterial::default());

    commands.spawn((
        Mesh3d(cuboid_mesh),
        MeshMaterial3d(material),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Static,
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));

}



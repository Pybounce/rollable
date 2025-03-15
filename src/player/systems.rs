

use avian3d::prelude::{Collider, CollidingEntities, CollisionLayers, GravityScale, LinearVelocity, RigidBody};
use bevy::prelude::*;

use crate::{physics::GamePhysicsLayer, stage::components::Ground};

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
        GravityScale(2.5),
        LinearVelocity::default(),
        PlayerController::default(),
        CollisionLayers::new(GamePhysicsLayer::Ball, [GamePhysicsLayer::Ground]),
        CollidingEntities::default()
    ));
    
}

pub fn move_balls(
    mut player_query: Query<(&mut LinearVelocity, &PlayerController, &Transform), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let camera_transform = camera_query.single();
    
    for (mut linvel, player_controller, player_transform) in &mut player_query {
        let dir = (player_transform.translation - camera_transform.translation).xz().extend(0.0).xzy().normalize_or_zero();
        let perpen_dir = Vec3::new(dir.z, 0.0, -dir.x);

         if input.pressed(player_controller.forwards_key) {
            linvel.0 += dir * player_controller.force * time.delta_secs();
        }

        if input.pressed(player_controller.backwards_key) {
            linvel.0 -= dir * player_controller.force * time.delta_secs();
        }
        
        if input.pressed(player_controller.right_key) {
            linvel.0 -= perpen_dir * player_controller.force * time.delta_secs();
        }

        if input.pressed(player_controller.left_key) {
            linvel.0 += perpen_dir * player_controller.force * time.delta_secs();
            
        }   
    }
}

pub fn apply_ball_friction(
    mut player_query: Query<(&mut LinearVelocity, &PlayerController), With<Player>>,
    time: Res<Time>
) {
    for (mut linvel, player_controller) in &mut player_query {
        let force = linvel.0 * linvel.0 * player_controller.friction_c;
        let dir = linvel.0.normalize_or_zero();
        linvel.0 -= dir * force * time.delta_secs();
    }
}

pub fn check_grounded(
    query: Query<(Entity, &CollidingEntities), With<Player>>,
    ground_query: Query<(), With<Ground>>,
    mut commands: Commands
) {
    for (player_entity, colliding_entities) in &query {
        commands.entity(player_entity).remove::<Grounded>();
        for colliding_entity in colliding_entities.iter() {
            if ground_query.contains(*colliding_entity) {
                commands.entity(player_entity).try_insert(Grounded);
            }
        }
    }
}

pub fn jump_balls(
    mut query: Query<(&mut LinearVelocity, &JumpController), (With<Player>, With<Grounded>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut linvel, jump_con) in &mut query {
        if input.pressed(jump_con.jump_key) {
            linvel.y = jump_con.jump_force;
        }
    }
}


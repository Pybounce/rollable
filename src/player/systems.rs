

use avian3d::prelude::{Collider, CollidingEntities, CollisionLayers, Friction, GravityScale, LinearVelocity, RigidBody, SpeculativeMargin};
use bevy::{math::VectorSpace, prelude::*};

use crate::{physics::GamePhysicsLayer, shared::{bouncy::components::Bounceable, death::MarkOfDeath}, stage::components::Ground};

use super::components::*;


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
        let mut vel_dir =  Vec3::ZERO;

         if input.pressed(player_controller.forwards_key) {
            vel_dir += dir;
        }

        if input.pressed(player_controller.backwards_key) {
            vel_dir -= dir;
        }
        
        if input.pressed(player_controller.right_key) {
            vel_dir -= perpen_dir;
        }

        if input.pressed(player_controller.left_key) {
            vel_dir += perpen_dir;
        }

        linvel.0 += vel_dir.normalize_or_zero() * player_controller.force * time.delta_secs();
    }
}

pub fn apply_ball_friction(
    mut player_query: Query<(&mut LinearVelocity, &PlayerController), With<Player>>,
    time: Res<Time>
) {
    for (mut linvel, player_controller) in &mut player_query {
        let mut force_mag = (linvel.0 * linvel.0 * player_controller.friction_c).length();
        force_mag = force_mag.max(player_controller.min_friction_force).min(linvel.0.length());
        let dir = linvel.0.normalize_or_zero();
        linvel.0 -= force_mag * dir * time.delta_secs();
    }
}

pub fn check_grounded(
    query: Query<(Entity, &CollidingEntities, Option<&Grounded>), With<Player>>,
    ground_query: Query<(), With<Ground>>,
    mut commands: Commands
) {
    for (player_entity, colliding_entities, grounded_opt) in &query {
        let mut add_grounded = false;
        for colliding_entity in colliding_entities.iter() {
            if ground_query.contains(*colliding_entity) {
                add_grounded = true;
            }
        }
        match (add_grounded, grounded_opt) {
            (true, None) => { commands.entity(player_entity).try_insert(Grounded); },
            (true, Some(_)) => (),
            (false, None) => (),
            (false, Some(_)) => { commands.entity(player_entity).remove::<Grounded>(); },
        };
    }
}

pub fn start_jumping_balls(
    mut query: Query<(&mut LinearVelocity, &mut JumpController), (With<Player>, With<Grounded>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut linvel, mut jump_con) in &mut query {
        if input.pressed(jump_con.key) {
            jump_con.current_force = jump_con.max_force;
            linvel.y = jump_con.initial_force.max(linvel.y);
            jump_con.timer.reset();
        }
    }
}

pub fn jumping_balls(
    mut query: Query<(&mut JumpController, &mut LinearVelocity), With<Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut jump_con, mut linvel) in &mut query {
        jump_con.timer.tick(time.delta());
        if !jump_con.timer.finished() && input.pressed(jump_con.key) {
            linvel.y += jump_con.current_force * time.delta_secs();
            jump_con.current_force -= jump_con.force_deceleration * time.delta_secs();
            jump_con.current_force = jump_con.current_force.max(0.0);
        }
    }
}

pub fn end_jumping_balls(
    mut query: Query<&mut JumpController, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for mut jump_con in &mut query {
        if !input.pressed(jump_con.key) && !jump_con.timer.finished() {
            let remaining = jump_con.timer.remaining();
            jump_con.timer.tick(remaining);
        }
    }
}

pub fn kill_ball(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Player>>,
) {
    for (e, t) in &mut query {
        if t.translation.y < -5.0 {
            commands.entity(e).try_insert(MarkOfDeath);
        }
    }
}
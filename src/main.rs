mod camera;
mod player;
mod stage;
pub mod physics;
pub mod shared;

use avian3d::{math::PI, prelude::{Gravity, PhysicsDebugPlugin}, PhysicsPlugins};
use bevy::prelude::*;
use camera::*;
use player::systems::*;
use shared::bouncy::systems::*;
use stage::systems::*;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_temp_stage, lighting, spawn_temp_bouncepad))
        .add_systems(Update, (move_camera, move_balls, apply_ball_friction, start_jumping_balls, jumping_balls, end_jumping_balls, check_grounded))
        .add_systems(Update, bounce)
        .insert_resource(Gravity(Vec3::NEG_Y * 10.0))
        .run();
}


fn lighting(
    mut commands: Commands
) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true, 
            ..default() 
        },
        Transform::from_rotation(Quat::from_rotation_x(-PI / 4.0))
    ));
}
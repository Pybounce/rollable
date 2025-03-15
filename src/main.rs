mod camera;
mod player;
mod stage;
pub mod physics;

use avian3d::{prelude::Gravity, PhysicsPlugins};
use bevy::prelude::*;
use camera::*;
use player::systems::*;
use stage::systems::spawn_temp_stage;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_temp_stage))
        .add_systems(Update, (move_camera, move_balls, apply_ball_friction, jump_balls, check_grounded))
        .insert_resource(Gravity(Vec3::NEG_Y * 10.0))
        .run();
}

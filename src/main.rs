mod camera;
mod player;
mod stage;

use avian3d::{prelude::Gravity, PhysicsPlugins};
use bevy::prelude::*;
use camera::*;
use player::systems::spawn_player;
use stage::systems::spawn_temp_stage;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_temp_stage))
        .add_systems(Update, (move_camera))
        .insert_resource(Gravity(Vec3::NEG_Y * 10.0))
        .run();
}

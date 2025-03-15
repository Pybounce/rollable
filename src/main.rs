mod camera;

use bevy::{prelude::*, winit::{UpdateMode, WinitSettings}};
use camera::*;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .run();
}

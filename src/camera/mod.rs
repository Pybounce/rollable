
use bevy::prelude::*;

pub fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera3d::default());
}
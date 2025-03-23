mod camera;
mod player;
mod stage;
pub mod physics;
pub mod shared;
pub mod loading;

use std::process::exit;

use avian3d::{math::PI, prelude::{Gravity, PhysicsDebugPlugin}, PhysicsPlugins};
use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use camera::{post_processing::PostProcessPlugin, *};
use loading::systems::load_stage_assets;
use player::systems::*;
use shared::{bouncy::systems::*, mover::systems::move_offset_movers};
use stage::systems::*;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PostProcessPlugin)
        //.add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.7, 0.85, 0.95)))
        .add_systems(Update, (try_exit_game, toggle_cursor_lock))
        .add_systems(Startup, (load_stage_assets, (spawn_camera, spawn_player, spawn_temp_stage, lighting)).chain())
        .add_systems(Update, (move_camera, zoom_camera, move_balls, apply_ball_friction, start_jumping_balls, jumping_balls, end_jumping_balls, check_grounded))
        .add_systems(Update, (bounce, move_offset_movers))
        .insert_resource(Gravity(Vec3::NEG_Y * 10.0))
        .run();
}


fn lighting(
    mut commands: Commands
) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 1000.0,
            ..default() 
        },
        Transform::from_rotation(Quat::from_rotation_x(-PI / 4.0))
    ));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2000.0,
    });
}

fn toggle_cursor_lock(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyL) {
        let mut primary_window = q_windows.single_mut();
        match primary_window.cursor_options.grab_mode {
            CursorGrabMode::None => {
                primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
                primary_window.cursor_options.visible = false;
            },
            CursorGrabMode::Confined => (),
            CursorGrabMode::Locked => {
                primary_window.cursor_options.grab_mode = CursorGrabMode::None;
                primary_window.cursor_options.visible = true;
            },
        }
    }
    
}

fn try_exit_game(
    input: Res<ButtonInput<KeyCode>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if input.pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();
        primary_window.cursor_options.grab_mode = CursorGrabMode::None;
        primary_window.cursor_options.visible = true;
        exit(0);
    }
}
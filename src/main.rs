mod camera;
mod player;
mod stage;
pub mod physics;
pub mod shared;
pub mod loading;
mod states;
mod main_menu;
mod world_select;
pub mod overworld;

use std::process::exit;

use avian3d::{math::{FRAC_PI_2, PI}, prelude::{Gravity, PhysicsDebugPlugin}, PhysicsPlugins};
use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_hanabi::HanabiPlugin;
use camera::{post_processing::PostProcessPlugin, *};
use loading::systems::load_stage_assets;
use main_menu::systems::{build_main_menu, continue_from_main_menu, teardown_main_menu};
use overworld::{stage_teleports::teleport_player_to_stage, systems::*};
use player::{death::kill_player, particles::register_player_ground_movement_particles, spawner::try_spawn_player, systems::*};
use shared::{bouncy::systems::*, follower::move_followers, mover::systems::move_offset_movers};
use stage::systems::*;
use states::AppState;
use world_select::systems::*;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(HanabiPlugin)
        .init_state::<AppState>()
        .add_plugins(PostProcessPlugin)
        //.add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.7, 0.85, 0.95)))
        .add_systems(Update, (move_followers, kill_ball, try_exit_game, toggle_cursor_lock))
        .add_systems(Update, (update_toon_shader_settings, move_camera, zoom_camera, move_balls, apply_ball_friction, start_jumping_balls, jumping_balls, end_jumping_balls, check_grounded))
        .add_systems(Update, (teleport_player_to_stage, kill_player, try_spawn_player, bounce, move_offset_movers))
        .insert_resource(Gravity(Vec3::NEG_Y * 10.0))
        .add_systems(Startup, (spawn_camera, lighting, load_stage_assets).chain())
        //main menu
        .add_systems(OnEnter(AppState::MainMenu), (build_main_menu, register_player_ground_movement_particles))
        .add_systems(OnExit(AppState::MainMenu), teardown_main_menu)
        .add_systems(Update, (continue_from_main_menu).run_if(in_state(AppState::MainMenu)))
        //world select
        .add_systems(OnEnter(AppState::WorldSelect), init_world_select)
        .add_systems(OnExit(AppState::WorldSelect), teardown_world_select)
        .add_systems(Update, (move_world_select_cam, cycle_selected_world, select_world).run_if(in_state(AppState::WorldSelect)))
        //overworld
        .add_systems(OnEnter(AppState::Overworld), spawn_overworld_stage)
        .add_systems(OnExit(AppState::Overworld), teardown_overworld)
        //.add_systems(Update, ().run_if(in_state(AppState::Overworld)))
        //in stage
        .add_systems(OnEnter(AppState::InStage), build_stage)
        .run();
}


fn lighting(
    mut commands: Commands
) {
    let mut t = Transform::from_rotation(Quat::from_rotation_x(-PI / 4.0)); 
    t.rotate_y(FRAC_PI_2);
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 1000.0,
            ..default() 
        },
        t
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
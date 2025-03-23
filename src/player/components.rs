
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerController {
    pub left_key: KeyCode,
    pub right_key: KeyCode,
    pub forwards_key: KeyCode,
    pub backwards_key: KeyCode,
    pub force: f32,
    pub friction_c: f32,
    pub min_friction_force: f32
}

impl Default for PlayerController {
    fn default() -> Self {
        return Self {
            left_key: KeyCode::KeyA,
            right_key: KeyCode::KeyD,
            forwards_key: KeyCode::KeyW,
            backwards_key: KeyCode::KeyS,
            force: 15.0,
            friction_c: 0.05,
            min_friction_force: 7.0
        }
    }
}

#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct JumpController {
    pub key: KeyCode,
    pub initial_force: f32,
    pub max_force: f32,
    pub current_force: f32,
    pub force_deceleration: f32,
    pub timer: Timer
}

impl Default for JumpController {
    fn default() -> Self {
        Self { 
            key: KeyCode::Space,
            initial_force: 15.0,
            max_force: 100.0,
            current_force: 20.0,
            force_deceleration: 150.0,
            timer: Timer::from_seconds(0.5, TimerMode::Once)
        }
    }
}
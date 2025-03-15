
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
    pub friction_c: f32
}

impl Default for PlayerController {
    fn default() -> Self {
        return Self {
            left_key: KeyCode::KeyA,
            right_key: KeyCode::KeyD,
            forwards_key: KeyCode::KeyW,
            backwards_key: KeyCode::KeyS,
            force: 10.0,
            friction_c: 0.2
        }
    }
}
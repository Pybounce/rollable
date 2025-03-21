
use bevy::{core_pipeline::prepass::{DepthPrepass, NormalPrepass}, input::mouse::MouseMotion, prelude::*};
use post_processing::PostProcessSettings;

use crate::player::components::Player;

pub mod post_processing;

#[derive(Component)]
pub struct CameraController {
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub x_speed: f32,
    pub y_speed: f32,
    pub distance: f32
}

impl Default for CameraController {
    fn default() -> Self {
        Self { 
            min_pitch: -1.0, 
            max_pitch: 0.0, 
            x_speed: 0.0015, 
            y_speed: 0.0015, 
            distance: 20.0
        }
    }
}

pub fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((
        Camera3d::default(),
        PerspectiveProjection {
            //near: 2.0,
            //far: 10000.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 10.0, 10.0)),
        CameraController::default(),
        PostProcessSettings {
            intensity: 0.02,
            ..default()
        },
        DepthPrepass,
        NormalPrepass,
        Msaa::Off,
    ));
}

pub fn move_camera(
    mut camera_query: Query<(&mut Transform, &CameraController), (With<Camera3d>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    mut mouse_motion_events: EventReader<MouseMotion>
) {
    let (mut ct, cam_con) = camera_query.single_mut();
    let pt = player_query.get_single();
    match pt {
        Ok(pt) => {
            for event in mouse_motion_events.read() {
                let (yaw, pitch, roll) = ct.rotation.to_euler(EulerRot::YXZ);
                ct.rotation = Quat::from_euler(EulerRot::YXZ, 
                    yaw - event.delta.x * cam_con.x_speed, 
                    (pitch - (event.delta.y * cam_con.y_speed)).clamp(cam_con.min_pitch, cam_con.max_pitch), 
                    roll
                );
            }
            ct.translation = pt.translation - ct.forward() * cam_con.distance;
        }
        Err(_) => (),
    }
}
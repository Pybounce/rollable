
use bevy::prelude::*;

use crate::player::components::Player;

pub fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 20.0))
    ));
}

pub fn move_camera(
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>
) {
    let mut ct = camera_query.single_mut();
    let pt = player_query.get_single();
    match pt {
        Ok(pt) => {
            let range = 5.0;
            let distance = (ct.translation.distance(pt.translation) - range).max(0.0);

            let speed = distance * 2.5;
            let dir = (pt.translation - ct.translation).normalize_or_zero();

            let delta = time.delta_secs() * speed * dir;
            ct.translation += delta;
            ct.look_at(pt.translation, Vec3::Y);
        }
        Err(_) => (),
    }
}




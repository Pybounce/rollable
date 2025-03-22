
use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use super::components::OffsetMover;

pub fn move_offset_movers(
    mut query: Query<(&mut OffsetMover, &mut LinearVelocity)>,
    time: Res<Time>
) {
    for (mut offset_mover, mut linvel) in &mut query {
        //transform.translation -= offset_mover.current_offset();
        linvel.0 -= offset_mover.current_velocity();
        offset_mover.progress(time.delta_secs());
        linvel.0 += offset_mover.current_velocity();
        //transform.translation += offset_mover.current_offset();
    }
}